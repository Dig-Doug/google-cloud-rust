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

/// Implements a [Workstations](crate::stubs::Workstations) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct Workstations<T>
where
    T: crate::stubs::Workstations + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> Workstations<T>
where
    T: crate::stubs::Workstations + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> crate::stubs::Workstations for Workstations<T>
where
    T: crate::stubs::Workstations + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn get_workstation_cluster(
        &self,
        req: crate::model::GetWorkstationClusterRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::WorkstationCluster> {
        self.inner.get_workstation_cluster(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_workstation_clusters(
        &self,
        req: crate::model::ListWorkstationClustersRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListWorkstationClustersResponse> {
        self.inner.list_workstation_clusters(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_workstation_cluster(
        &self,
        req: crate::model::CreateWorkstationClusterRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_workstation_cluster(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_workstation_cluster(
        &self,
        req: crate::model::UpdateWorkstationClusterRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.update_workstation_cluster(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_workstation_cluster(
        &self,
        req: crate::model::DeleteWorkstationClusterRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_workstation_cluster(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_workstation_config(
        &self,
        req: crate::model::GetWorkstationConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::WorkstationConfig> {
        self.inner.get_workstation_config(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_workstation_configs(
        &self,
        req: crate::model::ListWorkstationConfigsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListWorkstationConfigsResponse> {
        self.inner.list_workstation_configs(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_usable_workstation_configs(
        &self,
        req: crate::model::ListUsableWorkstationConfigsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListUsableWorkstationConfigsResponse> {
        self.inner
            .list_usable_workstation_configs(req, options)
            .await
    }

    #[tracing::instrument(ret)]
    async fn create_workstation_config(
        &self,
        req: crate::model::CreateWorkstationConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_workstation_config(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_workstation_config(
        &self,
        req: crate::model::UpdateWorkstationConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.update_workstation_config(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_workstation_config(
        &self,
        req: crate::model::DeleteWorkstationConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_workstation_config(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_workstation(
        &self,
        req: crate::model::GetWorkstationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Workstation> {
        self.inner.get_workstation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_workstations(
        &self,
        req: crate::model::ListWorkstationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListWorkstationsResponse> {
        self.inner.list_workstations(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_usable_workstations(
        &self,
        req: crate::model::ListUsableWorkstationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListUsableWorkstationsResponse> {
        self.inner.list_usable_workstations(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_workstation(
        &self,
        req: crate::model::CreateWorkstationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_workstation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_workstation(
        &self,
        req: crate::model::UpdateWorkstationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.update_workstation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_workstation(
        &self,
        req: crate::model::DeleteWorkstationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_workstation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn start_workstation(
        &self,
        req: crate::model::StartWorkstationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.start_workstation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn stop_workstation(
        &self,
        req: crate::model::StopWorkstationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.stop_workstation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn generate_access_token(
        &self,
        req: crate::model::GenerateAccessTokenRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::GenerateAccessTokenResponse> {
        self.inner.generate_access_token(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn set_iam_policy(
        &self,
        req: iam_v1::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<iam_v1::model::Policy> {
        self.inner.set_iam_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_iam_policy(
        &self,
        req: iam_v1::model::GetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<iam_v1::model::Policy> {
        self.inner.get_iam_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn test_iam_permissions(
        &self,
        req: iam_v1::model::TestIamPermissionsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<iam_v1::model::TestIamPermissionsResponse> {
        self.inner.test_iam_permissions(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::ListOperationsResponse> {
        self.inner.list_operations(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.get_operation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        self.inner.delete_operation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        self.inner.cancel_operation(req, options).await
    }

    fn get_polling_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_policy::PollingPolicy> {
        self.inner.get_polling_policy(options)
    }

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy> {
        self.inner.get_polling_backoff_policy(options)
    }
}
