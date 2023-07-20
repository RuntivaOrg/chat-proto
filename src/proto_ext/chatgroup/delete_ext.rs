use crate::chat as proto;

use crate::proto_ext::proto::{DataGetter, HeaderGetter};

// ***********************************  Request Getters ***********************************
// Delete Request Data message
impl DataGetter<proto::ChatGroupDeleteRequest> for proto::NatsChatGroupDeleteRequest {
    fn data(&self) -> &Option<proto::ChatGroupDeleteRequest> {
        &self.data
    }
}

// Delete Request Headers
impl HeaderGetter for proto::NatsChatGroupDeleteRequest {
    fn headers(&self) -> &Vec<proto::MetadataMap> {
        &self.headers
    }
}

// ***********************************  Response Setters ***********************************
//
// *** Uses the generic implementations from chat-proto\src\proto_ext\error_response_ext.rs
