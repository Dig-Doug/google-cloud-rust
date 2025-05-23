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

use crate::Result;
#[allow(unused_imports)]
use gax::error::Error;
use gaxi::prost::Convert;

const DEFAULT_HOST: &str = "https://storage.googleapis.com";

mod info {
    const NAME: &str = env!("CARGO_PKG_NAME");
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    lazy_static::lazy_static! {
        pub(crate) static ref X_GOOG_API_CLIENT_HEADER: String = {
            let ac = gaxi::api_header::XGoogApiClient{
                name:          NAME,
                version:       VERSION,
                library_type:  gaxi::api_header::GAPIC,
            };
            ac.header_value()
        };
    }
}

/// Implements [Storage](super::stub::Storage) using a Tonic-generated client.
#[derive(Clone)]
pub struct Storage {
    inner: gaxi::grpc::Client,
}

impl std::fmt::Debug for Storage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.debug_struct("Storage")
            .field("inner", &self.inner)
            .finish()
    }
}

impl Storage {
    pub async fn new(config: gaxi::options::ClientConfig) -> Result<Self> {
        let inner = gaxi::grpc::Client::new(config, DEFAULT_HOST).await?;
        Ok(Self { inner })
    }
}

impl super::stub::Storage for Storage {
    async fn delete_bucket(
        &self,
        req: crate::model::DeleteBucketRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<()>> {
        let options = gax::options::internal::set_default_idempotency(options, false);
        let extensions = {
            let mut e = tonic::Extensions::new();
            e.insert(tonic::GrpcMethod::new(
                "google.storage.v2.Storage",
                "DeleteBucket",
            ));
            e
        };
        let path = http::uri::PathAndQuery::from_static("/google.storage.v2.Storage/DeleteBucket");
        let x_goog_request_params = [""; 0].into_iter().fold(String::new(), |b, p| b + "&" + &p);

        self.inner
            .execute(
                extensions,
                path,
                req.cnv(),
                options,
                &info::X_GOOG_API_CLIENT_HEADER,
                &x_goog_request_params,
            )
            .await
            .map(gaxi::grpc::to_gax_response::<(), ()>)
    }

    async fn get_bucket(
        &self,
        req: crate::model::GetBucketRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Bucket>> {
        let options = gax::options::internal::set_default_idempotency(options, false);
        let extensions = {
            let mut e = tonic::Extensions::new();
            e.insert(tonic::GrpcMethod::new(
                "google.storage.v2.Storage",
                "GetBucket",
            ));
            e
        };
        let path = http::uri::PathAndQuery::from_static("/google.storage.v2.Storage/GetBucket");
        let x_goog_request_params = [""; 0].into_iter().fold(String::new(), |b, p| b + "&" + &p);

        self.inner
            .execute(
                extensions,
                path,
                req.cnv(),
                options,
                &info::X_GOOG_API_CLIENT_HEADER,
                &x_goog_request_params,
            )
            .await
            .map(
                gaxi::grpc::to_gax_response::<
                    crate::google::storage::v2::Bucket,
                    crate::model::Bucket,
                >,
            )
    }

    async fn create_bucket(
        &self,
        req: crate::model::CreateBucketRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Bucket>> {
        let options = gax::options::internal::set_default_idempotency(options, false);
        let extensions = {
            let mut e = tonic::Extensions::new();
            e.insert(tonic::GrpcMethod::new(
                "google.storage.v2.Storage",
                "CreateBucket",
            ));
            e
        };
        let path = http::uri::PathAndQuery::from_static("/google.storage.v2.Storage/CreateBucket");
        let x_goog_request_params = [""; 0].into_iter().fold(String::new(), |b, p| b + "&" + &p);

        self.inner
            .execute(
                extensions,
                path,
                req.cnv(),
                options,
                &info::X_GOOG_API_CLIENT_HEADER,
                &x_goog_request_params,
            )
            .await
            .map(
                gaxi::grpc::to_gax_response::<
                    crate::google::storage::v2::Bucket,
                    crate::model::Bucket,
                >,
            )
    }

    async fn list_buckets(
        &self,
        req: crate::model::ListBucketsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListBucketsResponse>> {
        let options = gax::options::internal::set_default_idempotency(options, false);
        let extensions = {
            let mut e = tonic::Extensions::new();
            e.insert(tonic::GrpcMethod::new(
                "google.storage.v2.Storage",
                "ListBuckets",
            ));
            e
        };
        let path = http::uri::PathAndQuery::from_static("/google.storage.v2.Storage/ListBuckets");
        let x_goog_request_params = [""; 0].into_iter().fold(String::new(), |b, p| b + "&" + &p);

        self.inner
            .execute(
                extensions,
                path,
                req.cnv(),
                options,
                &info::X_GOOG_API_CLIENT_HEADER,
                &x_goog_request_params,
            )
            .await
            .map(
                gaxi::grpc::to_gax_response::<
                    crate::google::storage::v2::ListBucketsResponse,
                    crate::model::ListBucketsResponse,
                >,
            )
    }

    async fn lock_bucket_retention_policy(
        &self,
        req: crate::model::LockBucketRetentionPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Bucket>> {
        let options = gax::options::internal::set_default_idempotency(options, false);
        let extensions = {
            let mut e = tonic::Extensions::new();
            e.insert(tonic::GrpcMethod::new(
                "google.storage.v2.Storage",
                "LockBucketRetentionPolicy",
            ));
            e
        };
        let path = http::uri::PathAndQuery::from_static(
            "/google.storage.v2.Storage/LockBucketRetentionPolicy",
        );
        let x_goog_request_params = [""; 0].into_iter().fold(String::new(), |b, p| b + "&" + &p);

        self.inner
            .execute(
                extensions,
                path,
                req.cnv(),
                options,
                &info::X_GOOG_API_CLIENT_HEADER,
                &x_goog_request_params,
            )
            .await
            .map(
                gaxi::grpc::to_gax_response::<
                    crate::google::storage::v2::Bucket,
                    crate::model::Bucket,
                >,
            )
    }

    async fn update_bucket(
        &self,
        req: crate::model::UpdateBucketRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Bucket>> {
        let options = gax::options::internal::set_default_idempotency(options, false);
        let extensions = {
            let mut e = tonic::Extensions::new();
            e.insert(tonic::GrpcMethod::new(
                "google.storage.v2.Storage",
                "UpdateBucket",
            ));
            e
        };
        let path = http::uri::PathAndQuery::from_static("/google.storage.v2.Storage/UpdateBucket");
        let x_goog_request_params = [""; 0].into_iter().fold(String::new(), |b, p| b + "&" + &p);

        self.inner
            .execute(
                extensions,
                path,
                req.cnv(),
                options,
                &info::X_GOOG_API_CLIENT_HEADER,
                &x_goog_request_params,
            )
            .await
            .map(
                gaxi::grpc::to_gax_response::<
                    crate::google::storage::v2::Bucket,
                    crate::model::Bucket,
                >,
            )
    }

    async fn compose_object(
        &self,
        req: crate::model::ComposeObjectRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Object>> {
        let options = gax::options::internal::set_default_idempotency(options, false);
        let extensions = {
            let mut e = tonic::Extensions::new();
            e.insert(tonic::GrpcMethod::new(
                "google.storage.v2.Storage",
                "ComposeObject",
            ));
            e
        };
        let path = http::uri::PathAndQuery::from_static("/google.storage.v2.Storage/ComposeObject");
        let x_goog_request_params = [""; 0].into_iter().fold(String::new(), |b, p| b + "&" + &p);

        self.inner
            .execute(
                extensions,
                path,
                req.cnv(),
                options,
                &info::X_GOOG_API_CLIENT_HEADER,
                &x_goog_request_params,
            )
            .await
            .map(
                gaxi::grpc::to_gax_response::<
                    crate::google::storage::v2::Object,
                    crate::model::Object,
                >,
            )
    }

    async fn delete_object(
        &self,
        req: crate::model::DeleteObjectRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<()>> {
        let options = gax::options::internal::set_default_idempotency(options, false);
        let extensions = {
            let mut e = tonic::Extensions::new();
            e.insert(tonic::GrpcMethod::new(
                "google.storage.v2.Storage",
                "DeleteObject",
            ));
            e
        };
        let path = http::uri::PathAndQuery::from_static("/google.storage.v2.Storage/DeleteObject");
        let x_goog_request_params = [""; 0].into_iter().fold(String::new(), |b, p| b + "&" + &p);

        self.inner
            .execute(
                extensions,
                path,
                req.cnv(),
                options,
                &info::X_GOOG_API_CLIENT_HEADER,
                &x_goog_request_params,
            )
            .await
            .map(gaxi::grpc::to_gax_response::<(), ()>)
    }

    async fn restore_object(
        &self,
        req: crate::model::RestoreObjectRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Object>> {
        let options = gax::options::internal::set_default_idempotency(options, false);
        let extensions = {
            let mut e = tonic::Extensions::new();
            e.insert(tonic::GrpcMethod::new(
                "google.storage.v2.Storage",
                "RestoreObject",
            ));
            e
        };
        let path = http::uri::PathAndQuery::from_static("/google.storage.v2.Storage/RestoreObject");
        let x_goog_request_params = [""; 0].into_iter().fold(String::new(), |b, p| b + "&" + &p);

        self.inner
            .execute(
                extensions,
                path,
                req.cnv(),
                options,
                &info::X_GOOG_API_CLIENT_HEADER,
                &x_goog_request_params,
            )
            .await
            .map(
                gaxi::grpc::to_gax_response::<
                    crate::google::storage::v2::Object,
                    crate::model::Object,
                >,
            )
    }

    async fn cancel_resumable_write(
        &self,
        req: crate::model::CancelResumableWriteRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::CancelResumableWriteResponse>> {
        let options = gax::options::internal::set_default_idempotency(options, false);
        let extensions = {
            let mut e = tonic::Extensions::new();
            e.insert(tonic::GrpcMethod::new(
                "google.storage.v2.Storage",
                "CancelResumableWrite",
            ));
            e
        };
        let path =
            http::uri::PathAndQuery::from_static("/google.storage.v2.Storage/CancelResumableWrite");
        let x_goog_request_params = [""; 0].into_iter().fold(String::new(), |b, p| b + "&" + &p);

        self.inner
            .execute(
                extensions,
                path,
                req.cnv(),
                options,
                &info::X_GOOG_API_CLIENT_HEADER,
                &x_goog_request_params,
            )
            .await
            .map(
                gaxi::grpc::to_gax_response::<
                    crate::google::storage::v2::CancelResumableWriteResponse,
                    crate::model::CancelResumableWriteResponse,
                >,
            )
    }

    async fn get_object(
        &self,
        req: crate::model::GetObjectRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Object>> {
        let options = gax::options::internal::set_default_idempotency(options, false);
        let extensions = {
            let mut e = tonic::Extensions::new();
            e.insert(tonic::GrpcMethod::new(
                "google.storage.v2.Storage",
                "GetObject",
            ));
            e
        };
        let path = http::uri::PathAndQuery::from_static("/google.storage.v2.Storage/GetObject");
        let x_goog_request_params = [""; 0].into_iter().fold(String::new(), |b, p| b + "&" + &p);

        self.inner
            .execute(
                extensions,
                path,
                req.cnv(),
                options,
                &info::X_GOOG_API_CLIENT_HEADER,
                &x_goog_request_params,
            )
            .await
            .map(
                gaxi::grpc::to_gax_response::<
                    crate::google::storage::v2::Object,
                    crate::model::Object,
                >,
            )
    }

    async fn update_object(
        &self,
        req: crate::model::UpdateObjectRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Object>> {
        let options = gax::options::internal::set_default_idempotency(options, false);
        let extensions = {
            let mut e = tonic::Extensions::new();
            e.insert(tonic::GrpcMethod::new(
                "google.storage.v2.Storage",
                "UpdateObject",
            ));
            e
        };
        let path = http::uri::PathAndQuery::from_static("/google.storage.v2.Storage/UpdateObject");
        let x_goog_request_params = [""; 0].into_iter().fold(String::new(), |b, p| b + "&" + &p);

        self.inner
            .execute(
                extensions,
                path,
                req.cnv(),
                options,
                &info::X_GOOG_API_CLIENT_HEADER,
                &x_goog_request_params,
            )
            .await
            .map(
                gaxi::grpc::to_gax_response::<
                    crate::google::storage::v2::Object,
                    crate::model::Object,
                >,
            )
    }

    async fn list_objects(
        &self,
        req: crate::model::ListObjectsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListObjectsResponse>> {
        let options = gax::options::internal::set_default_idempotency(options, false);
        let extensions = {
            let mut e = tonic::Extensions::new();
            e.insert(tonic::GrpcMethod::new(
                "google.storage.v2.Storage",
                "ListObjects",
            ));
            e
        };
        let path = http::uri::PathAndQuery::from_static("/google.storage.v2.Storage/ListObjects");
        let x_goog_request_params = [""; 0].into_iter().fold(String::new(), |b, p| b + "&" + &p);

        self.inner
            .execute(
                extensions,
                path,
                req.cnv(),
                options,
                &info::X_GOOG_API_CLIENT_HEADER,
                &x_goog_request_params,
            )
            .await
            .map(
                gaxi::grpc::to_gax_response::<
                    crate::google::storage::v2::ListObjectsResponse,
                    crate::model::ListObjectsResponse,
                >,
            )
    }

    async fn rewrite_object(
        &self,
        req: crate::model::RewriteObjectRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::RewriteResponse>> {
        let options = gax::options::internal::set_default_idempotency(options, false);
        let extensions = {
            let mut e = tonic::Extensions::new();
            e.insert(tonic::GrpcMethod::new(
                "google.storage.v2.Storage",
                "RewriteObject",
            ));
            e
        };
        let path = http::uri::PathAndQuery::from_static("/google.storage.v2.Storage/RewriteObject");
        let x_goog_request_params = [""; 0].into_iter().fold(String::new(), |b, p| b + "&" + &p);

        self.inner
            .execute(
                extensions,
                path,
                req.cnv(),
                options,
                &info::X_GOOG_API_CLIENT_HEADER,
                &x_goog_request_params,
            )
            .await
            .map(
                gaxi::grpc::to_gax_response::<
                    crate::google::storage::v2::RewriteResponse,
                    crate::model::RewriteResponse,
                >,
            )
    }

    async fn start_resumable_write(
        &self,
        req: crate::model::StartResumableWriteRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::StartResumableWriteResponse>> {
        let options = gax::options::internal::set_default_idempotency(options, false);
        let extensions = {
            let mut e = tonic::Extensions::new();
            e.insert(tonic::GrpcMethod::new(
                "google.storage.v2.Storage",
                "StartResumableWrite",
            ));
            e
        };
        let path =
            http::uri::PathAndQuery::from_static("/google.storage.v2.Storage/StartResumableWrite");
        let x_goog_request_params = [""; 0].into_iter().fold(String::new(), |b, p| b + "&" + &p);

        self.inner
            .execute(
                extensions,
                path,
                req.cnv(),
                options,
                &info::X_GOOG_API_CLIENT_HEADER,
                &x_goog_request_params,
            )
            .await
            .map(
                gaxi::grpc::to_gax_response::<
                    crate::google::storage::v2::StartResumableWriteResponse,
                    crate::model::StartResumableWriteResponse,
                >,
            )
    }

    async fn query_write_status(
        &self,
        req: crate::model::QueryWriteStatusRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::QueryWriteStatusResponse>> {
        let options = gax::options::internal::set_default_idempotency(options, false);
        let extensions = {
            let mut e = tonic::Extensions::new();
            e.insert(tonic::GrpcMethod::new(
                "google.storage.v2.Storage",
                "QueryWriteStatus",
            ));
            e
        };
        let path =
            http::uri::PathAndQuery::from_static("/google.storage.v2.Storage/QueryWriteStatus");
        let x_goog_request_params = [""; 0].into_iter().fold(String::new(), |b, p| b + "&" + &p);

        self.inner
            .execute(
                extensions,
                path,
                req.cnv(),
                options,
                &info::X_GOOG_API_CLIENT_HEADER,
                &x_goog_request_params,
            )
            .await
            .map(
                gaxi::grpc::to_gax_response::<
                    crate::google::storage::v2::QueryWriteStatusResponse,
                    crate::model::QueryWriteStatusResponse,
                >,
            )
    }

    async fn move_object(
        &self,
        req: crate::model::MoveObjectRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Object>> {
        let options = gax::options::internal::set_default_idempotency(options, false);
        let extensions = {
            let mut e = tonic::Extensions::new();
            e.insert(tonic::GrpcMethod::new(
                "google.storage.v2.Storage",
                "MoveObject",
            ));
            e
        };
        let path = http::uri::PathAndQuery::from_static("/google.storage.v2.Storage/MoveObject");
        let x_goog_request_params = [""; 0].into_iter().fold(String::new(), |b, p| b + "&" + &p);

        self.inner
            .execute(
                extensions,
                path,
                req.cnv(),
                options,
                &info::X_GOOG_API_CLIENT_HEADER,
                &x_goog_request_params,
            )
            .await
            .map(
                gaxi::grpc::to_gax_response::<
                    crate::google::storage::v2::Object,
                    crate::model::Object,
                >,
            )
    }
}
