use crate::runtiva::{nats::v1 as proto_nats, updates::v1 as proto_updates};

use crate::proto_ext::{DataGetter, HeaderGetter, NatsRequestSetter};

// ***********************************  Request Getters ***********************************
// ToggleDisable Request Data message
impl DataGetter<proto_updates::ConnEnvelope> for proto_updates::ChatServerEnvelope {
    fn to_data(self) -> Option<proto_updates::ConnEnvelope> {
        self.data
    }
}

// ToggleDisable Request Headers
impl HeaderGetter for proto_updates::ChatServerEnvelope {
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
impl NatsRequestSetter<proto_updates::ConnEnvelope, proto_updates::ChatServerEnvelope>
    for proto_updates::ChatServerEnvelope
{
    fn from_headers_and_message(
        headers: impl Into<Vec<proto_nats::MetadataMap>>,
        data: impl Into<proto_updates::ConnEnvelope>,
    ) -> Self {
        proto_updates::ChatServerEnvelope {
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
