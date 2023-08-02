use crate::chat as proto;

use crate::proto_ext::{DataGetter, HeaderGetter, NatsRequestSetter};

// ***********************************  Request Getters ***********************************
// EditPhoto Request Data message
impl DataGetter<proto::ChannelEditPhotoRequest> for proto::NatsChannelEditPhotoRequest {
    fn to_data(self) -> Option<proto::ChannelEditPhotoRequest> {
        self.data
    }
}

// EditPhoto Request Headers
impl HeaderGetter for proto::NatsChannelEditPhotoRequest {
    fn headers(&self) -> &Vec<proto::MetadataMap> {
        &self.headers
    }
}

// ********************************** NATS Request Setter **********************************
impl NatsRequestSetter<proto::ChannelEditPhotoRequest, proto::NatsChannelEditPhotoRequest>
    for proto::NatsChannelEditPhotoRequest
{
    fn from_headers_and_message(
        headers: impl Into<Vec<proto::MetadataMap>>,
        data: impl Into<proto::ChannelEditPhotoRequest>,
    ) -> Self {
        proto::NatsChannelEditPhotoRequest {
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
