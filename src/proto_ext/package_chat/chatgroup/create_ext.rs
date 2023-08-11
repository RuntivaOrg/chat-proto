use crate::chat as proto;

use crate::proto_ext::package_chat::{
    DataGetter, DataSetter, ErrorGetter, ErrorSetter, HeaderGetter, NatsRequestSetter,
};

// ***********************************  Request Getters ***********************************
// Create Request Data message
impl DataGetter<proto::ChatGroupCreateRequest> for proto::NatsChatGroupCreateRequest {
    fn to_data(self) -> Option<proto::ChatGroupCreateRequest> {
        self.data
    }
}

// Create Request Headers
impl HeaderGetter for proto::NatsChatGroupCreateRequest {
    fn headers(&self) -> &Vec<proto::MetadataMap> {
        &self.headers
    }
}

// ********************************** NATS Request Setter **********************************
impl NatsRequestSetter<proto::ChatGroupCreateRequest, proto::NatsChatGroupCreateRequest>
    for proto::NatsChatGroupCreateRequest
{
    fn from_headers_and_message(
        headers: impl Into<Vec<proto::MetadataMap>>,
        data: impl Into<proto::ChatGroupCreateRequest>,
    ) -> Self {
        proto::NatsChatGroupCreateRequest {
            headers: headers.into(),
            data: Some(data.into()),
        }
    }
}

// ***********************************  Response Getters ***********************************
// Create Response Data setter
impl DataGetter<proto::ChatGroup> for proto::NatsChatGroupCreateResponse {
    fn to_data(self) -> Option<proto::ChatGroup> {
        match self.msg {
            Some(proto::nats_chat_group_create_response::Msg::Data(data)) => Some(data),
            _ => None,
        }
    }
}

// Create Response Error setter
impl ErrorGetter for proto::NatsChatGroupCreateResponse {
    fn error(&self) -> Option<&proto::ErrorReply> {
        match &self.msg {
            Some(proto::nats_chat_group_create_response::Msg::Error(e)) => Some(e),
            _ => None,
        }
    }
}

// ***********************************  Response Setters ***********************************
// Create Response Data setter
impl DataSetter<proto::ChatGroup, proto::NatsChatGroupCreateResponse>
    for proto::NatsChatGroupCreateResponse
{
    fn set_data(data: impl Into<proto::ChatGroup>) -> Self {
        let data = data.into();
        proto::NatsChatGroupCreateResponse {
            msg: Some(proto::nats_chat_group_create_response::Msg::Data(data)),
        }
    }
}

// Create Response Error setter
impl ErrorSetter<proto::ErrorReply, proto::NatsChatGroupCreateResponse>
    for proto::NatsChatGroupCreateResponse
{
    fn set_error(error: impl Into<proto::ErrorReply>) -> Self {
        let error = error.into();
        proto::NatsChatGroupCreateResponse {
            msg: Some(proto::nats_chat_group_create_response::Msg::Error(error)),
        }
    }
}
