use crate::chat as proto;

use crate::proto_ext::{DataGetter, HeaderGetter, NatsRequestSetter};

// ***********************************  Request Getters ***********************************
// Delete Request Data message
impl DataGetter<proto::ChatGroupDeleteRequest> for proto::NatsChatGroupDeleteRequest {
    fn data(&self) -> Option<&proto::ChatGroupDeleteRequest> {
        self.data.as_ref()
    }
}

// Delete Request Headers
impl HeaderGetter for proto::NatsChatGroupDeleteRequest {
    fn headers(&self) -> &Vec<proto::MetadataMap> {
        &self.headers
    }
}

// ********************************** NATS Request Setter **********************************
impl NatsRequestSetter<proto::ChatGroupDeleteRequest, proto::NatsChatGroupDeleteRequest>
    for proto::NatsChatGroupDeleteRequest
{
    fn from_headers_and_message(
        headers: impl Into<Vec<proto::MetadataMap>>,
        data: impl Into<proto::ChatGroupDeleteRequest>,
    ) -> Self {
        proto::NatsChatGroupDeleteRequest {
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
