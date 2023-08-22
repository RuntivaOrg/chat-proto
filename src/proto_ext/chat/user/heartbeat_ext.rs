use crate::chat as proto;
use crate::runtiva::nats::v1 as proto_nats;

use crate::proto_ext::{DataGetter, HeaderGetter, NatsRequestSetter};

// ***********************************  Request Getters ***********************************
// Create Request Data message
impl DataGetter<proto::UserHeartbeatRequest> for proto_nats::NatsUserHeartbeatRequest {
    fn to_data(self) -> Option<proto::UserHeartbeatRequest> {
        self.data
    }
}

// Create Request Headers
impl HeaderGetter for proto_nats::NatsUserHeartbeatRequest {
    fn headers(&self) -> &Vec<proto_nats::MetadataMap> {
        &self.headers
    }
}

// ********************************** NATS Request Setter **********************************
impl NatsRequestSetter<proto::UserHeartbeatRequest, proto_nats::NatsUserHeartbeatRequest>
    for proto_nats::NatsUserHeartbeatRequest
{
    fn from_headers_and_message(
        headers: impl Into<Vec<proto_nats::MetadataMap>>,
        data: impl Into<proto::UserHeartbeatRequest>,
    ) -> Self {
        proto_nats::NatsUserHeartbeatRequest {
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
