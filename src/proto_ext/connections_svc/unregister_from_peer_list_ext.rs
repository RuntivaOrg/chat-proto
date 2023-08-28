use crate::proto_ext::{DataGetter, HeaderGetter, NatsRequestSetter};
use crate::runtiva::{connections::v1 as proto_connections, nats::v1 as proto_nats};

// ***********************************  Request Getters ***********************************
// EditAbout Request Data message
impl DataGetter<proto_connections::UpdatesConnectionKey>
    for proto_connections::NatsUnregisterFromPeerListRequest
{
    fn to_data(self) -> Option<proto_connections::UpdatesConnectionKey> {
        self.data
    }
}

// EditAbout Request Headers
impl HeaderGetter for proto_connections::NatsUnregisterFromPeerListRequest {
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
impl
    NatsRequestSetter<
        proto_connections::UpdatesConnectionKey,
        proto_connections::NatsUnregisterFromPeerListRequest,
    > for proto_connections::NatsUnregisterFromPeerListRequest
{
    fn from_headers_and_message(
        headers: impl Into<Vec<proto_nats::MetadataMap>>,
        data: impl Into<proto_connections::UpdatesConnectionKey>,
    ) -> Self {
        proto_connections::NatsUnregisterFromPeerListRequest {
            headers: headers.into(),
            data: Some(data.into()),
        }
    }
}

// ***********************************  Response Getters ***********************************
//
// *** Uses the generic implementations from chat-proto\src\proto_ext\empty_response_ext.rs

// ***********************************  Response Setters ***********************************
//
// *** Uses the generic implementations from chat-proto\src\proto_ext\error_response_ext.rs
