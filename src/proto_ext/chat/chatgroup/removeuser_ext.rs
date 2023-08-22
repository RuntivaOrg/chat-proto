use crate::chat as proto;
use crate::runtiva::chatgroups::v1 as proto_chatgroups;

use crate::proto_ext::chat::{DataGetter, HeaderGetter, NatsRequestSetter};

// ***********************************  Request Getters ***********************************
// RemoveUser Request Data message
impl DataGetter<proto_chatgroups::ChatGroupRemoveUserRequest>
    for proto_chatgroups::NatsChatGroupRemoveUserRequest
{
    fn to_data(self) -> Option<proto_chatgroups::ChatGroupRemoveUserRequest> {
        self.data
    }
}

// RemoveUser Request Headers
impl HeaderGetter for proto_chatgroups::NatsChatGroupRemoveUserRequest {
    fn headers(&self) -> &Vec<proto::MetadataMap> {
        &self.headers
    }
}

// ********************************** NATS Request Setter **********************************
impl
    NatsRequestSetter<
        proto_chatgroups::ChatGroupRemoveUserRequest,
        proto_chatgroups::NatsChatGroupRemoveUserRequest,
    > for proto_chatgroups::NatsChatGroupRemoveUserRequest
{
    fn from_headers_and_message(
        headers: impl Into<Vec<proto::MetadataMap>>,
        data: impl Into<proto_chatgroups::ChatGroupRemoveUserRequest>,
    ) -> Self {
        proto_chatgroups::NatsChatGroupRemoveUserRequest {
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
