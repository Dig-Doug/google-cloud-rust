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

/// Implements [DatabaseAdmin](super::stub::DatabaseAdmin) using a [gaxi::http::ReqwestClient].
#[derive(Clone)]
pub struct DatabaseAdmin {
    inner: gaxi::http::ReqwestClient,
}

impl std::fmt::Debug for DatabaseAdmin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.debug_struct("DatabaseAdmin")
            .field("inner", &self.inner)
            .finish()
    }
}

impl DatabaseAdmin {
    pub async fn new(config: gaxi::options::ClientConfig) -> Result<Self> {
        let inner = gaxi::http::ReqwestClient::new(config, crate::DEFAULT_HOST).await?;
        Ok(Self { inner })
    }
}

impl super::stub::DatabaseAdmin for DatabaseAdmin {
    async fn list_databases(
        &self,
        req: crate::model::ListDatabasesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListDatabasesResponse>> {
        let options = gax::options::internal::set_default_idempotency(options, true);
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!("/v1/{}/databases", req.parent),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = builder.query(&[("pageSize", &req.page_size)]);
        let builder = builder.query(&[("pageToken", &req.page_token)]);
        self.inner
            .execute(builder, None::<gaxi::http::NoBody>, options)
            .await
    }

    async fn create_database(
        &self,
        req: crate::model::CreateDatabaseRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        let options = gax::options::internal::set_default_idempotency(options, false);
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!("/v1/{}/databases", req.parent),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner.execute(builder, Some(req), options).await
    }

    async fn get_database(
        &self,
        req: crate::model::GetDatabaseRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Database>> {
        let options = gax::options::internal::set_default_idempotency(options, true);
        let builder = self
            .inner
            .builder(reqwest::Method::GET, format!("/v1/{}", req.name))
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, None::<gaxi::http::NoBody>, options)
            .await
    }

    async fn update_database(
        &self,
        req: crate::model::UpdateDatabaseRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        let options = gax::options::internal::set_default_idempotency(options, false);
        let builder = self
            .inner
            .builder(
                reqwest::Method::PATCH,
                format!(
                    "/v1/{}",
                    req.database
                        .as_ref()
                        .ok_or_else(|| gaxi::path_parameter::missing("database"))?
                        .name
                ),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = req
            .update_mask
            .as_ref()
            .iter()
            .flat_map(|p| p.paths.iter())
            .fold(builder, |builder, v| builder.query(&[("updateMask", v)]));
        self.inner
            .execute(builder, Some(req.database), options)
            .await
    }

    async fn update_database_ddl(
        &self,
        req: crate::model::UpdateDatabaseDdlRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        let options = gax::options::internal::set_default_idempotency(options, false);
        let builder = self
            .inner
            .builder(reqwest::Method::PATCH, format!("/v1/{}/ddl", req.database))
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner.execute(builder, Some(req), options).await
    }

    async fn drop_database(
        &self,
        req: crate::model::DropDatabaseRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<()>> {
        let options = gax::options::internal::set_default_idempotency(options, true);
        let builder = self
            .inner
            .builder(reqwest::Method::DELETE, format!("/v1/{}", req.database))
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, None::<gaxi::http::NoBody>, options)
            .await
            .map(|r: gax::response::Response<wkt::Empty>| {
                let (parts, _) = r.into_parts();
                gax::response::Response::from_parts(parts, ())
            })
    }

    async fn get_database_ddl(
        &self,
        req: crate::model::GetDatabaseDdlRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::GetDatabaseDdlResponse>> {
        let options = gax::options::internal::set_default_idempotency(options, true);
        let builder = self
            .inner
            .builder(reqwest::Method::GET, format!("/v1/{}/ddl", req.database))
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, None::<gaxi::http::NoBody>, options)
            .await
    }

    async fn set_iam_policy(
        &self,
        req: iam_v1::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<iam_v1::model::Policy>> {
        let options = gax::options::internal::set_default_idempotency(options, false);
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!("/v1/{}:setIamPolicy", req.resource),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner.execute(builder, Some(req), options).await
    }

    async fn get_iam_policy(
        &self,
        req: iam_v1::model::GetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<iam_v1::model::Policy>> {
        let options = gax::options::internal::set_default_idempotency(options, false);
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!("/v1/{}:getIamPolicy", req.resource),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner.execute(builder, Some(req), options).await
    }

    async fn test_iam_permissions(
        &self,
        req: iam_v1::model::TestIamPermissionsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<iam_v1::model::TestIamPermissionsResponse>> {
        let options = gax::options::internal::set_default_idempotency(options, false);
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!("/v1/{}:testIamPermissions", req.resource),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner.execute(builder, Some(req), options).await
    }

    async fn create_backup(
        &self,
        req: crate::model::CreateBackupRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        let options = gax::options::internal::set_default_idempotency(options, false);
        let builder = self
            .inner
            .builder(reqwest::Method::POST, format!("/v1/{}/backups", req.parent))
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = builder.query(&[("backupId", &req.backup_id)]);
        let builder = req
            .encryption_config
            .as_ref()
            .map(|p| serde_json::to_value(p).map_err(Error::serde))
            .transpose()?
            .into_iter()
            .fold(builder, |builder, v| {
                use gaxi::query_parameter::QueryParameter;
                v.add(builder, "encryptionConfig")
            });
        self.inner.execute(builder, Some(req.backup), options).await
    }

    async fn copy_backup(
        &self,
        req: crate::model::CopyBackupRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        let options = gax::options::internal::set_default_idempotency(options, false);
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!("/v1/{}/backups:copy", req.parent),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner.execute(builder, Some(req), options).await
    }

    async fn get_backup(
        &self,
        req: crate::model::GetBackupRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Backup>> {
        let options = gax::options::internal::set_default_idempotency(options, true);
        let builder = self
            .inner
            .builder(reqwest::Method::GET, format!("/v1/{}", req.name))
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, None::<gaxi::http::NoBody>, options)
            .await
    }

    async fn update_backup(
        &self,
        req: crate::model::UpdateBackupRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Backup>> {
        let options = gax::options::internal::set_default_idempotency(options, false);
        let builder = self
            .inner
            .builder(
                reqwest::Method::PATCH,
                format!(
                    "/v1/{}",
                    req.backup
                        .as_ref()
                        .ok_or_else(|| gaxi::path_parameter::missing("backup"))?
                        .name
                ),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = req
            .update_mask
            .as_ref()
            .iter()
            .flat_map(|p| p.paths.iter())
            .fold(builder, |builder, v| builder.query(&[("updateMask", v)]));
        self.inner.execute(builder, Some(req.backup), options).await
    }

    async fn delete_backup(
        &self,
        req: crate::model::DeleteBackupRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<()>> {
        let options = gax::options::internal::set_default_idempotency(options, true);
        let builder = self
            .inner
            .builder(reqwest::Method::DELETE, format!("/v1/{}", req.name))
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, None::<gaxi::http::NoBody>, options)
            .await
            .map(|r: gax::response::Response<wkt::Empty>| {
                let (parts, _) = r.into_parts();
                gax::response::Response::from_parts(parts, ())
            })
    }

    async fn list_backups(
        &self,
        req: crate::model::ListBackupsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListBackupsResponse>> {
        let options = gax::options::internal::set_default_idempotency(options, true);
        let builder = self
            .inner
            .builder(reqwest::Method::GET, format!("/v1/{}/backups", req.parent))
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = builder.query(&[("filter", &req.filter)]);
        let builder = builder.query(&[("pageSize", &req.page_size)]);
        let builder = builder.query(&[("pageToken", &req.page_token)]);
        self.inner
            .execute(builder, None::<gaxi::http::NoBody>, options)
            .await
    }

    async fn restore_database(
        &self,
        req: crate::model::RestoreDatabaseRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        let options = gax::options::internal::set_default_idempotency(options, false);
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!("/v1/{}/databases:restore", req.parent),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner.execute(builder, Some(req), options).await
    }

    async fn list_database_operations(
        &self,
        req: crate::model::ListDatabaseOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListDatabaseOperationsResponse>> {
        let options = gax::options::internal::set_default_idempotency(options, true);
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!("/v1/{}/databaseOperations", req.parent),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = builder.query(&[("filter", &req.filter)]);
        let builder = builder.query(&[("pageSize", &req.page_size)]);
        let builder = builder.query(&[("pageToken", &req.page_token)]);
        self.inner
            .execute(builder, None::<gaxi::http::NoBody>, options)
            .await
    }

    async fn list_backup_operations(
        &self,
        req: crate::model::ListBackupOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListBackupOperationsResponse>> {
        let options = gax::options::internal::set_default_idempotency(options, true);
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!("/v1/{}/backupOperations", req.parent),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = builder.query(&[("filter", &req.filter)]);
        let builder = builder.query(&[("pageSize", &req.page_size)]);
        let builder = builder.query(&[("pageToken", &req.page_token)]);
        self.inner
            .execute(builder, None::<gaxi::http::NoBody>, options)
            .await
    }

    async fn list_database_roles(
        &self,
        req: crate::model::ListDatabaseRolesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListDatabaseRolesResponse>> {
        let options = gax::options::internal::set_default_idempotency(options, true);
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!("/v1/{}/databaseRoles", req.parent),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = builder.query(&[("pageSize", &req.page_size)]);
        let builder = builder.query(&[("pageToken", &req.page_token)]);
        self.inner
            .execute(builder, None::<gaxi::http::NoBody>, options)
            .await
    }

    async fn add_split_points(
        &self,
        req: crate::model::AddSplitPointsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::AddSplitPointsResponse>> {
        let options = gax::options::internal::set_default_idempotency(options, false);
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!("/v1/{}:addSplitPoints", req.database),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner.execute(builder, Some(req), options).await
    }

    async fn create_backup_schedule(
        &self,
        req: crate::model::CreateBackupScheduleRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::BackupSchedule>> {
        let options = gax::options::internal::set_default_idempotency(options, false);
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!("/v1/{}/backupSchedules", req.parent),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = builder.query(&[("backupScheduleId", &req.backup_schedule_id)]);
        self.inner
            .execute(builder, Some(req.backup_schedule), options)
            .await
    }

    async fn get_backup_schedule(
        &self,
        req: crate::model::GetBackupScheduleRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::BackupSchedule>> {
        let options = gax::options::internal::set_default_idempotency(options, true);
        let builder = self
            .inner
            .builder(reqwest::Method::GET, format!("/v1/{}", req.name))
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, None::<gaxi::http::NoBody>, options)
            .await
    }

    async fn update_backup_schedule(
        &self,
        req: crate::model::UpdateBackupScheduleRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::BackupSchedule>> {
        let options = gax::options::internal::set_default_idempotency(options, false);
        let builder = self
            .inner
            .builder(
                reqwest::Method::PATCH,
                format!(
                    "/v1/{}",
                    req.backup_schedule
                        .as_ref()
                        .ok_or_else(|| gaxi::path_parameter::missing("backup_schedule"))?
                        .name
                ),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = req
            .update_mask
            .as_ref()
            .iter()
            .flat_map(|p| p.paths.iter())
            .fold(builder, |builder, v| builder.query(&[("updateMask", v)]));
        self.inner
            .execute(builder, Some(req.backup_schedule), options)
            .await
    }

    async fn delete_backup_schedule(
        &self,
        req: crate::model::DeleteBackupScheduleRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<()>> {
        let options = gax::options::internal::set_default_idempotency(options, true);
        let builder = self
            .inner
            .builder(reqwest::Method::DELETE, format!("/v1/{}", req.name))
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, None::<gaxi::http::NoBody>, options)
            .await
            .map(|r: gax::response::Response<wkt::Empty>| {
                let (parts, _) = r.into_parts();
                gax::response::Response::from_parts(parts, ())
            })
    }

    async fn list_backup_schedules(
        &self,
        req: crate::model::ListBackupSchedulesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListBackupSchedulesResponse>> {
        let options = gax::options::internal::set_default_idempotency(options, true);
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!("/v1/{}/backupSchedules", req.parent),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = builder.query(&[("pageSize", &req.page_size)]);
        let builder = builder.query(&[("pageToken", &req.page_token)]);
        self.inner
            .execute(builder, None::<gaxi::http::NoBody>, options)
            .await
    }

    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::ListOperationsResponse>> {
        let options = gax::options::internal::set_default_idempotency(options, true);
        let builder = self
            .inner
            .builder(reqwest::Method::GET, format!("/v1/{}", req.name))
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = builder.query(&[("filter", &req.filter)]);
        let builder = builder.query(&[("pageSize", &req.page_size)]);
        let builder = builder.query(&[("pageToken", &req.page_token)]);
        self.inner
            .execute(builder, None::<gaxi::http::NoBody>, options)
            .await
    }

    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        let options = gax::options::internal::set_default_idempotency(options, true);
        let builder = self
            .inner
            .builder(reqwest::Method::GET, format!("/v1/{}", req.name))
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, None::<gaxi::http::NoBody>, options)
            .await
    }

    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<()>> {
        let options = gax::options::internal::set_default_idempotency(options, true);
        let builder = self
            .inner
            .builder(reqwest::Method::DELETE, format!("/v1/{}", req.name))
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, None::<gaxi::http::NoBody>, options)
            .await
            .map(|r: gax::response::Response<wkt::Empty>| {
                let (parts, _) = r.into_parts();
                gax::response::Response::from_parts(parts, ())
            })
    }

    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<()>> {
        let options = gax::options::internal::set_default_idempotency(options, false);
        let builder = self
            .inner
            .builder(reqwest::Method::POST, format!("/v1/{}:cancel", req.name))
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, None::<gaxi::http::NoBody>, options)
            .await
            .map(|r: gax::response::Response<wkt::Empty>| {
                let (parts, _) = r.into_parts();
                gax::response::Response::from_parts(parts, ())
            })
    }

    fn get_polling_error_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_error_policy::PollingErrorPolicy> {
        self.inner.get_polling_error_policy(options)
    }

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy> {
        self.inner.get_polling_backoff_policy(options)
    }
}
