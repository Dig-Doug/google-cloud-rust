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

pub mod os_login_service {
    use crate::Result;
    use std::sync::Arc;

    /// Common implementation for [crate::client::OsLoginService] request builders.
    #[derive(Clone, Debug)]
    pub struct RequestBuilder<R: std::default::Default> {
        stub: Arc<dyn crate::stubs::dynamic::OsLoginService>,
        request: R,
        options: gax::options::RequestOptions,
    }

    impl<R> RequestBuilder<R>
    where
        R: std::default::Default,
    {
        pub(crate) fn new(stub: Arc<dyn crate::stubs::dynamic::OsLoginService>) -> Self {
            Self {
                stub,
                request: R::default(),
                options: gax::options::RequestOptions::default(),
            }
        }
    }

    /// The request builder for a OsLoginService::create_ssh_public_key call.
    #[derive(Clone, Debug)]
    pub struct CreateSshPublicKey(RequestBuilder<crate::model::CreateSshPublicKeyRequest>);

    impl CreateSshPublicKey {
        pub(crate) fn new(stub: Arc<dyn crate::stubs::dynamic::OsLoginService>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::CreateSshPublicKeyRequest>>(
            mut self,
            v: V,
        ) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<oslogin_common::model::SshPublicKey> {
            (*self.0.stub)
                .create_ssh_public_key(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [parent][crate::model::CreateSshPublicKeyRequest::parent].
        pub fn set_parent<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parent = v.into();
            self
        }

        /// Sets the value of [ssh_public_key][crate::model::CreateSshPublicKeyRequest::ssh_public_key].
        pub fn set_ssh_public_key<
            T: Into<std::option::Option<oslogin_common::model::SshPublicKey>>,
        >(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.ssh_public_key = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for CreateSshPublicKey {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a OsLoginService::delete_posix_account call.
    #[derive(Clone, Debug)]
    pub struct DeletePosixAccount(RequestBuilder<crate::model::DeletePosixAccountRequest>);

    impl DeletePosixAccount {
        pub(crate) fn new(stub: Arc<dyn crate::stubs::dynamic::OsLoginService>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::DeletePosixAccountRequest>>(
            mut self,
            v: V,
        ) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<wkt::Empty> {
            (*self.0.stub)
                .delete_posix_account(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][crate::model::DeletePosixAccountRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for DeletePosixAccount {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a OsLoginService::delete_ssh_public_key call.
    #[derive(Clone, Debug)]
    pub struct DeleteSshPublicKey(RequestBuilder<crate::model::DeleteSshPublicKeyRequest>);

    impl DeleteSshPublicKey {
        pub(crate) fn new(stub: Arc<dyn crate::stubs::dynamic::OsLoginService>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::DeleteSshPublicKeyRequest>>(
            mut self,
            v: V,
        ) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<wkt::Empty> {
            (*self.0.stub)
                .delete_ssh_public_key(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][crate::model::DeleteSshPublicKeyRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for DeleteSshPublicKey {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a OsLoginService::get_login_profile call.
    #[derive(Clone, Debug)]
    pub struct GetLoginProfile(RequestBuilder<crate::model::GetLoginProfileRequest>);

    impl GetLoginProfile {
        pub(crate) fn new(stub: Arc<dyn crate::stubs::dynamic::OsLoginService>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::GetLoginProfileRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::LoginProfile> {
            (*self.0.stub)
                .get_login_profile(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][crate::model::GetLoginProfileRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }

        /// Sets the value of [project_id][crate::model::GetLoginProfileRequest::project_id].
        pub fn set_project_id<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.project_id = v.into();
            self
        }

        /// Sets the value of [system_id][crate::model::GetLoginProfileRequest::system_id].
        pub fn set_system_id<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.system_id = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for GetLoginProfile {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a OsLoginService::get_ssh_public_key call.
    #[derive(Clone, Debug)]
    pub struct GetSshPublicKey(RequestBuilder<crate::model::GetSshPublicKeyRequest>);

    impl GetSshPublicKey {
        pub(crate) fn new(stub: Arc<dyn crate::stubs::dynamic::OsLoginService>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::GetSshPublicKeyRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<oslogin_common::model::SshPublicKey> {
            (*self.0.stub)
                .get_ssh_public_key(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][crate::model::GetSshPublicKeyRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for GetSshPublicKey {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a OsLoginService::import_ssh_public_key call.
    #[derive(Clone, Debug)]
    pub struct ImportSshPublicKey(RequestBuilder<crate::model::ImportSshPublicKeyRequest>);

    impl ImportSshPublicKey {
        pub(crate) fn new(stub: Arc<dyn crate::stubs::dynamic::OsLoginService>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::ImportSshPublicKeyRequest>>(
            mut self,
            v: V,
        ) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::ImportSshPublicKeyResponse> {
            (*self.0.stub)
                .import_ssh_public_key(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [parent][crate::model::ImportSshPublicKeyRequest::parent].
        pub fn set_parent<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parent = v.into();
            self
        }

        /// Sets the value of [ssh_public_key][crate::model::ImportSshPublicKeyRequest::ssh_public_key].
        pub fn set_ssh_public_key<
            T: Into<std::option::Option<oslogin_common::model::SshPublicKey>>,
        >(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.ssh_public_key = v.into();
            self
        }

        /// Sets the value of [project_id][crate::model::ImportSshPublicKeyRequest::project_id].
        pub fn set_project_id<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.project_id = v.into();
            self
        }

        /// Sets the value of [regions][crate::model::ImportSshPublicKeyRequest::regions].
        pub fn set_regions<T, V>(mut self, v: T) -> Self
        where
            T: std::iter::IntoIterator<Item = V>,
            V: std::convert::Into<std::string::String>,
        {
            use std::iter::Iterator;
            self.0.request.regions = v.into_iter().map(|i| i.into()).collect();
            self
        }
    }

    impl gax::options::RequestBuilder for ImportSshPublicKey {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a OsLoginService::update_ssh_public_key call.
    #[derive(Clone, Debug)]
    pub struct UpdateSshPublicKey(RequestBuilder<crate::model::UpdateSshPublicKeyRequest>);

    impl UpdateSshPublicKey {
        pub(crate) fn new(stub: Arc<dyn crate::stubs::dynamic::OsLoginService>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::UpdateSshPublicKeyRequest>>(
            mut self,
            v: V,
        ) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<oslogin_common::model::SshPublicKey> {
            (*self.0.stub)
                .update_ssh_public_key(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][crate::model::UpdateSshPublicKeyRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }

        /// Sets the value of [ssh_public_key][crate::model::UpdateSshPublicKeyRequest::ssh_public_key].
        pub fn set_ssh_public_key<
            T: Into<std::option::Option<oslogin_common::model::SshPublicKey>>,
        >(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.ssh_public_key = v.into();
            self
        }

        /// Sets the value of [update_mask][crate::model::UpdateSshPublicKeyRequest::update_mask].
        pub fn set_update_mask<T: Into<std::option::Option<wkt::FieldMask>>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.update_mask = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for UpdateSshPublicKey {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }
}
