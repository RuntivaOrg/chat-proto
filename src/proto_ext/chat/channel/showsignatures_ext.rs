use crate::chat as proto;

use crate::proto_ext::chat::{DataGetter, HeaderGetter, NatsRequestSetter};

// ***********************************  Request Getters ***********************************
// ShowSignatures Request Data message
impl DataGetter<proto::ChannelShowSignaturesRequest> for proto::NatsChannelShowSignaturesRequest {
    fn to_data(self) -> Option<proto::ChannelShowSignaturesRequest> {
        self.data
    }
}

// ShowSignatures Request Headers
impl HeaderGetter for proto::NatsChannelShowSignaturesRequest {
    fn headers(&self) -> &Vec<proto::MetadataMap> {
        &self.headers
    }
}

// ********************************** NATS Request Setter **********************************
impl NatsRequestSetter<proto::ChannelShowSignaturesRequest, proto::NatsChannelShowSignaturesRequest>
    for proto::NatsChannelShowSignaturesRequest
{
    fn from_headers_and_message(
        headers: impl Into<Vec<proto::MetadataMap>>,
        data: impl Into<proto::ChannelShowSignaturesRequest>,
    ) -> Self {
        proto::NatsChannelShowSignaturesRequest {
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
