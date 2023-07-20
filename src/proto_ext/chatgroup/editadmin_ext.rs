use crate::chat as proto;

use crate::proto_ext::proto::{DataGetter, HeaderGetter};

// ***********************************  Request Getters ***********************************
// EditAdmin Request Data message
impl DataGetter<proto::ChatGroupEditAdminRequest> for proto::NatsChatGroupEditAdminRequest {
    fn data(&self) -> &Option<proto::ChatGroupEditAdminRequest> {
        &self.data
    }
}

// EditAdmin Request Headers
impl HeaderGetter for proto::NatsChatGroupEditAdminRequest {
    fn headers(&self) -> &Vec<proto::MetadataMap> {
        &self.headers
    }
}

// ***********************************  Response Setters ***********************************
//
// *** Uses the generic implementations from chat-proto\src\proto_ext\error_response_ext.rs
