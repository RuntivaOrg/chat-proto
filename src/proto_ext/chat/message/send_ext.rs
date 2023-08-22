use crate::chat as proto;
use crate::runtiva::nats::v1 as proto_nats;

use crate::proto_ext::{DataGetter, HeaderGetter, NatsRequestSetter};

// ***********************************  Request Getters ***********************************
// Send Request Data message
impl DataGetter<proto::MessageSendRequest> for proto::NatsMessageSendRequest {
    fn to_data(self) -> Option<proto::MessageSendRequest> {
        self.data
    }
}

// Send Request Headers
impl HeaderGetter for proto::NatsMessageSendRequest {
    fn headers(&self) -> &Vec<proto_nats::MetadataMap> {
        &self.headers
    }
}

// ********************************** NATS Request Setter **********************************
impl NatsRequestSetter<proto::MessageSendRequest, proto::NatsMessageSendRequest>
    for proto::NatsMessageSendRequest
{
    fn from_headers_and_message(
        headers: impl Into<Vec<proto_nats::MetadataMap>>,
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
