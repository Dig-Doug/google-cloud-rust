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

/// The Google Cloud client for the Long Running Operations API.
///
/// Defines types and an abstract service to handle long-running operations.
///
/// [Long-running operations] are a common pattern to handle methods that may take
/// a significant amount of time to execute. Many Google APIs return an `Operation`
/// message (defined in this package) that are roughly analogous to a Future. The
/// operation will eventually complete, though it may still return an error on
/// completion. The client libraries provide helpers to simplify polling of these
/// operations.
///
/// [Long-running operations]: https://google.aip.dev/151
library;

import 'package:google_cloud_gax/gax.dart';
import 'package:google_cloud_gax/src/encoding.dart';
import 'package:google_cloud_protobuf/protobuf.dart';
import 'package:google_cloud_rpc/rpc.dart';
import 'package:http/http.dart' as http;

part 'src/longrunning.p.dart';

/// Manages long-running operations with an API service.
///
/// When an API method normally takes long time to complete, it can be designed
/// to return `Operation` to the client, and the
/// client can use this interface to receive the real response asynchronously by
/// polling the operation resource, or pass the operation resource to another API
/// (such as Pub/Sub API) to receive the response.  Any API service that returns
/// long-running operations should implement the `Operations` interface so
/// developers can have a consistent client experience.
class Operations {
  static const String _host = 'longrunning.googleapis.com';

  final ServiceClient _client;

  Operations({required http.Client client})
      : _client = ServiceClient(client: client);

  /// Lists operations that match the specified filter in the request. If the
  /// server doesn't support this method, it returns `UNIMPLEMENTED`.
  Future<ListOperationsResponse> listOperations(
      ListOperationsRequest request) async {
    final url = Uri.https(_host, '/v1/${request.name}');
    final response = await _client.get(url);
    return ListOperationsResponse.fromJson(response);
  }

  /// Gets the latest state of a long-running operation.  Clients can use this
  /// method to poll the operation result at intervals as recommended by the API
  /// service.
  Future<Operation> getOperation(GetOperationRequest request) async {
    final url = Uri.https(_host, '/v1/${request.name}');
    final response = await _client.get(url);
    return Operation.fromJson(response);
  }

  /// Deletes a long-running operation. This method indicates that the client is
  /// no longer interested in the operation result. It does not cancel the
  /// operation. If the server doesn't support this method, it returns
  /// `google.rpc.Code.UNIMPLEMENTED`.
  Future<void> deleteOperation(DeleteOperationRequest request) async {
    final url = Uri.https(_host, '/v1/${request.name}');
    await _client.delete(url);
  }

  /// Starts asynchronous cancellation on a long-running operation.  The server
  /// makes a best effort to cancel the operation, but success is not
  /// guaranteed.  If the server doesn't support this method, it returns
  /// `google.rpc.Code.UNIMPLEMENTED`.  Clients can use
  /// `Operations.GetOperation` or
  /// other methods to check whether the cancellation succeeded or whether the
  /// operation completed despite cancellation. On successful cancellation,
  /// the operation is not deleted; instead, it becomes an operation with
  /// an `Operation.error` value with a
  /// `google.rpc.Status.code` of `1`, corresponding to
  /// `Code.CANCELLED`.
  Future<void> cancelOperation(CancelOperationRequest request) async {
    final url = Uri.https(_host, '/v1/${request.name}:cancel');
    await _client.post(url, body: request);
  }

  /// Closes the client and cleans up any resources associated with it.
  ///
  /// Once [close] is called, no other methods should be called.
  void close() => _client.close();
}

/// The request message for
/// `Operations.GetOperation`.
class GetOperationRequest extends ProtoMessage {
  static const String fullyQualifiedName =
      'google.longrunning.GetOperationRequest';

  /// The name of the operation resource.
  final String name;

  GetOperationRequest({
    required this.name,
  }) : super(fullyQualifiedName);

  factory GetOperationRequest.fromJson(Map<String, dynamic> json) {
    return GetOperationRequest(
      name: json['name'],
    );
  }

  @override
  Object toJson() {
    return {
      'name': name,
    };
  }

  @override
  String toString() {
    final contents = [
      'name=$name',
    ].join(',');
    return 'GetOperationRequest($contents)';
  }
}

/// The request message for
/// `Operations.ListOperations`.
class ListOperationsRequest extends ProtoMessage {
  static const String fullyQualifiedName =
      'google.longrunning.ListOperationsRequest';

  /// The name of the operation's parent resource.
  final String name;

  /// The standard list filter.
  final String? filter;

  /// The standard list page size.
  final int? pageSize;

  /// The standard list page token.
  final String? pageToken;

  ListOperationsRequest({
    required this.name,
    this.filter,
    this.pageSize,
    this.pageToken,
  }) : super(fullyQualifiedName);

  factory ListOperationsRequest.fromJson(Map<String, dynamic> json) {
    return ListOperationsRequest(
      name: json['name'],
      filter: json['filter'],
      pageSize: json['pageSize'],
      pageToken: json['pageToken'],
    );
  }

  @override
  Object toJson() {
    return {
      'name': name,
      if (filter != null) 'filter': filter,
      if (pageSize != null) 'pageSize': pageSize,
      if (pageToken != null) 'pageToken': pageToken,
    };
  }

  @override
  String toString() {
    final contents = [
      'name=$name',
      if (filter != null) 'filter=$filter',
      if (pageSize != null) 'pageSize=$pageSize',
      if (pageToken != null) 'pageToken=$pageToken',
    ].join(',');
    return 'ListOperationsRequest($contents)';
  }
}

/// The response message for
/// `Operations.ListOperations`.
class ListOperationsResponse extends ProtoMessage {
  static const String fullyQualifiedName =
      'google.longrunning.ListOperationsResponse';

  /// A list of operations that matches the specified filter in the request.
  final List<Operation>? operations;

  /// The standard List next-page token.
  final String? nextPageToken;

  ListOperationsResponse({
    this.operations,
    this.nextPageToken,
  }) : super(fullyQualifiedName);

  factory ListOperationsResponse.fromJson(Map<String, dynamic> json) {
    return ListOperationsResponse(
      operations: decodeListMessage(json['operations'], Operation.fromJson),
      nextPageToken: json['nextPageToken'],
    );
  }

  @override
  Object toJson() {
    return {
      if (operations != null) 'operations': encodeList(operations),
      if (nextPageToken != null) 'nextPageToken': nextPageToken,
    };
  }

  @override
  String toString() {
    final contents = [
      if (nextPageToken != null) 'nextPageToken=$nextPageToken',
    ].join(',');
    return 'ListOperationsResponse($contents)';
  }
}

/// The request message for
/// `Operations.CancelOperation`.
class CancelOperationRequest extends ProtoMessage {
  static const String fullyQualifiedName =
      'google.longrunning.CancelOperationRequest';

  /// The name of the operation resource to be cancelled.
  final String name;

  CancelOperationRequest({
    required this.name,
  }) : super(fullyQualifiedName);

  factory CancelOperationRequest.fromJson(Map<String, dynamic> json) {
    return CancelOperationRequest(
      name: json['name'],
    );
  }

  @override
  Object toJson() {
    return {
      'name': name,
    };
  }

  @override
  String toString() {
    final contents = [
      'name=$name',
    ].join(',');
    return 'CancelOperationRequest($contents)';
  }
}

/// The request message for
/// `Operations.DeleteOperation`.
class DeleteOperationRequest extends ProtoMessage {
  static const String fullyQualifiedName =
      'google.longrunning.DeleteOperationRequest';

  /// The name of the operation resource to be deleted.
  final String name;

  DeleteOperationRequest({
    required this.name,
  }) : super(fullyQualifiedName);

  factory DeleteOperationRequest.fromJson(Map<String, dynamic> json) {
    return DeleteOperationRequest(
      name: json['name'],
    );
  }

  @override
  Object toJson() {
    return {
      'name': name,
    };
  }

  @override
  String toString() {
    final contents = [
      'name=$name',
    ].join(',');
    return 'DeleteOperationRequest($contents)';
  }
}

/// The request message for
/// `Operations.WaitOperation`.
class WaitOperationRequest extends ProtoMessage {
  static const String fullyQualifiedName =
      'google.longrunning.WaitOperationRequest';

  /// The name of the operation resource to wait on.
  final String? name;

  /// The maximum duration to wait before timing out. If left blank, the wait
  /// will be at most the time permitted by the underlying HTTP/RPC protocol.
  /// If RPC context deadline is also specified, the shorter one will be used.
  final Duration? timeout;

  WaitOperationRequest({
    this.name,
    this.timeout,
  }) : super(fullyQualifiedName);

  factory WaitOperationRequest.fromJson(Map<String, dynamic> json) {
    return WaitOperationRequest(
      name: json['name'],
      timeout: decodeCustom(json['timeout'], Duration.fromJson),
    );
  }

  @override
  Object toJson() {
    return {
      if (name != null) 'name': name,
      if (timeout != null) 'timeout': timeout!.toJson(),
    };
  }

  @override
  String toString() {
    final contents = [
      if (name != null) 'name=$name',
    ].join(',');
    return 'WaitOperationRequest($contents)';
  }
}

/// A message representing the message types used by a long-running operation.
///
/// Example:
///
///     rpc Export(ExportRequest) returns (google.longrunning.Operation) {
///       option (google.longrunning.operation_info) = {
///         response_type: "ExportResponse"
///         metadata_type: "ExportMetadata"
///       };
///     }
class OperationInfo extends ProtoMessage {
  static const String fullyQualifiedName = 'google.longrunning.OperationInfo';

  /// Required. The message name of the primary return type for this
  /// long-running operation.
  /// This type will be used to deserialize the LRO's response.
  ///
  /// If the response is in a different package from the rpc, a fully-qualified
  /// message name must be used (e.g. `google.protobuf.Struct`).
  ///
  /// Note: Altering this value constitutes a breaking change.
  final String? responseType;

  /// Required. The message name of the metadata type for this long-running
  /// operation.
  ///
  /// If the response is in a different package from the rpc, a fully-qualified
  /// message name must be used (e.g. `google.protobuf.Struct`).
  ///
  /// Note: Altering this value constitutes a breaking change.
  final String? metadataType;

  OperationInfo({
    this.responseType,
    this.metadataType,
  }) : super(fullyQualifiedName);

  factory OperationInfo.fromJson(Map<String, dynamic> json) {
    return OperationInfo(
      responseType: json['responseType'],
      metadataType: json['metadataType'],
    );
  }

  @override
  Object toJson() {
    return {
      if (responseType != null) 'responseType': responseType,
      if (metadataType != null) 'metadataType': metadataType,
    };
  }

  @override
  String toString() {
    final contents = [
      if (responseType != null) 'responseType=$responseType',
      if (metadataType != null) 'metadataType=$metadataType',
    ].join(',');
    return 'OperationInfo($contents)';
  }
}
