use crate::chat as proto;
use crate::runtiva::nats::v1 as proto_nats;

use crate::proto_ext::{DataGetter, HeaderGetter, NatsRequestSetter};

// ***********************************  Request Getters ***********************************
// EditTitle Request Data message
impl DataGetter<proto::ChannelEditTitleRequest> for proto_nats::NatsChannelEditTitleRequest {
    fn to_data(self) -> Option<proto::ChannelEditTitleRequest> {
        self.data
    }
}

// EditTitle Request Headers
impl HeaderGetter for proto_nats::NatsChannelEditTitleRequest {
    fn headers(&self) -> &Vec<proto_nats::MetadataMap> {
        &self.headers
    }
}

// ********************************** NATS Request Setter **********************************
impl NatsRequestSetter<proto::ChannelEditTitleRequest, proto_nats::NatsChannelEditTitleRequest>
    for proto_nats::NatsChannelEditTitleRequest
{
    fn from_headers_and_message(
        headers: impl Into<Vec<proto_nats::MetadataMap>>,
        data: impl Into<proto::ChannelEditTitleRequest>,
    ) -> Self {
        proto_nats::NatsChannelEditTitleRequest {
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
