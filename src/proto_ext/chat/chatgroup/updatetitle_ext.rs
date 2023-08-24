use crate::runtiva::{chat::v1 as proto_chat, nats::v1 as proto_nats};

use crate::proto_ext::{DataGetter, HeaderGetter, NatsRequestSetter};

// ***********************************  Request Getters ***********************************
// UpdateTitle Request Data message
impl DataGetter<proto_chat::ChatGroupUpdateTitleRequest>
    for proto_nats::NatsChatGroupUpdateTitleRequest
{
    fn to_data(self) -> Option<proto_chat::ChatGroupUpdateTitleRequest> {
        self.data
    }
}

// UpdateTitle Request Headers
impl HeaderGetter for proto_nats::NatsChatGroupUpdateTitleRequest {
    fn take_headers(&mut self) -> Vec<proto_nats::MetadataMap> {
        let mut swapped = vec![];
        std::mem::swap(&mut self.headers, &mut swapped);
        swapped
    }
}

// ********************************** NATS Request Setter **********************************
impl
    NatsRequestSetter<
        proto_chat::ChatGroupUpdateTitleRequest,
        proto_nats::NatsChatGroupUpdateTitleRequest,
    > for proto_nats::NatsChatGroupUpdateTitleRequest
{
    fn from_headers_and_message(
        headers: impl Into<Vec<proto_nats::MetadataMap>>,
        data: impl Into<proto_chat::ChatGroupUpdateTitleRequest>,
    ) -> Self {
        proto_nats::NatsChatGroupUpdateTitleRequest {
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
