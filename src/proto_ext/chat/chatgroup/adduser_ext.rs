use crate::runtiva::chatgroups::v1 as proto_chatgroups;
use crate::runtiva::nats::v1 as proto_nats;

use crate::proto_ext::{DataGetter, HeaderGetter, NatsRequestSetter};

// ***********************************  Request Getters ***********************************
// AddUser Request Data message
impl DataGetter<proto_chatgroups::ChatGroupAddUserRequest>
    for proto_chatgroups::NatsChatGroupAddUserRequest
{
    fn to_data(self) -> Option<proto_chatgroups::ChatGroupAddUserRequest> {
        self.data
    }
}

// AddUser Request Headers
impl HeaderGetter for proto_chatgroups::NatsChatGroupAddUserRequest {
    fn headers(&self) -> &Vec<proto_nats::MetadataMap> {
        &self.headers
    }
}

// ********************************** NATS Request Setter **********************************
impl
    NatsRequestSetter<
        proto_chatgroups::ChatGroupAddUserRequest,
        proto_chatgroups::NatsChatGroupAddUserRequest,
    > for proto_chatgroups::NatsChatGroupAddUserRequest
{
    fn from_headers_and_message(
        headers: impl Into<Vec<proto_nats::MetadataMap>>,
        data: impl Into<proto_chatgroups::ChatGroupAddUserRequest>,
    ) -> Self {
        proto_chatgroups::NatsChatGroupAddUserRequest {
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
