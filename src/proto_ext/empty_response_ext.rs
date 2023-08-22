use crate::runtiva::nats::v1 as proto_nats;

use crate::proto_ext::{DataGetter, DataSetter, ErrorGetter, ErrorSetter};

// Shared/common response for empty NATS responses
//
// ***********************************  Response Getters ***********************************
// Create Response Data setter
impl DataGetter<()> for proto_nats::NatsEmptyResponse {
    fn to_data(self) -> Option<()> {
        match self.msg {
            Some(proto_nats::nats_empty_response::Msg::Success(_)) => Some(()),
            _ => None,
        }
    }
}

// Create Response Error setter
impl ErrorGetter for proto_nats::NatsEmptyResponse {
    fn error(&self) -> Option<&proto_nats::ErrorReply> {
        match &self.msg {
            Some(proto_nats::nats_empty_response::Msg::Error(e)) => Some(e),
            _ => None,
        }
    }
}

// ***********************************  Response Setters ***********************************
// Empty Response Data setter
impl DataSetter<(), proto_nats::NatsEmptyResponse> for proto_nats::NatsEmptyResponse {
    fn set_data(_data: impl Into<()>) -> proto_nats::NatsEmptyResponse {
        //let data = data.into();
        proto_nats::NatsEmptyResponse {
            msg: Some(proto_nats::nats_empty_response::Msg::Success(true)),
        }
    }
}

// Empty Response Error setter
impl ErrorSetter<proto_nats::ErrorReply, proto_nats::NatsEmptyResponse>
    for proto_nats::NatsEmptyResponse
{
    fn set_error(error: impl Into<proto_nats::ErrorReply>) -> proto_nats::NatsEmptyResponse {
        let error = error.into();
        proto_nats::NatsEmptyResponse {
            msg: Some(proto_nats::nats_empty_response::Msg::Error(error)),
        }
    }
}
