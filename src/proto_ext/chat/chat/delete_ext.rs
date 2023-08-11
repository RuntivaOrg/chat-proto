use crate::chat as proto;

use crate::proto_ext::chat::{DataGetter, HeaderGetter, NatsRequestSetter};

// ***********************************  Request Getters ***********************************
// Delete Request Data message
impl DataGetter<proto::ChatDeleteRequest> for proto::NatsChatDeleteRequest {
    fn to_data(self) -> Option<proto::ChatDeleteRequest> {
        self.data
    }
}

// Delete Request Headers
impl HeaderGetter for proto::NatsChatDeleteRequest {
    fn headers(&self) -> &Vec<proto::MetadataMap> {
        &self.headers
    }
}

// ********************************** NATS Request Setter **********************************
impl NatsRequestSetter<proto::ChatDeleteRequest, proto::NatsChatDeleteRequest>
    for proto::NatsChatDeleteRequest
{
    fn from_headers_and_message(
        headers: impl Into<Vec<proto::MetadataMap>>,
        data: impl Into<proto::ChatDeleteRequest>,
    ) -> Self {
        proto::NatsChatDeleteRequest {
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
