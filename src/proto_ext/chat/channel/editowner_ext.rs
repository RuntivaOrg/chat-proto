use crate::chat as proto;

use crate::proto_ext::chat::{DataGetter, HeaderGetter, NatsRequestSetter};

// ***********************************  Request Getters ***********************************
// EditOwner Request Data message
impl DataGetter<proto::ChannelEditOwnerRequest> for proto::NatsChannelEditOwnerRequest {
    fn to_data(self) -> Option<proto::ChannelEditOwnerRequest> {
        self.data
    }
}

// EditOwner Request Headers
impl HeaderGetter for proto::NatsChannelEditOwnerRequest {
    fn headers(&self) -> &Vec<proto::MetadataMap> {
        &self.headers
    }
}

// ********************************** NATS Request Setter **********************************
impl NatsRequestSetter<proto::ChannelEditOwnerRequest, proto::NatsChannelEditOwnerRequest>
    for proto::NatsChannelEditOwnerRequest
{
    fn from_headers_and_message(
        headers: impl Into<Vec<proto::MetadataMap>>,
        data: impl Into<proto::ChannelEditOwnerRequest>,
    ) -> Self {
        proto::NatsChannelEditOwnerRequest {
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
