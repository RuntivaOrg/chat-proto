use crate::chat as proto;
use crate::runtiva::nats::v1 as proto_nats;

use crate::proto_ext::{DataGetter, HeaderGetter, NatsRequestSetter};

// ***********************************  Request Getters ***********************************
// EditAbout Request Data message
impl DataGetter<proto::ChannelEditAboutRequest> for proto_nats::NatsChannelEditAboutRequest {
    fn to_data(self) -> Option<proto::ChannelEditAboutRequest> {
        self.data
    }
}

// EditAbout Request Headers
impl HeaderGetter for proto_nats::NatsChannelEditAboutRequest {
    fn headers(&self) -> &Vec<proto_nats::MetadataMap> {
        &self.headers
    }
}

// ********************************** NATS Request Setter **********************************
impl NatsRequestSetter<proto::ChannelEditAboutRequest, proto_nats::NatsChannelEditAboutRequest>
    for proto_nats::NatsChannelEditAboutRequest
{
    fn from_headers_and_message(
        headers: impl Into<Vec<proto_nats::MetadataMap>>,
        data: impl Into<proto::ChannelEditAboutRequest>,
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
