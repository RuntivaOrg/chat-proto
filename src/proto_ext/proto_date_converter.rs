use chrono::{DateTime, Utc};

use crate::ChatProtoError;

/// utility struct that converts between proto dates and chrono dates
pub struct ProtoDateConverter(pub DateTime<Utc>);

impl From<ProtoDateConverter> for DateTime<Utc> {
    fn from(value: ProtoDateConverter) -> Self {
        value.0
    }
}

impl From<ProtoDateConverter> for prost_types::Timestamp {
    fn from(value: ProtoDateConverter) -> Self {
        prost_types::Timestamp {
            seconds: value.0.timestamp(),
            nanos: value.0.timestamp_subsec_nanos() as i32,
        }
    }
}

impl TryInto<ProtoDateConverter> for prost_types::Timestamp {
    type Error = ChatProtoError;

    fn try_into(self) -> Result<ProtoDateConverter, Self::Error> {
        Ok(ProtoDateConverter(DateTime::from_naive_utc_and_offset(
            chrono::NaiveDateTime::from_timestamp_opt(self.seconds, self.nanos as u32)
                .ok_or(ChatProtoError::DateOutOfRange(self.seconds, self.nanos))?,
            Utc,
        )))
    }
}

impl ProtoDateConverter {
    /// Converts a proto timestamp into a chrono timestamp. If there are any conversion errors, returns None
    pub fn into_timestamp_or_none(
        date: Option<prost_types::Timestamp>,
    ) -> Option<ProtoDateConverter> {
        match date {
            Some(dt) => <prost_types::Timestamp as TryInto<ProtoDateConverter>>::try_into(dt).ok(),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timestamp_conversion() {
        let now = Utc::now();
        let proto_timestamp: prost_types::Timestamp = ProtoDateConverter(now).into();
        let chrono_timestamp: DateTime<Utc> =
            <prost_types::Timestamp as TryInto<ProtoDateConverter>>::try_into(proto_timestamp)
                .unwrap()
                .into();
        assert_eq!(now, chrono_timestamp);
    }

    #[test]
    fn test_timestamp_conversion_none() {
        let proto_timestamp: Option<prost_types::Timestamp> = None;
        let chrono_timestamp: Option<DateTime<Utc>> =
            ProtoDateConverter::into_timestamp_or_none(proto_timestamp).map(|x| x.into());
        assert_eq!(None, chrono_timestamp);
    }

    #[test]
    fn test_timestamp_conversion_some() {
        let now = Utc::now();
        let proto_timestamp: Option<prost_types::Timestamp> = Some(ProtoDateConverter(now).into());
        let chrono_timestamp: Option<DateTime<Utc>> =
            ProtoDateConverter::into_timestamp_or_none(proto_timestamp).map(|x| x.into());
        assert_eq!(Some(now), chrono_timestamp);
    }

    #[test]
    fn test_timestamp_conversion_invalid() {
        let proto_timestamp: prost_types::Timestamp = prost_types::Timestamp {
            seconds: i64::MAX,
            nanos: 0,
        };
        let chrono_timestamp: Result<DateTime<Utc>, ChatProtoError> = proto_timestamp
            .try_into()
            .map(|x: ProtoDateConverter| x.into());
        assert!(chrono_timestamp.is_err());
    }
}
