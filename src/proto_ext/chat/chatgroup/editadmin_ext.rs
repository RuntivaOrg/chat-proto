use crate::chat as proto;
use crate::chatgroups::v1 as proto_chatgroups;

use crate::proto_ext::chat::{DataGetter, HeaderGetter, NatsRequestSetter};

// ***********************************  Request Getters ***********************************
// EditAdmin Request Data message
impl DataGetter<proto_chatgroups::ChatGroupEditAdminRequest>
    for proto_chatgroups::NatsChatGroupEditAdminRequest
{
    fn to_data(self) -> Option<proto_chatgroups::ChatGroupEditAdminRequest> {
        self.data
    }
}

// EditAdmin Request Headers
impl HeaderGetter for proto_chatgroups::NatsChatGroupEditAdminRequest {
    fn headers(&self) -> &Vec<proto::MetadataMap> {
        &self.headers
    }
}

// ********************************** NATS Request Setter **********************************
impl
    NatsRequestSetter<
        proto_chatgroups::ChatGroupEditAdminRequest,
        proto_chatgroups::NatsChatGroupEditAdminRequest,
    > for proto_chatgroups::NatsChatGroupEditAdminRequest
{
    fn from_headers_and_message(
        headers: impl Into<Vec<proto::MetadataMap>>,
        data: impl Into<proto_chatgroups::ChatGroupEditAdminRequest>,
    ) -> Self {
        proto_chatgroups::NatsChatGroupEditAdminRequest {
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
