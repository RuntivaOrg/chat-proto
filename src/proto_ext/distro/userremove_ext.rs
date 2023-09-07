use crate::runtiva::{chat::v1 as proto_chat, distro::v1 as proto_distro, nats::v1 as proto_nats};

use crate::proto_ext::{DataGetter, HeaderGetter, NatsRequestSetter};

// *********************************** DISTRO Request Getters ***********************************
// UserRemove Request Data message
impl DataGetter<proto_chat::ChatGroupRemoveUserRequest> for proto_distro::ChatGroupUserRemove {
    fn to_data(self) -> Option<proto_chat::ChatGroupRemoveUserRequest> {
        self.data
    }
}

// UserRemove Request Headers
impl HeaderGetter for proto_distro::ChatGroupUserRemove {
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
impl NatsRequestSetter<proto_chat::ChatGroupRemoveUserRequest, proto_distro::ChatGroupUserRemove>
    for proto_distro::ChatGroupUserRemove
{
    fn from_headers_and_message(
        headers: impl Into<Vec<proto_nats::MetadataMap>>,
        data: impl Into<proto_chat::ChatGroupRemoveUserRequest>,
    ) -> Self {
        proto_distro::ChatGroupUserRemove {
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
