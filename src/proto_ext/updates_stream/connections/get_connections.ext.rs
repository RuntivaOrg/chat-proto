use crate::runtiva::nats::v1 as proto_nats;
use crate::updates_stream as proto;

use crate::proto_ext::{DataGetter, HeaderGetter, NatsRequestSetter};

// ***********************************  Request Getters ***********************************
// EditAbout Request Data message
impl DataGetter<proto::GetConnectionsRequest> for proto::NatsGetConnectionsRequest {
    fn to_data(self) -> Option<proto::GetConnectionsRequest> {
        self.data
    }
}

// EditAbout Request Headers
impl HeaderGetter for proto::NatsGetConnectionsRequest {
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
impl NatsRequestSetter<proto::GetConnectionsRequest, proto::NatsGetConnectionsRequest>
    for proto::NatsGetConnectionsRequest
{
    fn from_headers_and_message(
        headers: impl Into<Vec<proto_nats::MetadataMap>>,
        data: impl Into<proto::GetConnectionsRequest>,
    ) -> Self {
        proto::NatsGetConnectionsRequest {
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
