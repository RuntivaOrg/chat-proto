use crate::chat as proto;

use crate::proto_ext::{DataGetter, HeaderGetter, NatsRequestSetter};

// ***********************************  Request Getters ***********************************
// UpdateTitle Request Data message
impl DataGetter<proto::ChatGroupUpdateTitleRequest> for proto::NatsChatGroupUpdateTitleRequest {
    fn data(&self) -> Option<&proto::ChatGroupUpdateTitleRequest> {
        self.data.as_ref()
    }
}

// UpdateTitle Request Headers
impl HeaderGetter for proto::NatsChatGroupUpdateTitleRequest {
    fn headers(&self) -> &Vec<proto::MetadataMap> {
        &self.headers
    }
}

// ********************************** NATS Request Setter **********************************
impl NatsRequestSetter<proto::ChatGroupUpdateTitleRequest, proto::NatsChatGroupUpdateTitleRequest>
    for proto::NatsChatGroupUpdateTitleRequest
{
    fn from_headers_and_message(
        headers: impl Into<Vec<proto::MetadataMap>>,
        data: impl Into<proto::ChatGroupUpdateTitleRequest>,
    ) -> Self {
        proto::NatsChatGroupUpdateTitleRequest {
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