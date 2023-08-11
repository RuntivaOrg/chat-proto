use crate::updates_stream as proto;

use crate::proto_ext::updates_stream::{DataGetter, HeaderGetter, NatsRequestSetter};

// ***********************************  Request Getters ***********************************
// EditAbout Request Data message
impl DataGetter<proto::ConnectionPeer> for proto::NatsUnregisterFromPeer {
    fn to_data(self) -> Option<proto::ConnectionPeer> {
        self.data
    }
}

// EditAbout Request Headers
impl HeaderGetter for proto::NatsUnregisterFromPeer {
    fn headers(&self) -> &Vec<proto::MetadataMap> {
        &self.headers
    }
}

// ********************************** NATS Request Setter **********************************
impl NatsRequestSetter<proto::ConnectionPeer, proto::NatsUnregisterFromPeer>
    for proto::NatsUnregisterFromPeer
{
    fn from_headers_and_message(
        headers: impl Into<Vec<proto::MetadataMap>>,
        data: impl Into<proto::ConnectionPeer>,
    ) -> Self {
        proto::NatsUnregisterFromPeer {
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
