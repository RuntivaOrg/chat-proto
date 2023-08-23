use crate::runtiva::{chat::v1 as proto_chat, nats::v1 as proto_nats};

use crate::proto_ext::{DataGetter, HeaderGetter, NatsRequestSetter};

// ***********************************  Request Getters ***********************************
// Delete Request Data message
impl DataGetter<proto_chat::ChatDeleteRequest> for proto_nats::NatsChatDeleteRequest {
    fn to_data(self) -> Option<proto_chat::ChatDeleteRequest> {
        self.data
    }
}

// Delete Request Headers
impl HeaderGetter for proto_nats::NatsChatDeleteRequest {
    fn headers(&self) -> &Vec<proto_nats::MetadataMap> {
        &self.headers
    }
}

// ********************************** NATS Request Setter **********************************
impl NatsRequestSetter<proto_chat::ChatDeleteRequest, proto_nats::NatsChatDeleteRequest>
    for proto_nats::NatsChatDeleteRequest
{
    fn from_headers_and_message(
        headers: impl Into<Vec<proto_nats::MetadataMap>>,
        data: impl Into<proto_chat::ChatDeleteRequest>,
    ) -> Self {
        proto_nats::NatsChatDeleteRequest {
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
