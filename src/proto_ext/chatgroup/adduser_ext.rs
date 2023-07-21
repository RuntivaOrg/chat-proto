use crate::chat as proto;

use crate::proto_ext::{DataGetter, HeaderGetter, NatsRequestSetter};

// ***********************************  Request Getters ***********************************
// AddUser Request Data message
impl DataGetter<proto::ChatGroupAddUserRequest> for proto::NatsChatGroupAddUserRequest {
    fn to_data(self) -> Option<proto::ChatGroupAddUserRequest> {
        self.data
    }
}

// AddUser Request Headers
impl HeaderGetter for proto::NatsChatGroupAddUserRequest {
    fn headers(&self) -> &Vec<proto::MetadataMap> {
        &self.headers
    }
}

// ********************************** NATS Request Setter **********************************
impl NatsRequestSetter<proto::ChatGroupAddUserRequest, proto::NatsChatGroupAddUserRequest>
    for proto::NatsChatGroupAddUserRequest
{
    fn from_headers_and_message(
        headers: impl Into<Vec<proto::MetadataMap>>,
        data: impl Into<proto::ChatGroupAddUserRequest>,
    ) -> Self {
        proto::NatsChatGroupAddUserRequest {
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
// *** Uses the generic implementations from chat-proto\src\proto_ext\empty_response_ext.rs
