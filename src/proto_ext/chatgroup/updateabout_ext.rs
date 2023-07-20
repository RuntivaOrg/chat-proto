use crate::chat as proto;

use crate::proto_ext::proto::{DataGetter, HeaderGetter};

// ***********************************  Request Getters ***********************************
// UpdateAbout Request Data message
impl DataGetter<proto::ChatGroupUpdateAboutRequest> for proto::NatsChatGroupUpdateAboutRequest {
    fn data(&self) -> &Option<proto::ChatGroupUpdateAboutRequest> {
        &self.data
    }
}

// UpdateAbout Request Headers
impl HeaderGetter for proto::NatsChatGroupUpdateAboutRequest {
    fn headers(&self) -> &Vec<proto::MetadataMap> {
        &self.headers
    }
}

// ***********************************  Response Setters ***********************************
//
// *** Uses the generic implementations from chat-proto\src\proto_ext\error_response_ext.rs
