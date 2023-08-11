use crate::updates_stream as proto;

use crate::proto_ext::package_updates_stream::{DataGetter, HeaderGetter, NatsRequestSetter};

// ***********************************  Request Getters ***********************************
// EditAbout Request Data message
impl DataGetter<proto::UpdatesConnection> for proto::NatsRegisterOnPeerList {
    fn to_data(self) -> Option<proto::UpdatesConnection> {
        self.data
    }
}

// EditAbout Request Headers
impl HeaderGetter for proto::NatsRegisterOnPeerList {
    fn headers(&self) -> &Vec<proto::MetadataMap> {
        &self.headers
    }
}

// ********************************** NATS Request Setter **********************************
impl NatsRequestSetter<proto::UpdatesConnection, proto::NatsRegisterOnPeerList>
    for proto::NatsRegisterOnPeerList
{
    fn from_headers_and_message(
        headers: impl Into<Vec<proto::MetadataMap>>,
        data: impl Into<proto::UpdatesConnection>,
    ) -> Self {
        proto::NatsRegisterOnPeerList {
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
