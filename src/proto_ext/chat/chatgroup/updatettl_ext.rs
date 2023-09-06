use crate::runtiva::{chat::v1 as proto_chat, nats::v1 as proto_nats};

use crate::proto_ext::{DataGetter, HeaderGetter, NatsRequestSetter};

// ***********************************  Request Getters ***********************************
// UpdateTtl Request Data message
impl DataGetter<proto_chat::ChatGroupUpdateTtlRequest>
    for proto_nats::NatsChatGroupUpdateTtlRequest
{
    fn to_data(self) -> Option<proto_chat::ChatGroupUpdateTtlRequest> {
        self.data
    }
}

// UpdateTtl Request Headers
impl HeaderGetter for proto_nats::NatsChatGroupUpdateTtlRequest {
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
impl
    NatsRequestSetter<
        proto_chat::ChatGroupUpdateTtlRequest,
        proto_nats::NatsChatGroupUpdateTtlRequest,
    > for proto_nats::NatsChatGroupUpdateTtlRequest
{
    fn from_headers_and_message(
        headers: impl Into<Vec<proto_nats::MetadataMap>>,
        data: impl Into<proto_chat::ChatGroupUpdateTtlRequest>,
    ) -> Self {
        proto_nats::NatsChatGroupUpdateTtlRequest {
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
