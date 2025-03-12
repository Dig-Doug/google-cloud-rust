// Copyright 2024 Google LLC
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

impl wkt::prost::Convert<ErrorInfo> for crate::error::rpc::generated::ErrorInfo {
    fn cnv(self) -> ErrorInfo {
        ErrorInfo {
            reason: self.reason.cnv(),
            domain: self.domain.cnv(),
            metadata: self.metadata.into_iter().map(|(k, v)| (k.cnv(), v.cnv())).collect(),
        }
    }
}

impl wkt::prost::Convert<crate::error::rpc::generated::ErrorInfo> for ErrorInfo {
    fn cnv(self) -> crate::error::rpc::generated::ErrorInfo {
        crate::error::rpc::generated::ErrorInfo::new()
            .set_reason(self.reason)
            .set_domain(self.domain)
            .set_metadata(self.metadata.into_iter().map(|(k, v)| (k.cnv(), v.cnv())))
    }
}

impl wkt::prost::Convert<RetryInfo> for crate::error::rpc::generated::RetryInfo {
    fn cnv(self) -> RetryInfo {
        RetryInfo {
            retry_delay: self.retry_delay.map(|v| v.cnv()),
        }
    }
}

impl wkt::prost::Convert<crate::error::rpc::generated::RetryInfo> for RetryInfo {
    fn cnv(self) -> crate::error::rpc::generated::RetryInfo {
        crate::error::rpc::generated::RetryInfo::new()
            .set_retry_delay(self.retry_delay.map(|v| v.cnv()))
    }
}

impl wkt::prost::Convert<DebugInfo> for crate::error::rpc::generated::DebugInfo {
    fn cnv(self) -> DebugInfo {
        DebugInfo {
            detail: self.detail.cnv(),
            stack_entries: self.stack_entries.into_iter().map(|v| v.cnv()).collect(),
        }
    }
}

impl wkt::prost::Convert<crate::error::rpc::generated::DebugInfo> for DebugInfo {
    fn cnv(self) -> crate::error::rpc::generated::DebugInfo {
        crate::error::rpc::generated::DebugInfo::new()
            .set_detail(self.detail)
            .set_stack_entries(self.stack_entries.into_iter().map(|v| v.cnv()))
    }
}

impl wkt::prost::Convert<quota_failure::Violation> for crate::error::rpc::generated::quota_failure::Violation {
    fn cnv(self) -> quota_failure::Violation {
        quota_failure::Violation {
            subject: self.subject.cnv(),
            description: self.description.cnv(),
        }
    }
}

impl wkt::prost::Convert<crate::error::rpc::generated::quota_failure::Violation> for quota_failure::Violation {
    fn cnv(self) -> crate::error::rpc::generated::quota_failure::Violation {
        crate::error::rpc::generated::quota_failure::Violation::new()
            .set_subject(self.subject)
            .set_description(self.description)
    }
}

impl wkt::prost::Convert<QuotaFailure> for crate::error::rpc::generated::QuotaFailure {
    fn cnv(self) -> QuotaFailure {
        QuotaFailure {
            violations: self.violations.into_iter().map(|v| v.cnv()).collect(),
        }
    }
}

impl wkt::prost::Convert<crate::error::rpc::generated::QuotaFailure> for QuotaFailure {
    fn cnv(self) -> crate::error::rpc::generated::QuotaFailure {
        crate::error::rpc::generated::QuotaFailure::new()
            .set_violations(self.violations.into_iter().map(|v| v.cnv()))
    }
}

impl wkt::prost::Convert<precondition_failure::Violation> for crate::error::rpc::generated::precondition_failure::Violation {
    fn cnv(self) -> precondition_failure::Violation {
        precondition_failure::Violation {
            r#type: self.r#type.cnv(),
            subject: self.subject.cnv(),
            description: self.description.cnv(),
        }
    }
}

impl wkt::prost::Convert<crate::error::rpc::generated::precondition_failure::Violation> for precondition_failure::Violation {
    fn cnv(self) -> crate::error::rpc::generated::precondition_failure::Violation {
        crate::error::rpc::generated::precondition_failure::Violation::new()
            .set_type(self.r#type)
            .set_subject(self.subject)
            .set_description(self.description)
    }
}

impl wkt::prost::Convert<PreconditionFailure> for crate::error::rpc::generated::PreconditionFailure {
    fn cnv(self) -> PreconditionFailure {
        PreconditionFailure {
            violations: self.violations.into_iter().map(|v| v.cnv()).collect(),
        }
    }
}

impl wkt::prost::Convert<crate::error::rpc::generated::PreconditionFailure> for PreconditionFailure {
    fn cnv(self) -> crate::error::rpc::generated::PreconditionFailure {
        crate::error::rpc::generated::PreconditionFailure::new()
            .set_violations(self.violations.into_iter().map(|v| v.cnv()))
    }
}

impl wkt::prost::Convert<bad_request::FieldViolation> for crate::error::rpc::generated::bad_request::FieldViolation {
    fn cnv(self) -> bad_request::FieldViolation {
        bad_request::FieldViolation {
            field: self.field.cnv(),
            description: self.description.cnv(),
        }
    }
}

impl wkt::prost::Convert<crate::error::rpc::generated::bad_request::FieldViolation> for bad_request::FieldViolation {
    fn cnv(self) -> crate::error::rpc::generated::bad_request::FieldViolation {
        crate::error::rpc::generated::bad_request::FieldViolation::new()
            .set_field(self.field)
            .set_description(self.description)
    }
}

impl wkt::prost::Convert<BadRequest> for crate::error::rpc::generated::BadRequest {
    fn cnv(self) -> BadRequest {
        BadRequest {
            field_violations: self.field_violations.into_iter().map(|v| v.cnv()).collect(),
        }
    }
}

impl wkt::prost::Convert<crate::error::rpc::generated::BadRequest> for BadRequest {
    fn cnv(self) -> crate::error::rpc::generated::BadRequest {
        crate::error::rpc::generated::BadRequest::new()
            .set_field_violations(self.field_violations.into_iter().map(|v| v.cnv()))
    }
}

impl wkt::prost::Convert<RequestInfo> for crate::error::rpc::generated::RequestInfo {
    fn cnv(self) -> RequestInfo {
        RequestInfo {
            request_id: self.request_id.cnv(),
            serving_data: self.serving_data.cnv(),
        }
    }
}

impl wkt::prost::Convert<crate::error::rpc::generated::RequestInfo> for RequestInfo {
    fn cnv(self) -> crate::error::rpc::generated::RequestInfo {
        crate::error::rpc::generated::RequestInfo::new()
            .set_request_id(self.request_id)
            .set_serving_data(self.serving_data)
    }
}

impl wkt::prost::Convert<ResourceInfo> for crate::error::rpc::generated::ResourceInfo {
    fn cnv(self) -> ResourceInfo {
        ResourceInfo {
            resource_type: self.resource_type.cnv(),
            resource_name: self.resource_name.cnv(),
            owner: self.owner.cnv(),
            description: self.description.cnv(),
        }
    }
}

impl wkt::prost::Convert<crate::error::rpc::generated::ResourceInfo> for ResourceInfo {
    fn cnv(self) -> crate::error::rpc::generated::ResourceInfo {
        crate::error::rpc::generated::ResourceInfo::new()
            .set_resource_type(self.resource_type)
            .set_resource_name(self.resource_name)
            .set_owner(self.owner)
            .set_description(self.description)
    }
}

impl wkt::prost::Convert<help::Link> for crate::error::rpc::generated::help::Link {
    fn cnv(self) -> help::Link {
        help::Link {
            description: self.description.cnv(),
            url: self.url.cnv(),
        }
    }
}

impl wkt::prost::Convert<crate::error::rpc::generated::help::Link> for help::Link {
    fn cnv(self) -> crate::error::rpc::generated::help::Link {
        crate::error::rpc::generated::help::Link::new()
            .set_description(self.description)
            .set_url(self.url)
    }
}

impl wkt::prost::Convert<Help> for crate::error::rpc::generated::Help {
    fn cnv(self) -> Help {
        Help {
            links: self.links.into_iter().map(|v| v.cnv()).collect(),
        }
    }
}

impl wkt::prost::Convert<crate::error::rpc::generated::Help> for Help {
    fn cnv(self) -> crate::error::rpc::generated::Help {
        crate::error::rpc::generated::Help::new()
            .set_links(self.links.into_iter().map(|v| v.cnv()))
    }
}

impl wkt::prost::Convert<LocalizedMessage> for crate::error::rpc::generated::LocalizedMessage {
    fn cnv(self) -> LocalizedMessage {
        LocalizedMessage {
            locale: self.locale.cnv(),
            message: self.message.cnv(),
        }
    }
}

impl wkt::prost::Convert<crate::error::rpc::generated::LocalizedMessage> for LocalizedMessage {
    fn cnv(self) -> crate::error::rpc::generated::LocalizedMessage {
        crate::error::rpc::generated::LocalizedMessage::new()
            .set_locale(self.locale)
            .set_message(self.message)
    }
}
