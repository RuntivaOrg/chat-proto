use crate::chat as proto;

use crate::proto_ext::package_chat::{DataGetter, HeaderGetter, NatsRequestSetter};

// ***********************************  Request Getters ***********************************
// SetJoinApprovalRequired Request Data message
impl DataGetter<proto::ChannelSetJoinApprovalRequiredRequest>
    for proto::NatsChannelSetJoinApprovalRequiredRequest
{
    fn to_data(self) -> Option<proto::ChannelSetJoinApprovalRequiredRequest> {
        self.data
    }
}

// SetJoinApprovalRequired Request Headers
impl HeaderGetter for proto::NatsChannelSetJoinApprovalRequiredRequest {
    fn headers(&self) -> &Vec<proto::MetadataMap> {
        &self.headers
    }
}

// ********************************** NATS Request Setter **********************************
impl
    NatsRequestSetter<
        proto::ChannelSetJoinApprovalRequiredRequest,
        proto::NatsChannelSetJoinApprovalRequiredRequest,
    > for proto::NatsChannelSetJoinApprovalRequiredRequest
{
    fn from_headers_and_message(
        headers: impl Into<Vec<proto::MetadataMap>>,
        data: impl Into<proto::ChannelSetJoinApprovalRequiredRequest>,
    ) -> Self {
        proto::NatsChannelSetJoinApprovalRequiredRequest {
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
