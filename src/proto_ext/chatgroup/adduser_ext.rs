use crate::chat as proto;

use crate::proto_ext::proto::{DataGetter, HeaderGetter};

// ***********************************  Request Getters ***********************************
// AddUser Request Data message
impl DataGetter<proto::ChatGroupAddUserRequest> for proto::NatsChatGroupAddUserRequest {
    fn data(&self) -> &Option<proto::ChatGroupAddUserRequest> {
        &self.data
    }
}

// AddUser Request Headers
impl HeaderGetter for proto::NatsChatGroupAddUserRequest {
    fn headers(&self) -> &Vec<proto::MetadataMap> {
        &self.headers
    }
}

// ***********************************  Response Setters ***********************************
//
// *** Uses the generic implementations from chat-proto\src\proto_ext\error_response_ext.rs
