use crate::chat as proto;

use crate::proto_ext::{
    DataGetter, DataSetter, ErrorGetter, ErrorSetter, HeaderGetter, NatsRequestSetter,
};

// ***********************************  Request Getters ***********************************
// Join Request Data message
impl DataGetter<proto::ChannelJoinRequest> for proto::NatsChannelJoinRequest {
    fn to_data(self) -> Option<proto::ChannelJoinRequest> {
        self.data
    }
}

// Join Request Headers
impl HeaderGetter for proto::NatsChannelJoinRequest {
    fn headers(&self) -> &Vec<proto::MetadataMap> {
        &self.headers
    }
}

// ********************************** NATS Request Setter **********************************
impl NatsRequestSetter<proto::ChannelJoinRequest, proto::NatsChannelJoinRequest>
    for proto::NatsChannelJoinRequest
{
    fn from_headers_and_message(
        headers: impl Into<Vec<proto::MetadataMap>>,
        data: impl Into<proto::ChannelJoinRequest>,
    ) -> Self {
        proto::NatsChannelJoinRequest {
            headers: headers.into(),
            data: Some(data.into()),
        }
    }
}

// ***********************************  Response Getters ***********************************
// Join Response Data getter
impl DataGetter<proto::ChannelJoinResponse> for proto::NatsChannelJoinResponse {
    fn to_data(self) -> Option<proto::ChannelJoinResponse> {
        match self.msg {
            Some(proto::nats_channel_join_response::Msg::Data(data)) => Some(data),
            _ => None,
        }
    }
}

// Join Response Error getter
impl ErrorGetter for proto::NatsChannelJoinResponse {
    fn error(&self) -> Option<&proto::ErrorReply> {
        match &self.msg {
            Some(proto::nats_channel_join_response::Msg::Error(e)) => Some(e),
            _ => None,
        }
    }
}

// ***********************************  Response Setters ***********************************
// Join Response Data setter
impl DataSetter<proto::ChannelJoinResponse, proto::NatsChannelJoinResponse>
    for proto::NatsChannelJoinResponse
{
    fn set_data(data: impl Into<proto::ChannelJoinResponse>) -> Self {
        let data = data.into();
        proto::NatsChannelJoinResponse {
            msg: Some(proto::nats_channel_join_response::Msg::Data(data)),
        }
    }
}

// Join Response Error setter
impl ErrorSetter<proto::ErrorReply, proto::NatsChannelJoinResponse>
    for proto::NatsChannelJoinResponse
{
    fn set_error(error: impl Into<proto::ErrorReply>) -> Self {
        let error = error.into();
        proto::NatsChannelJoinResponse {
            msg: Some(proto::nats_channel_join_response::Msg::Error(error)),
        }
    }
}
