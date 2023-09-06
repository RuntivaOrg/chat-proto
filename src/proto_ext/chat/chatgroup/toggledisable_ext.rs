use crate::runtiva::{chat::v1 as proto_chat, nats::v1 as proto_nats};

use crate::proto_ext::{DataGetter, HeaderGetter, NatsRequestSetter};

// ***********************************  Request Getters ***********************************
// ToggleDisable Request Data message
impl DataGetter<proto_chat::ChatGroupToggleDisableRequest>
    for proto_nats::NatsChatGroupToggleDisableRequest
{
    fn to_data(self) -> Option<proto_chat::ChatGroupToggleDisableRequest> {
        self.data
    }
}

// ToggleDisable Request Headers
impl HeaderGetter for proto_nats::NatsChatGroupToggleDisableRequest {
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
        proto_chat::ChatGroupToggleDisableRequest,
        proto_nats::NatsChatGroupToggleDisableRequest,
    > for proto_nats::NatsChatGroupToggleDisableRequest
{
    fn from_headers_and_message(
        headers: impl Into<Vec<proto_nats::MetadataMap>>,
        data: impl Into<proto_chat::ChatGroupToggleDisableRequest>,
    ) -> Self {
        proto_nats::NatsChatGroupToggleDisableRequest {
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
