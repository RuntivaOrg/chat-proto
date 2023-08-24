use crate::runtiva::{chat::v1 as proto_chat, nats::v1 as proto_nats};

use crate::proto_ext::{DataGetter, HeaderGetter, NatsRequestSetter};

// ***********************************  Request Getters ***********************************
// EditAbout Request Data message
impl DataGetter<proto_chat::ChannelEditAboutRequest> for proto_nats::NatsChannelEditAboutRequest {
    fn to_data(self) -> Option<proto_chat::ChannelEditAboutRequest> {
        self.data
    }
}

// EditAbout Request Headers
impl HeaderGetter for proto_nats::NatsChannelEditAboutRequest {
    fn take_headers(&mut self) -> Vec<proto_nats::MetadataMap> {
        let mut swapped = vec![];
        std::mem::swap(&mut self.headers, &mut swapped);
        swapped
    }
}

// ********************************** NATS Request Setter **********************************
impl NatsRequestSetter<proto_chat::ChannelEditAboutRequest, proto_nats::NatsChannelEditAboutRequest>
    for proto_nats::NatsChannelEditAboutRequest
{
    fn from_headers_and_message(
        headers: impl Into<Vec<proto_nats::MetadataMap>>,
        data: impl Into<proto_chat::ChannelEditAboutRequest>,
    ) -> Self {
        proto_nats::NatsChannelEditAboutRequest {
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
