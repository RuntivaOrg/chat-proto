use crate::chat as proto;
use crate::runtiva::nats::v1 as proto_nats;

use crate::proto_ext::{
    DataGetter, DataSetter, ErrorGetter, ErrorSetter, HeaderGetter, NatsRequestSetter,
};

// ***********************************  Request Getters ***********************************
// Create Request Data message
impl DataGetter<proto::UserCreateRequest> for proto_nats::NatsUserCreateRequest {
    fn to_data(self) -> Option<proto::UserCreateRequest> {
        self.data
    }
}

// Create Request Headers
impl HeaderGetter for proto_nats::NatsUserCreateRequest {
    fn headers(&self) -> &Vec<proto_nats::MetadataMap> {
        &self.headers
    }
}

// ********************************** NATS Request Setter **********************************
impl NatsRequestSetter<proto::UserCreateRequest, proto_nats::NatsUserCreateRequest>
    for proto_nats::NatsUserCreateRequest
{
    fn from_headers_and_message(
        headers: impl Into<Vec<proto_nats::MetadataMap>>,
        data: impl Into<proto::UserCreateRequest>,
    ) -> Self {
        proto_nats::NatsUserCreateRequest {
            headers: headers.into(),
            data: Some(data.into()),
        }
    }
}

// ***********************************  Response Getters ***********************************
// Create Response Data setter
impl DataGetter<proto::User> for proto_nats::NatsUserCreateResponse {
    fn to_data(self) -> Option<proto::User> {
        match self.msg {
            Some(proto_nats::nats_user_create_response::Msg::Data(data)) => Some(data),
            _ => None,
        }
    }
}

// Create Response Error setter
impl ErrorGetter for proto_nats::NatsUserCreateResponse {
    fn error(&self) -> Option<&proto_nats::ErrorReply> {
        match &self.msg {
            Some(proto_nats::nats_user_create_response::Msg::Error(e)) => Some(e),
            _ => None,
        }
    }
}

// ***********************************  Response Setters ***********************************
// Create Response Data setter
impl DataSetter<proto::User, proto_nats::NatsUserCreateResponse>
    for proto_nats::NatsUserCreateResponse
{
    fn set_data(data: impl Into<proto::User>) -> Self {
        let data = data.into();
        proto_nats::NatsUserCreateResponse {
            msg: Some(proto_nats::nats_user_create_response::Msg::Data(data)),
        }
    }
}

// Create Response Error setter
impl ErrorSetter<proto_nats::ErrorReply, proto_nats::NatsUserCreateResponse>
    for proto_nats::NatsUserCreateResponse
{
    fn set_error(error: impl Into<proto_nats::ErrorReply>) -> Self {
        let error = error.into();
        proto_nats::NatsUserCreateResponse {
            msg: Some(proto_nats::nats_user_create_response::Msg::Error(error)),
        }
    }
}
