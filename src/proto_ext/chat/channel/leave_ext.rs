use crate::chat as proto;
use crate::runtiva::nats::v1 as proto_nats;

use crate::proto_ext::{DataGetter, HeaderGetter, NatsRequestSetter};

// ***********************************  Request Getters ***********************************
// Leave Request Data message
impl DataGetter<proto::ChannelLeaveRequest> for proto_nats::NatsChannelLeaveRequest {
    fn to_data(self) -> Option<proto::ChannelLeaveRequest> {
        self.data
    }
}

// Leave Request Headers
impl HeaderGetter for proto_nats::NatsChannelLeaveRequest {
    fn headers(&self) -> &Vec<proto_nats::MetadataMap> {
        &self.headers
    }
}

// ********************************** NATS Request Setter **********************************
impl NatsRequestSetter<proto::ChannelLeaveRequest, proto_nats::NatsChannelLeaveRequest>
    for proto_nats::NatsChannelLeaveRequest
{
    fn from_headers_and_message(
        headers: impl Into<Vec<proto_nats::MetadataMap>>,
        data: impl Into<proto::ChannelLeaveRequest>,
    ) -> Self {
        proto_nats::NatsChannelLeaveRequest {
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
