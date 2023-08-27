use crate::proto_ext::{DataGetter, HeaderGetter, NatsRequestSetter};
use crate::runtiva::{nats::v1 as proto_nats, updates::v1 as proto_updates};

// ***********************************  Request Getters ***********************************
// EditAbout Request Data message
impl DataGetter<proto_updates::UpdatesConnectionKey>
    for proto_updates::NatsRegisterOnPeerListRequest
{
    fn to_data(self) -> Option<proto_updates::UpdatesConnectionKey> {
        self.data
    }
}

// EditAbout Request Headers
impl HeaderGetter for proto_updates::NatsRegisterOnPeerListRequest {
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
        proto_updates::UpdatesConnectionKey,
        proto_updates::NatsRegisterOnPeerListRequest,
    > for proto_updates::NatsRegisterOnPeerListRequest
{
    fn from_headers_and_message(
        headers: impl Into<Vec<proto_nats::MetadataMap>>,
        data: impl Into<proto_updates::UpdatesConnectionKey>,
    ) -> Self {
        proto_updates::NatsRegisterOnPeerListRequest {
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