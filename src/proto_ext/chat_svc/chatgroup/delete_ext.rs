use crate::runtiva::{chat::v1 as proto_chat, nats::v1 as proto_nats};

use crate::proto_ext::{DataGetter, HeaderGetter, NatsRequestSetter};

// ***********************************  Request Getters ***********************************
// Delete Request Data message
impl DataGetter<proto_chat::ChatGroupDeleteRequest> for proto_nats::NatsChatGroupDeleteRequest {
    fn to_data(self) -> Option<proto_chat::ChatGroupDeleteRequest> {
        self.data
    }
}

// Delete Request Headers
impl HeaderGetter for proto_nats::NatsChatGroupDeleteRequest {
    fn headers(&self) -> &[proto_nats::MetadataMap] {
        &self.headers
    }

    fn take_headers(&mut self) -> Vec<proto_nats::MetadataMap> {
        let mut swapped = vec![];
        std::mem::swap(&mut self.headers, &mut swapped);
        swapped
    }
}

// ********************************** NATS Request Setter **********************************
impl NatsRequestSetter<proto_chat::ChatGroupDeleteRequest, proto_nats::NatsChatGroupDeleteRequest>
    for proto_nats::NatsChatGroupDeleteRequest
{
    fn from_headers_and_message(
        headers: impl Into<Vec<proto_nats::MetadataMap>>,
        data: impl Into<proto_chat::ChatGroupDeleteRequest>,
    ) -> Self {
        proto_nats::NatsChatGroupDeleteRequest {
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
