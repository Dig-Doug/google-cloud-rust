// Copyright 2025 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// Code generated by sidekick. DO NOT EDIT.
#![allow(rustdoc::redundant_explicit_links)]
#![allow(rustdoc::broken_intra_doc_links)]

use crate::Result;
use std::sync::Arc;

/// Implements a client for the Cloud Monitoring API.
///
/// # Service Description
///
/// Manages Cloud Monitoring Metrics Scopes, and the monitoring of Google Cloud
/// projects and AWS accounts.
///
/// # Configuration
///
/// `MetricsScopes` has various configuration parameters, the defaults should
/// work with most applications.
///
/// # Pooling and Cloning
///
/// `MetricsScopes` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `MetricsScopes` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct MetricsScopes {
    inner: Arc<dyn crate::stubs::dynamic::MetricsScopes>,
}

impl MetricsScopes {
    /// Creates a new client with the default configuration.
    pub async fn new() -> Result<Self> {
        Self::new_with_config(gax::options::ClientConfig::default()).await
    }

    /// Creates a new client with the specified configuration.
    pub async fn new_with_config(conf: gax::options::ClientConfig) -> Result<Self> {
        let inner = Self::build_inner(conf).await?;
        Ok(Self { inner })
    }

    /// Creates a new client from the provided stub.
    ///
    /// The most common case for calling this function is when mocking the
    /// client.
    pub fn from_stub<T>(stub: T) -> Self
    where
        T: crate::stubs::MetricsScopes + 'static,
    {
        Self {
            inner: Arc::new(stub),
        }
    }

    async fn build_inner(
        conf: gax::options::ClientConfig,
    ) -> Result<Arc<dyn crate::stubs::dynamic::MetricsScopes>> {
        if conf.tracing_enabled() {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gax::options::ClientConfig,
    ) -> Result<impl crate::stubs::MetricsScopes> {
        crate::transport::MetricsScopes::new(conf).await
    }

    async fn build_with_tracing(
        conf: gax::options::ClientConfig,
    ) -> Result<impl crate::stubs::MetricsScopes> {
        Self::build_transport(conf)
            .await
            .map(crate::tracing::MetricsScopes::new)
    }

    /// Returns a specific `Metrics Scope`.
    pub fn get_metrics_scope(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::metrics_scopes::GetMetricsScope {
        crate::builders::metrics_scopes::GetMetricsScope::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Returns a list of every `Metrics Scope` that a specific `MonitoredProject`
    /// has been added to. The metrics scope representing the specified monitored
    /// project will always be the first entry in the response.
    pub fn list_metrics_scopes_by_monitored_project(
        &self,
    ) -> crate::builders::metrics_scopes::ListMetricsScopesByMonitoredProject {
        crate::builders::metrics_scopes::ListMetricsScopesByMonitoredProject::new(
            self.inner.clone(),
        )
    }

    /// Adds a `MonitoredProject` with the given project ID
    /// to the specified `Metrics Scope`.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn create_monitored_project(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::metrics_scopes::CreateMonitoredProject {
        crate::builders::metrics_scopes::CreateMonitoredProject::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Deletes a `MonitoredProject` from the specified `Metrics Scope`.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn delete_monitored_project(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::metrics_scopes::DeleteMonitoredProject {
        crate::builders::metrics_scopes::DeleteMonitoredProject::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn get_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::metrics_scopes::GetOperation {
        crate::builders::metrics_scopes::GetOperation::new(self.inner.clone()).set_name(name.into())
    }
}
