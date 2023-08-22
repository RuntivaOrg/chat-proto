use crate::chat as proto;
use crate::runtiva::nats::v1 as proto_nats;

use crate::proto_ext::{
    DataGetter, DataSetter, ErrorGetter, ErrorSetter, HeaderGetter, NatsRequestSetter,
};

// ***********************************  Request Getters ***********************************
// Create Request Data message
impl DataGetter<proto::GetUserChatsRequest> for proto_nats::NatsGetUserChatsRequest {
    fn to_data(self) -> Option<proto::GetUserChatsRequest> {
        self.data
    }
}

// Create Request Headers
impl HeaderGetter for proto_nats::NatsGetUserChatsRequest {
    fn headers(&self) -> &Vec<proto_nats::MetadataMap> {
        &self.headers
    }
}

// ********************************** NATS Request Setter **********************************
impl NatsRequestSetter<proto::GetUserChatsRequest, proto_nats::NatsGetUserChatsRequest>
    for proto_nats::NatsGetUserChatsRequest
{
    fn from_headers_and_message(
        headers: impl Into<Vec<proto_nats::MetadataMap>>,
        data: impl Into<proto::GetUserChatsRequest>,
    ) -> Self {
        proto_nats::NatsGetUserChatsRequest {
            headers: headers.into(),
            data: Some(data.into()),
        }
    }
}

// ***********************************  Response Getters ***********************************
// Create Response Data setter
impl DataGetter<proto::GetUserChatsResponse> for proto_nats::NatsGetUserChatsResponse {
    fn to_data(self) -> Option<proto::GetUserChatsResponse> {
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
impl DataSetter<proto::GetUserChatsResponse, proto_nats::NatsGetUserChatsResponse>
    for proto_nats::NatsGetUserChatsResponse
{
    fn set_data(data: impl Into<proto::GetUserChatsResponse>) -> Self {
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
