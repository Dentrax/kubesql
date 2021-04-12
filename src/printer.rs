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

use crate::parser::ResourceType;
use crate::planner::Query;
use k8s_openapi::api::apps::v1::Deployment;
use k8s_openapi::api::core::v1::Pod;
use kube::api::ObjectList;
use kube::Resource;
use prettytable::{Cell, Row, Table};
use std::collections::HashMap;

#[derive(Debug)]
pub struct PrintItem<'a> {
    pub context: &'a str,
    pub namespace: &'a str,
    pub kind: ResourceType,
    pub value: String,
}

#[derive(Debug)]
pub struct Printer<'a> {
    //items: Option<&'a Vec<PrintItem>>,
    items: Vec<PrintItem<'a>>,
    contexts: Option<&'a [String]>,
    namespaces: Option<&'a [String]>,
    queries: Option<&'a [Query]>,
    a: Option<&'a String>,
}

impl<'a> Default for Printer<'a> {
    fn default() -> Self {
        Self {
            items: vec![],
            contexts: None,
            namespaces: None,
            queries: None,
            a: None,
        }
    }
}

impl<'a> Printer<'a> {
    pub fn new() -> Printer<'a> {
        Printer::default()
    }

    pub fn builder() -> Printer<'a> {
        Printer::new()
    }

    /// Set the given context
    pub fn contexts(mut self, ctx: &'a [String]) -> Printer<'a> {
        self.contexts = Option::from(ctx);
        self
    }

    /// Set the given namespace
    pub fn namespaces(mut self, ns: &'a [String]) -> Printer<'a> {
        self.namespaces = Option::from(ns);
        self
    }

    /// Set the given namespace
    pub fn queries(mut self, queries: &'a [Query]) -> Printer<'a> {
        self.queries = Option::from(queries);
        self
    }

    pub fn insert_deployments(
        &mut self,
        ctx: &'a str,
        ns: &'a str,
        objects: ObjectList<Deployment>,
    ) {
        let v = objects
            .items
            .into_iter()
            .map(|x| x.name())
            .collect::<Vec<String>>();
        self.items.push(PrintItem {
            context: ctx,
            namespace: ns,
            kind: ResourceType::Deployment,
            value: v.join("\n"),
        })
    }

    pub fn insert_pods(&mut self, ctx: &'a str, ns: &'a str, objects: ObjectList<Pod>) {
        let v = objects
            .items
            .into_iter()
            .map(|x| x.name())
            .collect::<Vec<String>>();
        self.items.push(PrintItem {
            context: ctx,
            namespace: ns,
            kind: ResourceType::Pod,
            value: v.join("\n"),
        });
    }

    pub fn print(self) {
        // 1. Creating tables for all given contexts

        // Represents 'Context - Table' mapping
        let mut table_context_pods: HashMap<String, Table> = HashMap::new();
        let mut table_context_deployments: HashMap<String, Table> = HashMap::new();

        let should_append_pod: bool = self
            .queries
            .unwrap()
            .iter()
            .any(|x| x.kind.eq_ignore_ascii_case(&*ResourceType::Pod.to_string()));
        let should_append_deployment: bool = self.queries.unwrap().iter().any(|x| {
            x.kind
                .eq_ignore_ascii_case(&*ResourceType::Deployment.to_string())
        });

        // 2. Initialize the all contexts
        for context in self.contexts.unwrap() {
            let mut table_ctx = Table::new();
            let cells = self
                .namespaces
                .unwrap()
                .iter()
                .map(|x| Cell::new(x))
                .collect::<Vec<Cell>>();
            table_ctx.add_row(Row::new(cells));

            let mut table_ctx_pods = table_ctx.clone();
            let mut table_ctx_deployments = table_ctx.clone();

            let mut cells_pods: Vec<Cell> = Vec::new();
            let mut cells_deployments: Vec<Cell> = Vec::new();

            for ns in self.namespaces.unwrap() {
                if should_append_pod {
                    let pods = self
                        .items
                        .iter()
                        .filter(|f| {
                            f.kind == ResourceType::Pod
                                && *f.context == *context
                                && *f.namespace == *ns
                        })
                        .map(|m| m.value.clone())
                        .collect::<String>();
                    if !pods.is_empty() {
                        cells_pods.push(Cell::new(&*pods));
                    } else {
                        cells_pods.push(Cell::new("-"));
                    }
                }

                if should_append_deployment {
                    let deployments = self
                        .items
                        .iter()
                        .filter(|f| {
                            f.kind == ResourceType::Deployment
                                && *f.context == *context
                                && *f.namespace == *ns
                        })
                        .map(|m| m.value.clone())
                        .collect::<String>();
                    if !deployments.is_empty() {
                        cells_deployments.push(Cell::new(&*deployments));
                    } else {
                        cells_deployments.push(Cell::new("-"));
                    }
                }
            }

            table_ctx_pods.add_row(Row::new(cells_pods));
            table_ctx_deployments.add_row(Row::new(cells_deployments));

            table_context_pods.insert(context.clone(), table_ctx_pods);
            table_context_deployments.insert(context.clone(), table_ctx_deployments);
        }

        let mut row: Vec<Row> = vec![];

        let mut cs = self
            .contexts
            .unwrap()
            .iter()
            .map(|x| Cell::new(x.as_str()))
            .collect::<Vec<Cell>>();
        cs.insert(0, Cell::new("KIND / CONTEXT"));
        row.push(Row::new(cs));

        if should_append_pod {
            let mut rows_pod: Row = table_context_pods
                .iter()
                .map(|x| Cell::from(x.1))
                .collect::<Row>();
            rows_pod.insert_cell(0, Cell::new("pod"));
            row.push(rows_pod);
        }

        if should_append_deployment {
            let mut rows_deployment: Row = table_context_deployments
                .iter()
                .map(|x| Cell::from(x.1))
                .collect::<Row>();
            rows_deployment.insert_cell(0, Cell::new("deployment"));
            row.push(rows_deployment);
        }

        Table::init(row).printstd();
    }
}
