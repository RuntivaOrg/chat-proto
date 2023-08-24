use crate::runtiva::{chat::v1 as proto_chat, nats::v1 as proto_nats};

use crate::proto_ext::{
    DataGetter, DataSetter, ErrorGetter, ErrorSetter, HeaderGetter, NatsRequestSetter,
};

// ***********************************  Request Getters ***********************************
// Create Request Data message
impl DataGetter<proto_chat::ChannelCreateRequest> for proto_nats::NatsChannelCreateRequest {
    fn to_data(self) -> Option<proto_chat::ChannelCreateRequest> {
        self.data
    }
}

// Create Request Headers
impl HeaderGetter for proto_nats::NatsChannelCreateRequest {
    fn take_headers(&mut self) -> Vec<proto_nats::MetadataMap> {
        let mut swapped = vec![];
        std::mem::swap(&mut self.headers, &mut swapped);
        swapped
    }
}

// ********************************** NATS Request Setter **********************************
impl NatsRequestSetter<proto_chat::ChannelCreateRequest, proto_nats::NatsChannelCreateRequest>
    for proto_nats::NatsChannelCreateRequest
{
    fn from_headers_and_message(
        headers: impl Into<Vec<proto_nats::MetadataMap>>,
        data: impl Into<proto_chat::ChannelCreateRequest>,
    ) -> Self {
        proto_nats::NatsChannelCreateRequest {
            headers: headers.into(),
            data: Some(data.into()),
        }
    }
}

// ***********************************  Response Getters ***********************************
// Create Response Data getter
impl DataGetter<proto_chat::Channel> for proto_nats::NatsChannelCreateResponse {
    fn to_data(self) -> Option<proto_chat::Channel> {
        match self.msg {
            Some(proto_nats::nats_channel_create_response::Msg::Data(data)) => Some(data),
            _ => None,
        }
    }
}

// Create Response Error getter
impl ErrorGetter for proto_nats::NatsChannelCreateResponse {
    fn error(&self) -> Option<&proto_nats::ErrorReply> {
        match &self.msg {
            Some(proto_nats::nats_channel_create_response::Msg::Error(e)) => Some(e),
            _ => None,
        }
    }
}

// ***********************************  Response Setters ***********************************
// Create Response Data setter
impl DataSetter<proto_chat::Channel, proto_nats::NatsChannelCreateResponse>
    for proto_nats::NatsChannelCreateResponse
{
    fn set_data(data: impl Into<proto_chat::Channel>) -> Self {
        let data = data.into();
        proto_nats::NatsChannelCreateResponse {
            msg: Some(proto_nats::nats_channel_create_response::Msg::Data(data)),
        }
    }
}

// Create Response Error setter
impl ErrorSetter<proto_nats::ErrorReply, proto_nats::NatsChannelCreateResponse>
    for proto_nats::NatsChannelCreateResponse
{
    fn set_error(error: impl Into<proto_nats::ErrorReply>) -> Self {
        let error = error.into();
        proto_nats::NatsChannelCreateResponse {
            msg: Some(proto_nats::nats_channel_create_response::Msg::Error(error)),
        }
    }
}
