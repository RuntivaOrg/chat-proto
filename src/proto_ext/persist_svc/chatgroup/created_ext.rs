use crate::runtiva::{nats::v1 as proto_nats, persist::v1 as proto_persist};

use crate::proto_ext::{DataGetter, HeaderGetter, NatsRequestSetter};

// ***********************************  Event Getters ***********************************
// Request Data message
impl DataGetter<proto_persist::ChatGroupCreated> for proto_persist::NatsChatGroupCreatedEvent {
    fn to_data(self) -> Option<proto_persist::ChatGroupCreated> {
        self.data
    }
}

// Request Headers
impl HeaderGetter for proto_persist::NatsChatGroupCreatedEvent {
    fn headers(&self) -> &[proto_nats::MetadataMap] {
        &self.headers
    }

    fn take_headers(&mut self) -> Vec<proto_nats::MetadataMap> {
        let mut swapped = vec![];
        std::mem::swap(&mut self.headers, &mut swapped);
        swapped
    }
}

// ********************************** NATS Event Setter **********************************
impl NatsRequestSetter<proto_persist::ChatGroupCreated, proto_persist::NatsChatGroupCreatedEvent>
    for proto_persist::NatsChatGroupCreatedEvent
{
    fn from_headers_and_message(
        headers: impl Into<Vec<proto_nats::MetadataMap>>,
        data: impl Into<proto_persist::ChatGroupCreated>,
    ) -> Self {
        proto_persist::NatsChatGroupCreatedEvent {
            headers: headers.into(),
            data: Some(data.into()),
        }
    }
}
