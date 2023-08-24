use crate::runtiva::{nats::v1 as proto_nats, persist::v1 as proto_persist};

use crate::proto_ext::{DataGetter, HeaderGetter, NatsRequestSetter};

// ***********************************  Event Getters ***********************************
// ActionJoinRequest Request Data message
impl DataGetter<proto_persist::ChatGroupParticipantAdded>
    for proto_persist::NatsChatGroupParticipantAddedEvent
{
    fn to_data(self) -> Option<proto_persist::ChatGroupParticipantAdded> {
        self.data
    }
}

// ActionJoinRequest Request Headers
impl HeaderGetter for proto_persist::NatsChatGroupParticipantAddedEvent {
    fn headers(&self) -> &Vec<proto_nats::MetadataMap> {
        &self.headers
    }
}

// ********************************** NATS Event Setter **********************************
impl
    NatsRequestSetter<
        proto_persist::ChatGroupParticipantAdded,
        proto_persist::NatsChatGroupParticipantAddedEvent,
    > for proto_persist::NatsChatGroupParticipantAddedEvent
{
    fn from_headers_and_message(
        headers: impl Into<Vec<proto_nats::MetadataMap>>,
        data: impl Into<proto_persist::ChatGroupParticipantAdded>,
    ) -> Self {
        proto_persist::NatsChatGroupParticipantAddedEvent {
            headers: headers.into(),
            data: Some(data.into()),
        }
    }
}
