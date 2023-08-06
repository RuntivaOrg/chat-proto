use crate::chat as proto;

use crate::proto_ext::{DataGetter, HeaderGetter, NatsRequestSetter};

// ***********************************  Request Getters ***********************************
// RemoveUser Request Data message
impl DataGetter<proto::ChannelDisableRequest> for proto::NatsChannelDisableRequest {
    fn to_data(self) -> Option<proto::ChannelDisableRequest> {
        self.data
    }
}

// RemoveUser Request Headers
impl HeaderGetter for proto::NatsChannelDisableRequest {
    fn headers(&self) -> &Vec<proto::MetadataMap> {
        &self.headers
    }
}

// ********************************** NATS Request Setter **********************************
impl NatsRequestSetter<proto::ChannelDisableRequest, proto::NatsChannelDisableRequest>
    for proto::NatsChannelDisableRequest
{
    fn from_headers_and_message(
        headers: impl Into<Vec<proto::MetadataMap>>,
        data: impl Into<proto::ChannelDisableRequest>,
    ) -> Self {
        proto::NatsChannelDisableRequest {
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
