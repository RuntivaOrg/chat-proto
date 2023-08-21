use crate::chat as proto;
use crate::chatgroups::v1 as proto_chatgroups;

use crate::proto_ext::chat::{DataGetter, HeaderGetter, NatsRequestSetter};

// ***********************************  Request Getters ***********************************
// Delete Request Data message
impl DataGetter<proto_chatgroups::ChatGroupDeleteRequest>
    for proto_chatgroups::NatsChatGroupDeleteRequest
{
    fn to_data(self) -> Option<proto_chatgroups::ChatGroupDeleteRequest> {
        self.data
    }
}

// Delete Request Headers
impl HeaderGetter for proto_chatgroups::NatsChatGroupDeleteRequest {
    fn headers(&self) -> &Vec<proto::MetadataMap> {
        &self.headers
    }
}

// ********************************** NATS Request Setter **********************************
impl
    NatsRequestSetter<
        proto_chatgroups::ChatGroupDeleteRequest,
        proto_chatgroups::NatsChatGroupDeleteRequest,
    > for proto_chatgroups::NatsChatGroupDeleteRequest
{
    fn from_headers_and_message(
        headers: impl Into<Vec<proto::MetadataMap>>,
        data: impl Into<proto_chatgroups::ChatGroupDeleteRequest>,
    ) -> Self {
        proto_chatgroups::NatsChatGroupDeleteRequest {
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
