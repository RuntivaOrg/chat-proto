use crate::chat as proto;

use crate::proto_ext::proto::{DataGetter, DataSetter, ErrorSetter, HeaderGetter};

// ***********************************  Request Getters ***********************************
// Create Request Data message
impl DataGetter<proto::ChatGroupCreateRequest> for proto::NatsChatGroupCreateRequest {
    fn data(&self) -> &Option<proto::ChatGroupCreateRequest> {
        &self.data
    }
}

// Create Request Headers
impl HeaderGetter for proto::NatsChatGroupCreateRequest {
    fn headers(&self) -> &Vec<proto::MetadataMap> {
        &self.headers
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
