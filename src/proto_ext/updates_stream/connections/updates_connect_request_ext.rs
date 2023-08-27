use crate::runtiva::nats::v1 as proto_nats;
use crate::updates_stream as proto;

use crate::proto_ext::{DataGetter, HeaderGetter, NatsRequestSetter};

// ***********************************  Request Getters ***********************************
// EditAbout Request Data message
impl DataGetter<proto::UpdatesConnection> for proto::NatsUpdatesConnectRequest {
    fn to_data(self) -> Option<proto::UpdatesConnection> {
        self.data
    }
}

// EditAbout Request Headers
impl HeaderGetter for proto::NatsUpdatesConnectRequest {
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
impl NatsRequestSetter<proto::UpdatesConnection, proto::NatsUpdatesConnectRequest>
    for proto::NatsUpdatesConnectRequest
{
    fn from_headers_and_message(
        headers: impl Into<Vec<proto_nats::MetadataMap>>,
        data: impl Into<proto::UpdatesConnection>,
    ) -> Self {
        proto::NatsUpdatesConnectRequest {
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
