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

pub use gax::error::Error;
pub use gax::Result;
pub(crate) mod generated;

pub mod model {
    pub use super::generated::model::*;
}

pub(crate) mod google {
    pub mod firestore {
        #[allow(clippy::enum_variant_names)]
        pub mod v1 {
            include!("generated/protos/google.firestore.v1.rs");
        }
    }
    pub mod rpc {
        include!("generated/protos/google.rpc.rs");
    }
    pub mod r#type {
        include!("generated/protos/google.r#type.rs");
    }
}
