use crate::chat as proto;
use crate::runtiva::nats::v1 as proto_nats;

use crate::proto_ext::{DataGetter, HeaderGetter, NatsRequestSetter};

// ***********************************  Request Getters ***********************************
// Delete Request Data message
impl DataGetter<proto::UserDeleteRequest> for proto_nats::NatsUserDeleteRequest {
    fn to_data(self) -> Option<proto::UserDeleteRequest> {
        self.data
    }
}

// Delete Request Headers
impl HeaderGetter for proto_nats::NatsUserDeleteRequest {
    fn headers(&self) -> &Vec<proto_nats::MetadataMap> {
        &self.headers
    }
}

// ********************************** NATS Request Setter **********************************
impl NatsRequestSetter<proto::UserDeleteRequest, proto_nats::NatsUserDeleteRequest>
    for proto_nats::NatsUserDeleteRequest
{
    fn from_headers_and_message(
        headers: impl Into<Vec<proto_nats::MetadataMap>>,
        data: impl Into<proto::UserDeleteRequest>,
    ) -> Self {
        proto_nats::NatsUserDeleteRequest {
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
