use crate::chat as proto;

use crate::proto_ext::proto::{DataSetter, ErrorSetter};

// Shared/common response for empty NATS responses
//
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
