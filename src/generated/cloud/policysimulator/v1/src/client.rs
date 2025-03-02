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
#![allow(rustdoc::broken_intra_doc_links)]

use crate::Result;
use std::sync::Arc;

/// Implements a client for the Policy Simulator API.
///
/// # Service Description
///
/// Policy Simulator API service.
///
/// Policy Simulator is a collection of endpoints for creating, running, and
/// viewing a [Replay][google.cloud.policysimulator.v1.Replay]. A
/// [Replay][google.cloud.policysimulator.v1.Replay] is a type of simulation that
/// lets you see how your principals' access to resources might change if you
/// changed your IAM policy.
///
/// During a [Replay][google.cloud.policysimulator.v1.Replay], Policy Simulator
/// re-evaluates, or replays, past access attempts under both the current policy
/// and  your proposed policy, and compares those results to determine how your
/// principals' access might change under the proposed policy.
///
/// [google.cloud.policysimulator.v1.Replay]: crate::model::Replay
///
/// # Configuration
///
/// `Simulator` has various configuration parameters, the defaults should
/// work with most applications.
///
/// # Pooling and Cloning
///
/// `Simulator` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `Simulator` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct Simulator {
    inner: Arc<dyn crate::stubs::dynamic::Simulator>,
}

impl Simulator {
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
        T: crate::stubs::Simulator + 'static,
    {
        Self {
            inner: Arc::new(stub),
        }
    }

    async fn build_inner(
        conf: gax::options::ClientConfig,
    ) -> Result<Arc<dyn crate::stubs::dynamic::Simulator>> {
        if conf.tracing_enabled() {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gax::options::ClientConfig,
    ) -> Result<impl crate::stubs::Simulator> {
        crate::transport::Simulator::new(conf).await
    }

    async fn build_with_tracing(
        conf: gax::options::ClientConfig,
    ) -> Result<impl crate::stubs::Simulator> {
        Self::build_transport(conf)
            .await
            .map(crate::tracing::Simulator::new)
    }

    /// Gets the specified [Replay][google.cloud.policysimulator.v1.Replay]. Each
    /// `Replay` is available for at least 7 days.
    ///
    /// [google.cloud.policysimulator.v1.Replay]: crate::model::Replay
    pub fn get_replay(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::simulator::GetReplay {
        crate::builders::simulator::GetReplay::new(self.inner.clone()).set_name(name.into())
    }

    /// Creates and starts a [Replay][google.cloud.policysimulator.v1.Replay] using
    /// the given [ReplayConfig][google.cloud.policysimulator.v1.ReplayConfig].
    ///
    /// [google.cloud.policysimulator.v1.Replay]: crate::model::Replay
    /// [google.cloud.policysimulator.v1.ReplayConfig]: crate::model::ReplayConfig
    ///
    /// # Long running operations
    ///
    /// Calling [poller()] on the resulting builder returns an implementation of
    /// the [lro::Poller] trait. You need to call `Poller::poll` on this
    /// `Poller` at least once to start the LRO. You may periodically poll this
    /// object to find the status of the operation. The poller automatically
    /// extract the final response value and any intermediate metadata values.
    ///
    /// Calling [send()] on the resulting builder starts a LRO (long-Running
    /// Operation). LROs run in the background, and the application may poll
    /// them periodically to find out if they have succeeded, or failed. See
    /// below for instructions on how to manually use the resulting [Operation].
    /// We recommend `poller()` in favor of `send()`.
    ///
    /// ## Polling until completion
    ///
    /// Applications that do not care about intermediate results in a
    /// long-running operation may use the [until_done()] function:
    ///
    /// ```
    /// # use gax::Result;
    /// # use google_cloud_policysimulator_v1::model;
    /// async fn wait(
    ///     mut poller: impl lro::Poller<model::Replay, model::ReplayOperationMetadata>
    /// ) -> Result<model::Replay> {
    ///     poller.until_done().await
    /// }
    /// ```
    ///
    /// This will wait until the LRO completes (successfully or with an error).
    /// Applications can set the [PollingPolicy] and [PollingBackoffPolicy] to
    /// control for how long the function runs.
    ///
    /// ## Polling with detailed metadata updates
    ///
    /// Using the result of [poller()] follows a common pattern:
    ///
    /// ```
    /// # use gax::Result;
    /// # use google_cloud_policysimulator_v1::model;
    /// async fn wait(
    ///     mut poller: impl lro::Poller<model::Replay, model::ReplayOperationMetadata>
    /// ) -> Result<model::Replay> {
    ///     while let Some(p) = poller.poll().await {
    ///         match p {
    ///             lro::PollingResult::Completed(r) => { return r; },
    ///             lro::PollingResult::InProgress(m) => { println!("in progress {m:?}"); },
    ///             lro::PollingResult::PollingError(_) => { /* ignored */ },
    ///         }
    ///         tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    ///     }
    ///     Err(gax::error::Error::other("LRO never completed"))
    /// }
    /// ```
    ///
    /// ## Manually polling long-running operations
    ///
    /// If you call [send()], you need to examine the contents of the resulting
    /// [Operation][longrunning::model::Operation] to determine the result of
    /// the operation.
    ///
    /// If the `done` field is `true`, the operation has completed. The `result`
    /// field contains the final response, this will be a [crate::model::Replay] (as
    /// an [Any]), or the error (as a `Status`).
    ///
    /// If the `done` field is `false`, the operation has not completed.  The
    /// operation may also include a [crate::model::ReplayOperationMetadata] value in the `metadata`
    /// field. This value would also be encoded as an [Any]. The metadata may
    /// include information about how much progress the LRO has made.
    ///
    /// To find out if the operation has completed, use the [get_operation]
    /// method and repeat the steps outlined above.
    ///
    /// Note that most errors on [get_operation] do not indicate that the
    /// long-running operation failed. Long-running operation failures return
    /// the error status in the [result] field.
    ///
    /// [send()]: crate::builders::simulator::CreateReplay::send
    /// [poller()]: crate::builders::simulator::CreateReplay::poller
    /// [until_done()]: lro::Poller::until_done
    /// [PollingPolicy]: gax::polling_policy::PollingPolicy
    /// [PollingBackoffPolicy]: gax::polling_backoff_policy::PollingBackoffPolicy
    /// [get_operation]: Self::get_operation
    /// [metadata]: longrunning::model::Operation::result
    /// [name]: longrunning::model::Operation::name
    /// [Operation]: longrunning::model::Operation
    /// [result]: longrunning::model::Operation::result
    /// [Any]: wkt::Any
    pub fn create_replay(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::simulator::CreateReplay {
        crate::builders::simulator::CreateReplay::new(self.inner.clone()).set_parent(parent.into())
    }

    /// Lists the results of running a
    /// [Replay][google.cloud.policysimulator.v1.Replay].
    ///
    /// [google.cloud.policysimulator.v1.Replay]: crate::model::Replay
    pub fn list_replay_results(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::simulator::ListReplayResults {
        crate::builders::simulator::ListReplayResults::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn list_operations(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::simulator::ListOperations {
        crate::builders::simulator::ListOperations::new(self.inner.clone()).set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn get_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::simulator::GetOperation {
        crate::builders::simulator::GetOperation::new(self.inner.clone()).set_name(name.into())
    }
}
