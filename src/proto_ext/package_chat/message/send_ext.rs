use crate::chat as proto;

use crate::proto_ext::package_chat::{DataGetter, HeaderGetter, NatsRequestSetter};

// ***********************************  Request Getters ***********************************
// Send Request Data message
impl DataGetter<proto::MessageSendRequest> for proto::NatsMessageSendRequest {
    fn to_data(self) -> Option<proto::MessageSendRequest> {
        self.data
    }
}

// Send Request Headers
impl HeaderGetter for proto::NatsMessageSendRequest {
    fn headers(&self) -> &Vec<proto::MetadataMap> {
        &self.headers
    }
}

// ********************************** NATS Request Setter **********************************
impl NatsRequestSetter<proto::MessageSendRequest, proto::NatsMessageSendRequest>
    for proto::NatsMessageSendRequest
{
    fn from_headers_and_message(
        headers: impl Into<Vec<proto::MetadataMap>>,
        data: impl Into<proto::MessageSendRequest>,
    ) -> Self {
        proto::NatsMessageSendRequest {
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
