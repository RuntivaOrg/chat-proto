use crate::runtiva::{nats::v1 as proto_nats, updates::v1 as proto_updates};

use crate::proto_ext::{DataGetter, HeaderGetter, NatsRequestSetter};

// ***********************************  Request Getters ***********************************
// RemoveUser Request Data message
impl DataGetter<proto_updates::Envelope> for proto_updates::DistroEnvelope {
    fn to_data(self) -> Option<proto_updates::Envelope> {
        self.data
    }
}

// RemoveUser Request Headers
impl HeaderGetter for proto_updates::DistroEnvelope {
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
impl NatsRequestSetter<proto_updates::Envelope, proto_updates::DistroEnvelope>
    for proto_updates::DistroEnvelope
{
    fn from_headers_and_message(
        headers: impl Into<Vec<proto_nats::MetadataMap>>,
        data: impl Into<proto_updates::Envelope>,
    ) -> Self {
        proto_updates::DistroEnvelope {
            headers: headers.into(),
            data: Some(data.into()),
        }
    }
}
