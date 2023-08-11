use crate::chat as proto;

use crate::proto_ext::chat::{DataGetter, HeaderGetter, NatsRequestSetter};

// ***********************************  Request Getters ***********************************
// Create Request Data message
impl DataGetter<proto::UserHeartbeatRequest> for proto::NatsUserHeartbeatRequest {
    fn to_data(self) -> Option<proto::UserHeartbeatRequest> {
        self.data
    }
}

// Create Request Headers
impl HeaderGetter for proto::NatsUserHeartbeatRequest {
    fn headers(&self) -> &Vec<proto::MetadataMap> {
        &self.headers
    }
}

// ********************************** NATS Request Setter **********************************
impl NatsRequestSetter<proto::UserHeartbeatRequest, proto::NatsUserHeartbeatRequest>
    for proto::NatsUserHeartbeatRequest
{
    fn from_headers_and_message(
        headers: impl Into<Vec<proto::MetadataMap>>,
        data: impl Into<proto::UserHeartbeatRequest>,
    ) -> Self {
        proto::NatsUserHeartbeatRequest {
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
