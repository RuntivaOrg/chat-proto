use crate::runtiva::nats::v1 as proto_nats;
use crate::updates_stream as proto;

use crate::proto_ext::{DataGetter, HeaderGetter, NatsRequestSetter};

// ***********************************  Request Getters ***********************************
// EditAbout Request Data message
impl DataGetter<proto::ConnectionPeer> for proto::NatsRegisterOnPeerRequest {
    fn to_data(self) -> Option<proto::ConnectionPeer> {
        self.data
    }
}

// EditAbout Request Headers
impl HeaderGetter for proto::NatsRegisterOnPeerRequest {
    fn take_headers(&mut self) -> Vec<proto_nats::MetadataMap> {
        let mut swapped = vec![];
        std::mem::swap(&mut self.headers, &mut swapped);
        swapped
    }
}

// ********************************** NATS Request Setter **********************************
impl NatsRequestSetter<proto::ConnectionPeer, proto::NatsRegisterOnPeerRequest>
    for proto::NatsRegisterOnPeerRequest
{
    fn from_headers_and_message(
        headers: impl Into<Vec<proto_nats::MetadataMap>>,
        data: impl Into<proto::ConnectionPeer>,
    ) -> Self {
        proto::NatsRegisterOnPeerRequest {
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
