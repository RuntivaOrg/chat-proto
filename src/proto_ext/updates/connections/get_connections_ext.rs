use crate::proto_ext::{
    DataGetter, DataSetter, ErrorGetter, ErrorSetter, HeaderGetter, NatsRequestSetter,
};
use crate::runtiva::{nats::v1 as proto_nats, updates::v1 as proto_updates};

// ***********************************  Request Getters ***********************************
// EditAbout Request Data message
impl DataGetter<proto_updates::GetConnectionsRequest> for proto_updates::NatsGetConnectionsRequest {
    fn to_data(self) -> Option<proto_updates::GetConnectionsRequest> {
        self.data
    }
}

// EditAbout Request Headers
impl HeaderGetter for proto_updates::NatsGetConnectionsRequest {
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
        proto_updates::GetConnectionsRequest,
        proto_updates::NatsGetConnectionsRequest,
    > for proto_updates::NatsGetConnectionsRequest
{
    fn from_headers_and_message(
        headers: impl Into<Vec<proto_nats::MetadataMap>>,
        data: impl Into<proto_updates::GetConnectionsRequest>,
    ) -> Self {
        proto_updates::NatsGetConnectionsRequest {
            headers: headers.into(),
            data: Some(data.into()),
        }
    }
}

// ***********************************  Response Getters ***********************************
// Create Response Data getter
impl DataGetter<proto_updates::GetConnectionsResponse>
    for proto_updates::NatsGetConnectionsResponse
{
    fn to_data(self) -> Option<proto_updates::GetConnectionsResponse> {
        match self.msg {
            Some(proto_updates::nats_get_connections_response::Msg::Data(data)) => Some(data),
            _ => None,
        }
    }
}

// Create Response Error getter
impl ErrorGetter for proto_updates::NatsGetConnectionsResponse {
    fn error(&self) -> Option<&proto_nats::ErrorReply> {
        match &self.msg {
            Some(proto_updates::nats_get_connections_response::Msg::Error(e)) => Some(e),
            _ => None,
        }
    }
}

// ***********************************  Response Setters ***********************************
// Create Response Data setter
impl DataSetter<proto_updates::GetConnectionsResponse, proto_updates::NatsGetConnectionsResponse>
    for proto_updates::NatsGetConnectionsResponse
{
    fn set_data(data: impl Into<proto_updates::GetConnectionsResponse>) -> Self {
        let data = data.into();
        proto_updates::NatsGetConnectionsResponse {
            msg: Some(proto_updates::nats_get_connections_response::Msg::Data(
                data,
            )),
        }
    }
}

// Create Response Error setter
impl ErrorSetter<proto_nats::ErrorReply, proto_updates::NatsGetConnectionsResponse>
    for proto_updates::NatsGetConnectionsResponse
{
    fn set_error(error: impl Into<proto_nats::ErrorReply>) -> Self {
        let error = error.into();
        proto_updates::NatsGetConnectionsResponse {
            msg: Some(proto_updates::nats_get_connections_response::Msg::Error(
                error,
            )),
        }
    }
}
