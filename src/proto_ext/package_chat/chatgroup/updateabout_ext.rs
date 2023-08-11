use crate::chat as proto;

use crate::proto_ext::package_chat::{DataGetter, HeaderGetter, NatsRequestSetter};

// ***********************************  Request Getters ***********************************
// UpdateAbout Request Data message
impl DataGetter<proto::ChatGroupUpdateAboutRequest> for proto::NatsChatGroupUpdateAboutRequest {
    fn to_data(self) -> Option<proto::ChatGroupUpdateAboutRequest> {
        self.data
    }
}

// UpdateAbout Request Headers
impl HeaderGetter for proto::NatsChatGroupUpdateAboutRequest {
    fn headers(&self) -> &Vec<proto::MetadataMap> {
        &self.headers
    }
}

// ********************************** NATS Request Setter **********************************
impl NatsRequestSetter<proto::ChatGroupUpdateAboutRequest, proto::NatsChatGroupUpdateAboutRequest>
    for proto::NatsChatGroupUpdateAboutRequest
{
    fn from_headers_and_message(
        headers: impl Into<Vec<proto::MetadataMap>>,
        data: impl Into<proto::ChatGroupUpdateAboutRequest>,
    ) -> Self {
        proto::NatsChatGroupUpdateAboutRequest {
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
