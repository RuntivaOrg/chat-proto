use crate::chat as proto;

use crate::proto_ext::chat::{DataGetter, HeaderGetter, NatsRequestSetter};

// ***********************************  Request Getters ***********************************
// EditAdmin Request Data message
impl DataGetter<proto::ChannelEditAdminRequest> for proto::NatsChannelEditAdminRequest {
    fn to_data(self) -> Option<proto::ChannelEditAdminRequest> {
        self.data
    }
}

// EditAdmin Request Headers
impl HeaderGetter for proto::NatsChannelEditAdminRequest {
    fn headers(&self) -> &Vec<proto::MetadataMap> {
        &self.headers
    }
}

// ********************************** NATS Request Setter **********************************
impl NatsRequestSetter<proto::ChannelEditAdminRequest, proto::NatsChannelEditAdminRequest>
    for proto::NatsChannelEditAdminRequest
{
    fn from_headers_and_message(
        headers: impl Into<Vec<proto::MetadataMap>>,
        data: impl Into<proto::ChannelEditAdminRequest>,
    ) -> Self {
        proto::NatsChannelEditAdminRequest {
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
