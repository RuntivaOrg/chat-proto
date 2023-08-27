use crate::proto_ext::{DataGetter, HeaderGetter, NatsRequestSetter};
use crate::runtiva::{nats::v1 as proto_nats, updates::v1 as proto_updates};

// ***********************************  Request Getters ***********************************
// EditAbout Request Data message
impl DataGetter<proto_updates::ConnectionPeer> for proto_updates::NatsRegisterOnPeerRequest {
    fn to_data(self) -> Option<proto_updates::ConnectionPeer> {
        self.data
    }
}

// EditAbout Request Headers
impl HeaderGetter for proto_updates::NatsRegisterOnPeerRequest {
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
impl NatsRequestSetter<proto_updates::ConnectionPeer, proto_updates::NatsRegisterOnPeerRequest>
    for proto_updates::NatsRegisterOnPeerRequest
{
    fn from_headers_and_message(
        headers: impl Into<Vec<proto_nats::MetadataMap>>,
        data: impl Into<proto_updates::ConnectionPeer>,
    ) -> Self {
        proto_updates::NatsRegisterOnPeerRequest {
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
