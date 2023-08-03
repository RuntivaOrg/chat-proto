use crate::chat as proto;

use crate::proto_ext::{DataGetter, HeaderGetter, NatsRequestSetter};

// ***********************************  Request Getters ***********************************
// ActionJoinRequest Request Data message
impl DataGetter<proto::ChannelActionJoinRequestRequest>
    for proto::NatsChannelActionJoinRequestRequest
{
    fn to_data(self) -> Option<proto::ChannelActionJoinRequestRequest> {
        self.data
    }
}

// ActionJoinRequest Request Headers
impl HeaderGetter for proto::NatsChannelActionJoinRequestRequest {
    fn headers(&self) -> &Vec<proto::MetadataMap> {
        &self.headers
    }
}

// ********************************** NATS Request Setter **********************************
impl
    NatsRequestSetter<
        proto::ChannelActionJoinRequestRequest,
        proto::NatsChannelActionJoinRequestRequest,
    > for proto::NatsChannelActionJoinRequestRequest
{
    fn from_headers_and_message(
        headers: impl Into<Vec<proto::MetadataMap>>,
        data: impl Into<proto::ChannelActionJoinRequestRequest>,
    ) -> Self {
        proto::NatsChannelActionJoinRequestRequest {
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
