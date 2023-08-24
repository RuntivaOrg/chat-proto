use crate::runtiva::{nats::v1 as proto_nats, persist::v1 as proto_persist};

use crate::proto_ext::{DataGetter, HeaderGetter, NatsRequestSetter};

// ***********************************  Event Getters ***********************************
// ActionJoinRequest Request Data message
impl DataGetter<proto_persist::ChatParticipantRemovedMsg>
    for proto_persist::NatsChatGroupParticipantRemovedEvent
{
    fn to_data(self) -> Option<proto_persist::ChatParticipantRemovedMsg> {
        self.data
    }
}

// ActionJoinRequest Request Headers
impl HeaderGetter for proto_persist::NatsChatGroupParticipantRemovedEvent {
    fn headers(&self) -> &Vec<proto_nats::MetadataMap> {
        &self.headers
    }
}

// ********************************** NATS Event Setter **********************************
impl
    NatsRequestSetter<
        proto_persist::ChatParticipantRemovedMsg,
        proto_persist::NatsChatGroupParticipantRemovedEvent,
    > for proto_persist::NatsChatGroupParticipantRemovedEvent
{
    fn from_headers_and_message(
        headers: impl Into<Vec<proto_nats::MetadataMap>>,
        data: impl Into<proto_persist::ChatParticipantRemovedMsg>,
    ) -> Self {
        proto_persist::NatsChatGroupParticipantRemovedEvent {
            headers: headers.into(),
            data: Some(data.into()),
        }
    }
}
