use crate::runtiva::{chat::v1 as proto_chat, nats::v1 as proto_nats};

use crate::proto_ext::{DataGetter, HeaderGetter, NatsRequestSetter};

// ***********************************  Request Getters ***********************************
// ShowSignatures Request Data message
impl DataGetter<proto_chat::ChannelShowSignaturesRequest>
    for proto_nats::NatsChannelShowSignaturesRequest
{
    fn to_data(self) -> Option<proto_chat::ChannelShowSignaturesRequest> {
        self.data
    }
}

// ShowSignatures Request Headers
impl HeaderGetter for proto_nats::NatsChannelShowSignaturesRequest {
    fn headers(&self) -> &Vec<proto_nats::MetadataMap> {
        &self.headers
    }
}

// ********************************** NATS Request Setter **********************************
impl
    NatsRequestSetter<
        proto_chat::ChannelShowSignaturesRequest,
        proto_nats::NatsChannelShowSignaturesRequest,
    > for proto_nats::NatsChannelShowSignaturesRequest
{
    fn from_headers_and_message(
        headers: impl Into<Vec<proto_nats::MetadataMap>>,
        data: impl Into<proto_chat::ChannelShowSignaturesRequest>,
    ) -> Self {
        proto_nats::NatsChannelShowSignaturesRequest {
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
