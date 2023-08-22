use crate::chat as proto;
use crate::runtiva::nats::v1 as proto_nats;

use crate::proto_ext::{
    DataGetter, DataSetter, ErrorGetter, ErrorSetter, HeaderGetter, NatsRequestSetter,
};

// ***********************************  Request Getters ***********************************
// Create Request Data message
impl DataGetter<proto::ChatCreateRequest> for proto_nats::NatsChatCreateRequest {
    fn to_data(self) -> Option<proto::ChatCreateRequest> {
        self.data
    }
}

// Create Request Headers
impl HeaderGetter for proto_nats::NatsChatCreateRequest {
    fn headers(&self) -> &Vec<proto_nats::MetadataMap> {
        &self.headers
    }
}

// ********************************** NATS Request Setter **********************************
impl NatsRequestSetter<proto::ChatCreateRequest, proto_nats::NatsChatCreateRequest>
    for proto_nats::NatsChatCreateRequest
{
    fn from_headers_and_message(
        headers: impl Into<Vec<proto_nats::MetadataMap>>,
        data: impl Into<proto::ChatCreateRequest>,
    ) -> Self {
        proto_nats::NatsChatCreateRequest {
            headers: headers.into(),
            data: Some(data.into()),
        }
    }
}

// ***********************************  Response Getters ***********************************
// Create Response Data setter
impl DataGetter<proto::Chat> for proto_nats::NatsChatCreateResponse {
    fn to_data(self) -> Option<proto::Chat> {
        match self.msg {
            Some(proto_nats::nats_chat_create_response::Msg::Data(data)) => Some(data),
            _ => None,
        }
    }
}

// Create Response Error setter
impl ErrorGetter for proto_nats::NatsChatCreateResponse {
    fn error(&self) -> Option<&proto_nats::ErrorReply> {
        match &self.msg {
            Some(proto_nats::nats_chat_create_response::Msg::Error(e)) => Some(e),
            _ => None,
        }
    }
}

// ***********************************  Response Setters ***********************************
// Create Response Data setter
impl DataSetter<proto::Chat, proto_nats::NatsChatCreateResponse>
    for proto_nats::NatsChatCreateResponse
{
    fn set_data(data: impl Into<proto::Chat>) -> Self {
        let data = data.into();
        proto_nats::NatsChatCreateResponse {
            msg: Some(proto_nats::nats_chat_create_response::Msg::Data(data)),
        }
    }
}

// Create Response Error setter
impl ErrorSetter<proto_nats::ErrorReply, proto_nats::NatsChatCreateResponse>
    for proto_nats::NatsChatCreateResponse
{
    fn set_error(error: impl Into<proto_nats::ErrorReply>) -> Self {
        let error = error.into();
        proto_nats::NatsChatCreateResponse {
            msg: Some(proto_nats::nats_chat_create_response::Msg::Error(error)),
        }
    }
}
