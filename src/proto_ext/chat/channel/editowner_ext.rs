use crate::chat as proto;
use crate::runtiva::nats::v1 as proto_nats;

use crate::proto_ext::{DataGetter, HeaderGetter, NatsRequestSetter};

// ***********************************  Request Getters ***********************************
// EditOwner Request Data message
impl DataGetter<proto::ChannelEditOwnerRequest> for proto_nats::NatsChannelEditOwnerRequest {
    fn to_data(self) -> Option<proto::ChannelEditOwnerRequest> {
        self.data
    }
}

// EditOwner Request Headers
impl HeaderGetter for proto_nats::NatsChannelEditOwnerRequest {
    fn headers(&self) -> &Vec<proto_nats::MetadataMap> {
        &self.headers
    }
}

// ********************************** NATS Request Setter **********************************
impl NatsRequestSetter<proto::ChannelEditOwnerRequest, proto_nats::NatsChannelEditOwnerRequest>
    for proto_nats::NatsChannelEditOwnerRequest
{
    fn from_headers_and_message(
        headers: impl Into<Vec<proto_nats::MetadataMap>>,
        data: impl Into<proto::ChannelEditOwnerRequest>,
    ) -> Self {
        proto_nats::NatsChannelEditOwnerRequest {
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
