// This module contains traits for extending NATS messages in order to
// provide generic traits to cut down the amount of boilerplate code
// required.

use crate::runtiva::nats::v1 as proto_nats;

// Trait for generiically setting the headers and data component of a NATS **Request** message
/// *** Used in chat-server::api::api_handler.rs
pub trait NatsRequestSetter<M, N>
where
    M: prost::Message,
    N: prost::Message,
{
    fn from_headers_and_message(
        headers: impl Into<Vec<proto_nats::MetadataMap>>,
        data: impl Into<M>,
    ) -> N;
}

/// Trait for generically retrieving the data component of a NATS **Request** and non-NATS **Request** messages
/// *** Used in chat-server::api::api_handler.rs
pub trait DataGetter<T>
where
    T: prost::Message,
{
    fn to_data(self) -> Option<T>;
}

/// Trait for generically getting the error component of a NATS **Response** message
/// *** Used in chat-server::api::api_handler.rs
pub trait ErrorGetter {
    fn error(&self) -> Option<&proto_nats::ErrorReply>;
}

/// Trait for generically retrieving the header component of a NATS **Request** message
pub trait HeaderGetter {
    fn take_headers(&mut self) -> Vec<proto_nats::MetadataMap>;
}

/// Trait for generically setting the data component of a NATS **Response** message
/// *** Used in chat-persistance-server::domains::[domain]_api.rs
pub trait DataSetter<T, N>
where
    T: prost::Message,
    N: prost::Message,
{
    fn set_data(data: impl Into<T>) -> N;
}

/// Trait for generically setting the error component of a NATS **Response** message
/// *** Used in chat-persistance-server::domains::[domain]_api.rs
pub trait ErrorSetter<M, N>
where
    M: prost::Message,
    N: prost::Message,
{
    fn set_error(data: impl Into<M>) -> N;
}
