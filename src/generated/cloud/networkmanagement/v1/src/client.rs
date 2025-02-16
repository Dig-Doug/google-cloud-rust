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

/// Implements a client for the Network Management API.
///
/// # Service Description
///
/// The Reachability service in the Google Cloud Network Management API provides
/// services that analyze the reachability within a single Google Virtual Private
/// Cloud (VPC) network, between peered VPC networks, between VPC and on-premises
/// networks, or between VPC networks and internet hosts. A reachability analysis
/// is based on Google Cloud network configurations.
///
/// You can use the analysis results to verify these configurations and
/// to troubleshoot connectivity issues.
///
/// # Configuration
///
/// `ReachabilityService` has various configuration parameters, the defaults should
/// work with most applications.
///
/// # Pooling and Cloning
///
/// `ReachabilityService` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `ReachabilityService` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct ReachabilityService {
    inner: Arc<dyn crate::stubs::dynamic::ReachabilityService>,
}

impl ReachabilityService {
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
        T: crate::stubs::ReachabilityService + 'static,
    {
        Self {
            inner: Arc::new(stub),
        }
    }

    async fn build_inner(
        conf: gax::options::ClientConfig,
    ) -> Result<Arc<dyn crate::stubs::dynamic::ReachabilityService>> {
        if conf.tracing_enabled() {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gax::options::ClientConfig,
    ) -> Result<impl crate::stubs::ReachabilityService> {
        crate::transport::ReachabilityService::new(conf).await
    }

    async fn build_with_tracing(
        conf: gax::options::ClientConfig,
    ) -> Result<impl crate::stubs::ReachabilityService> {
        Self::build_transport(conf)
            .await
            .map(crate::tracing::ReachabilityService::new)
    }

    /// Lists all Connectivity Tests owned by a project.
    pub fn list_connectivity_tests(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::reachability_service::ListConnectivityTests {
        crate::builders::reachability_service::ListConnectivityTests::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets the details of a specific Connectivity Test.
    pub fn get_connectivity_test(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::reachability_service::GetConnectivityTest {
        crate::builders::reachability_service::GetConnectivityTest::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Creates a new Connectivity Test.
    /// After you create a test, the reachability analysis is performed as part
    /// of the long running operation, which completes when the analysis completes.
    ///
    /// If the endpoint specifications in `ConnectivityTest` are invalid
    /// (for example, containing non-existent resources in the network, or you
    /// don't have read permissions to the network configurations of listed
    /// projects), then the reachability result returns a value of `UNKNOWN`.
    ///
    /// If the endpoint specifications in `ConnectivityTest` are
    /// incomplete, the reachability result returns a value of
    /// \<code\>AMBIGUOUS\</code\>. For more information,
    /// see the Connectivity Test documentation.
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
    pub fn create_connectivity_test(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::reachability_service::CreateConnectivityTest {
        crate::builders::reachability_service::CreateConnectivityTest::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Updates the configuration of an existing `ConnectivityTest`.
    /// After you update a test, the reachability analysis is performed as part
    /// of the long running operation, which completes when the analysis completes.
    /// The Reachability state in the test resource is updated with the new result.
    ///
    /// If the endpoint specifications in `ConnectivityTest` are invalid
    /// (for example, they contain non-existent resources in the network, or the
    /// user does not have read permissions to the network configurations of
    /// listed projects), then the reachability result returns a value of
    /// \<code\>UNKNOWN\</code\>.
    ///
    /// If the endpoint specifications in `ConnectivityTest` are incomplete, the
    /// reachability result returns a value of `AMBIGUOUS`. See the documentation
    /// in `ConnectivityTest` for more details.
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
    pub fn update_connectivity_test(
        &self,
        resource: impl Into<crate::model::ConnectivityTest>,
    ) -> crate::builders::reachability_service::UpdateConnectivityTest {
        crate::builders::reachability_service::UpdateConnectivityTest::new(self.inner.clone())
            .set_resource(resource.into())
    }

    /// Rerun an existing `ConnectivityTest`.
    /// After the user triggers the rerun, the reachability analysis is performed
    /// as part of the long running operation, which completes when the analysis
    /// completes.
    ///
    /// Even though the test configuration remains the same, the reachability
    /// result may change due to underlying network configuration changes.
    ///
    /// If the endpoint specifications in `ConnectivityTest` become invalid (for
    /// example, specified resources are deleted in the network, or you lost
    /// read permissions to the network configurations of listed projects), then
    /// the reachability result returns a value of `UNKNOWN`.
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
    pub fn rerun_connectivity_test(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::reachability_service::RerunConnectivityTest {
        crate::builders::reachability_service::RerunConnectivityTest::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Deletes a specific `ConnectivityTest`.
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
    pub fn delete_connectivity_test(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::reachability_service::DeleteConnectivityTest {
        crate::builders::reachability_service::DeleteConnectivityTest::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Lists information about the supported locations for this service.
    pub fn list_locations(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::reachability_service::ListLocations {
        crate::builders::reachability_service::ListLocations::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Gets information about a location.
    pub fn get_location(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::reachability_service::GetLocation {
        crate::builders::reachability_service::GetLocation::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Sets the access control policy on the specified resource. Replaces
    /// any existing policy.
    ///
    /// Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED`
    /// errors.
    pub fn set_iam_policy(
        &self,
        resource: impl Into<std::string::String>,
    ) -> crate::builders::reachability_service::SetIamPolicy {
        crate::builders::reachability_service::SetIamPolicy::new(self.inner.clone())
            .set_resource(resource.into())
    }

    /// Gets the access control policy for a resource. Returns an empty policy
    /// if the resource exists and does not have a policy set.
    pub fn get_iam_policy(
        &self,
        resource: impl Into<std::string::String>,
    ) -> crate::builders::reachability_service::GetIamPolicy {
        crate::builders::reachability_service::GetIamPolicy::new(self.inner.clone())
            .set_resource(resource.into())
    }

    /// Returns permissions that a caller has on the specified resource. If the
    /// resource does not exist, this will return an empty set of
    /// permissions, not a `NOT_FOUND` error.
    ///
    /// Note: This operation is designed to be used for building
    /// permission-aware UIs and command-line tools, not for authorization
    /// checking. This operation may "fail open" without warning.
    pub fn test_iam_permissions(
        &self,
        resource: impl Into<std::string::String>,
    ) -> crate::builders::reachability_service::TestIamPermissions {
        crate::builders::reachability_service::TestIamPermissions::new(self.inner.clone())
            .set_resource(resource.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn list_operations(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::reachability_service::ListOperations {
        crate::builders::reachability_service::ListOperations::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn get_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::reachability_service::GetOperation {
        crate::builders::reachability_service::GetOperation::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn delete_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::reachability_service::DeleteOperation {
        crate::builders::reachability_service::DeleteOperation::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn cancel_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::reachability_service::CancelOperation {
        crate::builders::reachability_service::CancelOperation::new(self.inner.clone())
            .set_name(name.into())
    }
}

/// Implements a client for the Network Management API.
///
/// # Service Description
///
/// The VPC Flow Logs service in the Google Cloud Network Management API provides
/// configurations that generate Flow Logs. The service and the configuration
/// resources created using this service are global.
///
/// # Configuration
///
/// `VpcFlowLogsService` has various configuration parameters, the defaults should
/// work with most applications.
///
/// # Pooling and Cloning
///
/// `VpcFlowLogsService` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `VpcFlowLogsService` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct VpcFlowLogsService {
    inner: Arc<dyn crate::stubs::dynamic::VpcFlowLogsService>,
}

impl VpcFlowLogsService {
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
        T: crate::stubs::VpcFlowLogsService + 'static,
    {
        Self {
            inner: Arc::new(stub),
        }
    }

    async fn build_inner(
        conf: gax::options::ClientConfig,
    ) -> Result<Arc<dyn crate::stubs::dynamic::VpcFlowLogsService>> {
        if conf.tracing_enabled() {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gax::options::ClientConfig,
    ) -> Result<impl crate::stubs::VpcFlowLogsService> {
        crate::transport::VpcFlowLogsService::new(conf).await
    }

    async fn build_with_tracing(
        conf: gax::options::ClientConfig,
    ) -> Result<impl crate::stubs::VpcFlowLogsService> {
        Self::build_transport(conf)
            .await
            .map(crate::tracing::VpcFlowLogsService::new)
    }

    /// Lists all `VpcFlowLogsConfigs` in a given project.
    pub fn list_vpc_flow_logs_configs(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::vpc_flow_logs_service::ListVpcFlowLogsConfigs {
        crate::builders::vpc_flow_logs_service::ListVpcFlowLogsConfigs::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets the details of a specific `VpcFlowLogsConfig`.
    pub fn get_vpc_flow_logs_config(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::vpc_flow_logs_service::GetVpcFlowLogsConfig {
        crate::builders::vpc_flow_logs_service::GetVpcFlowLogsConfig::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Creates a new `VpcFlowLogsConfig`.
    /// If a configuration with the exact same settings already exists (even if the
    /// ID is different), the creation fails.
    /// Notes:
    ///
    /// . Creating a configuration with state=DISABLED will fail
    /// . The following fields are not considered as `settings` for the purpose
    ///   of the check mentioned above, therefore - creating another configuration
    ///   with the same fields but different values for the following fields will
    ///   fail as well:
    ///   * name
    ///   * create_time
    ///   * update_time
    ///   * labels
    ///   * description
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
    pub fn create_vpc_flow_logs_config(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::vpc_flow_logs_service::CreateVpcFlowLogsConfig {
        crate::builders::vpc_flow_logs_service::CreateVpcFlowLogsConfig::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Updates an existing `VpcFlowLogsConfig`.
    /// If a configuration with the exact same settings already exists (even if the
    /// ID is different), the creation fails.
    /// Notes:
    ///
    /// . Updating a configuration with state=DISABLED will fail.
    /// . The following fields are not considered as `settings` for the purpose
    ///   of the check mentioned above, therefore - updating another configuration
    ///   with the same fields but different values for the following fields will
    ///   fail as well:
    ///   * name
    ///   * create_time
    ///   * update_time
    ///   * labels
    ///   * description
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
    pub fn update_vpc_flow_logs_config(
        &self,
        vpc_flow_logs_config: impl Into<crate::model::VpcFlowLogsConfig>,
    ) -> crate::builders::vpc_flow_logs_service::UpdateVpcFlowLogsConfig {
        crate::builders::vpc_flow_logs_service::UpdateVpcFlowLogsConfig::new(self.inner.clone())
            .set_vpc_flow_logs_config(vpc_flow_logs_config.into())
    }

    /// Deletes a specific `VpcFlowLogsConfig`.
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
    pub fn delete_vpc_flow_logs_config(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::vpc_flow_logs_service::DeleteVpcFlowLogsConfig {
        crate::builders::vpc_flow_logs_service::DeleteVpcFlowLogsConfig::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Lists information about the supported locations for this service.
    pub fn list_locations(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::vpc_flow_logs_service::ListLocations {
        crate::builders::vpc_flow_logs_service::ListLocations::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Gets information about a location.
    pub fn get_location(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::vpc_flow_logs_service::GetLocation {
        crate::builders::vpc_flow_logs_service::GetLocation::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Sets the access control policy on the specified resource. Replaces
    /// any existing policy.
    ///
    /// Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED`
    /// errors.
    pub fn set_iam_policy(
        &self,
        resource: impl Into<std::string::String>,
    ) -> crate::builders::vpc_flow_logs_service::SetIamPolicy {
        crate::builders::vpc_flow_logs_service::SetIamPolicy::new(self.inner.clone())
            .set_resource(resource.into())
    }

    /// Gets the access control policy for a resource. Returns an empty policy
    /// if the resource exists and does not have a policy set.
    pub fn get_iam_policy(
        &self,
        resource: impl Into<std::string::String>,
    ) -> crate::builders::vpc_flow_logs_service::GetIamPolicy {
        crate::builders::vpc_flow_logs_service::GetIamPolicy::new(self.inner.clone())
            .set_resource(resource.into())
    }

    /// Returns permissions that a caller has on the specified resource. If the
    /// resource does not exist, this will return an empty set of
    /// permissions, not a `NOT_FOUND` error.
    ///
    /// Note: This operation is designed to be used for building
    /// permission-aware UIs and command-line tools, not for authorization
    /// checking. This operation may "fail open" without warning.
    pub fn test_iam_permissions(
        &self,
        resource: impl Into<std::string::String>,
    ) -> crate::builders::vpc_flow_logs_service::TestIamPermissions {
        crate::builders::vpc_flow_logs_service::TestIamPermissions::new(self.inner.clone())
            .set_resource(resource.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn list_operations(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::vpc_flow_logs_service::ListOperations {
        crate::builders::vpc_flow_logs_service::ListOperations::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn get_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::vpc_flow_logs_service::GetOperation {
        crate::builders::vpc_flow_logs_service::GetOperation::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn delete_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::vpc_flow_logs_service::DeleteOperation {
        crate::builders::vpc_flow_logs_service::DeleteOperation::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn cancel_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::vpc_flow_logs_service::CancelOperation {
        crate::builders::vpc_flow_logs_service::CancelOperation::new(self.inner.clone())
            .set_name(name.into())
    }
}
