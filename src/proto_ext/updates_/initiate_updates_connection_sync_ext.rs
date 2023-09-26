use crate::runtiva::{nats::v1 as proto_nats, updates::v1 as proto_updates};

use crate::proto_ext::{DataGetter, HeaderGetter, NatsRequestSetter};

// ***********************************  Request Getters ***********************************
// Create Request Data message
impl DataGetter<proto_updates::InitiateUpdatesConnectionSyncRequest>
    for proto_nats::NatsInitiateUpdatesConnectionSyncRequest
{
    fn to_data(self) -> Option<proto_updates::InitiateUpdatesConnectionSyncRequest> {
        self.data
    }
}

// Create Request Headers
impl HeaderGetter for proto_nats::NatsInitiateUpdatesConnectionSyncRequest {
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
        proto_updates::InitiateUpdatesConnectionSyncRequest,
        proto_nats::NatsInitiateUpdatesConnectionSyncRequest,
    > for proto_nats::NatsInitiateUpdatesConnectionSyncRequest
{
    fn from_headers_and_message(
        headers: impl Into<Vec<proto_nats::MetadataMap>>,
        data: impl Into<proto_updates::InitiateUpdatesConnectionSyncRequest>,
    ) -> Self {
        proto_nats::NatsInitiateUpdatesConnectionSyncRequest {
            headers: headers.into(),
            data: Some(data.into()),
        }
    }
}
