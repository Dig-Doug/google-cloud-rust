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

/// Implements a client for the Cloud Run Admin API.
///
/// # Service Description
///
/// Cloud Run Build Control Plane API
///
/// # Configuration
///
/// `Builds` has various configuration parameters, the defaults should
/// work with most applications.
///
/// # Pooling and Cloning
///
/// `Builds` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `Builds` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct Builds {
    inner: Arc<dyn crate::stubs::dynamic::Builds>,
}

impl Builds {
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
        T: crate::stubs::Builds + 'static,
    {
        Self {
            inner: Arc::new(stub),
        }
    }

    async fn build_inner(
        conf: gax::options::ClientConfig,
    ) -> Result<Arc<dyn crate::stubs::dynamic::Builds>> {
        if conf.tracing_enabled() {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gax::options::ClientConfig,
    ) -> Result<impl crate::stubs::Builds> {
        crate::transport::Builds::new(conf).await
    }

    async fn build_with_tracing(
        conf: gax::options::ClientConfig,
    ) -> Result<impl crate::stubs::Builds> {
        Self::build_transport(conf)
            .await
            .map(crate::tracing::Builds::new)
    }

    /// Submits a build in a given project.
    pub fn submit_build(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::builds::SubmitBuild {
        crate::builders::builds::SubmitBuild::new(self.inner.clone()).set_parent(parent.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn list_operations(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::builds::ListOperations {
        crate::builders::builds::ListOperations::new(self.inner.clone()).set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn get_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::builds::GetOperation {
        crate::builders::builds::GetOperation::new(self.inner.clone()).set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn delete_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::builds::DeleteOperation {
        crate::builders::builds::DeleteOperation::new(self.inner.clone()).set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn wait_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::builds::WaitOperation {
        crate::builders::builds::WaitOperation::new(self.inner.clone()).set_name(name.into())
    }
}

/// Implements a client for the Cloud Run Admin API.
///
/// # Service Description
///
/// Cloud Run Execution Control Plane API.
///
/// # Configuration
///
/// `Executions` has various configuration parameters, the defaults should
/// work with most applications.
///
/// # Pooling and Cloning
///
/// `Executions` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `Executions` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct Executions {
    inner: Arc<dyn crate::stubs::dynamic::Executions>,
}

impl Executions {
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
        T: crate::stubs::Executions + 'static,
    {
        Self {
            inner: Arc::new(stub),
        }
    }

    async fn build_inner(
        conf: gax::options::ClientConfig,
    ) -> Result<Arc<dyn crate::stubs::dynamic::Executions>> {
        if conf.tracing_enabled() {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gax::options::ClientConfig,
    ) -> Result<impl crate::stubs::Executions> {
        crate::transport::Executions::new(conf).await
    }

    async fn build_with_tracing(
        conf: gax::options::ClientConfig,
    ) -> Result<impl crate::stubs::Executions> {
        Self::build_transport(conf)
            .await
            .map(crate::tracing::Executions::new)
    }

    /// Gets information about an Execution.
    pub fn get_execution(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::executions::GetExecution {
        crate::builders::executions::GetExecution::new(self.inner.clone()).set_name(name.into())
    }

    /// Lists Executions from a Job. Results are sorted by creation time,
    /// descending.
    pub fn list_executions(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::executions::ListExecutions {
        crate::builders::executions::ListExecutions::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Deletes an Execution.
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
    pub fn delete_execution(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::executions::DeleteExecution {
        crate::builders::executions::DeleteExecution::new(self.inner.clone()).set_name(name.into())
    }

    /// Cancels an Execution.
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
    pub fn cancel_execution(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::executions::CancelExecution {
        crate::builders::executions::CancelExecution::new(self.inner.clone()).set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn list_operations(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::executions::ListOperations {
        crate::builders::executions::ListOperations::new(self.inner.clone()).set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn get_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::executions::GetOperation {
        crate::builders::executions::GetOperation::new(self.inner.clone()).set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn delete_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::executions::DeleteOperation {
        crate::builders::executions::DeleteOperation::new(self.inner.clone()).set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn wait_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::executions::WaitOperation {
        crate::builders::executions::WaitOperation::new(self.inner.clone()).set_name(name.into())
    }
}

/// Implements a client for the Cloud Run Admin API.
///
/// # Service Description
///
/// Cloud Run Job Control Plane API.
///
/// # Configuration
///
/// `Jobs` has various configuration parameters, the defaults should
/// work with most applications.
///
/// # Pooling and Cloning
///
/// `Jobs` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `Jobs` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct Jobs {
    inner: Arc<dyn crate::stubs::dynamic::Jobs>,
}

impl Jobs {
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
        T: crate::stubs::Jobs + 'static,
    {
        Self {
            inner: Arc::new(stub),
        }
    }

    async fn build_inner(
        conf: gax::options::ClientConfig,
    ) -> Result<Arc<dyn crate::stubs::dynamic::Jobs>> {
        if conf.tracing_enabled() {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(conf: gax::options::ClientConfig) -> Result<impl crate::stubs::Jobs> {
        crate::transport::Jobs::new(conf).await
    }

    async fn build_with_tracing(
        conf: gax::options::ClientConfig,
    ) -> Result<impl crate::stubs::Jobs> {
        Self::build_transport(conf)
            .await
            .map(crate::tracing::Jobs::new)
    }

    /// Creates a Job.
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
    pub fn create_job(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::jobs::CreateJob {
        crate::builders::jobs::CreateJob::new(self.inner.clone()).set_parent(parent.into())
    }

    /// Gets information about a Job.
    pub fn get_job(&self, name: impl Into<std::string::String>) -> crate::builders::jobs::GetJob {
        crate::builders::jobs::GetJob::new(self.inner.clone()).set_name(name.into())
    }

    /// Lists Jobs. Results are sorted by creation time, descending.
    pub fn list_jobs(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::jobs::ListJobs {
        crate::builders::jobs::ListJobs::new(self.inner.clone()).set_parent(parent.into())
    }

    /// Updates a Job.
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
    pub fn update_job(
        &self,
        job: impl Into<crate::model::Job>,
    ) -> crate::builders::jobs::UpdateJob {
        crate::builders::jobs::UpdateJob::new(self.inner.clone()).set_job(job.into())
    }

    /// Deletes a Job.
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
    pub fn delete_job(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::jobs::DeleteJob {
        crate::builders::jobs::DeleteJob::new(self.inner.clone()).set_name(name.into())
    }

    /// Triggers creation of a new Execution of this Job.
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
    pub fn run_job(&self, name: impl Into<std::string::String>) -> crate::builders::jobs::RunJob {
        crate::builders::jobs::RunJob::new(self.inner.clone()).set_name(name.into())
    }

    /// Gets the IAM Access Control policy currently in effect for the given Job.
    /// This result does not include any inherited policies.
    pub fn get_iam_policy(
        &self,
        resource: impl Into<std::string::String>,
    ) -> crate::builders::jobs::GetIamPolicy {
        crate::builders::jobs::GetIamPolicy::new(self.inner.clone()).set_resource(resource.into())
    }

    /// Sets the IAM Access control policy for the specified Job. Overwrites
    /// any existing policy.
    pub fn set_iam_policy(
        &self,
        resource: impl Into<std::string::String>,
    ) -> crate::builders::jobs::SetIamPolicy {
        crate::builders::jobs::SetIamPolicy::new(self.inner.clone()).set_resource(resource.into())
    }

    /// Returns permissions that a caller has on the specified Project.
    ///
    /// There are no permissions required for making this API call.
    pub fn test_iam_permissions(
        &self,
        resource: impl Into<std::string::String>,
    ) -> crate::builders::jobs::TestIamPermissions {
        crate::builders::jobs::TestIamPermissions::new(self.inner.clone())
            .set_resource(resource.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn list_operations(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::jobs::ListOperations {
        crate::builders::jobs::ListOperations::new(self.inner.clone()).set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn get_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::jobs::GetOperation {
        crate::builders::jobs::GetOperation::new(self.inner.clone()).set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn delete_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::jobs::DeleteOperation {
        crate::builders::jobs::DeleteOperation::new(self.inner.clone()).set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn wait_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::jobs::WaitOperation {
        crate::builders::jobs::WaitOperation::new(self.inner.clone()).set_name(name.into())
    }
}

/// Implements a client for the Cloud Run Admin API.
///
/// # Service Description
///
/// Cloud Run Revision Control Plane API.
///
/// # Configuration
///
/// `Revisions` has various configuration parameters, the defaults should
/// work with most applications.
///
/// # Pooling and Cloning
///
/// `Revisions` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `Revisions` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct Revisions {
    inner: Arc<dyn crate::stubs::dynamic::Revisions>,
}

impl Revisions {
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
        T: crate::stubs::Revisions + 'static,
    {
        Self {
            inner: Arc::new(stub),
        }
    }

    async fn build_inner(
        conf: gax::options::ClientConfig,
    ) -> Result<Arc<dyn crate::stubs::dynamic::Revisions>> {
        if conf.tracing_enabled() {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gax::options::ClientConfig,
    ) -> Result<impl crate::stubs::Revisions> {
        crate::transport::Revisions::new(conf).await
    }

    async fn build_with_tracing(
        conf: gax::options::ClientConfig,
    ) -> Result<impl crate::stubs::Revisions> {
        Self::build_transport(conf)
            .await
            .map(crate::tracing::Revisions::new)
    }

    /// Gets information about a Revision.
    pub fn get_revision(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::revisions::GetRevision {
        crate::builders::revisions::GetRevision::new(self.inner.clone()).set_name(name.into())
    }

    /// Lists Revisions from a given Service, or from a given location.  Results
    /// are sorted by creation time, descending.
    pub fn list_revisions(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::revisions::ListRevisions {
        crate::builders::revisions::ListRevisions::new(self.inner.clone()).set_parent(parent.into())
    }

    /// Deletes a Revision.
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
    pub fn delete_revision(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::revisions::DeleteRevision {
        crate::builders::revisions::DeleteRevision::new(self.inner.clone()).set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn list_operations(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::revisions::ListOperations {
        crate::builders::revisions::ListOperations::new(self.inner.clone()).set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn get_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::revisions::GetOperation {
        crate::builders::revisions::GetOperation::new(self.inner.clone()).set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn delete_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::revisions::DeleteOperation {
        crate::builders::revisions::DeleteOperation::new(self.inner.clone()).set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn wait_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::revisions::WaitOperation {
        crate::builders::revisions::WaitOperation::new(self.inner.clone()).set_name(name.into())
    }
}

/// Implements a client for the Cloud Run Admin API.
///
/// # Service Description
///
/// Cloud Run Service Control Plane API
///
/// # Configuration
///
/// `Services` has various configuration parameters, the defaults should
/// work with most applications.
///
/// # Pooling and Cloning
///
/// `Services` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `Services` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct Services {
    inner: Arc<dyn crate::stubs::dynamic::Services>,
}

impl Services {
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
        T: crate::stubs::Services + 'static,
    {
        Self {
            inner: Arc::new(stub),
        }
    }

    async fn build_inner(
        conf: gax::options::ClientConfig,
    ) -> Result<Arc<dyn crate::stubs::dynamic::Services>> {
        if conf.tracing_enabled() {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gax::options::ClientConfig,
    ) -> Result<impl crate::stubs::Services> {
        crate::transport::Services::new(conf).await
    }

    async fn build_with_tracing(
        conf: gax::options::ClientConfig,
    ) -> Result<impl crate::stubs::Services> {
        Self::build_transport(conf)
            .await
            .map(crate::tracing::Services::new)
    }

    /// Creates a new Service in a given project and location.
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
    pub fn create_service(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::services::CreateService {
        crate::builders::services::CreateService::new(self.inner.clone()).set_parent(parent.into())
    }

    /// Gets information about a Service.
    pub fn get_service(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::services::GetService {
        crate::builders::services::GetService::new(self.inner.clone()).set_name(name.into())
    }

    /// Lists Services. Results are sorted by creation time, descending.
    pub fn list_services(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::services::ListServices {
        crate::builders::services::ListServices::new(self.inner.clone()).set_parent(parent.into())
    }

    /// Updates a Service.
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
    pub fn update_service(
        &self,
        service: impl Into<crate::model::Service>,
    ) -> crate::builders::services::UpdateService {
        crate::builders::services::UpdateService::new(self.inner.clone())
            .set_service(service.into())
    }

    /// Deletes a Service.
    /// This will cause the Service to stop serving traffic and will delete all
    /// revisions.
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
    pub fn delete_service(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::services::DeleteService {
        crate::builders::services::DeleteService::new(self.inner.clone()).set_name(name.into())
    }

    /// Gets the IAM Access Control policy currently in effect for the given
    /// Cloud Run Service. This result does not include any inherited policies.
    pub fn get_iam_policy(
        &self,
        resource: impl Into<std::string::String>,
    ) -> crate::builders::services::GetIamPolicy {
        crate::builders::services::GetIamPolicy::new(self.inner.clone())
            .set_resource(resource.into())
    }

    /// Sets the IAM Access control policy for the specified Service. Overwrites
    /// any existing policy.
    pub fn set_iam_policy(
        &self,
        resource: impl Into<std::string::String>,
    ) -> crate::builders::services::SetIamPolicy {
        crate::builders::services::SetIamPolicy::new(self.inner.clone())
            .set_resource(resource.into())
    }

    /// Returns permissions that a caller has on the specified Project.
    ///
    /// There are no permissions required for making this API call.
    pub fn test_iam_permissions(
        &self,
        resource: impl Into<std::string::String>,
    ) -> crate::builders::services::TestIamPermissions {
        crate::builders::services::TestIamPermissions::new(self.inner.clone())
            .set_resource(resource.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn list_operations(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::services::ListOperations {
        crate::builders::services::ListOperations::new(self.inner.clone()).set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn get_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::services::GetOperation {
        crate::builders::services::GetOperation::new(self.inner.clone()).set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn delete_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::services::DeleteOperation {
        crate::builders::services::DeleteOperation::new(self.inner.clone()).set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn wait_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::services::WaitOperation {
        crate::builders::services::WaitOperation::new(self.inner.clone()).set_name(name.into())
    }
}

/// Implements a client for the Cloud Run Admin API.
///
/// # Service Description
///
/// Cloud Run Task Control Plane API.
///
/// # Configuration
///
/// `Tasks` has various configuration parameters, the defaults should
/// work with most applications.
///
/// # Pooling and Cloning
///
/// `Tasks` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `Tasks` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct Tasks {
    inner: Arc<dyn crate::stubs::dynamic::Tasks>,
}

impl Tasks {
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
        T: crate::stubs::Tasks + 'static,
    {
        Self {
            inner: Arc::new(stub),
        }
    }

    async fn build_inner(
        conf: gax::options::ClientConfig,
    ) -> Result<Arc<dyn crate::stubs::dynamic::Tasks>> {
        if conf.tracing_enabled() {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(conf: gax::options::ClientConfig) -> Result<impl crate::stubs::Tasks> {
        crate::transport::Tasks::new(conf).await
    }

    async fn build_with_tracing(
        conf: gax::options::ClientConfig,
    ) -> Result<impl crate::stubs::Tasks> {
        Self::build_transport(conf)
            .await
            .map(crate::tracing::Tasks::new)
    }

    /// Gets information about a Task.
    pub fn get_task(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::tasks::GetTask {
        crate::builders::tasks::GetTask::new(self.inner.clone()).set_name(name.into())
    }

    /// Lists Tasks from an Execution of a Job.
    pub fn list_tasks(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::tasks::ListTasks {
        crate::builders::tasks::ListTasks::new(self.inner.clone()).set_parent(parent.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn list_operations(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::tasks::ListOperations {
        crate::builders::tasks::ListOperations::new(self.inner.clone()).set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn get_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::tasks::GetOperation {
        crate::builders::tasks::GetOperation::new(self.inner.clone()).set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn delete_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::tasks::DeleteOperation {
        crate::builders::tasks::DeleteOperation::new(self.inner.clone()).set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn wait_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::tasks::WaitOperation {
        crate::builders::tasks::WaitOperation::new(self.inner.clone()).set_name(name.into())
    }
}
