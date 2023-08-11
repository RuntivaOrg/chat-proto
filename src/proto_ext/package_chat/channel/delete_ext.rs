use crate::chat as proto;

use crate::proto_ext::package_chat::{DataGetter, HeaderGetter, NatsRequestSetter};

// ***********************************  Request Getters ***********************************
// RemoveUser Request Data message
impl DataGetter<proto::ChannelDeleteRequest> for proto::NatsChannelDeleteRequest {
    fn to_data(self) -> Option<proto::ChannelDeleteRequest> {
        self.data
    }
}

// RemoveUser Request Headers
impl HeaderGetter for proto::NatsChannelDeleteRequest {
    fn headers(&self) -> &Vec<proto::MetadataMap> {
        &self.headers
    }
}

// ********************************** NATS Request Setter **********************************
impl NatsRequestSetter<proto::ChannelDeleteRequest, proto::NatsChannelDeleteRequest>
    for proto::NatsChannelDeleteRequest
{
    fn from_headers_and_message(
        headers: impl Into<Vec<proto::MetadataMap>>,
        data: impl Into<proto::ChannelDeleteRequest>,
    ) -> Self {
        proto::NatsChannelDeleteRequest {
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
