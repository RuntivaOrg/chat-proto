use crate::chat as proto;
use crate::chatgroups::v1 as proto_chatgroups;
use crate::proto_ext::chat::{
    DataGetter, DataSetter, ErrorGetter, ErrorSetter, HeaderGetter, NatsRequestSetter,
};

// ***********************************  Request Getters ***********************************
// Create Request Data message
impl DataGetter<proto_chatgroups::ChatGroupCreateRequest>
    for proto_chatgroups::NatsChatGroupCreateRequest
{
    fn to_data(self) -> Option<proto_chatgroups::ChatGroupCreateRequest> {
        self.data
    }
}

// Create Request Headers
impl HeaderGetter for proto_chatgroups::NatsChatGroupCreateRequest {
    fn headers(&self) -> &Vec<proto::MetadataMap> {
        &self.headers
    }
}

// ********************************** NATS Request Setter **********************************
impl
    NatsRequestSetter<
        proto_chatgroups::ChatGroupCreateRequest,
        proto_chatgroups::NatsChatGroupCreateRequest,
    > for proto_chatgroups::NatsChatGroupCreateRequest
{
    fn from_headers_and_message(
        headers: impl Into<Vec<proto::MetadataMap>>,
        data: impl Into<proto_chatgroups::ChatGroupCreateRequest>,
    ) -> Self {
        proto_chatgroups::NatsChatGroupCreateRequest {
            headers: headers.into(),
            data: Some(data.into()),
        }
    }
}

// ***********************************  Response Getters ***********************************
// Create Response Data setter
impl DataGetter<proto_chatgroups::ChatGroup> for proto_chatgroups::NatsChatGroupCreateResponse {
    fn to_data(self) -> Option<proto_chatgroups::ChatGroup> {
        match self.msg {
            Some(proto_chatgroups::nats_chat_group_create_response::Msg::Data(data)) => Some(data),
            _ => None,
        }
    }
}

// Create Response Error setter
impl ErrorGetter for proto_chatgroups::NatsChatGroupCreateResponse {
    fn error(&self) -> Option<&proto::ErrorReply> {
        match &self.msg {
            Some(proto_chatgroups::nats_chat_group_create_response::Msg::Error(e)) => Some(e),
            _ => None,
        }
    }
}

// ***********************************  Response Setters ***********************************
// Create Response Data setter
impl DataSetter<proto_chatgroups::ChatGroup, proto_chatgroups::NatsChatGroupCreateResponse>
    for proto_chatgroups::NatsChatGroupCreateResponse
{
    fn set_data(data: impl Into<proto_chatgroups::ChatGroup>) -> Self {
        let data = data.into();
        proto_chatgroups::NatsChatGroupCreateResponse {
            msg: Some(proto_chatgroups::nats_chat_group_create_response::Msg::Data(data)),
        }
    }
}

// Create Response Error setter
impl ErrorSetter<proto::ErrorReply, proto_chatgroups::NatsChatGroupCreateResponse>
    for proto_chatgroups::NatsChatGroupCreateResponse
{
    fn set_error(error: impl Into<proto::ErrorReply>) -> Self {
        let error = error.into();
        proto_chatgroups::NatsChatGroupCreateResponse {
            msg: Some(proto_chatgroups::nats_chat_group_create_response::Msg::Error(error)),
        }
    }
}
