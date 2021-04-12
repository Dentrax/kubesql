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

use crate::planner::Query;
use anyhow::{Context, Result};
use k8s_openapi::api::apps::v1::Deployment;
use k8s_openapi::api::core::v1::Pod;
use kube::Api;
use std::convert::TryFrom;

/// A high level wrapper for kube::Api struct
pub struct ApiBuilder<'a> {
    /// The name of the kubeconfig context to use
    context: Option<String>,

    /// Show only from this namespace
    namespace: Option<String>,

    //queries: &'a Vec<Query>,
    queries: Option<&'a [Query]>,

    /// Api::Namespaced Deployment value
    deployment: Option<Api<Deployment>>,

    // Api::Namespaced Pod value
    pod: Option<Api<Pod>>,
}

impl<'a> Default for ApiBuilder<'a> {
    fn default() -> Self {
        ApiBuilder {
            context: None,
            namespace: Option::from("default".to_string()),
            queries: None,
            deployment: None,
            pod: None,
        }
    }
}

impl<'a> ApiBuilder<'a> {
    /// Creates a new API and returns default states
    pub fn new() -> ApiBuilder<'a> {
        ApiBuilder::default()
    }

    /// Creates a new builder-style object to manufacture a `API`
    pub fn builder() -> ApiBuilder<'a> {
        ApiBuilder::new()
    }

    /// Set the given context
    pub fn context(mut self, ctx: String) -> ApiBuilder<'a> {
        self.context = Option::from(ctx);
        self
    }

    /// Set the given namespace
    pub fn namespace(mut self, ns: String) -> ApiBuilder<'a> {
        self.namespace = Option::from(ns);
        self
    }

    pub fn queries(mut self, queries: &'a [Query]) -> ApiBuilder<'a> {
        self.queries = Option::from(queries);
        self
    }

    /// Try build the whole API or throw a panic
    pub(crate) async fn build(mut self) -> Result<ApiBuilder<'a>> {
        let client_config = kube::Config::from_kubeconfig(&kube::config::KubeConfigOptions {
            context: Some(self.context.clone().unwrap()),
            ..Default::default()
        })
        .await?;

        match kube::Client::try_from(client_config)
            .with_context(|| "failed to create the kube client with context".to_string())
        {
            Ok(c) => {
                self.deployment = Option::from(Api::namespaced(
                    c.clone(),
                    self.namespace.clone().unwrap().as_str(),
                ));
                self.pod =
                    Option::from(Api::namespaced(c, self.namespace.clone().unwrap().as_str()));
            }
            Err(e) => {
                panic!("an error occurred during creating kube client: {:?}", e)
            }
        }

        Ok(self)
    }

    pub fn get_context(&'a self) -> &String {
        self.context.as_ref().unwrap()
    }

    pub fn get_namespace(&'a self) -> &String {
        self.namespace.as_ref().unwrap()
    }

    pub fn get_deployment(&'a self) -> &Api<Deployment> {
        self.deployment.as_ref().unwrap()
    }

    pub fn get_pod(&'a self) -> &Api<Pod> {
        self.pod.as_ref().unwrap()
    }
}
