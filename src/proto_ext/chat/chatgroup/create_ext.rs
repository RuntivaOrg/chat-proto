use crate::runtiva::{chat::v1 as proto_chat, nats::v1 as proto_nats};

use crate::proto_ext::{
    DataGetter, DataSetter, ErrorGetter, ErrorSetter, HeaderGetter, NatsRequestSetter,
};

// ***********************************  Request Getters ***********************************
// Create Request Data message
impl DataGetter<proto_chat::ChatGroupCreateRequest> for proto_nats::NatsChatGroupCreateRequest {
    fn to_data(self) -> Option<proto_chat::ChatGroupCreateRequest> {
        self.data
    }
}

// Create Request Headers
impl HeaderGetter for proto_nats::NatsChatGroupCreateRequest {
    fn take_headers(&mut self) -> Vec<proto_nats::MetadataMap> {
        let mut swapped = vec![];
        std::mem::swap(&mut self.headers, &mut swapped);
        swapped
    }
}

// ********************************** NATS Request Setter **********************************
impl NatsRequestSetter<proto_chat::ChatGroupCreateRequest, proto_nats::NatsChatGroupCreateRequest>
    for proto_nats::NatsChatGroupCreateRequest
{
    fn from_headers_and_message(
        headers: impl Into<Vec<proto_nats::MetadataMap>>,
        data: impl Into<proto_chat::ChatGroupCreateRequest>,
    ) -> Self {
        proto_nats::NatsChatGroupCreateRequest {
            headers: headers.into(),
            data: Some(data.into()),
        }
    }
}

// ***********************************  Response Getters ***********************************
// Create Response Data setter
impl DataGetter<proto_chat::ChatGroup> for proto_nats::NatsChatGroupCreateResponse {
    fn to_data(self) -> Option<proto_chat::ChatGroup> {
        match self.msg {
            Some(proto_nats::nats_chat_group_create_response::Msg::Data(data)) => Some(data),
            _ => None,
        }
    }
}

// Create Response Error setter
impl ErrorGetter for proto_nats::NatsChatGroupCreateResponse {
    fn error(&self) -> Option<&proto_nats::ErrorReply> {
        match &self.msg {
            Some(proto_nats::nats_chat_group_create_response::Msg::Error(e)) => Some(e),
            _ => None,
        }
    }
}

// ***********************************  Response Setters ***********************************
// Create Response Data setter
impl DataSetter<proto_chat::ChatGroup, proto_nats::NatsChatGroupCreateResponse>
    for proto_nats::NatsChatGroupCreateResponse
{
    fn set_data(data: impl Into<proto_chat::ChatGroup>) -> Self {
        let data = data.into();
        proto_nats::NatsChatGroupCreateResponse {
            msg: Some(proto_nats::nats_chat_group_create_response::Msg::Data(data)),
        }
    }
}

// Create Response Error setter
impl ErrorSetter<proto_nats::ErrorReply, proto_nats::NatsChatGroupCreateResponse>
    for proto_nats::NatsChatGroupCreateResponse
{
    fn set_error(error: impl Into<proto_nats::ErrorReply>) -> Self {
        let error = error.into();
        proto_nats::NatsChatGroupCreateResponse {
            msg: Some(proto_nats::nats_chat_group_create_response::Msg::Error(
                error,
            )),
        }
    }
}
