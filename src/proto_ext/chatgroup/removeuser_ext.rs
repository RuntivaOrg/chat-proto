use crate::chat as proto;

use crate::proto_ext::{DataGetter, HeaderGetter, NatsRequestSetter};

// ***********************************  Request Getters ***********************************
// RemoveUser Request Data message
impl DataGetter<proto::ChatGroupRemoveUserRequest> for proto::NatsChatGroupRemoveUserRequest {
    fn data(&self) -> Option<&proto::ChatGroupRemoveUserRequest> {
        self.data.as_ref()
    }
}

// RemoveUser Request Headers
impl HeaderGetter for proto::NatsChatGroupRemoveUserRequest {
    fn headers(&self) -> &Vec<proto::MetadataMap> {
        &self.headers
    }
}

// ********************************** NATS Request Setter **********************************
impl NatsRequestSetter<proto::ChatGroupRemoveUserRequest, proto::NatsChatGroupRemoveUserRequest>
    for proto::NatsChatGroupRemoveUserRequest
{
    fn from_headers_and_message(
        headers: impl Into<Vec<proto::MetadataMap>>,
        data: impl Into<proto::ChatGroupRemoveUserRequest>,
    ) -> Self {
        proto::NatsChatGroupRemoveUserRequest {
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
