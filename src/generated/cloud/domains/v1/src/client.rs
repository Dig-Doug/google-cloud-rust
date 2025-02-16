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

/// Implements a client for the Cloud Domains API.
///
/// # Service Description
///
/// The Cloud Domains API enables management and configuration of domain names.
///
/// # Configuration
///
/// `Domains` has various configuration parameters, the defaults should
/// work with most applications.
///
/// # Pooling and Cloning
///
/// `Domains` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `Domains` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct Domains {
    inner: Arc<dyn crate::stubs::dynamic::Domains>,
}

impl Domains {
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
        T: crate::stubs::Domains + 'static,
    {
        Self {
            inner: Arc::new(stub),
        }
    }

    async fn build_inner(
        conf: gax::options::ClientConfig,
    ) -> Result<Arc<dyn crate::stubs::dynamic::Domains>> {
        if conf.tracing_enabled() {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gax::options::ClientConfig,
    ) -> Result<impl crate::stubs::Domains> {
        crate::transport::Domains::new(conf).await
    }

    async fn build_with_tracing(
        conf: gax::options::ClientConfig,
    ) -> Result<impl crate::stubs::Domains> {
        Self::build_transport(conf)
            .await
            .map(crate::tracing::Domains::new)
    }

    /// Searches for available domain names similar to the provided query.
    ///
    /// Availability results from this method are approximate; call
    /// `RetrieveRegisterParameters` on a domain before registering to confirm
    /// availability.
    pub fn search_domains(
        &self,
        location: impl Into<std::string::String>,
    ) -> crate::builders::domains::SearchDomains {
        crate::builders::domains::SearchDomains::new(self.inner.clone())
            .set_location(location.into())
    }

    /// Gets parameters needed to register a new domain name, including price and
    /// up-to-date availability. Use the returned values to call `RegisterDomain`.
    pub fn retrieve_register_parameters(
        &self,
        location: impl Into<std::string::String>,
    ) -> crate::builders::domains::RetrieveRegisterParameters {
        crate::builders::domains::RetrieveRegisterParameters::new(self.inner.clone())
            .set_location(location.into())
    }

    /// Registers a new domain name and creates a corresponding `Registration`
    /// resource.
    ///
    /// Call `RetrieveRegisterParameters` first to check availability of the domain
    /// name and determine parameters like price that are needed to build a call to
    /// this method.
    ///
    /// A successful call creates a `Registration` resource in state
    /// `REGISTRATION_PENDING`, which resolves to `ACTIVE` within 1-2
    /// minutes, indicating that the domain was successfully registered. If the
    /// resource ends up in state `REGISTRATION_FAILED`, it indicates that the
    /// domain was not registered successfully, and you can safely delete the
    /// resource and retry registration.
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
    pub fn register_domain(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::domains::RegisterDomain {
        crate::builders::domains::RegisterDomain::new(self.inner.clone()).set_parent(parent.into())
    }

    /// Gets parameters needed to transfer a domain name from another registrar to
    /// Cloud Domains. For domains managed by Google Domains, transferring to Cloud
    /// Domains is not supported.
    ///
    /// Use the returned values to call `TransferDomain`.
    pub fn retrieve_transfer_parameters(
        &self,
        location: impl Into<std::string::String>,
    ) -> crate::builders::domains::RetrieveTransferParameters {
        crate::builders::domains::RetrieveTransferParameters::new(self.inner.clone())
            .set_location(location.into())
    }

    /// Transfers a domain name from another registrar to Cloud Domains.  For
    /// domains managed by Google Domains, transferring to Cloud Domains is not
    /// supported.
    ///
    /// Before calling this method, go to the domain's current registrar to unlock
    /// the domain for transfer and retrieve the domain's transfer authorization
    /// code. Then call `RetrieveTransferParameters` to confirm that the domain is
    /// unlocked and to get values needed to build a call to this method.
    ///
    /// A successful call creates a `Registration` resource in state
    /// `TRANSFER_PENDING`. It can take several days to complete the transfer
    /// process. The registrant can often speed up this process by approving the
    /// transfer through the current registrar, either by clicking a link in an
    /// email from the registrar or by visiting the registrar's website.
    ///
    /// A few minutes after transfer approval, the resource transitions to state
    /// `ACTIVE`, indicating that the transfer was successful. If the transfer is
    /// rejected or the request expires without being approved, the resource can
    /// end up in state `TRANSFER_FAILED`. If transfer fails, you can safely delete
    /// the resource and retry the transfer.
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
    pub fn transfer_domain(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::domains::TransferDomain {
        crate::builders::domains::TransferDomain::new(self.inner.clone()).set_parent(parent.into())
    }

    /// Lists the `Registration` resources in a project.
    pub fn list_registrations(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::domains::ListRegistrations {
        crate::builders::domains::ListRegistrations::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets the details of a `Registration` resource.
    pub fn get_registration(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::domains::GetRegistration {
        crate::builders::domains::GetRegistration::new(self.inner.clone()).set_name(name.into())
    }

    /// Updates select fields of a `Registration` resource, notably `labels`. To
    /// update other fields, use the appropriate custom update method:
    ///
    /// * To update management settings, see `ConfigureManagementSettings`
    /// * To update DNS configuration, see `ConfigureDnsSettings`
    /// * To update contact information, see `ConfigureContactSettings`
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
    pub fn update_registration(
        &self,
        registration: impl Into<crate::model::Registration>,
    ) -> crate::builders::domains::UpdateRegistration {
        crate::builders::domains::UpdateRegistration::new(self.inner.clone())
            .set_registration(registration.into())
    }

    /// Updates a `Registration`'s management settings.
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
    pub fn configure_management_settings(
        &self,
        registration: impl Into<std::string::String>,
    ) -> crate::builders::domains::ConfigureManagementSettings {
        crate::builders::domains::ConfigureManagementSettings::new(self.inner.clone())
            .set_registration(registration.into())
    }

    /// Updates a `Registration`'s DNS settings.
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
    pub fn configure_dns_settings(
        &self,
        registration: impl Into<std::string::String>,
    ) -> crate::builders::domains::ConfigureDnsSettings {
        crate::builders::domains::ConfigureDnsSettings::new(self.inner.clone())
            .set_registration(registration.into())
    }

    /// Updates a `Registration`'s contact settings. Some changes require
    /// confirmation by the domain's registrant contact .
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
    pub fn configure_contact_settings(
        &self,
        registration: impl Into<std::string::String>,
    ) -> crate::builders::domains::ConfigureContactSettings {
        crate::builders::domains::ConfigureContactSettings::new(self.inner.clone())
            .set_registration(registration.into())
    }

    /// Exports a `Registration` resource, such that it is no longer managed by
    /// Cloud Domains.
    ///
    /// When an active domain is successfully exported, you can continue to use the
    /// domain in [Google Domains](https://domains.google/) until it expires. The
    /// calling user becomes the domain's sole owner in Google Domains, and
    /// permissions for the domain are subsequently managed there. The domain does
    /// not renew automatically unless the new owner sets up billing in Google
    /// Domains.
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
    pub fn export_registration(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::domains::ExportRegistration {
        crate::builders::domains::ExportRegistration::new(self.inner.clone()).set_name(name.into())
    }

    /// Deletes a `Registration` resource.
    ///
    /// This method works on any `Registration` resource using [Subscription or
    /// Commitment billing](/domains/pricing#billing-models), provided that the
    /// resource was created at least 1 day in the past.
    ///
    /// For `Registration` resources using
    /// [Monthly billing](/domains/pricing#billing-models), this method works if:
    ///
    /// * `state` is `EXPORTED` with `expire_time` in the past
    /// * `state` is `REGISTRATION_FAILED`
    /// * `state` is `TRANSFER_FAILED`
    ///
    /// When an active registration is successfully deleted, you can continue to
    /// use the domain in [Google Domains](https://domains.google/) until it
    /// expires. The calling user becomes the domain's sole owner in Google
    /// Domains, and permissions for the domain are subsequently managed there. The
    /// domain does not renew automatically unless the new owner sets up billing in
    /// Google Domains.
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
    pub fn delete_registration(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::domains::DeleteRegistration {
        crate::builders::domains::DeleteRegistration::new(self.inner.clone()).set_name(name.into())
    }

    /// Gets the authorization code of the `Registration` for the purpose of
    /// transferring the domain to another registrar.
    ///
    /// You can call this method only after 60 days have elapsed since the initial
    /// domain registration.
    pub fn retrieve_authorization_code(
        &self,
        registration: impl Into<std::string::String>,
    ) -> crate::builders::domains::RetrieveAuthorizationCode {
        crate::builders::domains::RetrieveAuthorizationCode::new(self.inner.clone())
            .set_registration(registration.into())
    }

    /// Resets the authorization code of the `Registration` to a new random string.
    ///
    /// You can call this method only after 60 days have elapsed since the initial
    /// domain registration.
    pub fn reset_authorization_code(
        &self,
        registration: impl Into<std::string::String>,
    ) -> crate::builders::domains::ResetAuthorizationCode {
        crate::builders::domains::ResetAuthorizationCode::new(self.inner.clone())
            .set_registration(registration.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn list_operations(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::domains::ListOperations {
        crate::builders::domains::ListOperations::new(self.inner.clone()).set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn get_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::domains::GetOperation {
        crate::builders::domains::GetOperation::new(self.inner.clone()).set_name(name.into())
    }
}
