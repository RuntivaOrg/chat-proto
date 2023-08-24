use crate::runtiva::{chat::v1 as proto_chat, nats::v1 as proto_nats};

use crate::proto_ext::{DataGetter, HeaderGetter, NatsRequestSetter};

// ***********************************  Request Getters ***********************************
// Send Request Data message
impl DataGetter<proto_chat::MessageSendRequest> for proto_nats::NatsMessageSendRequest {
    fn to_data(self) -> Option<proto_chat::MessageSendRequest> {
        self.data
    }
}

// Send Request Headers
impl HeaderGetter for proto_nats::NatsMessageSendRequest {
    fn take_headers(&mut self) -> Vec<proto_nats::MetadataMap> {
        let mut swapped = vec![];
        std::mem::swap(&mut self.headers, &mut swapped);
        swapped
    }
}

// ********************************** NATS Request Setter **********************************
impl NatsRequestSetter<proto_chat::MessageSendRequest, proto_nats::NatsMessageSendRequest>
    for proto_nats::NatsMessageSendRequest
{
    fn from_headers_and_message(
        headers: impl Into<Vec<proto_nats::MetadataMap>>,
        data: impl Into<proto_chat::MessageSendRequest>,
    ) -> Self {
        proto_nats::NatsMessageSendRequest {
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
