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

mod api_builder;
mod parser;
mod planner;
mod printer;
mod validator;

use crate::api_builder::ApiBuilder;
use crate::parser::ResourceType;
use crate::printer::Printer;
use anyhow::Result;
use clap::{App, Arg};
use kube::api::ListParams;
use sqlparser::ast::BinaryOperator;
use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<()> {
    let matches = App::new("kubesql")
        .version("0.1.0")
        .author("Dentrax <furkan.turkal@hotmail.com>")
        .about("kubesql is an experimental tool for querying your Kubernetes API Server using SQL")
        .arg(
            Arg::with_name("query")
                .short("q")
                .long("query")
                .multiple(false)
                .overrides_with("file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .multiple(false)
                .overrides_with("query")
                .value_name("FILE")
                .takes_value(true),
        )
        .get_matches();

    let sql = if matches.is_present("query") {
        matches.value_of("query").unwrap().to_string()
    } else if matches.is_present("file") {
        let v = matches.value_of("file").unwrap();
        let mut file = File::open(v).expect("Unable to open the query file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Unable to read the query file");
        contents
    } else {
        panic!("Either --query or --file required")
    };

    let api_queries = parser::parse_sql(&sql);

    let mut apis: Vec<ApiBuilder> = Vec::new();

    let kubeconfig = parser::parse_kubeconfig();

    validator::validate_contexts(kubeconfig, &api_queries.contexts);

    for ctx in &api_queries.contexts {
        for ns in &api_queries.namespaces {
            apis.push(
                ApiBuilder::builder()
                    .context(ctx.clone())
                    .namespace(ns.clone())
                    .queries(api_queries.queries.as_slice())
                    .build()
                    .await?,
            )
        }
    }

    let mut printer = Printer::builder()
        .contexts(&api_queries.contexts)
        .namespaces(&api_queries.namespaces)
        .queries(api_queries.queries.as_slice());

    //1. Query { key: None, kind: "pod", field1: "status", field2: "phase", eq: "Running", op: Eq }
    //2. Query { key: Some(And), kind: "deployment", field1: "metadata", field2: "name", eq: "my-awesome-deployment", op: Eq }
    for q in api_queries.queries.clone() {
        // a.k.a '--field-selector': https://v1-18.docs.kubernetes.io/docs/concepts/overview/working-with-objects/field-selectors/
        let list_params =
            ListParams::default().fields(&format!("{}.{}={}", q.field1, q.field2, q.eq));

        for api in &apis {
            let mut found: bool = false;
            match parser::ResourceType::from_str(&q.kind.to_lowercase()).unwrap() {
                ResourceType::Deployment => {
                    let o = api.get_deployment().list(&list_params).await?;
                    if !o.items.is_empty() {
                        printer.insert_deployments(api.get_context(), api.get_namespace(), o);
                        found = true;
                    }
                }
                ResourceType::Pod => {
                    let o = api.get_pod().list(&list_params).await?;
                    if !o.items.is_empty() {
                        printer.insert_pods(api.get_context(), api.get_namespace(), o);
                        found = true;
                    }
                }
            }

            // we will decide according to given operator, in case if resource not found or empty
            if !found {
                if let Some(k) = &q.key {
                    if *k == BinaryOperator::And {
                        panic!(
                            "No resource found: 'kubectl get {} --field-selector={}.{}={}'",
                            q.kind, q.field1, q.field2, q.eq
                        );
                    }
                }
            }
        }
    }

    printer.print();

    Ok(())
}
