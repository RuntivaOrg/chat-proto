use crate::runtiva::chatgroups::v1 as proto_chatgroups;
use crate::runtiva::nats::v1 as proto_nats;

use crate::proto_ext::{DataGetter, HeaderGetter, NatsRequestSetter};

// ***********************************  Request Getters ***********************************
// UpdateAbout Request Data message
impl DataGetter<proto_chatgroups::ChatGroupUpdateAboutRequest>
    for proto_chatgroups::NatsChatGroupUpdateAboutRequest
{
    fn to_data(self) -> Option<proto_chatgroups::ChatGroupUpdateAboutRequest> {
        self.data
    }
}

// UpdateAbout Request Headers
impl HeaderGetter for proto_chatgroups::NatsChatGroupUpdateAboutRequest {
    fn headers(&self) -> &Vec<proto_nats::MetadataMap> {
        &self.headers
    }
}

// ********************************** NATS Request Setter **********************************
impl
    NatsRequestSetter<
        proto_chatgroups::ChatGroupUpdateAboutRequest,
        proto_chatgroups::NatsChatGroupUpdateAboutRequest,
    > for proto_chatgroups::NatsChatGroupUpdateAboutRequest
{
    fn from_headers_and_message(
        headers: impl Into<Vec<proto_nats::MetadataMap>>,
        data: impl Into<proto_chatgroups::ChatGroupUpdateAboutRequest>,
    ) -> Self {
        proto_chatgroups::NatsChatGroupUpdateAboutRequest {
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
