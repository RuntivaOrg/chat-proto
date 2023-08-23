use crate::runtiva::{nats::v1 as proto_nats, user::v1 as proto_user};

use crate::proto_ext::{DataGetter, HeaderGetter, NatsRequestSetter};

// ***********************************  Request Getters ***********************************
// Create Request Data message
impl DataGetter<proto_user::UserHeartbeatRequest> for proto_nats::NatsUserHeartbeatRequest {
    fn to_data(self) -> Option<proto_user::UserHeartbeatRequest> {
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
impl NatsRequestSetter<proto_user::UserHeartbeatRequest, proto_nats::NatsUserHeartbeatRequest>
    for proto_nats::NatsUserHeartbeatRequest
{
    fn from_headers_and_message(
        headers: impl Into<Vec<proto_nats::MetadataMap>>,
        data: impl Into<proto_user::UserHeartbeatRequest>,
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
