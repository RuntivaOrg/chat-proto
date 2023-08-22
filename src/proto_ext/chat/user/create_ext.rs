use crate::chat as proto;
use crate::runtiva::nats::v1 as proto_nats;

use crate::proto_ext::{
    DataGetter, DataSetter, ErrorGetter, ErrorSetter, HeaderGetter, NatsRequestSetter,
};

// ***********************************  Request Getters ***********************************
// Create Request Data message
impl DataGetter<proto::UserCreateRequest> for proto::NatsUserCreateRequest {
    fn to_data(self) -> Option<proto::UserCreateRequest> {
        self.data
    }
}

// Create Request Headers
impl HeaderGetter for proto::NatsUserCreateRequest {
    fn headers(&self) -> &Vec<proto_nats::MetadataMap> {
        &self.headers
    }
}

// ********************************** NATS Request Setter **********************************
impl NatsRequestSetter<proto::UserCreateRequest, proto::NatsUserCreateRequest>
    for proto::NatsUserCreateRequest
{
    fn from_headers_and_message(
        headers: impl Into<Vec<proto_nats::MetadataMap>>,
        data: impl Into<proto::UserCreateRequest>,
    ) -> Self {
        proto::NatsUserCreateRequest {
            headers: headers.into(),
            data: Some(data.into()),
        }
    }
}

// ***********************************  Response Getters ***********************************
// Create Response Data setter
impl DataGetter<proto::User> for proto::NatsUserCreateResponse {
    fn to_data(self) -> Option<proto::User> {
        match self.msg {
            Some(proto::nats_user_create_response::Msg::Data(data)) => Some(data),
            _ => None,
        }
    }
}

// Create Response Error setter
impl ErrorGetter for proto::NatsUserCreateResponse {
    fn error(&self) -> Option<&proto::ErrorReply> {
        match &self.msg {
            Some(proto::nats_user_create_response::Msg::Error(e)) => Some(e),
            _ => None,
        }
    }
}

// ***********************************  Response Setters ***********************************
// Create Response Data setter
impl DataSetter<proto::User, proto::NatsUserCreateResponse> for proto::NatsUserCreateResponse {
    fn set_data(data: impl Into<proto::User>) -> Self {
        let data = data.into();
        proto::NatsUserCreateResponse {
            msg: Some(proto::nats_user_create_response::Msg::Data(data)),
        }
    }
}

// Create Response Error setter
impl ErrorSetter<proto::ErrorReply, proto::NatsUserCreateResponse>
    for proto::NatsUserCreateResponse
{
    fn set_error(error: impl Into<proto::ErrorReply>) -> Self {
        let error = error.into();
        proto::NatsUserCreateResponse {
            msg: Some(proto::nats_user_create_response::Msg::Error(error)),
        }
    }
}
