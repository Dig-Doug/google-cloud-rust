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

//! Traits to mock the clients in this library.
//!
//! Application developers may need to mock the clients in this library to test
//! how their application works with different (and sometimes hard to trigger)
//! client and service behavior. Such test can define mocks implementing the
//! trait(s) defined in this module, initialize the client with an instance of
//! this mock in their tests, and verify their application responds as expected.

#![allow(rustdoc::broken_intra_doc_links)]

use gax::error::Error;
use std::sync::Arc;

pub(crate) mod dynamic;

/// Defines the trait used to implement [crate::client::Speech].
///
/// Application developers may need to implement this trait to mock
/// `client::Speech`.  In other use-cases, application developers only
/// use `client::Speech` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
pub trait Speech: std::fmt::Debug + Send + Sync {
    /// Implements [crate::client::Speech::create_recognizer].
    fn create_recognizer(
        &self,
        _req: crate::model::CreateRecognizerRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::Speech::list_recognizers].
    fn list_recognizers(
        &self,
        _req: crate::model::ListRecognizersRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListRecognizersResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListRecognizersResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [crate::client::Speech::get_recognizer].
    fn get_recognizer(
        &self,
        _req: crate::model::GetRecognizerRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Recognizer>> + Send {
        std::future::ready::<crate::Result<crate::model::Recognizer>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::Speech::update_recognizer].
    fn update_recognizer(
        &self,
        _req: crate::model::UpdateRecognizerRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::Speech::delete_recognizer].
    fn delete_recognizer(
        &self,
        _req: crate::model::DeleteRecognizerRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::Speech::undelete_recognizer].
    fn undelete_recognizer(
        &self,
        _req: crate::model::UndeleteRecognizerRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::Speech::recognize].
    fn recognize(
        &self,
        _req: crate::model::RecognizeRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::RecognizeResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::RecognizeResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::Speech::batch_recognize].
    fn batch_recognize(
        &self,
        _req: crate::model::BatchRecognizeRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::Speech::get_config].
    fn get_config(
        &self,
        _req: crate::model::GetConfigRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Config>> + Send {
        std::future::ready::<crate::Result<crate::model::Config>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::Speech::update_config].
    fn update_config(
        &self,
        _req: crate::model::UpdateConfigRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Config>> + Send {
        std::future::ready::<crate::Result<crate::model::Config>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::Speech::create_custom_class].
    fn create_custom_class(
        &self,
        _req: crate::model::CreateCustomClassRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::Speech::list_custom_classes].
    fn list_custom_classes(
        &self,
        _req: crate::model::ListCustomClassesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListCustomClassesResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListCustomClassesResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [crate::client::Speech::get_custom_class].
    fn get_custom_class(
        &self,
        _req: crate::model::GetCustomClassRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::CustomClass>> + Send {
        std::future::ready::<crate::Result<crate::model::CustomClass>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::Speech::update_custom_class].
    fn update_custom_class(
        &self,
        _req: crate::model::UpdateCustomClassRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::Speech::delete_custom_class].
    fn delete_custom_class(
        &self,
        _req: crate::model::DeleteCustomClassRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::Speech::undelete_custom_class].
    fn undelete_custom_class(
        &self,
        _req: crate::model::UndeleteCustomClassRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::Speech::create_phrase_set].
    fn create_phrase_set(
        &self,
        _req: crate::model::CreatePhraseSetRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::Speech::list_phrase_sets].
    fn list_phrase_sets(
        &self,
        _req: crate::model::ListPhraseSetsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListPhraseSetsResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListPhraseSetsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [crate::client::Speech::get_phrase_set].
    fn get_phrase_set(
        &self,
        _req: crate::model::GetPhraseSetRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::PhraseSet>> + Send {
        std::future::ready::<crate::Result<crate::model::PhraseSet>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::Speech::update_phrase_set].
    fn update_phrase_set(
        &self,
        _req: crate::model::UpdatePhraseSetRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::Speech::delete_phrase_set].
    fn delete_phrase_set(
        &self,
        _req: crate::model::DeletePhraseSetRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::Speech::undelete_phrase_set].
    fn undelete_phrase_set(
        &self,
        _req: crate::model::UndeletePhraseSetRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::Speech::list_locations].
    fn list_locations(
        &self,
        _req: location::model::ListLocationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<location::model::ListLocationsResponse>> + Send
    {
        std::future::ready::<crate::Result<location::model::ListLocationsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [crate::client::Speech::get_location].
    fn get_location(
        &self,
        _req: location::model::GetLocationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<location::model::Location>> + Send {
        std::future::ready::<crate::Result<location::model::Location>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::Speech::list_operations].
    fn list_operations(
        &self,
        _req: longrunning::model::ListOperationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::ListOperationsResponse>>
           + Send {
        std::future::ready::<crate::Result<longrunning::model::ListOperationsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [crate::client::Speech::get_operation].
    fn get_operation(
        &self,
        _req: longrunning::model::GetOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::Speech::delete_operation].
    fn delete_operation(
        &self,
        _req: longrunning::model::DeleteOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Implements [crate::client::Speech::cancel_operation].
    fn cancel_operation(
        &self,
        _req: longrunning::model::CancelOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Returns the polling policy.
    ///
    /// When mocking, this method is typically irrelevant. Do not try to verify
    /// it is called by your mocks.
    fn get_polling_policy(
        &self,
        _options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_policy::PollingPolicy> {
        Arc::new(gax::polling_policy::Aip194Strict)
    }

    /// Returns the polling backoff policy.
    ///
    /// When mocking, this method is typically irrelevant. Do not try to verify
    /// it is called by your mocks.
    fn get_polling_backoff_policy(
        &self,
        _options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy> {
        Arc::new(gax::exponential_backoff::ExponentialBackoff::default())
    }
}
