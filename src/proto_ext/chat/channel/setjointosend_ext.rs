use crate::chat as proto;
use crate::runtiva::nats::v1 as proto_nats;

use crate::proto_ext::{DataGetter, HeaderGetter, NatsRequestSetter};

// ***********************************  Request Getters ***********************************
// SetJoinToSend Request Data message
impl DataGetter<proto::ChannelSetJoinToSendRequest>
    for proto_nats::NatsChannelSetJoinToSendRequest
{
    fn to_data(self) -> Option<proto::ChannelSetJoinToSendRequest> {
        self.data
    }
}

// SetJoinToSend Request Headers
impl HeaderGetter for proto_nats::NatsChannelSetJoinToSendRequest {
    fn headers(&self) -> &Vec<proto_nats::MetadataMap> {
        &self.headers
    }
}

// ********************************** NATS Request Setter **********************************
impl
    NatsRequestSetter<
        proto::ChannelSetJoinToSendRequest,
        proto_nats::NatsChannelSetJoinToSendRequest,
    > for proto_nats::NatsChannelSetJoinToSendRequest
{
    fn from_headers_and_message(
        headers: impl Into<Vec<proto_nats::MetadataMap>>,
        data: impl Into<proto::ChannelSetJoinToSendRequest>,
    ) -> Self {
        proto_nats::NatsChannelSetJoinToSendRequest {
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
