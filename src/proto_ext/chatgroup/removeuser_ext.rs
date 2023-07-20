use crate::chat as proto;

use crate::proto_ext::proto::{DataGetter, HeaderGetter};

// ***********************************  Request Getters ***********************************
// RemoveUser Request Data message
impl DataGetter<proto::ChatGroupRemoveUserRequest> for proto::NatsChatGroupRemoveUserRequest {
    fn data(&self) -> &Option<proto::ChatGroupRemoveUserRequest> {
        &self.data
    }
}

// RemoveUser Request Headers
impl HeaderGetter for proto::NatsChatGroupRemoveUserRequest {
    fn headers(&self) -> &Vec<proto::MetadataMap> {
        &self.headers
    }
}

// ***********************************  Response Setters ***********************************
//
// *** Uses the generic implementations from chat-proto\src\proto_ext\error_response_ext.rs
