use crate::proto_ext::{DataGetter, HeaderGetter, NatsRequestSetter};
use crate::runtiva::{connections::v1 as proto_connections, nats::v1 as proto_nats};

// ***********************************  Request Getters ***********************************
// EditAbout Request Data message
impl DataGetter<proto_connections::ConnectionPeer>
    for proto_connections::NatsUnregisterFromPeerRequest
{
    fn to_data(self) -> Option<proto_connections::ConnectionPeer> {
        self.data
    }
}

// EditAbout Request Headers
impl HeaderGetter for proto_connections::NatsUnregisterFromPeerRequest {
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
        proto_connections::ConnectionPeer,
        proto_connections::NatsUnregisterFromPeerRequest,
    > for proto_connections::NatsUnregisterFromPeerRequest
{
    fn from_headers_and_message(
        headers: impl Into<Vec<proto_nats::MetadataMap>>,
        data: impl Into<proto_connections::ConnectionPeer>,
    ) -> Self {
        proto_connections::NatsUnregisterFromPeerRequest {
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
