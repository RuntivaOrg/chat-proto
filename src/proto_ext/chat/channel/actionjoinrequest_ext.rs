use crate::proto::chat as proto;
use crate::runtiva::nats::v1 as proto_nats;

use crate::proto_ext::{DataGetter, HeaderGetter, NatsRequestSetter};

// ***********************************  Request Getters ***********************************
// ActionJoinRequest Request Data message
impl DataGetter<proto::ChannelActionJoinRequestRequest>
    for proto_nats::NatsChannelActionJoinRequestRequest
{
    fn to_data(self) -> Option<proto::ChannelActionJoinRequestRequest> {
        self.data
    }
}

// ActionJoinRequest Request Headers
impl HeaderGetter for proto_nats::NatsChannelActionJoinRequestRequest {
    fn headers(&self) -> &Vec<proto_nats::MetadataMap> {
        &self.headers
    }
}

// ********************************** NATS Request Setter **********************************
impl
    NatsRequestSetter<
        proto::ChannelActionJoinRequestRequest,
        proto_nats::NatsChannelActionJoinRequestRequest,
    > for proto_nats::NatsChannelActionJoinRequestRequest
{
    fn from_headers_and_message(
        headers: impl Into<Vec<proto_nats::MetadataMap>>,
        data: impl Into<proto::ChannelActionJoinRequestRequest>,
    ) -> Self {
        proto_nats::NatsChannelActionJoinRequestRequest {
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
