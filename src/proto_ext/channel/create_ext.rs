use crate::chat as proto;

use crate::proto_ext::{
    DataGetter, DataSetter, ErrorGetter, ErrorSetter, HeaderGetter, NatsRequestSetter,
};

// ***********************************  Request Getters ***********************************
// Create Request Data message
impl DataGetter<proto::ChannelCreateRequest> for proto::NatsChannelCreateRequest {
    fn to_data(self) -> Option<proto::ChannelCreateRequest> {
        self.data
    }
}

// Create Request Headers
impl HeaderGetter for proto::NatsChannelCreateRequest {
    fn headers(&self) -> &Vec<proto::MetadataMap> {
        &self.headers
    }
}

// ********************************** NATS Request Setter **********************************
impl NatsRequestSetter<proto::ChannelCreateRequest, proto::NatsChannelCreateRequest>
    for proto::NatsChannelCreateRequest
{
    fn from_headers_and_message(
        headers: impl Into<Vec<proto::MetadataMap>>,
        data: impl Into<proto::ChannelCreateRequest>,
    ) -> Self {
        proto::NatsChannelCreateRequest {
            headers: headers.into(),
            data: Some(data.into()),
        }
    }
}

// ***********************************  Response Getters ***********************************
// Create Response Data setter
impl DataGetter<proto::Channel> for proto::NatsChannelCreateResponse {
    fn to_data(self) -> Option<proto::Channel> {
        match self.msg {
            Some(proto::nats_channel_create_response::Msg::Data(data)) => Some(data),
            _ => None,
        }
    }
}

// Create Response Error setter
impl ErrorGetter for proto::NatsChannelCreateResponse {
    fn error(&self) -> Option<&proto::ErrorReply> {
        match &self.msg {
            Some(proto::nats_channel_create_response::Msg::Error(e)) => Some(e),
            _ => None,
        }
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
