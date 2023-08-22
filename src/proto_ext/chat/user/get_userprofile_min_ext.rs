use crate::chat as proto;
use crate::runtiva::nats::v1 as proto_nats;

use crate::proto_ext::{
    DataGetter, DataSetter, ErrorGetter, ErrorSetter, HeaderGetter, NatsRequestSetter,
};

// ***********************************  Request Getters ***********************************
// Create Request Data message
impl DataGetter<proto::UserProfilesMinRequest> for proto::NatsGetUserProfilesMinRequest {
    fn to_data(self) -> Option<proto::UserProfilesMinRequest> {
        self.data
    }
}

// Create Request Headers
impl HeaderGetter for proto::NatsGetUserProfilesMinRequest {
    fn headers(&self) -> &Vec<proto_nats::MetadataMap> {
        &self.headers
    }
}

// ********************************** NATS Request Setter **********************************
impl NatsRequestSetter<proto::UserProfilesMinRequest, proto::NatsGetUserProfilesMinRequest>
    for proto::NatsGetUserProfilesMinRequest
{
    fn from_headers_and_message(
        headers: impl Into<Vec<proto_nats::MetadataMap>>,
        data: impl Into<proto::UserProfilesMinRequest>,
    ) -> Self {
        proto::NatsGetUserProfilesMinRequest {
            headers: headers.into(),
            data: Some(data.into()),
        }
    }
}

// ***********************************  Response Getters ***********************************
// Create Response Data setter
impl DataGetter<proto::UserProfileMinArray> for proto::NatsGetUserProfilesMinResponse {
    fn to_data(self) -> Option<proto::UserProfileMinArray> {
        match self.msg {
            Some(proto::nats_get_user_profiles_min_response::Msg::Data(data)) => Some(data),
            _ => None,
        }
    }
}

// Create Response Error setter
impl ErrorGetter for proto::NatsGetUserProfilesMinResponse {
    fn error(&self) -> Option<&proto::ErrorReply> {
        match &self.msg {
            Some(proto::nats_get_user_profiles_min_response::Msg::Error(e)) => Some(e),
            _ => None,
        }
    }
}

// ***********************************  Response Setters ***********************************
// Create Response Data setter
impl DataSetter<proto::UserProfileMinArray, proto::NatsGetUserProfilesMinResponse>
    for proto::NatsGetUserProfilesMinResponse
{
    fn set_data(data: impl Into<proto::UserProfileMinArray>) -> Self {
        let data = data.into();
        proto::NatsGetUserProfilesMinResponse {
            msg: Some(proto::nats_get_user_profiles_min_response::Msg::Data(data)),
        }
    }
}

// Create Response Error setter
impl ErrorSetter<proto::ErrorReply, proto::NatsGetUserProfilesMinResponse>
    for proto::NatsGetUserProfilesMinResponse
{
    fn set_error(error: impl Into<proto::ErrorReply>) -> Self {
        let error = error.into();
        proto::NatsGetUserProfilesMinResponse {
            msg: Some(proto::nats_get_user_profiles_min_response::Msg::Error(
                error,
            )),
        }
    }
}
