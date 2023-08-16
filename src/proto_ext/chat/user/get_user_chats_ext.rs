use crate::chat as proto;

use crate::proto_ext::chat::{
    DataGetter, DataSetter, ErrorGetter, ErrorSetter, HeaderGetter, NatsRequestSetter,
};

// ***********************************  Request Getters ***********************************
// Create Request Data message
impl DataGetter<proto::GetUserChatsRequest> for proto::NatsGetUserChatsRequest {
    fn to_data(self) -> Option<proto::GetUserChatsRequest> {
        self.data
    }
}

// Create Request Headers
impl HeaderGetter for proto::NatsGetUserChatsRequest {
    fn headers(&self) -> &Vec<proto::MetadataMap> {
        &self.headers
    }
}

// ********************************** NATS Request Setter **********************************
impl NatsRequestSetter<proto::GetUserChatsRequest, proto::NatsGetUserChatsRequest>
    for proto::NatsGetUserChatsRequest
{
    fn from_headers_and_message(
        headers: impl Into<Vec<proto::MetadataMap>>,
        data: impl Into<proto::GetUserChatsRequest>,
    ) -> Self {
        proto::NatsGetUserChatsRequest {
            headers: headers.into(),
            data: Some(data.into()),
        }
    }
}

// ***********************************  Response Getters ***********************************
// Create Response Data setter
impl DataGetter<proto::UserPeer> for proto::NatsGetUserChatsResponse {
    fn to_data(self) -> Option<proto::UserPeer> {
        match self.msg {
            Some(proto::nats_get_user_chats_response::Msg::Data(data)) => Some(data),
            _ => None,
        }
    }
}

// Create Response Error setter
impl ErrorGetter for proto::NatsGetUserChatsResponse {
    fn error(&self) -> Option<&proto::ErrorReply> {
        match &self.msg {
            Some(proto::nats_get_user_chats_response::Msg::Error(e)) => Some(e),
            _ => None,
        }
    }
}

// ***********************************  Response Setters ***********************************
// Create Response Data setter
impl DataSetter<proto::UserPeer, proto::NatsGetUserChatsResponse>
    for proto::NatsGetUserChatsResponse
{
    fn set_data(data: impl Into<proto::UserPeer>) -> Self {
        let data = data.into();
        proto::NatsGetUserChatsResponse {
            msg: Some(proto::nats_get_user_chats_response::Msg::Data(data)),
        }
    }
}

// Create Response Error setter
impl ErrorSetter<proto::ErrorReply, proto::NatsGetUserChatsResponse>
    for proto::NatsGetUserChatsResponse
{
    fn set_error(error: impl Into<proto::ErrorReply>) -> Self {
        let error = error.into();
        proto::NatsGetUserChatsResponse {
            msg: Some(proto::nats_get_user_chats_response::Msg::Error(error)),
        }
    }
}
