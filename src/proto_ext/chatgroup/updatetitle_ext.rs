use crate::chat as proto;

use crate::proto_ext::proto::{DataGetter, HeaderGetter};

// ***********************************  Request Getters ***********************************
// UpdateTitle Request Data message
impl DataGetter<proto::ChatGroupUpdateTitleRequest> for proto::NatsChatGroupUpdateTitleRequest {
    fn data(&self) -> &Option<proto::ChatGroupUpdateTitleRequest> {
        &self.data
    }
}

// UpdateTitle Request Headers
impl HeaderGetter for proto::NatsChatGroupUpdateTitleRequest {
    fn headers(&self) -> &Vec<proto::MetadataMap> {
        &self.headers
    }
}

// ***********************************  Response Setters ***********************************
//
// *** Uses the generic implementations from chat-proto\src\proto_ext\error_response_ext.rs
