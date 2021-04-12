// Copyright (c) 2021 Dentrax
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of
// this software and associated documentation files (the "Software"), to deal in
// the Software without restriction, including without limitation the rights to
// use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software is furnished to do so,
// subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
// FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
// COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
// IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use crate::planner;
use crate::planner::{Object, Query};
use kube::config::Kubeconfig;
use sqlparser::ast::{SelectItem, SetExpr, Statement, TableFactor};
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;
use std::fmt;
use std::fmt::Formatter;
use std::str::FromStr;

#[derive(Debug)]
pub struct ApiQueries {
    pub namespaces: Vec<String>,
    pub contexts: Vec<String>,
    pub queries: Vec<Query>,
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum ResourceType {
    Deployment,
    Pod,
}

impl fmt::Display for ResourceType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ResourceType::Deployment => write!(f, "deployment"),
            ResourceType::Pod => write!(f, "pod"),
        }
    }
}

impl FromStr for ResourceType {
    type Err = ();

    fn from_str(input: &str) -> Result<ResourceType, Self::Err> {
        match input {
            "deployment" => Ok(ResourceType::Deployment),
            "pod" => Ok(ResourceType::Pod),
            _ => panic!("Unexpected ResourceType for {}", input),
        }
    }
}

pub(crate) fn parse_sql(sql: &str) -> ApiQueries {
    let dialect = GenericDialect {};

    // `-` is an incorrect char for SQL Queries, so we need to replace with another char
    // We will undo this replace during parsing stage
    let sql_replace = sql.replace("-", "_");

    // Parse the given SQL to AST
    let mut ast = Parser::parse_sql(&dialect, &sql_replace).unwrap();

    let query = match ast.pop().unwrap() {
        Statement::Query(query) => query,
        _ => {
            panic!("Only QUERY statements are supported!");
        }
    };

    let mut queries = ApiQueries {
        namespaces: vec![],
        contexts: vec![],
        queries: vec![],
    };

    match query.body {
        SetExpr::Select(s) => {
            if s.projection.is_empty() {
                panic!("SELECT statement is required to call the given namespace(s)!")
            }

            // SELECT ...
            for p in s.projection {
                match p {
                    SelectItem::UnnamedExpr(o) => {
                        queries.namespaces.push(o.to_string().replace("_", "-"));
                    }
                    SelectItem::ExprWithAlias { .. } => {
                        panic!("SELECT statement does not support ExprWithAlias selector!")
                    }
                    SelectItem::QualifiedWildcard(_) => {
                        panic!("SELECT statement does not support QualifiedWildcard selector!")
                    }
                    SelectItem::Wildcard => {
                        panic!("SELECT statement does not support Widcard selector!")
                    }
                }
            }

            if s.from.is_empty() {
                panic!("FROM statement is required to call the given context(s)!")
            }

            // FROM ...
            for f in s.from {
                if !f.joins.is_empty() {
                    panic!("FROM statement does not support Join!")
                }
                match f.relation {
                    TableFactor::Table {
                        name,
                        alias,
                        args,
                        with_hints,
                        ..
                    } => {
                        if alias.is_some() {
                            panic!("FROM statement does not support Table aliases!")
                        }
                        if !args.is_empty() {
                            panic!("FROM statement does not support Table ARGS!")
                        }
                        if !with_hints.is_empty() {
                            panic!("FROM statement does not support Table HINT!")
                        }
                        queries.contexts.push(name.to_string().replace("_", "-"));
                    }
                    TableFactor::Derived { .. } => {
                        panic!("FROM statement does not support Derived!")
                    }
                    TableFactor::TableFunction { .. } => {
                        panic!("FROM statement does not support TableFunction!")
                    }
                    TableFactor::NestedJoin(_) => {
                        panic!("FROM statement does not support NestedJoin!")
                    }
                }
            }

            // WHERE
            if let Some(w) = s.selection {
                let plan = planner::plan_expr(w);
                match plan {
                    Object::Queries(q) => queries.queries = q,
                    Object::Query(q) => queries.queries.push(q),
                    _ => {
                        panic!("Unable to handle unsupported query plan: {:?}", plan)
                    }
                }
            } else {
                panic!("WHERE statement is required in order to set --field-selector!")
            }
        }
        _ => {
            panic!("An unsupported query body given: {:?}", query.body)
        }
    }

    queries
}

pub(crate) fn parse_kubeconfig() -> Kubeconfig {
    kube::config::Kubeconfig::read()
        .unwrap_or_else(|err| panic!("Could not read KUBECONFIG: {:?}", err))
}
