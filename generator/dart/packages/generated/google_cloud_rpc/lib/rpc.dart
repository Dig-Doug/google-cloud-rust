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

/// The Google Cloud client for the Google RPC Types.
///
/// Defines RPC types.
library;

import 'dart:typed_data';

import 'package:google_cloud_protobuf/protobuf.dart';

/// Describes the cause of the error with structured details.
/// 
/// Example of an error when contacting the "pubsub.googleapis.com" API when it
/// is not enabled:
/// 
///     { "reason": "API_DISABLED"
///       "domain": "googleapis.com"
///       "metadata": {
///         "resource": "projects/123",
///         "service": "pubsub.googleapis.com"
///       }
///     }
/// 
/// This response indicates that the pubsub.googleapis.com API is not enabled.
/// 
/// Example of an error that is returned when attempting to create a Spanner
/// instance in a region that is out of stock:
/// 
///     { "reason": "STOCKOUT"
///       "domain": "spanner.googleapis.com",
///       "metadata": {
///         "availableRegions": "us-central1,us-east2"
///       }
///     }
class ErrorInfo {

  /// The reason of the error. This is a constant value that identifies the
  /// proximate cause of the error. Error reasons are unique within a particular
  /// domain of errors. This should be at most 63 characters and match a
  /// regular expression of `[A-Z][A-Z0-9_]+[A-Z0-9]`, which represents
  /// UPPER_SNAKE_CASE.
  final String? reason;

  /// The logical grouping to which the "reason" belongs. The error domain
  /// is typically the registered service name of the tool or product that
  /// generates the error. Example: "pubsub.googleapis.com". If the error is
  /// generated by some common infrastructure, the error domain must be a
  /// globally unique value that identifies the infrastructure. For Google API
  /// infrastructure, the error domain is "googleapis.com".
  final String? domain;

  /// Additional structured details about this error.
  /// 
  /// Keys must match a regular expression of `[a-z][a-zA-Z0-9-_]+` but should
  /// ideally be lowerCamelCase. Also, they must be limited to 64 characters in
  /// length. When identifying the current value of an exceeded limit, the units
  /// should be contained in the key, not the value.  For example, rather than
  /// `{"instanceLimit": "100/request"}`, should be returned as,
  /// `{"instanceLimitPerRequest": "100"}`, if the client exceeds the number of
  /// instances that can be created in a single (batch) request.
  final Map<String, String>? metadata;

  ErrorInfo({
    this.reason,
    this.domain,
    this.metadata,
  });
}

/// Describes when the clients can retry a failed request. Clients could ignore
/// the recommendation here or retry when this information is missing from error
/// responses.
/// 
/// It's always recommended that clients should use exponential backoff when
/// retrying.
/// 
/// Clients should wait until `retry_delay` amount of time has passed since
/// receiving the error response before retrying.  If retrying requests also
/// fail, clients should use an exponential backoff scheme to gradually increase
/// the delay between retries based on `retry_delay`, until either a maximum
/// number of retries have been reached or a maximum retry delay cap has been
/// reached.
class RetryInfo {

  /// Clients should wait at least this long between retrying the same request.
  final PbDuration? retryDelay;

  RetryInfo({
    this.retryDelay,
  });
}

/// Describes additional debugging info.
class DebugInfo {

  /// The stack trace entries indicating where the error occurred.
  final List<String>? stackEntries;

  /// Additional debugging information provided by the server.
  final String? detail;

  DebugInfo({
    this.stackEntries,
    this.detail,
  });
}

/// Describes how a quota check failed.
/// 
/// For example if a daily limit was exceeded for the calling project,
/// a service could respond with a QuotaFailure detail containing the project
/// id and the description of the quota limit that was exceeded.  If the
/// calling project hasn't enabled the service in the developer console, then
/// a service could respond with the project id and set `service_disabled`
/// to true.
/// 
/// Also see RetryInfo and Help types for other details about handling a
/// quota failure.
class QuotaFailure {

  /// Describes all quota violations.
  final List<QuotaFailure$Violation>? violations;

  QuotaFailure({
    this.violations,
  });
}

/// A message type used to describe a single quota violation.  For example, a
/// daily quota or a custom quota that was exceeded.
class QuotaFailure$Violation {

  /// The subject on which the quota check failed.
  /// For example, "clientip:<ip address of client>" or "project:<Google
  /// developer project id>".
  final String? subject;

  /// A description of how the quota check failed. Clients can use this
  /// description to find more about the quota configuration in the service's
  /// public documentation, or find the relevant quota limit to adjust through
  /// developer console.
  /// 
  /// For example: "Service disabled" or "Daily Limit for read operations
  /// exceeded".
  final String? description;

  QuotaFailure$Violation({
    this.subject,
    this.description,
  });
}

/// Describes what preconditions have failed.
/// 
/// For example, if an RPC failed because it required the Terms of Service to be
/// acknowledged, it could list the terms of service violation in the
/// PreconditionFailure message.
class PreconditionFailure {

  /// Describes all precondition violations.
  final List<PreconditionFailure$Violation>? violations;

  PreconditionFailure({
    this.violations,
  });
}

/// A message type used to describe a single precondition failure.
class PreconditionFailure$Violation {

  /// The type of PreconditionFailure. We recommend using a service-specific
  /// enum type to define the supported precondition violation subjects. For
  /// example, "TOS" for "Terms of Service violation".
  final String? type;

  /// The subject, relative to the type, that failed.
  /// For example, "google.com/cloud" relative to the "TOS" type would indicate
  /// which terms of service is being referenced.
  final String? subject;

  /// A description of how the precondition failed. Developers can use this
  /// description to understand how to fix the failure.
  /// 
  /// For example: "Terms of service not accepted".
  final String? description;

  PreconditionFailure$Violation({
    this.type,
    this.subject,
    this.description,
  });
}

/// Describes violations in a client request. This error type focuses on the
/// syntactic aspects of the request.
class BadRequest {

  /// Describes all violations in a client request.
  final List<BadRequest$FieldViolation>? fieldViolations;

  BadRequest({
    this.fieldViolations,
  });
}

/// A message type used to describe a single bad request field.
class BadRequest$FieldViolation {

  /// A path that leads to a field in the request body. The value will be a
  /// sequence of dot-separated identifiers that identify a protocol buffer
  /// field.
  /// 
  /// Consider the following:
  /// 
  ///     message CreateContactRequest {
  ///       message EmailAddress {
  ///         enum Type {
  ///           TYPE_UNSPECIFIED = 0;
  ///           HOME = 1;
  ///           WORK = 2;
  ///         }
  /// 
  ///         optional string email = 1;
  ///         repeated EmailType type = 2;
  ///       }
  /// 
  ///       string full_name = 1;
  ///       repeated EmailAddress email_addresses = 2;
  ///     }
  /// 
  /// In this example, in proto `field` could take one of the following values:
  /// 
  /// * `full_name` for a violation in the `full_name` value
  /// * `email_addresses[1].email` for a violation in the `email` field of the
  ///   first `email_addresses` message
  /// * `email_addresses[3].type[2]` for a violation in the second `type`
  ///   value in the third `email_addresses` message.
  /// 
  /// In JSON, the same values are represented as:
  /// 
  /// * `fullName` for a violation in the `fullName` value
  /// * `emailAddresses[1].email` for a violation in the `email` field of the
  ///   first `emailAddresses` message
  /// * `emailAddresses[3].type[2]` for a violation in the second `type`
  ///   value in the third `emailAddresses` message.
  final String? field;

  /// A description of why the request element is bad.
  final String? description;

  /// The reason of the field-level error. This is a constant value that
  /// identifies the proximate cause of the field-level error. It should
  /// uniquely identify the type of the FieldViolation within the scope of the
  /// google.rpc.ErrorInfo.domain. This should be at most 63
  /// characters and match a regular expression of `[A-Z][A-Z0-9_]+[A-Z0-9]`,
  /// which represents UPPER_SNAKE_CASE.
  final String? reason;

  /// Provides a localized error message for field-level errors that is safe to
  /// return to the API consumer.
  final LocalizedMessage? localizedMessage;

  BadRequest$FieldViolation({
    this.field,
    this.description,
    this.reason,
    this.localizedMessage,
  });
}

/// Contains metadata about the request that clients can attach when filing a bug
/// or providing other forms of feedback.
class RequestInfo {

  /// An opaque string that should only be interpreted by the service generating
  /// it. For example, it can be used to identify requests in the service's logs.
  final String? requestId;

  /// Any data that was used to serve this request. For example, an encrypted
  /// stack trace that can be sent back to the service provider for debugging.
  final String? servingData;

  RequestInfo({
    this.requestId,
    this.servingData,
  });
}

/// Describes the resource that is being accessed.
class ResourceInfo {

  /// A name for the type of resource being accessed, e.g. "sql table",
  /// "cloud storage bucket", "file", "Google calendar"; or the type URL
  /// of the resource: e.g. "type.googleapis.com/google.pubsub.v1.Topic".
  final String? resourceType;

  /// The name of the resource being accessed.  For example, a shared calendar
  /// name: "example.com_4fghdhgsrgh@group.calendar.google.com", if the current
  /// error is
  /// [google.rpc.Code.PERMISSION_DENIED][google.rpc.Code.PERMISSION_DENIED].
  final String? resourceName;

  /// The owner of the resource (optional).
  /// For example, "user:<owner email>" or "project:<Google developer project
  /// id>".
  final String? owner;

  /// Describes what error is encountered when accessing this resource.
  /// For example, updating a cloud project may require the `writer` permission
  /// on the developer console project.
  final String? description;

  ResourceInfo({
    this.resourceType,
    this.resourceName,
    this.owner,
    this.description,
  });
}

/// Provides links to documentation or for performing an out of band action.
/// 
/// For example, if a quota check failed with an error indicating the calling
/// project hasn't enabled the accessed service, this can contain a URL pointing
/// directly to the right place in the developer console to flip the bit.
class Help {

  /// URL(s) pointing to additional information on handling the current error.
  final List<Help$Link>? links;

  Help({
    this.links,
  });
}

/// Describes a URL link.
class Help$Link {

  /// Describes what the link offers.
  final String? description;

  /// The URL of the link.
  final String? url;

  Help$Link({
    this.description,
    this.url,
  });
}

/// Provides a localized error message that is safe to return to the user
/// which can be attached to an RPC error.
class LocalizedMessage {

  /// The locale used following the specification defined at
  /// https://www.rfc-editor.org/rfc/bcp/bcp47.txt.
  /// Examples are: "en-US", "fr-CH", "es-MX"
  final String? locale;

  /// The localized error message in the above locale.
  final String? message;

  LocalizedMessage({
    this.locale,
    this.message,
  });
}

/// Represents an HTTP request.
class HttpRequest {

  /// The HTTP request method.
  final String? method;

  /// The HTTP request URI.
  final String? uri;

  /// The HTTP request headers. The ordering of the headers is significant.
  /// Multiple headers with the same key may present for the request.
  final List<HttpHeader>? headers;

  /// The HTTP request body. If the body is not expected, it should be empty.
  final Uint8List? body;

  HttpRequest({
    this.method,
    this.uri,
    this.headers,
    this.body,
  });
}

/// Represents an HTTP response.
class HttpResponse {

  /// The HTTP status code, such as 200 or 404.
  final int? status;

  /// The HTTP reason phrase, such as "OK" or "Not Found".
  final String? reason;

  /// The HTTP response headers. The ordering of the headers is significant.
  /// Multiple headers with the same key may present for the response.
  final List<HttpHeader>? headers;

  /// The HTTP response body. If the body is not expected, it should be empty.
  final Uint8List? body;

  HttpResponse({
    this.status,
    this.reason,
    this.headers,
    this.body,
  });
}

/// Represents an HTTP header.
class HttpHeader {

  /// The HTTP header key. It is case insensitive.
  final String? key;

  /// The HTTP header value.
  final String? value;

  HttpHeader({
    this.key,
    this.value,
  });
}

/// The `Status` type defines a logical error model that is suitable for
/// different programming environments, including REST APIs and RPC APIs. It is
/// used by [gRPC](https://github.com/grpc). Each `Status` message contains
/// three pieces of data: error code, error message, and error details.
/// 
/// You can find out more about this error model and how to work with it in the
/// [API Design Guide](https://cloud.google.com/apis/design/errors).
class Status {

  /// The status code, which should be an enum value of
  /// [google.rpc.Code][google.rpc.Code].
  final int? code;

  /// A developer-facing error message, which should be in English. Any
  /// user-facing error message should be localized and sent in the
  /// [google.rpc.Status.details][google.rpc.Status.details] field, or localized
  /// by the client.
  final String? message;

  /// A list of messages that carry the error details.  There is a common set of
  /// message types for APIs to use.
  final List<Any>? details;

  Status({
    this.code,
    this.message,
    this.details,
  });
}

/// The canonical error codes for gRPC APIs.
/// 
/// 
/// Sometimes multiple error codes may apply.  Services should return
/// the most specific error code that applies.  For example, prefer
/// `OUT_OF_RANGE` over `FAILED_PRECONDITION` if both codes apply.
/// Similarly prefer `NOT_FOUND` or `ALREADY_EXISTS` over `FAILED_PRECONDITION`.
class Code {
  /// Not an error; returned on success.
  /// 
  /// HTTP Mapping: 200 OK
  static const Code ok = Code('OK');

  /// The operation was cancelled, typically by the caller.
  /// 
  /// HTTP Mapping: 499 Client Closed Request
  static const Code cancelled = Code('CANCELLED');

  /// Unknown error.  For example, this error may be returned when
  /// a `Status` value received from another address space belongs to
  /// an error space that is not known in this address space.  Also
  /// errors raised by APIs that do not return enough error information
  /// may be converted to this error.
  /// 
  /// HTTP Mapping: 500 Internal Server Error
  static const Code unknown = Code('UNKNOWN');

  /// The client specified an invalid argument.  Note that this differs
  /// from `FAILED_PRECONDITION`.  `INVALID_ARGUMENT` indicates arguments
  /// that are problematic regardless of the state of the system
  /// (e.g., a malformed file name).
  /// 
  /// HTTP Mapping: 400 Bad Request
  static const Code invalidArgument = Code('INVALID_ARGUMENT');

  /// The deadline expired before the operation could complete. For operations
  /// that change the state of the system, this error may be returned
  /// even if the operation has completed successfully.  For example, a
  /// successful response from a server could have been delayed long
  /// enough for the deadline to expire.
  /// 
  /// HTTP Mapping: 504 Gateway Timeout
  static const Code deadlineExceeded = Code('DEADLINE_EXCEEDED');

  /// Some requested entity (e.g., file or directory) was not found.
  /// 
  /// Note to server developers: if a request is denied for an entire class
  /// of users, such as gradual feature rollout or undocumented allowlist,
  /// `NOT_FOUND` may be used. If a request is denied for some users within
  /// a class of users, such as user-based access control, `PERMISSION_DENIED`
  /// must be used.
  /// 
  /// HTTP Mapping: 404 Not Found
  static const Code notFound = Code('NOT_FOUND');

  /// The entity that a client attempted to create (e.g., file or directory)
  /// already exists.
  /// 
  /// HTTP Mapping: 409 Conflict
  static const Code alreadyExists = Code('ALREADY_EXISTS');

  /// The caller does not have permission to execute the specified
  /// operation. `PERMISSION_DENIED` must not be used for rejections
  /// caused by exhausting some resource (use `RESOURCE_EXHAUSTED`
  /// instead for those errors). `PERMISSION_DENIED` must not be
  /// used if the caller can not be identified (use `UNAUTHENTICATED`
  /// instead for those errors). This error code does not imply the
  /// request is valid or the requested entity exists or satisfies
  /// other pre-conditions.
  /// 
  /// HTTP Mapping: 403 Forbidden
  static const Code permissionDenied = Code('PERMISSION_DENIED');

  /// The request does not have valid authentication credentials for the
  /// operation.
  /// 
  /// HTTP Mapping: 401 Unauthorized
  static const Code unauthenticated = Code('UNAUTHENTICATED');

  /// Some resource has been exhausted, perhaps a per-user quota, or
  /// perhaps the entire file system is out of space.
  /// 
  /// HTTP Mapping: 429 Too Many Requests
  static const Code resourceExhausted = Code('RESOURCE_EXHAUSTED');

  /// The operation was rejected because the system is not in a state
  /// required for the operation's execution.  For example, the directory
  /// to be deleted is non-empty, an rmdir operation is applied to
  /// a non-directory, etc.
  /// 
  /// Service implementors can use the following guidelines to decide
  /// between `FAILED_PRECONDITION`, `ABORTED`, and `UNAVAILABLE`:
  ///  (a) Use `UNAVAILABLE` if the client can retry just the failing call.
  ///  (b) Use `ABORTED` if the client should retry at a higher level. For
  ///      example, when a client-specified test-and-set fails, indicating the
  ///      client should restart a read-modify-write sequence.
  ///  (c) Use `FAILED_PRECONDITION` if the client should not retry until
  ///      the system state has been explicitly fixed. For example, if an "rmdir"
  ///      fails because the directory is non-empty, `FAILED_PRECONDITION`
  ///      should be returned since the client should not retry unless
  ///      the files are deleted from the directory.
  /// 
  /// HTTP Mapping: 400 Bad Request
  static const Code failedPrecondition = Code('FAILED_PRECONDITION');

  /// The operation was aborted, typically due to a concurrency issue such as
  /// a sequencer check failure or transaction abort.
  /// 
  /// See the guidelines above for deciding between `FAILED_PRECONDITION`,
  /// `ABORTED`, and `UNAVAILABLE`.
  /// 
  /// HTTP Mapping: 409 Conflict
  static const Code aborted = Code('ABORTED');

  /// The operation was attempted past the valid range.  E.g., seeking or
  /// reading past end-of-file.
  /// 
  /// Unlike `INVALID_ARGUMENT`, this error indicates a problem that may
  /// be fixed if the system state changes. For example, a 32-bit file
  /// system will generate `INVALID_ARGUMENT` if asked to read at an
  /// offset that is not in the range [0,2^32-1], but it will generate
  /// `OUT_OF_RANGE` if asked to read from an offset past the current
  /// file size.
  /// 
  /// There is a fair bit of overlap between `FAILED_PRECONDITION` and
  /// `OUT_OF_RANGE`.  We recommend using `OUT_OF_RANGE` (the more specific
  /// error) when it applies so that callers who are iterating through
  /// a space can easily look for an `OUT_OF_RANGE` error to detect when
  /// they are done.
  /// 
  /// HTTP Mapping: 400 Bad Request
  static const Code outOfRange = Code('OUT_OF_RANGE');

  /// The operation is not implemented or is not supported/enabled in this
  /// service.
  /// 
  /// HTTP Mapping: 501 Not Implemented
  static const Code unimplemented = Code('UNIMPLEMENTED');

  /// Internal errors.  This means that some invariants expected by the
  /// underlying system have been broken.  This error code is reserved
  /// for serious errors.
  /// 
  /// HTTP Mapping: 500 Internal Server Error
  static const Code internal = Code('INTERNAL');

  /// The service is currently unavailable.  This is most likely a
  /// transient condition, which can be corrected by retrying with
  /// a backoff. Note that it is not always safe to retry
  /// non-idempotent operations.
  /// 
  /// See the guidelines above for deciding between `FAILED_PRECONDITION`,
  /// `ABORTED`, and `UNAVAILABLE`.
  /// 
  /// HTTP Mapping: 503 Service Unavailable
  static const Code unavailable = Code('UNAVAILABLE');

  /// Unrecoverable data loss or corruption.
  /// 
  /// HTTP Mapping: 500 Internal Server Error
  static const Code dataLoss = Code('DATA_LOSS');

  final String value;

  const Code(this.value);
}
