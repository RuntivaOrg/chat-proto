use crate::runtiva::{chat::v1 as proto_chat, distro::v1 as proto_distro, nats::v1 as proto_nats};

use crate::proto_ext::{DataGetter, HeaderGetter, NatsRequestSetter};

// *********************************** DISTRO Request Getters ***********************************
// Disable Update Request Data message
impl DataGetter<proto_chat::ChatGroupToggleDisableRequest>
    for proto_distro::ChatGroupDisableUpdate
{
    fn to_data(self) -> Option<proto_chat::ChatGroupToggleDisableRequest> {
        self.data
    }
}

// Disable Update Request Headers
impl HeaderGetter for proto_distro::ChatGroupDisableUpdate {
    fn headers(&self) -> &[proto_nats::MetadataMap] {
        &self.headers
    }

    fn take_headers(&mut self) -> Vec<proto_nats::MetadataMap> {
        let mut swapped = vec![];
        std::mem::swap(&mut self.headers, &mut swapped);
        swapped
    }
}

// ********************************** DISTRO Request Setter **********************************
impl
    NatsRequestSetter<
        proto_chat::ChatGroupToggleDisableRequest,
        proto_distro::ChatGroupDisableUpdate,
    > for proto_distro::ChatGroupDisableUpdate
{
    fn from_headers_and_message(
        headers: impl Into<Vec<proto_nats::MetadataMap>>,
        data: impl Into<proto_chat::ChatGroupToggleDisableRequest>,
    ) -> Self {
        proto_distro::ChatGroupDisableUpdate {
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
// *** Uses the generic implementations from chat-proto\src\proto_ext\empty_response_ext.rs
