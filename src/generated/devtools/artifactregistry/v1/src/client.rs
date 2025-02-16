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

/// Implements a client for the Artifact Registry API.
///
/// # Service Description
///
/// The Artifact Registry API service.
///
/// Artifact Registry is an artifact management system for storing artifacts
/// from different package management systems.
///
/// The resources managed by this API are:
///
/// * Repositories, which group packages and their data.
/// * Packages, which group versions and their tags.
/// * Versions, which are specific forms of a package.
/// * Tags, which represent alternative names for versions.
/// * Files, which contain content and are optionally associated with a Package
///   or Version.
///
/// # Configuration
///
/// `ArtifactRegistry` has various configuration parameters, the defaults should
/// work with most applications.
///
/// # Pooling and Cloning
///
/// `ArtifactRegistry` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `ArtifactRegistry` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct ArtifactRegistry {
    inner: Arc<dyn crate::stubs::dynamic::ArtifactRegistry>,
}

impl ArtifactRegistry {
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
        T: crate::stubs::ArtifactRegistry + 'static,
    {
        Self {
            inner: Arc::new(stub),
        }
    }

    async fn build_inner(
        conf: gax::options::ClientConfig,
    ) -> Result<Arc<dyn crate::stubs::dynamic::ArtifactRegistry>> {
        if conf.tracing_enabled() {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gax::options::ClientConfig,
    ) -> Result<impl crate::stubs::ArtifactRegistry> {
        crate::transport::ArtifactRegistry::new(conf).await
    }

    async fn build_with_tracing(
        conf: gax::options::ClientConfig,
    ) -> Result<impl crate::stubs::ArtifactRegistry> {
        Self::build_transport(conf)
            .await
            .map(crate::tracing::ArtifactRegistry::new)
    }

    /// Lists docker images.
    pub fn list_docker_images(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::artifact_registry::ListDockerImages {
        crate::builders::artifact_registry::ListDockerImages::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets a docker image.
    pub fn get_docker_image(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::artifact_registry::GetDockerImage {
        crate::builders::artifact_registry::GetDockerImage::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Lists maven artifacts.
    pub fn list_maven_artifacts(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::artifact_registry::ListMavenArtifacts {
        crate::builders::artifact_registry::ListMavenArtifacts::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets a maven artifact.
    pub fn get_maven_artifact(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::artifact_registry::GetMavenArtifact {
        crate::builders::artifact_registry::GetMavenArtifact::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Lists npm packages.
    pub fn list_npm_packages(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::artifact_registry::ListNpmPackages {
        crate::builders::artifact_registry::ListNpmPackages::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets a npm package.
    pub fn get_npm_package(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::artifact_registry::GetNpmPackage {
        crate::builders::artifact_registry::GetNpmPackage::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Lists python packages.
    pub fn list_python_packages(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::artifact_registry::ListPythonPackages {
        crate::builders::artifact_registry::ListPythonPackages::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets a python package.
    pub fn get_python_package(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::artifact_registry::GetPythonPackage {
        crate::builders::artifact_registry::GetPythonPackage::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Imports Apt artifacts. The returned Operation will complete once the
    /// resources are imported. Package, Version, and File resources are created
    /// based on the imported artifacts. Imported artifacts that conflict with
    /// existing resources are ignored.
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
    pub fn import_apt_artifacts(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::artifact_registry::ImportAptArtifacts {
        crate::builders::artifact_registry::ImportAptArtifacts::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Imports Yum (RPM) artifacts. The returned Operation will complete once the
    /// resources are imported. Package, Version, and File resources are created
    /// based on the imported artifacts. Imported artifacts that conflict with
    /// existing resources are ignored.
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
    pub fn import_yum_artifacts(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::artifact_registry::ImportYumArtifacts {
        crate::builders::artifact_registry::ImportYumArtifacts::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Lists repositories.
    pub fn list_repositories(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::artifact_registry::ListRepositories {
        crate::builders::artifact_registry::ListRepositories::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets a repository.
    pub fn get_repository(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::artifact_registry::GetRepository {
        crate::builders::artifact_registry::GetRepository::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Creates a repository. The returned Operation will finish once the
    /// repository has been created. Its response will be the created Repository.
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
    pub fn create_repository(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::artifact_registry::CreateRepository {
        crate::builders::artifact_registry::CreateRepository::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Updates a repository.
    pub fn update_repository(
        &self,
        repository: impl Into<crate::model::Repository>,
    ) -> crate::builders::artifact_registry::UpdateRepository {
        crate::builders::artifact_registry::UpdateRepository::new(self.inner.clone())
            .set_repository(repository.into())
    }

    /// Deletes a repository and all of its contents. The returned Operation will
    /// finish once the repository has been deleted. It will not have any Operation
    /// metadata and will return a google.protobuf.Empty response.
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
    pub fn delete_repository(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::artifact_registry::DeleteRepository {
        crate::builders::artifact_registry::DeleteRepository::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Lists packages.
    pub fn list_packages(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::artifact_registry::ListPackages {
        crate::builders::artifact_registry::ListPackages::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets a package.
    pub fn get_package(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::artifact_registry::GetPackage {
        crate::builders::artifact_registry::GetPackage::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Deletes a package and all of its versions and tags. The returned operation
    /// will complete once the package has been deleted.
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
    pub fn delete_package(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::artifact_registry::DeletePackage {
        crate::builders::artifact_registry::DeletePackage::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Lists versions.
    pub fn list_versions(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::artifact_registry::ListVersions {
        crate::builders::artifact_registry::ListVersions::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets a version
    pub fn get_version(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::artifact_registry::GetVersion {
        crate::builders::artifact_registry::GetVersion::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Deletes a version and all of its content. The returned operation will
    /// complete once the version has been deleted.
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
    pub fn delete_version(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::artifact_registry::DeleteVersion {
        crate::builders::artifact_registry::DeleteVersion::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Deletes multiple versions across a repository. The returned operation will
    /// complete once the versions have been deleted.
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
    pub fn batch_delete_versions(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::artifact_registry::BatchDeleteVersions {
        crate::builders::artifact_registry::BatchDeleteVersions::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Updates a version.
    pub fn update_version(
        &self,
        version: impl Into<crate::model::Version>,
    ) -> crate::builders::artifact_registry::UpdateVersion {
        crate::builders::artifact_registry::UpdateVersion::new(self.inner.clone())
            .set_version(version.into())
    }

    /// Lists files.
    pub fn list_files(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::artifact_registry::ListFiles {
        crate::builders::artifact_registry::ListFiles::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets a file.
    pub fn get_file(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::artifact_registry::GetFile {
        crate::builders::artifact_registry::GetFile::new(self.inner.clone()).set_name(name.into())
    }

    /// Deletes a file and all of its content. It is only allowed on generic
    /// repositories. The returned operation will complete once the file has been
    /// deleted.
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
    pub fn delete_file(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::artifact_registry::DeleteFile {
        crate::builders::artifact_registry::DeleteFile::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Updates a file.
    pub fn update_file(
        &self,
        file: impl Into<crate::model::File>,
    ) -> crate::builders::artifact_registry::UpdateFile {
        crate::builders::artifact_registry::UpdateFile::new(self.inner.clone())
            .set_file(file.into())
    }

    /// Lists tags.
    pub fn list_tags(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::artifact_registry::ListTags {
        crate::builders::artifact_registry::ListTags::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets a tag.
    pub fn get_tag(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::artifact_registry::GetTag {
        crate::builders::artifact_registry::GetTag::new(self.inner.clone()).set_name(name.into())
    }

    /// Creates a tag.
    pub fn create_tag(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::artifact_registry::CreateTag {
        crate::builders::artifact_registry::CreateTag::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Updates a tag.
    pub fn update_tag(
        &self,
        tag: impl Into<crate::model::Tag>,
    ) -> crate::builders::artifact_registry::UpdateTag {
        crate::builders::artifact_registry::UpdateTag::new(self.inner.clone()).set_tag(tag.into())
    }

    /// Deletes a tag.
    pub fn delete_tag(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::artifact_registry::DeleteTag {
        crate::builders::artifact_registry::DeleteTag::new(self.inner.clone()).set_name(name.into())
    }

    /// Creates a rule.
    pub fn create_rule(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::artifact_registry::CreateRule {
        crate::builders::artifact_registry::CreateRule::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Lists rules.
    pub fn list_rules(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::artifact_registry::ListRules {
        crate::builders::artifact_registry::ListRules::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets a rule.
    pub fn get_rule(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::artifact_registry::GetRule {
        crate::builders::artifact_registry::GetRule::new(self.inner.clone()).set_name(name.into())
    }

    /// Updates a rule.
    pub fn update_rule(
        &self,
        rule: impl Into<crate::model::Rule>,
    ) -> crate::builders::artifact_registry::UpdateRule {
        crate::builders::artifact_registry::UpdateRule::new(self.inner.clone())
            .set_rule(rule.into())
    }

    /// Deletes a rule.
    pub fn delete_rule(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::artifact_registry::DeleteRule {
        crate::builders::artifact_registry::DeleteRule::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Updates the IAM policy for a given resource.
    pub fn set_iam_policy(
        &self,
        resource: impl Into<std::string::String>,
    ) -> crate::builders::artifact_registry::SetIamPolicy {
        crate::builders::artifact_registry::SetIamPolicy::new(self.inner.clone())
            .set_resource(resource.into())
    }

    /// Gets the IAM policy for a given resource.
    pub fn get_iam_policy(
        &self,
        resource: impl Into<std::string::String>,
    ) -> crate::builders::artifact_registry::GetIamPolicy {
        crate::builders::artifact_registry::GetIamPolicy::new(self.inner.clone())
            .set_resource(resource.into())
    }

    /// Tests if the caller has a list of permissions on a resource.
    pub fn test_iam_permissions(
        &self,
        resource: impl Into<std::string::String>,
    ) -> crate::builders::artifact_registry::TestIamPermissions {
        crate::builders::artifact_registry::TestIamPermissions::new(self.inner.clone())
            .set_resource(resource.into())
    }

    /// Retrieves the Settings for the Project.
    pub fn get_project_settings(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::artifact_registry::GetProjectSettings {
        crate::builders::artifact_registry::GetProjectSettings::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Updates the Settings for the Project.
    pub fn update_project_settings(
        &self,
        project_settings: impl Into<crate::model::ProjectSettings>,
    ) -> crate::builders::artifact_registry::UpdateProjectSettings {
        crate::builders::artifact_registry::UpdateProjectSettings::new(self.inner.clone())
            .set_project_settings(project_settings.into())
    }

    /// Retrieves the VPCSC Config for the Project.
    pub fn get_vpcsc_config(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::artifact_registry::GetVPCSCConfig {
        crate::builders::artifact_registry::GetVPCSCConfig::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Updates the VPCSC Config for the Project.
    pub fn update_vpcsc_config(
        &self,
        vpcsc_config: impl Into<crate::model::VPCSCConfig>,
    ) -> crate::builders::artifact_registry::UpdateVPCSCConfig {
        crate::builders::artifact_registry::UpdateVPCSCConfig::new(self.inner.clone())
            .set_vpcsc_config(vpcsc_config.into())
    }

    /// Updates a package.
    pub fn update_package(
        &self,
        package: impl Into<crate::model::Package>,
    ) -> crate::builders::artifact_registry::UpdatePackage {
        crate::builders::artifact_registry::UpdatePackage::new(self.inner.clone())
            .set_package(package.into())
    }

    /// Lists attachments.
    pub fn list_attachments(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::artifact_registry::ListAttachments {
        crate::builders::artifact_registry::ListAttachments::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets an attachment.
    pub fn get_attachment(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::artifact_registry::GetAttachment {
        crate::builders::artifact_registry::GetAttachment::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Creates an attachment. The returned Operation will finish once the
    /// attachment has been created. Its response will be the created attachment.
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
    pub fn create_attachment(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::artifact_registry::CreateAttachment {
        crate::builders::artifact_registry::CreateAttachment::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Deletes an attachment. The returned Operation will
    /// finish once the attachments has been deleted. It will not have any
    /// Operation metadata and will return a `google.protobuf.Empty` response.
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
    pub fn delete_attachment(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::artifact_registry::DeleteAttachment {
        crate::builders::artifact_registry::DeleteAttachment::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Lists information about the supported locations for this service.
    pub fn list_locations(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::artifact_registry::ListLocations {
        crate::builders::artifact_registry::ListLocations::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Gets information about a location.
    pub fn get_location(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::artifact_registry::GetLocation {
        crate::builders::artifact_registry::GetLocation::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn get_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::artifact_registry::GetOperation {
        crate::builders::artifact_registry::GetOperation::new(self.inner.clone())
            .set_name(name.into())
    }
}
