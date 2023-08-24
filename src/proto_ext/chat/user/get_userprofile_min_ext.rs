use crate::runtiva::{nats::v1 as proto_nats, user::v1 as proto_user};

use crate::proto_ext::{
    DataGetter, DataSetter, ErrorGetter, ErrorSetter, HeaderGetter, NatsRequestSetter,
};

// ***********************************  Request Getters ***********************************
// Create Request Data message
impl DataGetter<proto_user::UserProfilesMinRequest> for proto_nats::NatsGetUserProfilesMinRequest {
    fn to_data(self) -> Option<proto_user::UserProfilesMinRequest> {
        self.data
    }
}

// Create Request Headers
impl HeaderGetter for proto_nats::NatsGetUserProfilesMinRequest {
    fn take_headers(&mut self) -> Vec<proto_nats::MetadataMap> {
        let mut swapped = vec![];
        std::mem::swap(&mut self.headers, &mut swapped);
        swapped
    }
}

// ********************************** NATS Request Setter **********************************
impl
    NatsRequestSetter<proto_user::UserProfilesMinRequest, proto_nats::NatsGetUserProfilesMinRequest>
    for proto_nats::NatsGetUserProfilesMinRequest
{
    fn from_headers_and_message(
        headers: impl Into<Vec<proto_nats::MetadataMap>>,
        data: impl Into<proto_user::UserProfilesMinRequest>,
    ) -> Self {
        proto_nats::NatsGetUserProfilesMinRequest {
            headers: headers.into(),
            data: Some(data.into()),
        }
    }
}

// ***********************************  Response Getters ***********************************
// Create Response Data setter
impl DataGetter<proto_user::UserProfileMinArray> for proto_nats::NatsGetUserProfilesMinResponse {
    fn to_data(self) -> Option<proto_user::UserProfileMinArray> {
        match self.msg {
            Some(proto_nats::nats_get_user_profiles_min_response::Msg::Data(data)) => Some(data),
            _ => None,
        }
    }
}

// Create Response Error setter
impl ErrorGetter for proto_nats::NatsGetUserProfilesMinResponse {
    fn error(&self) -> Option<&proto_nats::ErrorReply> {
        match &self.msg {
            Some(proto_nats::nats_get_user_profiles_min_response::Msg::Error(e)) => Some(e),
            _ => None,
        }
    }
}

// ***********************************  Response Setters ***********************************
// Create Response Data setter
impl DataSetter<proto_user::UserProfileMinArray, proto_nats::NatsGetUserProfilesMinResponse>
    for proto_nats::NatsGetUserProfilesMinResponse
{
    fn set_data(data: impl Into<proto_user::UserProfileMinArray>) -> Self {
        let data = data.into();
        proto_nats::NatsGetUserProfilesMinResponse {
            msg: Some(proto_nats::nats_get_user_profiles_min_response::Msg::Data(
                data,
            )),
        }
    }
}

// Create Response Error setter
impl ErrorSetter<proto_nats::ErrorReply, proto_nats::NatsGetUserProfilesMinResponse>
    for proto_nats::NatsGetUserProfilesMinResponse
{
    fn set_error(error: impl Into<proto_nats::ErrorReply>) -> Self {
        let error = error.into();
        proto_nats::NatsGetUserProfilesMinResponse {
            msg: Some(proto_nats::nats_get_user_profiles_min_response::Msg::Error(
                error,
            )),
        }
    }
}
