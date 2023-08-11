use crate::chat as proto;

use crate::proto_ext::package_chat::{DataGetter, HeaderGetter, NatsRequestSetter};

// ***********************************  Request Getters ***********************************
// EditParticipant Request Data message
impl DataGetter<proto::ChannelEditParticipantRequest> for proto::NatsChannelEditParticipantRequest {
    fn to_data(self) -> Option<proto::ChannelEditParticipantRequest> {
        self.data
    }
}

// EditParticipant Request Headers
impl HeaderGetter for proto::NatsChannelEditParticipantRequest {
    fn headers(&self) -> &Vec<proto::MetadataMap> {
        &self.headers
    }
}

// ********************************** NATS Request Setter **********************************
impl
    NatsRequestSetter<
        proto::ChannelEditParticipantRequest,
        proto::NatsChannelEditParticipantRequest,
    > for proto::NatsChannelEditParticipantRequest
{
    fn from_headers_and_message(
        headers: impl Into<Vec<proto::MetadataMap>>,
        data: impl Into<proto::ChannelEditParticipantRequest>,
    ) -> Self {
        proto::NatsChannelEditParticipantRequest {
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
