use crate::chat as proto;

use crate::proto_ext::chat::{DataGetter, HeaderGetter, NatsRequestSetter};

// ***********************************  Request Getters ***********************************
// SetJoinToSend Request Data message
impl DataGetter<proto::ChannelSetJoinToSendRequest> for proto::NatsChannelSetJoinToSendRequest {
    fn to_data(self) -> Option<proto::ChannelSetJoinToSendRequest> {
        self.data
    }
}

// SetJoinToSend Request Headers
impl HeaderGetter for proto::NatsChannelSetJoinToSendRequest {
    fn headers(&self) -> &Vec<proto::MetadataMap> {
        &self.headers
    }
}

// ********************************** NATS Request Setter **********************************
impl NatsRequestSetter<proto::ChannelSetJoinToSendRequest, proto::NatsChannelSetJoinToSendRequest>
    for proto::NatsChannelSetJoinToSendRequest
{
    fn from_headers_and_message(
        headers: impl Into<Vec<proto::MetadataMap>>,
        data: impl Into<proto::ChannelSetJoinToSendRequest>,
    ) -> Self {
        proto::NatsChannelSetJoinToSendRequest {
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
