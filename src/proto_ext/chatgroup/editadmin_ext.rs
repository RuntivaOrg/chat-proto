use crate::chat as proto;

use crate::proto_ext::{DataGetter, HeaderGetter, NatsRequestSetter};

// ***********************************  Request Getters ***********************************
// EditAdmin Request Data message
impl DataGetter<proto::ChatGroupEditAdminRequest> for proto::NatsChatGroupEditAdminRequest {
    fn data(&self) -> Option<&proto::ChatGroupEditAdminRequest> {
        self.data.as_ref()
    }
}

// EditAdmin Request Headers
impl HeaderGetter for proto::NatsChatGroupEditAdminRequest {
    fn headers(&self) -> &Vec<proto::MetadataMap> {
        &self.headers
    }
}

// ********************************** NATS Request Setter **********************************
impl NatsRequestSetter<proto::ChatGroupEditAdminRequest, proto::NatsChatGroupEditAdminRequest>
    for proto::NatsChatGroupEditAdminRequest
{
    fn from_headers_and_message(
        headers: impl Into<Vec<proto::MetadataMap>>,
        data: impl Into<proto::ChatGroupEditAdminRequest>,
    ) -> Self {
        proto::NatsChatGroupEditAdminRequest {
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
