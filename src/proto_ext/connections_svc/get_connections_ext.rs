use crate::proto_ext::{
    DataGetter, DataSetter, ErrorGetter, ErrorSetter, HeaderGetter, NatsRequestSetter,
};
use crate::runtiva::{connections::v1 as proto_connections, nats::v1 as proto_nats};

// ***********************************  Request Getters ***********************************
// EditAbout Request Data message
impl DataGetter<proto_connections::GetConnectionsRequest>
    for proto_connections::NatsGetConnectionsRequest
{
    fn to_data(self) -> Option<proto_connections::GetConnectionsRequest> {
        self.data
    }
}

// EditAbout Request Headers
impl HeaderGetter for proto_connections::NatsGetConnectionsRequest {
    fn headers(&self) -> &[proto_nats::MetadataMap] {
        &self.headers
    }

    fn take_headers(&mut self) -> Vec<proto_nats::MetadataMap> {
        let mut swapped = vec![];
        std::mem::swap(&mut self.headers, &mut swapped);
        swapped
    }
}

// ********************************** NATS Request Setter **********************************
impl
    NatsRequestSetter<
        proto_connections::GetConnectionsRequest,
        proto_connections::NatsGetConnectionsRequest,
    > for proto_connections::NatsGetConnectionsRequest
{
    fn from_headers_and_message(
        headers: impl Into<Vec<proto_nats::MetadataMap>>,
        data: impl Into<proto_connections::GetConnectionsRequest>,
    ) -> Self {
        proto_connections::NatsGetConnectionsRequest {
            headers: headers.into(),
            data: Some(data.into()),
        }
    }
}

// ***********************************  Response Getters ***********************************
// Create Response Data getter
impl DataGetter<proto_connections::GetConnectionsResponse>
    for proto_connections::NatsGetConnectionsResponse
{
    fn to_data(self) -> Option<proto_connections::GetConnectionsResponse> {
        match self.msg {
            Some(proto_connections::nats_get_connections_response::Msg::Data(data)) => Some(data),
            _ => None,
        }
    }
}

// Create Response Error getter
impl ErrorGetter for proto_connections::NatsGetConnectionsResponse {
    fn error(&self) -> Option<&proto_nats::ErrorReply> {
        match &self.msg {
            Some(proto_connections::nats_get_connections_response::Msg::Error(e)) => Some(e),
            _ => None,
        }
    }
}

// ***********************************  Response Setters ***********************************
// Create Response Data setter
impl
    DataSetter<
        proto_connections::GetConnectionsResponse,
        proto_connections::NatsGetConnectionsResponse,
    > for proto_connections::NatsGetConnectionsResponse
{
    fn set_data(data: impl Into<proto_connections::GetConnectionsResponse>) -> Self {
        let data = data.into();
        proto_connections::NatsGetConnectionsResponse {
            msg: Some(proto_connections::nats_get_connections_response::Msg::Data(
                data,
            )),
        }
    }
}

// Create Response Error setter
impl ErrorSetter<proto_nats::ErrorReply, proto_connections::NatsGetConnectionsResponse>
    for proto_connections::NatsGetConnectionsResponse
{
    fn set_error(error: impl Into<proto_nats::ErrorReply>) -> Self {
        let error = error.into();
        proto_connections::NatsGetConnectionsResponse {
            msg: Some(proto_connections::nats_get_connections_response::Msg::Error(error)),
        }
    }
}
