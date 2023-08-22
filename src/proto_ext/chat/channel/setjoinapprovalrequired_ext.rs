use crate::chat as proto;
use crate::runtiva::nats::v1 as proto_nats;

use crate::proto_ext::{DataGetter, HeaderGetter, NatsRequestSetter};

// ***********************************  Request Getters ***********************************
// SetJoinApprovalRequired Request Data message
impl DataGetter<proto::ChannelSetJoinApprovalRequiredRequest>
    for proto_nats::NatsChannelSetJoinApprovalRequiredRequest
{
    fn to_data(self) -> Option<proto::ChannelSetJoinApprovalRequiredRequest> {
        self.data
    }
}

// SetJoinApprovalRequired Request Headers
impl HeaderGetter for proto_nats::NatsChannelSetJoinApprovalRequiredRequest {
    fn headers(&self) -> &Vec<proto_nats::MetadataMap> {
        &self.headers
    }
}

// ********************************** NATS Request Setter **********************************
impl
    NatsRequestSetter<
        proto::ChannelSetJoinApprovalRequiredRequest,
        proto_nats::NatsChannelSetJoinApprovalRequiredRequest,
    > for proto_nats::NatsChannelSetJoinApprovalRequiredRequest
{
    fn from_headers_and_message(
        headers: impl Into<Vec<proto_nats::MetadataMap>>,
        data: impl Into<proto::ChannelSetJoinApprovalRequiredRequest>,
    ) -> Self {
        proto_nats::NatsChannelSetJoinApprovalRequiredRequest {
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
