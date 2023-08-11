use crate::chat as proto;

use crate::proto_ext::chat::{DataGetter, HeaderGetter, NatsRequestSetter};

// ***********************************  Request Getters ***********************************
// RemoveUser Request Data message
impl DataGetter<proto::ChannelEnablePrehistoryRequest>
    for proto::NatsChannelEnablePrehistoryRequest
{
    fn to_data(self) -> Option<proto::ChannelEnablePrehistoryRequest> {
        self.data
    }
}

// RemoveUser Request Headers
impl HeaderGetter for proto::NatsChannelEnablePrehistoryRequest {
    fn headers(&self) -> &Vec<proto::MetadataMap> {
        &self.headers
    }
}

// ********************************** NATS Request Setter **********************************
impl
    NatsRequestSetter<
        proto::ChannelEnablePrehistoryRequest,
        proto::NatsChannelEnablePrehistoryRequest,
    > for proto::NatsChannelEnablePrehistoryRequest
{
    fn from_headers_and_message(
        headers: impl Into<Vec<proto::MetadataMap>>,
        data: impl Into<proto::ChannelEnablePrehistoryRequest>,
    ) -> Self {
        proto::NatsChannelEnablePrehistoryRequest {
            headers: headers.into(),
            data: Some(data.into()),
        }
    }
}

// ***********************************  Response Getters ***********************************
//
// *** Uses the generic implementations from chat-proto\src\proto_ext\empty_response_ext.rs

// ***********************************  Response Setters ***********************************
//
// *** Uses the generic implementations from chat-proto\src\proto_ext\error_response_ext.rs
