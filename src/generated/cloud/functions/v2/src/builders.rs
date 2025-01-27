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

pub mod function_service {
    use crate::Result;
    use std::sync::Arc;

    /// Common implementation for [crate::client::FunctionService] request builders.
    #[derive(Clone, Debug)]
    pub struct RequestBuilder<R: std::default::Default> {
        stub: Arc<dyn crate::traits::dyntraits::FunctionService>,
        request: R,
        options: gax::options::RequestOptions,
    }

    impl<R> RequestBuilder<R>
    where
        R: std::default::Default,
    {
        pub(crate) fn new(stub: Arc<dyn crate::traits::dyntraits::FunctionService>) -> Self {
            Self {
                stub,
                request: R::default(),
                options: gax::options::RequestOptions::default(),
            }
        }
    }

    /// The request builder for a FunctionService::get_function call.
    #[derive(Clone, Debug)]
    pub struct GetFunction(RequestBuilder<crate::model::GetFunctionRequest>);

    impl GetFunction {
        pub(crate) fn new(stub: Arc<dyn crate::traits::dyntraits::FunctionService>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::GetFunctionRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::Function> {
            (*self.0.stub)
                .get_function(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][crate::model::GetFunctionRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }

        /// Sets the value of [revision][crate::model::GetFunctionRequest::revision].
        pub fn set_revision<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.revision = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for GetFunction {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a FunctionService::list_functions call.
    #[derive(Clone, Debug)]
    pub struct ListFunctions(RequestBuilder<crate::model::ListFunctionsRequest>);

    impl ListFunctions {
        pub(crate) fn new(stub: Arc<dyn crate::traits::dyntraits::FunctionService>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::ListFunctionsRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::ListFunctionsResponse> {
            (*self.0.stub)
                .list_functions(self.0.request, self.0.options)
                .await
        }

        /// Streams the responses back.
        #[cfg(feature = "unstable-stream")]
        pub async fn stream(
            self,
        ) -> gax::paginator::Paginator<crate::model::ListFunctionsResponse, gax::error::Error>
        {
            let token = gax::paginator::extract_token(&self.0.request.page_token);
            let execute = move |token: String| {
                let builder = self.clone();
                builder.0.request.clone().set_page_token(token);
                builder.send()
            };
            gax::paginator::Paginator::new(token, execute)
        }

        /// Sets the value of [parent][crate::model::ListFunctionsRequest::parent].
        pub fn set_parent<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parent = v.into();
            self
        }

        /// Sets the value of [page_size][crate::model::ListFunctionsRequest::page_size].
        pub fn set_page_size<T: Into<i32>>(mut self, v: T) -> Self {
            self.0.request.page_size = v.into();
            self
        }

        /// Sets the value of [page_token][crate::model::ListFunctionsRequest::page_token].
        pub fn set_page_token<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.page_token = v.into();
            self
        }

        /// Sets the value of [filter][crate::model::ListFunctionsRequest::filter].
        pub fn set_filter<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.filter = v.into();
            self
        }

        /// Sets the value of [order_by][crate::model::ListFunctionsRequest::order_by].
        pub fn set_order_by<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.order_by = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for ListFunctions {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a FunctionService::create_function call.
    #[derive(Clone, Debug)]
    pub struct CreateFunction(RequestBuilder<crate::model::CreateFunctionRequest>);

    impl CreateFunction {
        pub(crate) fn new(stub: Arc<dyn crate::traits::dyntraits::FunctionService>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::CreateFunctionRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        ///
        /// # Long running operations
        ///
        /// This starts, but does not poll, a longrunning operation. More information
        /// on [create_function][crate::client::FunctionService::create_function].
        pub async fn send(self) -> Result<longrunning::model::Operation> {
            (*self.0.stub)
                .create_function(self.0.request, self.0.options)
                .await
        }

        /// Creates a [Poller][lro::Poller] to work with `create_function`.
        pub fn poller(
            self,
        ) -> impl lro::Poller<crate::model::Function, crate::model::OperationMetadata> {
            type Operation =
                lro::Operation<crate::model::Function, crate::model::OperationMetadata>;
            let polling_policy = self.0.stub.get_polling_policy(&self.0.options);
            let polling_backoff_policy = self.0.stub.get_polling_backoff_policy(&self.0.options);

            let stub = self.0.stub.clone();
            let mut options = self.0.options.clone();
            options.set_retry_policy(gax::retry_policy::NeverRetry);
            let query = move |name| {
                let stub = stub.clone();
                let options = options.clone();
                async {
                    let op = GetOperation::new(stub)
                        .set_name(name)
                        .with_options(options)
                        .send()
                        .await?;
                    Ok(Operation::new(op))
                }
            };

            let start = move || async {
                let op = self.send().await?;
                Ok(Operation::new(op))
            };

            lro::new_poller(polling_policy, polling_backoff_policy, start, query)
        }

        /// Sets the value of [parent][crate::model::CreateFunctionRequest::parent].
        pub fn set_parent<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parent = v.into();
            self
        }

        /// Sets the value of [function][crate::model::CreateFunctionRequest::function].
        pub fn set_function<T: Into<std::option::Option<crate::model::Function>>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.function = v.into();
            self
        }

        /// Sets the value of [function_id][crate::model::CreateFunctionRequest::function_id].
        pub fn set_function_id<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.function_id = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for CreateFunction {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a FunctionService::update_function call.
    #[derive(Clone, Debug)]
    pub struct UpdateFunction(RequestBuilder<crate::model::UpdateFunctionRequest>);

    impl UpdateFunction {
        pub(crate) fn new(stub: Arc<dyn crate::traits::dyntraits::FunctionService>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::UpdateFunctionRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        ///
        /// # Long running operations
        ///
        /// This starts, but does not poll, a longrunning operation. More information
        /// on [update_function][crate::client::FunctionService::update_function].
        pub async fn send(self) -> Result<longrunning::model::Operation> {
            (*self.0.stub)
                .update_function(self.0.request, self.0.options)
                .await
        }

        /// Creates a [Poller][lro::Poller] to work with `update_function`.
        pub fn poller(
            self,
        ) -> impl lro::Poller<crate::model::Function, crate::model::OperationMetadata> {
            type Operation =
                lro::Operation<crate::model::Function, crate::model::OperationMetadata>;
            let polling_policy = self.0.stub.get_polling_policy(&self.0.options);
            let polling_backoff_policy = self.0.stub.get_polling_backoff_policy(&self.0.options);

            let stub = self.0.stub.clone();
            let mut options = self.0.options.clone();
            options.set_retry_policy(gax::retry_policy::NeverRetry);
            let query = move |name| {
                let stub = stub.clone();
                let options = options.clone();
                async {
                    let op = GetOperation::new(stub)
                        .set_name(name)
                        .with_options(options)
                        .send()
                        .await?;
                    Ok(Operation::new(op))
                }
            };

            let start = move || async {
                let op = self.send().await?;
                Ok(Operation::new(op))
            };

            lro::new_poller(polling_policy, polling_backoff_policy, start, query)
        }

        /// Sets the value of [function][crate::model::UpdateFunctionRequest::function].
        pub fn set_function<T: Into<std::option::Option<crate::model::Function>>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.function = v.into();
            self
        }

        /// Sets the value of [update_mask][crate::model::UpdateFunctionRequest::update_mask].
        pub fn set_update_mask<T: Into<std::option::Option<wkt::FieldMask>>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.update_mask = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for UpdateFunction {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a FunctionService::delete_function call.
    #[derive(Clone, Debug)]
    pub struct DeleteFunction(RequestBuilder<crate::model::DeleteFunctionRequest>);

    impl DeleteFunction {
        pub(crate) fn new(stub: Arc<dyn crate::traits::dyntraits::FunctionService>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::DeleteFunctionRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        ///
        /// # Long running operations
        ///
        /// This starts, but does not poll, a longrunning operation. More information
        /// on [delete_function][crate::client::FunctionService::delete_function].
        pub async fn send(self) -> Result<longrunning::model::Operation> {
            (*self.0.stub)
                .delete_function(self.0.request, self.0.options)
                .await
        }

        /// Creates a [Poller][lro::Poller] to work with `delete_function`.
        pub fn poller(self) -> impl lro::Poller<wkt::Empty, crate::model::OperationMetadata> {
            type Operation = lro::Operation<wkt::Empty, crate::model::OperationMetadata>;
            let polling_policy = self.0.stub.get_polling_policy(&self.0.options);
            let polling_backoff_policy = self.0.stub.get_polling_backoff_policy(&self.0.options);

            let stub = self.0.stub.clone();
            let mut options = self.0.options.clone();
            options.set_retry_policy(gax::retry_policy::NeverRetry);
            let query = move |name| {
                let stub = stub.clone();
                let options = options.clone();
                async {
                    let op = GetOperation::new(stub)
                        .set_name(name)
                        .with_options(options)
                        .send()
                        .await?;
                    Ok(Operation::new(op))
                }
            };

            let start = move || async {
                let op = self.send().await?;
                Ok(Operation::new(op))
            };

            lro::new_poller(polling_policy, polling_backoff_policy, start, query)
        }

        /// Sets the value of [name][crate::model::DeleteFunctionRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for DeleteFunction {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a FunctionService::generate_upload_url call.
    #[derive(Clone, Debug)]
    pub struct GenerateUploadUrl(RequestBuilder<crate::model::GenerateUploadUrlRequest>);

    impl GenerateUploadUrl {
        pub(crate) fn new(stub: Arc<dyn crate::traits::dyntraits::FunctionService>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::GenerateUploadUrlRequest>>(
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
        pub async fn send(self) -> Result<crate::model::GenerateUploadUrlResponse> {
            (*self.0.stub)
                .generate_upload_url(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [parent][crate::model::GenerateUploadUrlRequest::parent].
        pub fn set_parent<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parent = v.into();
            self
        }

        /// Sets the value of [kms_key_name][crate::model::GenerateUploadUrlRequest::kms_key_name].
        pub fn set_kms_key_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.kms_key_name = v.into();
            self
        }

        /// Sets the value of [environment][crate::model::GenerateUploadUrlRequest::environment].
        pub fn set_environment<T: Into<crate::model::Environment>>(mut self, v: T) -> Self {
            self.0.request.environment = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for GenerateUploadUrl {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a FunctionService::generate_download_url call.
    #[derive(Clone, Debug)]
    pub struct GenerateDownloadUrl(RequestBuilder<crate::model::GenerateDownloadUrlRequest>);

    impl GenerateDownloadUrl {
        pub(crate) fn new(stub: Arc<dyn crate::traits::dyntraits::FunctionService>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::GenerateDownloadUrlRequest>>(
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
        pub async fn send(self) -> Result<crate::model::GenerateDownloadUrlResponse> {
            (*self.0.stub)
                .generate_download_url(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][crate::model::GenerateDownloadUrlRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for GenerateDownloadUrl {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a FunctionService::list_runtimes call.
    #[derive(Clone, Debug)]
    pub struct ListRuntimes(RequestBuilder<crate::model::ListRuntimesRequest>);

    impl ListRuntimes {
        pub(crate) fn new(stub: Arc<dyn crate::traits::dyntraits::FunctionService>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::ListRuntimesRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::ListRuntimesResponse> {
            (*self.0.stub)
                .list_runtimes(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [parent][crate::model::ListRuntimesRequest::parent].
        pub fn set_parent<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parent = v.into();
            self
        }

        /// Sets the value of [filter][crate::model::ListRuntimesRequest::filter].
        pub fn set_filter<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.filter = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for ListRuntimes {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a FunctionService::list_locations call.
    #[derive(Clone, Debug)]
    pub struct ListLocations(RequestBuilder<location::model::ListLocationsRequest>);

    impl ListLocations {
        pub(crate) fn new(stub: Arc<dyn crate::traits::dyntraits::FunctionService>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<location::model::ListLocationsRequest>>(
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
        pub async fn send(self) -> Result<location::model::ListLocationsResponse> {
            (*self.0.stub)
                .list_locations(self.0.request, self.0.options)
                .await
        }

        /// Streams the responses back.
        #[cfg(feature = "unstable-stream")]
        pub async fn stream(
            self,
        ) -> gax::paginator::Paginator<location::model::ListLocationsResponse, gax::error::Error>
        {
            let token = gax::paginator::extract_token(&self.0.request.page_token);
            let execute = move |token: String| {
                let builder = self.clone();
                builder.0.request.clone().set_page_token(token);
                builder.send()
            };
            gax::paginator::Paginator::new(token, execute)
        }

        /// Sets the value of [name][location::model::ListLocationsRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }

        /// Sets the value of [filter][location::model::ListLocationsRequest::filter].
        pub fn set_filter<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.filter = v.into();
            self
        }

        /// Sets the value of [page_size][location::model::ListLocationsRequest::page_size].
        pub fn set_page_size<T: Into<i32>>(mut self, v: T) -> Self {
            self.0.request.page_size = v.into();
            self
        }

        /// Sets the value of [page_token][location::model::ListLocationsRequest::page_token].
        pub fn set_page_token<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.page_token = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for ListLocations {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a FunctionService::set_iam_policy call.
    #[derive(Clone, Debug)]
    pub struct SetIamPolicy(RequestBuilder<iam_v1::model::SetIamPolicyRequest>);

    impl SetIamPolicy {
        pub(crate) fn new(stub: Arc<dyn crate::traits::dyntraits::FunctionService>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<iam_v1::model::SetIamPolicyRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<iam_v1::model::Policy> {
            (*self.0.stub)
                .set_iam_policy(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [resource][iam_v1::model::SetIamPolicyRequest::resource].
        pub fn set_resource<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.resource = v.into();
            self
        }

        /// Sets the value of [policy][iam_v1::model::SetIamPolicyRequest::policy].
        pub fn set_policy<T: Into<std::option::Option<iam_v1::model::Policy>>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.policy = v.into();
            self
        }

        /// Sets the value of [update_mask][iam_v1::model::SetIamPolicyRequest::update_mask].
        pub fn set_update_mask<T: Into<std::option::Option<wkt::FieldMask>>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.update_mask = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for SetIamPolicy {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a FunctionService::get_iam_policy call.
    #[derive(Clone, Debug)]
    pub struct GetIamPolicy(RequestBuilder<iam_v1::model::GetIamPolicyRequest>);

    impl GetIamPolicy {
        pub(crate) fn new(stub: Arc<dyn crate::traits::dyntraits::FunctionService>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<iam_v1::model::GetIamPolicyRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<iam_v1::model::Policy> {
            (*self.0.stub)
                .get_iam_policy(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [resource][iam_v1::model::GetIamPolicyRequest::resource].
        pub fn set_resource<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.resource = v.into();
            self
        }

        /// Sets the value of [options][iam_v1::model::GetIamPolicyRequest::options].
        pub fn set_options<T: Into<std::option::Option<iam_v1::model::GetPolicyOptions>>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.options = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for GetIamPolicy {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a FunctionService::test_iam_permissions call.
    #[derive(Clone, Debug)]
    pub struct TestIamPermissions(RequestBuilder<iam_v1::model::TestIamPermissionsRequest>);

    impl TestIamPermissions {
        pub(crate) fn new(stub: Arc<dyn crate::traits::dyntraits::FunctionService>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<iam_v1::model::TestIamPermissionsRequest>>(
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
        pub async fn send(self) -> Result<iam_v1::model::TestIamPermissionsResponse> {
            (*self.0.stub)
                .test_iam_permissions(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [resource][iam_v1::model::TestIamPermissionsRequest::resource].
        pub fn set_resource<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.resource = v.into();
            self
        }

        /// Sets the value of [permissions][iam_v1::model::TestIamPermissionsRequest::permissions].
        pub fn set_permissions<T, V>(mut self, v: T) -> Self
        where
            T: std::iter::IntoIterator<Item = V>,
            V: std::convert::Into<std::string::String>,
        {
            use std::iter::Iterator;
            self.0.request.permissions = v.into_iter().map(|i| i.into()).collect();
            self
        }
    }

    impl gax::options::RequestBuilder for TestIamPermissions {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a FunctionService::list_operations call.
    #[derive(Clone, Debug)]
    pub struct ListOperations(RequestBuilder<longrunning::model::ListOperationsRequest>);

    impl ListOperations {
        pub(crate) fn new(stub: Arc<dyn crate::traits::dyntraits::FunctionService>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<longrunning::model::ListOperationsRequest>>(
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
        pub async fn send(self) -> Result<longrunning::model::ListOperationsResponse> {
            (*self.0.stub)
                .list_operations(self.0.request, self.0.options)
                .await
        }

        /// Streams the responses back.
        #[cfg(feature = "unstable-stream")]
        pub async fn stream(
            self,
        ) -> gax::paginator::Paginator<longrunning::model::ListOperationsResponse, gax::error::Error>
        {
            let token = gax::paginator::extract_token(&self.0.request.page_token);
            let execute = move |token: String| {
                let builder = self.clone();
                builder.0.request.clone().set_page_token(token);
                builder.send()
            };
            gax::paginator::Paginator::new(token, execute)
        }

        /// Sets the value of [name][longrunning::model::ListOperationsRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }

        /// Sets the value of [filter][longrunning::model::ListOperationsRequest::filter].
        pub fn set_filter<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.filter = v.into();
            self
        }

        /// Sets the value of [page_size][longrunning::model::ListOperationsRequest::page_size].
        pub fn set_page_size<T: Into<i32>>(mut self, v: T) -> Self {
            self.0.request.page_size = v.into();
            self
        }

        /// Sets the value of [page_token][longrunning::model::ListOperationsRequest::page_token].
        pub fn set_page_token<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.page_token = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for ListOperations {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a FunctionService::get_operation call.
    #[derive(Clone, Debug)]
    pub struct GetOperation(RequestBuilder<longrunning::model::GetOperationRequest>);

    impl GetOperation {
        pub(crate) fn new(stub: Arc<dyn crate::traits::dyntraits::FunctionService>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<longrunning::model::GetOperationRequest>>(
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
        pub async fn send(self) -> Result<longrunning::model::Operation> {
            (*self.0.stub)
                .get_operation(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][longrunning::model::GetOperationRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for GetOperation {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }
}
