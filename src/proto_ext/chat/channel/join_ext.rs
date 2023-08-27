use crate::runtiva::{chat::v1 as proto_chat, nats::v1 as proto_nats};

use crate::proto_ext::{
    DataGetter, DataSetter, ErrorGetter, ErrorSetter, HeaderGetter, NatsRequestSetter,
};

// ***********************************  Request Getters ***********************************
// Join Request Data message
impl DataGetter<proto_chat::ChannelJoinRequest> for proto_nats::NatsChannelJoinRequest {
    fn to_data(self) -> Option<proto_chat::ChannelJoinRequest> {
        self.data
    }
}

// Join Request Headers
impl HeaderGetter for proto_nats::NatsChannelJoinRequest {
    fn headers(&self) -> &[proto_nats::MetadataMap] {
        &self.headers
    }

    fn take_headers(&mut self) -> Vec<proto_nats::MetadataMap> {
        let mut swapped = vec![];
        std::mem::swap(&mut self.headers, &mut swapped);
        swapped
    }
}

// ********************************** NATS Request Setter **********************************
impl NatsRequestSetter<proto_chat::ChannelJoinRequest, proto_nats::NatsChannelJoinRequest>
    for proto_nats::NatsChannelJoinRequest
{
    fn from_headers_and_message(
        headers: impl Into<Vec<proto_nats::MetadataMap>>,
        data: impl Into<proto_chat::ChannelJoinRequest>,
    ) -> Self {
        proto_nats::NatsChannelJoinRequest {
            headers: headers.into(),
            data: Some(data.into()),
        }
    }
}

// ***********************************  Response Getters ***********************************
// Join Response Data getter
impl DataGetter<proto_chat::ChannelJoinResponse> for proto_nats::NatsChannelJoinResponse {
    fn to_data(self) -> Option<proto_chat::ChannelJoinResponse> {
        match self.msg {
            Some(proto_nats::nats_channel_join_response::Msg::Data(data)) => Some(data),
            _ => None,
        }
    }
}

// Join Response Error getter
impl ErrorGetter for proto_nats::NatsChannelJoinResponse {
    fn error(&self) -> Option<&proto_nats::ErrorReply> {
        match &self.msg {
            Some(proto_nats::nats_channel_join_response::Msg::Error(e)) => Some(e),
            _ => None,
        }
    }
}

// ***********************************  Response Setters ***********************************
// Join Response Data setter
impl DataSetter<proto_chat::ChannelJoinResponse, proto_nats::NatsChannelJoinResponse>
    for proto_nats::NatsChannelJoinResponse
{
    fn set_data(data: impl Into<proto_chat::ChannelJoinResponse>) -> Self {
        let data = data.into();
        proto_nats::NatsChannelJoinResponse {
            msg: Some(proto_nats::nats_channel_join_response::Msg::Data(data)),
        }
    }
}

// Join Response Error setter
impl ErrorSetter<proto_nats::ErrorReply, proto_nats::NatsChannelJoinResponse>
    for proto_nats::NatsChannelJoinResponse
{
    fn set_error(error: impl Into<proto_nats::ErrorReply>) -> Self {
        let error = error.into();
        proto_nats::NatsChannelJoinResponse {
            msg: Some(proto_nats::nats_channel_join_response::Msg::Error(error)),
        }
    }
}
