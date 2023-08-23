use crate::runtiva::{chat::v1 as proto_chat, nats::v1 as proto_nats};

use crate::proto_ext::{DataGetter, HeaderGetter, NatsRequestSetter};

// ***********************************  Request Getters ***********************************
// EditPhoto Request Data message
impl DataGetter<proto_chat::ChannelEditPhotoRequest> for proto_nats::NatsChannelEditPhotoRequest {
    fn to_data(self) -> Option<proto_chat::ChannelEditPhotoRequest> {
        self.data
    }
}

// EditPhoto Request Headers
impl HeaderGetter for proto_nats::NatsChannelEditPhotoRequest {
    fn headers(&self) -> &Vec<proto_nats::MetadataMap> {
        &self.headers
    }
}

// ********************************** NATS Request Setter **********************************
impl NatsRequestSetter<proto_chat::ChannelEditPhotoRequest, proto_nats::NatsChannelEditPhotoRequest>
    for proto_nats::NatsChannelEditPhotoRequest
{
    fn from_headers_and_message(
        headers: impl Into<Vec<proto_nats::MetadataMap>>,
        data: impl Into<proto_chat::ChannelEditPhotoRequest>,
    ) -> Self {
        proto_nats::NatsChannelEditPhotoRequest {
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
