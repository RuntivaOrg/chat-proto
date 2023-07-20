use crate::chat as proto;

use crate::proto_ext::proto::{DataGetter, DataSetter, ErrorSetter, HeaderGetter};

// ***********************************  Request Getters ***********************************
// Create Request Data message
impl DataGetter<proto::ChannelCreateRequest> for proto::NatsChannelCreateRequest {
    fn data(&self) -> &Option<proto::ChannelCreateRequest> {
        &self.data
    }
}

// Create Request Headers
impl HeaderGetter for proto::NatsChannelCreateRequest {
    fn headers(&self) -> &Vec<proto::MetadataMap> {
        &self.headers
    }
}

// ***********************************  Response Setters ***********************************
// Create Response Data setter
impl DataSetter<proto::Channel, proto::NatsChannelCreateResponse>
    for proto::NatsChannelCreateResponse
{
    fn set_data(data: impl Into<proto::Channel>) -> Self {
        let data = data.into();
        proto::NatsChannelCreateResponse {
            msg: Some(proto::nats_channel_create_response::Msg::Data(data)),
        }
    }
}

// Create Response Error setter
impl ErrorSetter<proto::ErrorReply, proto::NatsChannelCreateResponse>
    for proto::NatsChannelCreateResponse
{
    fn set_error(error: impl Into<proto::ErrorReply>) -> Self {
        let error = error.into();
        proto::NatsChannelCreateResponse {
            msg: Some(proto::nats_channel_create_response::Msg::Error(error)),
        }
    }
}
