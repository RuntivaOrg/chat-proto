// This module contains traits for extending NATS messages in order to
// provide generic traits to cut down the amount of boilerplate code
// required.

use crate::proto::chat as proto;

/// Trait for generically retrieving the data component of a NATS **Request** and non-NATS **Request** messages
pub trait DataGetter<T>
where
    T: prost::Message,
{
    fn to_data(self) -> Option<T>;
}

/// Trait for generically retrieving the header component of a NATS **Request** message
pub trait HeaderGetter {
    fn headers(&self) -> &Vec<proto::MetadataMap>;
}

// Trait for generiically setting the headers and data component of a NATS **Request** message
pub trait NatsRequestSetter<M, N>
where
    M: prost::Message,
    N: prost::Message,
{
    fn from_headers_and_message(
        headers: impl Into<Vec<proto::MetadataMap>>,
        data: impl Into<M>,
    ) -> N;
}

/// Trait for generically setting the data component of a NATS **Response** message
pub trait DataSetter<T, N>
where
    T: prost::Message,
    N: prost::Message,
{
    fn set_data(data: impl Into<T>) -> N;
}

/// Trait for generically setting the error component of a NATS **Response** message
pub trait ErrorSetter<M, N>
where
    M: prost::Message,
    N: prost::Message,
{
    fn set_error(data: impl Into<M>) -> N;
}

/// Trait for generically getting the error component of a NATS **Response** message
pub trait ErrorGetter {
    fn error(&self) -> Option<&proto::ErrorReply>;
}
