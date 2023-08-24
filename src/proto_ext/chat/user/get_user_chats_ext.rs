use crate::runtiva::{nats::v1 as proto_nats, user::v1 as proto_user};

use crate::proto_ext::{
    DataGetter, DataSetter, ErrorGetter, ErrorSetter, HeaderGetter, NatsRequestSetter,
};

// ***********************************  Request Getters ***********************************
// Create Request Data message
impl DataGetter<proto_user::GetUserChatsRequest> for proto_nats::NatsGetUserChatsRequest {
    fn to_data(self) -> Option<proto_user::GetUserChatsRequest> {
        self.data
    }
}

// Create Request Headers
impl HeaderGetter for proto_nats::NatsGetUserChatsRequest {
    fn take_headers(&mut self) -> Vec<proto_nats::MetadataMap> {
        let mut swapped = vec![];
        std::mem::swap(&mut self.headers, &mut swapped);
        swapped
    }
}

// ********************************** NATS Request Setter **********************************
impl NatsRequestSetter<proto_user::GetUserChatsRequest, proto_nats::NatsGetUserChatsRequest>
    for proto_nats::NatsGetUserChatsRequest
{
    fn from_headers_and_message(
        headers: impl Into<Vec<proto_nats::MetadataMap>>,
        data: impl Into<proto_user::GetUserChatsRequest>,
    ) -> Self {
        proto_nats::NatsGetUserChatsRequest {
            headers: headers.into(),
            data: Some(data.into()),
        }
    }
}

// ***********************************  Response Getters ***********************************
// Create Response Data setter
impl DataGetter<proto_user::GetUserChatsResponse> for proto_nats::NatsGetUserChatsResponse {
    fn to_data(self) -> Option<proto_user::GetUserChatsResponse> {
        match self.msg {
            Some(proto_nats::nats_get_user_chats_response::Msg::Data(data)) => Some(data),
            _ => None,
        }
    }
}

// Create Response Error setter
impl ErrorGetter for proto_nats::NatsGetUserChatsResponse {
    fn error(&self) -> Option<&proto_nats::ErrorReply> {
        match &self.msg {
            Some(proto_nats::nats_get_user_chats_response::Msg::Error(e)) => Some(e),
            _ => None,
        }
    }
}

// ***********************************  Response Setters ***********************************
// Create Response Data setter
impl DataSetter<proto_user::GetUserChatsResponse, proto_nats::NatsGetUserChatsResponse>
    for proto_nats::NatsGetUserChatsResponse
{
    fn set_data(data: impl Into<proto_user::GetUserChatsResponse>) -> Self {
        let data = data.into();
        proto_nats::NatsGetUserChatsResponse {
            msg: Some(proto_nats::nats_get_user_chats_response::Msg::Data(data)),
        }
    }
}

// Create Response Error setter
impl ErrorSetter<proto_nats::ErrorReply, proto_nats::NatsGetUserChatsResponse>
    for proto_nats::NatsGetUserChatsResponse
{
    fn set_error(error: impl Into<proto_nats::ErrorReply>) -> Self {
        let error = error.into();
        proto_nats::NatsGetUserChatsResponse {
            msg: Some(proto_nats::nats_get_user_chats_response::Msg::Error(error)),
        }
    }
}
