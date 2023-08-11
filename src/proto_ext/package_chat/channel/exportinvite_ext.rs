use crate::chat as proto;

use crate::proto_ext::package_chat::{DataGetter, HeaderGetter, NatsRequestSetter};

// ***********************************  Request Getters ***********************************
// ExportInvite Request Data message
impl DataGetter<proto::ChannelExportInviteRequest> for proto::NatsChannelExportInviteRequest {
    fn to_data(self) -> Option<proto::ChannelExportInviteRequest> {
        self.data
    }
}

// ExportInvite Request Headers
impl HeaderGetter for proto::NatsChannelExportInviteRequest {
    fn headers(&self) -> &Vec<proto::MetadataMap> {
        &self.headers
    }
}

// ********************************** NATS Request Setter **********************************
impl NatsRequestSetter<proto::ChannelExportInviteRequest, proto::NatsChannelExportInviteRequest>
    for proto::NatsChannelExportInviteRequest
{
    fn from_headers_and_message(
        headers: impl Into<Vec<proto::MetadataMap>>,
        data: impl Into<proto::ChannelExportInviteRequest>,
    ) -> Self {
        proto::NatsChannelExportInviteRequest {
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
