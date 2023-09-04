use crate::runtiva::{nats::v1 as proto_nats, updates::v1 as proto_updates};

use crate::proto_ext::{
    DataGetter, DataSetter, ErrorGetter, ErrorSetter, HeaderGetter, NatsRequestSetter,
};

// ***********************************  Request Getters ***********************************
// Create Request Data message
impl DataGetter<proto_updates::GetUserStateRequest> for proto_nats::NatsGetUserStateRequest {
    fn to_data(self) -> Option<proto_updates::GetUserStateRequest> {
        self.data
    }
}

// Create Request Headers
impl HeaderGetter for proto_nats::NatsGetUserStateRequest {
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
impl NatsRequestSetter<proto_updates::GetUserStateRequest, proto_nats::NatsGetUserStateRequest>
    for proto_nats::NatsGetUserStateRequest
{
    fn from_headers_and_message(
        headers: impl Into<Vec<proto_nats::MetadataMap>>,
        data: impl Into<proto_updates::GetUserStateRequest>,
    ) -> Self {
        proto_nats::NatsGetUserStateRequest {
            headers: headers.into(),
            data: Some(data.into()),
        }
    }
}

// ***********************************  Response Getters ***********************************
// Create Response Data setter
impl DataGetter<proto_updates::GetUserStateResponse> for proto_nats::NatsGetUserStateResponse {
    fn to_data(self) -> Option<proto_updates::GetUserStateResponse> {
        match self.msg {
            Some(proto_nats::nats_get_user_state_response::Msg::Data(data)) => Some(data),
            _ => None,
        }
    }
}

// Create Response Error setter
impl ErrorGetter for proto_nats::NatsGetUserStateResponse {
    fn error(&self) -> Option<&proto_nats::ErrorReply> {
        match &self.msg {
            Some(proto_nats::nats_get_user_state_response::Msg::Error(e)) => Some(e),
            _ => None,
        }
    }
}

// ***********************************  Response Setters ***********************************
// Create Response Data setter
impl DataSetter<proto_updates::GetUserStateResponse, proto_nats::NatsGetUserStateResponse>
    for proto_nats::NatsGetUserStateResponse
{
    fn set_data(data: impl Into<proto_updates::GetUserStateResponse>) -> Self {
        let data = data.into();
        proto_nats::NatsGetUserStateResponse {
            msg: Some(proto_nats::nats_get_user_state_response::Msg::Data(data)),
        }
    }
}

// Create Response Error setter
impl ErrorSetter<proto_nats::ErrorReply, proto_nats::NatsGetUserStateResponse>
    for proto_nats::NatsGetUserStateResponse
{
    fn set_error(error: impl Into<proto_nats::ErrorReply>) -> Self {
        let error = error.into();
        proto_nats::NatsGetUserStateResponse {
            msg: Some(proto_nats::nats_get_user_state_response::Msg::Error(error)),
        }
    }
}
