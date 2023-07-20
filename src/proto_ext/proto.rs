// This module contains traits for extending NATS messages in order to
// provide generic traits to cut down the amount of boilerplate code
// required.

use crate::proto::chat::MetadataMap;

/// Trait for generically retrieving the data component of a NATS Request message
pub trait DataGetter<T>
where
    T: prost::Message,
{
    fn data(&self) -> &Option<T>;
}

/// Trait for generically retrieving the header component of a NATS Request message
pub trait HeaderGetter {
    fn headers(&self) -> &Vec<MetadataMap>;
}

/// Trait for generically setting the data component of a NATS Response message
pub trait DataSetter<T, N>
where
    T: prost::Message,
    N: prost::Message,
{
    fn set_data(data: impl Into<T>) -> N;
}

/// Trait for generically setting the error component of a NATS Response message
pub trait ErrorSetter<T, N>
where
    T: prost::Message,
    N: prost::Message,
{
    fn set_error(data: impl Into<T>) -> N;
}
