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

pub mod iam_credentials {
    use crate::Result;
    use std::sync::Arc;

    /// Common implementation for [crate::client::IAMCredentials] request builders.
    #[derive(Clone, Debug)]
    pub struct RequestBuilder<R: std::default::Default> {
        stub: Arc<dyn crate::traits::dyntraits::IAMCredentials>,
        request: R,
        options: gax::options::RequestOptions,
    }

    impl<R> RequestBuilder<R>
    where
        R: std::default::Default,
    {
        pub(crate) fn new(stub: Arc<dyn crate::traits::dyntraits::IAMCredentials>) -> Self {
            Self {
                stub,
                request: R::default(),
                options: gax::options::RequestOptions::default(),
            }
        }
    }

    /// The request builder for a IAMCredentials::generate_access_token call.
    #[derive(Clone, Debug)]
    pub struct GenerateAccessToken(RequestBuilder<crate::model::GenerateAccessTokenRequest>);

    impl GenerateAccessToken {
        pub(crate) fn new(stub: Arc<dyn crate::traits::dyntraits::IAMCredentials>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::GenerateAccessTokenRequest>>(
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
        pub async fn send(self) -> Result<crate::model::GenerateAccessTokenResponse> {
            (*self.0.stub)
                .generate_access_token(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][crate::model::GenerateAccessTokenRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }

        /// Sets the value of [lifetime][crate::model::GenerateAccessTokenRequest::lifetime].
        pub fn set_lifetime<T: Into<std::option::Option<wkt::Duration>>>(mut self, v: T) -> Self {
            self.0.request.lifetime = v.into();
            self
        }

        /// Sets the value of [delegates][crate::model::GenerateAccessTokenRequest::delegates].
        pub fn set_delegates<T, V>(mut self, v: T) -> Self
        where
            T: std::iter::IntoIterator<Item = V>,
            V: std::convert::Into<std::string::String>,
        {
            use std::iter::Iterator;
            self.0.request.delegates = v.into_iter().map(|i| i.into()).collect();
            self
        }

        /// Sets the value of [scope][crate::model::GenerateAccessTokenRequest::scope].
        pub fn set_scope<T, V>(mut self, v: T) -> Self
        where
            T: std::iter::IntoIterator<Item = V>,
            V: std::convert::Into<std::string::String>,
        {
            use std::iter::Iterator;
            self.0.request.scope = v.into_iter().map(|i| i.into()).collect();
            self
        }
    }

    impl gax::options::RequestBuilder for GenerateAccessToken {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a IAMCredentials::generate_id_token call.
    #[derive(Clone, Debug)]
    pub struct GenerateIdToken(RequestBuilder<crate::model::GenerateIdTokenRequest>);

    impl GenerateIdToken {
        pub(crate) fn new(stub: Arc<dyn crate::traits::dyntraits::IAMCredentials>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::GenerateIdTokenRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::GenerateIdTokenResponse> {
            (*self.0.stub)
                .generate_id_token(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][crate::model::GenerateIdTokenRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }

        /// Sets the value of [audience][crate::model::GenerateIdTokenRequest::audience].
        pub fn set_audience<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.audience = v.into();
            self
        }

        /// Sets the value of [include_email][crate::model::GenerateIdTokenRequest::include_email].
        pub fn set_include_email<T: Into<bool>>(mut self, v: T) -> Self {
            self.0.request.include_email = v.into();
            self
        }

        /// Sets the value of [delegates][crate::model::GenerateIdTokenRequest::delegates].
        pub fn set_delegates<T, V>(mut self, v: T) -> Self
        where
            T: std::iter::IntoIterator<Item = V>,
            V: std::convert::Into<std::string::String>,
        {
            use std::iter::Iterator;
            self.0.request.delegates = v.into_iter().map(|i| i.into()).collect();
            self
        }
    }

    impl gax::options::RequestBuilder for GenerateIdToken {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a IAMCredentials::sign_blob call.
    #[derive(Clone, Debug)]
    pub struct SignBlob(RequestBuilder<crate::model::SignBlobRequest>);

    impl SignBlob {
        pub(crate) fn new(stub: Arc<dyn crate::traits::dyntraits::IAMCredentials>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::SignBlobRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::SignBlobResponse> {
            (*self.0.stub)
                .sign_blob(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][crate::model::SignBlobRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }

        /// Sets the value of [payload][crate::model::SignBlobRequest::payload].
        pub fn set_payload<T: Into<bytes::Bytes>>(mut self, v: T) -> Self {
            self.0.request.payload = v.into();
            self
        }

        /// Sets the value of [delegates][crate::model::SignBlobRequest::delegates].
        pub fn set_delegates<T, V>(mut self, v: T) -> Self
        where
            T: std::iter::IntoIterator<Item = V>,
            V: std::convert::Into<std::string::String>,
        {
            use std::iter::Iterator;
            self.0.request.delegates = v.into_iter().map(|i| i.into()).collect();
            self
        }
    }

    impl gax::options::RequestBuilder for SignBlob {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a IAMCredentials::sign_jwt call.
    #[derive(Clone, Debug)]
    pub struct SignJwt(RequestBuilder<crate::model::SignJwtRequest>);

    impl SignJwt {
        pub(crate) fn new(stub: Arc<dyn crate::traits::dyntraits::IAMCredentials>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::SignJwtRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::SignJwtResponse> {
            (*self.0.stub)
                .sign_jwt(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][crate::model::SignJwtRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }

        /// Sets the value of [payload][crate::model::SignJwtRequest::payload].
        pub fn set_payload<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.payload = v.into();
            self
        }

        /// Sets the value of [delegates][crate::model::SignJwtRequest::delegates].
        pub fn set_delegates<T, V>(mut self, v: T) -> Self
        where
            T: std::iter::IntoIterator<Item = V>,
            V: std::convert::Into<std::string::String>,
        {
            use std::iter::Iterator;
            self.0.request.delegates = v.into_iter().map(|i| i.into()).collect();
            self
        }
    }

    impl gax::options::RequestBuilder for SignJwt {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }
}
