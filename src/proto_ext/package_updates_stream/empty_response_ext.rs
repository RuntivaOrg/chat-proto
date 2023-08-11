use crate::updates_stream as proto;

use super::{DataGetter, DataSetter, ErrorGetter, ErrorSetter};

// Shared/common response for empty NATS responses
//
// ***********************************  Response Getters ***********************************
// Create Response Data setter
impl DataGetter<()> for proto::NatsEmptyResponse {
    fn to_data(self) -> Option<()> {
        match self.msg {
            Some(proto::nats_empty_response::Msg::Success(_)) => Some(()),
            _ => None,
        }
    }
}

// Create Response Error setter
impl ErrorGetter for proto::NatsEmptyResponse {
    fn error(&self) -> Option<&proto::ErrorReply> {
        match &self.msg {
            Some(proto::nats_empty_response::Msg::Error(e)) => Some(e),
            _ => None,
        }
    }
}

// ***********************************  Response Setters ***********************************
// Empty Response Data setter
impl DataSetter<(), proto::NatsEmptyResponse> for proto::NatsEmptyResponse {
    fn set_data(_data: impl Into<()>) -> proto::NatsEmptyResponse {
        //let data = data.into();
        proto::NatsEmptyResponse {
            msg: Some(proto::nats_empty_response::Msg::Success(true)),
        }
    }
}

// Empty Response Error setter
impl ErrorSetter<proto::ErrorReply, proto::NatsEmptyResponse> for proto::NatsEmptyResponse {
    fn set_error(error: impl Into<proto::ErrorReply>) -> proto::NatsEmptyResponse {
        let error = error.into();
        proto::NatsEmptyResponse {
            msg: Some(proto::nats_empty_response::Msg::Error(error)),
        }
    }
}
