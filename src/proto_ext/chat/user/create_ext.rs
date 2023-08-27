use crate::runtiva::{nats::v1 as proto_nats, user::v1 as proto_user};

use crate::proto_ext::{
    DataGetter, DataSetter, ErrorGetter, ErrorSetter, HeaderGetter, NatsRequestSetter,
};

// ***********************************  Request Getters ***********************************
// Create Request Data message
impl DataGetter<proto_user::UserCreateRequest> for proto_nats::NatsUserCreateRequest {
    fn to_data(self) -> Option<proto_user::UserCreateRequest> {
        self.data
    }
}

// Create Request Headers
impl HeaderGetter for proto_nats::NatsUserCreateRequest {
    fn headers(&self) -> &[proto_nats::MetadataMap] {
        &self.headers
    }

    fn take_headers(&mut self) -> Vec<proto_nats::MetadataMap> {
        let mut swapped = vec![];
        std::mem::swap(&mut self.headers, &mut swapped);
        swapped
    }
}

// ********************************** NATS Request Setter **********************************
impl NatsRequestSetter<proto_user::UserCreateRequest, proto_nats::NatsUserCreateRequest>
    for proto_nats::NatsUserCreateRequest
{
    fn from_headers_and_message(
        headers: impl Into<Vec<proto_nats::MetadataMap>>,
        data: impl Into<proto_user::UserCreateRequest>,
    ) -> Self {
        proto_nats::NatsUserCreateRequest {
            headers: headers.into(),
            data: Some(data.into()),
        }
    }
}

// ***********************************  Response Getters ***********************************
// Create Response Data setter
impl DataGetter<proto_user::User> for proto_nats::NatsUserCreateResponse {
    fn to_data(self) -> Option<proto_user::User> {
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
impl DataSetter<proto_user::User, proto_nats::NatsUserCreateResponse>
    for proto_nats::NatsUserCreateResponse
{
    fn set_data(data: impl Into<proto_user::User>) -> Self {
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
