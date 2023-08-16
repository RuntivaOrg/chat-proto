#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelAdminRights {
    #[prost(bool, tag = "1")]
    pub is_anonymous: bool,
    #[prost(bool, tag = "2")]
    pub change_info: bool,
    #[prost(bool, tag = "3")]
    pub post_messages: bool,
    #[prost(bool, tag = "4")]
    pub edit_messages: bool,
    #[prost(bool, tag = "5")]
    pub delete_messages: bool,
    #[prost(bool, tag = "6")]
    pub ban_users: bool,
    #[prost(bool, tag = "7")]
    pub invite_users: bool,
    #[prost(bool, tag = "8")]
    pub pin_messages: bool,
    #[prost(bool, tag = "9")]
    pub add_admins: bool,
    #[prost(bool, tag = "10")]
    pub view_statistics: bool,
    #[prost(bool, tag = "11")]
    pub view_participants: bool,
    #[prost(bool, tag = "12")]
    pub disable_channel: bool,
    #[prost(bool, tag = "13")]
    pub delete_channel: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelParticipantRights {
    #[prost(bool, tag = "1")]
    pub is_banned: bool,
    #[prost(bool, tag = "2")]
    pub view_messages: bool,
    #[prost(bool, tag = "3")]
    pub send_messages: bool,
    #[prost(bool, tag = "4")]
    pub send_media: bool,
    #[prost(bool, tag = "5")]
    pub embed_links: bool,
    #[prost(bool, tag = "6")]
    pub invite_users: bool,
    #[prost(message, optional, tag = "7")]
    pub until_date: ::core::option::Option<::prost_types::Timestamp>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ChannelType {
    Unknown = 0,
    Broadcast = 1,
    Discussiongroup = 2,
    Forum = 3,
}
impl ChannelType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ChannelType::Unknown => "CHANNEL_TYPE_UNKNOWN",
            ChannelType::Broadcast => "CHANNEL_TYPE_BROADCAST",
            ChannelType::Discussiongroup => "CHANNEL_TYPE_DISCUSSIONGROUP",
            ChannelType::Forum => "CHANNEL_TYPE_FORUM",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CHANNEL_TYPE_UNKNOWN" => Some(Self::Unknown),
            "CHANNEL_TYPE_BROADCAST" => Some(Self::Broadcast),
            "CHANNEL_TYPE_DISCUSSIONGROUP" => Some(Self::Discussiongroup),
            "CHANNEL_TYPE_FORUM" => Some(Self::Forum),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum JoinResponse {
    Approved = 0,
    WaitingApproval = 1,
    NoPendingRequest = 2,
}
impl JoinResponse {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            JoinResponse::Approved => "JOIN_RESPONSE_APPROVED",
            JoinResponse::WaitingApproval => "JOIN_RESPONSE_WAITING_APPROVAL",
            JoinResponse::NoPendingRequest => "JOIN_RESPONSE_NO_PENDING_REQUEST",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "JOIN_RESPONSE_APPROVED" => Some(Self::Approved),
            "JOIN_RESPONSE_WAITING_APPROVAL" => Some(Self::WaitingApproval),
            "JOIN_RESPONSE_NO_PENDING_REQUEST" => Some(Self::NoPendingRequest),
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Channel {
    /// ID of the channel
    #[prost(int64, tag = "1")]
    pub id: i64,
    /// The channel title that is searchable by users
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    /// The description of the channel
    #[prost(string, tag = "3")]
    pub about: ::prost::alloc::string::String,
    /// Optional photo assigned to the channel
    #[prost(int64, optional, tag = "4")]
    pub photo_id: ::core::option::Option<i64>,
    /// channel owner. Initial creator until changed/reassigned to another user
    #[prost(int64, tag = "5")]
    pub owner_id: i64,
    /// Date channel was created
    #[prost(message, optional, tag = "6")]
    pub created_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Channel type - Broadcast or Discussion Group or Forum
    #[prost(enumeration = "ChannelType", tag = "7")]
    pub channel_type: i32,
    /// ** operational fields **
    /// Flag indicating whether or not this channel is verified by us
    #[prost(bool, tag = "8")]
    pub verified: bool,
    /// if permissions allow, this username can be used to join the channel.
    #[prost(string, optional, tag = "9")]
    pub username: ::core::option::Option<::prost::alloc::string::String>,
    /// Flag indicating whether the invite code can be used to join this channel
    #[prost(bool, tag = "10")]
    pub invite_code_enabled: bool,
    /// ID of the message pinned to the top of the channel
    #[prost(int64, optional, tag = "11")]
    pub pinned_msg_id: ::core::option::Option<i64>,
    /// ** Settings fields **
    /// Flag indicating whether this channel does not allow forwarding of messages
    #[prost(bool, tag = "12")]
    pub no_forwarding: bool,
    /// Flag indicating whether a user needs to join the channel before they can send messages
    #[prost(bool, tag = "13")]
    pub join_to_send: bool,
    /// Flag indicating whether a user needs to be approved by admin before they can join the channel
    #[prost(bool, tag = "14")]
    pub join_approval_req: bool,
    /// Flag indicating whether user participates are visible to other participants
    #[prost(bool, tag = "15")]
    pub participants_hidden: bool,
    /// Time to live of messages in this channel
    #[prost(int32, optional, tag = "16")]
    pub ttl_period: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelSettings {
    /// Flag indicating whether the channel has been verified
    #[prost(bool, tag = "1")]
    pub verified: bool,
    /// Flag indicating Whether this channel has a private join link
    #[prost(bool, tag = "2")]
    pub has_link: bool,
    /// Flag indicating whether this channel has a geoposition
    #[prost(bool, tag = "3")]
    pub has_geo: bool,
    /// Flag indicating whether the participant list is hidden
    #[prost(bool, tag = "4")]
    pub participants_hidden: bool,
    /// If set, channel was reported by many users as a fake or scam: be careful when interacting with it.
    #[prost(bool, tag = "5")]
    pub fake: bool,
    /// This channel has been flagged as a scam: be careful when interacting with it.
    #[prost(bool, tag = "6")]
    pub scam: bool,
    /// Time-To-Live period for messages in this channel. After this time has expired, messages are deleted
    #[prost(int32, tag = "7")]
    pub ttl_period: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelPermissions {
    /// Flag indicating whether this channel/group is protected -- not allowing forwarding of messages
    #[prost(bool, tag = "1")]
    pub noforwards: bool,
    /// Flag indicating whether a user needs to join the channel before they can send messages:
    /// can be false only for discussion groups », toggle using Channels_SetJoinToSend
    #[prost(bool, tag = "2")]
    pub join_to_send: bool,
    /// Flag indicating whether a user's join request will have to be approved by a channel admin, toggle using Channels_SetJoinApprovalRequired
    #[prost(bool, tag = "3")]
    pub join_approval_req: bool,
    /// Flag indicating whether viewing/writing in this channel for a reason (see restriction_reason)
    #[prost(bool, tag = "5")]
    pub restricted: bool,
    /// Restriction reason
    #[prost(string, tag = "6")]
    pub restriction_reason: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelUserSettings {
    /// Flag indicating whether the current user is the creator of this channel
    #[prost(bool, tag = "1")]
    pub is_creator: bool,
    /// Flag indicating whether the current user has left this channel
    #[prost(bool, tag = "2")]
    pub has_left: bool,
    /// Flag indicating whether the current user is an administrator of this channel
    #[prost(bool, tag = "3")]
    pub is_admin: bool,
    /// Admin rights of the user in this channel
    /// ChatAdminRights admin_rights = 4;
    /// Banned rights of the user in this channel
    /// ChatBannedRights banned_rights = 5;
    /// Flag indicating whether we can view the participants of this channel
    #[prost(bool, tag = "6")]
    pub can_view_participants: bool,
    /// Flag indicating whether we can delete this channel
    #[prost(bool, tag = "7")]
    pub can_delete_channel: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelStats {
    /// Participant count
    #[prost(int32, tag = "1")]
    pub participant_count: i32,
    /// Number of users currently online
    #[prost(int32, tag = "2")]
    pub online_count: i32,
    /// Number of channel admins
    #[prost(int32, tag = "3")]
    pub admins_count: i32,
    /// Number of users kicked from the channel
    #[prost(int32, tag = "4")]
    pub kicked_count: i32,
    /// Number of users banned from the channel
    #[prost(int32, tag = "5")]
    pub banned_count: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeoPoint {
    #[prost(double, tag = "1")]
    pub lat: f64,
    #[prost(double, tag = "2")]
    pub long: f64,
    #[prost(double, tag = "3")]
    pub radius: f64,
}
/// Inputs are the request messages/objects that the client sends to the server as part of the service call requests.
/// Request Message for `Channel_Create` RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelCreateRequest {
    /// Nanoid
    #[prost(string, tag = "1")]
    pub unique_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub about: ::prost::alloc::string::String,
    #[prost(enumeration = "ChannelType", tag = "4")]
    pub channel_type: i32,
    #[prost(int32, tag = "9")]
    pub ttl_period: i32,
}
/// Request Message for `Channel_EditAdmin` RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelEditAdminRequest {
    #[prost(int64, tag = "1")]
    pub channel_id: i64,
    #[prost(int64, tag = "2")]
    pub user_id: i64,
    #[prost(bool, tag = "3")]
    pub is_admin: bool,
    #[prost(message, optional, tag = "4")]
    pub admin_rights: ::core::option::Option<ChannelAdminRights>,
}
/// Request Message for `Channel_EditOwner` RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelEditOwnerRequest {
    #[prost(int64, tag = "1")]
    pub channel_id: i64,
    #[prost(int64, tag = "2")]
    pub user_id: i64,
    #[prost(string, tag = "3")]
    pub needs_2fa_reconfirmation_tbd: ::prost::alloc::string::String,
}
/// Request Message for `Channel_EditParticipant` RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelEditParticipantRequest {
    #[prost(int64, tag = "1")]
    pub channel_id: i64,
    #[prost(int64, tag = "2")]
    pub user_id: i64,
    #[prost(message, optional, tag = "3")]
    pub participant_rights: ::core::option::Option<ChannelParticipantRights>,
}
/// Request Message for `Channel_EditPhoto` RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelEditPhotoRequest {
    #[prost(int64, tag = "1")]
    pub channel_id: i64,
    #[prost(oneof = "channel_edit_photo_request::Photo", tags = "2, 3")]
    pub photo: ::core::option::Option<channel_edit_photo_request::Photo>,
}
/// Nested message and enum types in `ChannelEditPhotoRequest`.
pub mod channel_edit_photo_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Photo {
        #[prost(int64, tag = "2")]
        Photo64(i64),
        #[prost(string, tag = "3")]
        InputFileTbd(::prost::alloc::string::String),
    }
}
/// Request Message for `Channel_EditTitle` RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelEditTitleRequest {
    #[prost(int64, tag = "1")]
    pub channel_id: i64,
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
}
/// Request Message for `Channel_EditAbout` RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelEditAboutRequest {
    #[prost(int64, tag = "1")]
    pub channel_id: i64,
    #[prost(string, tag = "2")]
    pub about: ::prost::alloc::string::String,
}
/// Request Message for `Channel_Delete` RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelDeleteRequest {
    #[prost(int64, tag = "1")]
    pub channel_id: i64,
}
/// Request Message for `Channel_Disable` RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelDisableRequest {
    #[prost(int64, tag = "1")]
    pub channel_id: i64,
    #[prost(bool, tag = "2")]
    pub disable: bool,
}
/// ***************** Settings Methods ***************** //
/// Request Message for `Channel_EnablePrehistory` RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelEnablePrehistoryRequest {
    #[prost(int64, tag = "1")]
    pub channel_id: i64,
    #[prost(bool, tag = "2")]
    pub enabled: bool,
}
/// Request Message for `Channel_ShowSignatures` RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelShowSignaturesRequest {
    #[prost(int64, tag = "1")]
    pub channel_id: i64,
    #[prost(bool, tag = "2")]
    pub enabled: bool,
}
/// Request Message for `Channel_SetJoinToSend` RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelSetJoinToSendRequest {
    #[prost(int64, tag = "1")]
    pub channel_id: i64,
    #[prost(bool, tag = "2")]
    pub enabled: bool,
}
/// Request Message for `Channel_JoinApprovalRequired` RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelSetJoinApprovalRequiredRequest {
    #[prost(int64, tag = "1")]
    pub channel_id: i64,
    #[prost(bool, tag = "2")]
    pub enabled: bool,
}
/// ****************** Joining/Leaving Methods ****************** //
/// Request Message for `Channel_Join` RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelJoinRequest {
    #[prost(int64, tag = "1")]
    pub channel_id: i64,
}
/// Response Message for `Channel_Join` RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelJoinResponse {
    #[prost(enumeration = "JoinResponse", tag = "1")]
    pub join_status: i32,
}
/// Request Message for `Channel_Leave` RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelLeaveRequest {
    #[prost(int64, tag = "1")]
    pub channel_id: i64,
}
/// Request Message for `Channel_ActionJoinRequest` RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelActionJoinRequestRequest {
    #[prost(int64, tag = "1")]
    pub channel_id: i64,
    #[prost(int64, repeated, tag = "2")]
    pub user_id: ::prost::alloc::vec::Vec<i64>,
    #[prost(bool, tag = "3")]
    pub approve: bool,
}
/// Schema for Chat
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Chat {
    #[prost(int64, tag = "1")]
    pub id: i64,
    #[prost(int64, tag = "2")]
    pub user1_id: i64,
    #[prost(int64, tag = "3")]
    pub user2_id: i64,
    #[prost(int32, tag = "4")]
    pub ttl_period: i32,
    #[prost(string, optional, tag = "5")]
    pub theme_emoticon: ::core::option::Option<::prost::alloc::string::String>,
    /// Date when chat was created
    #[prost(message, optional, tag = "6")]
    pub date: ::core::option::Option<::prost_types::Timestamp>,
}
/// Inputs are the request messages/objects that the client sends to the server as part of the service call requests.
/// Request Message for `Chat_Create` RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatCreateRequest {
    #[prost(string, tag = "1")]
    pub unique_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub initiating_user_id: i64,
    #[prost(int64, tag = "3")]
    pub other_user_id: i64,
    #[prost(int32, tag = "4")]
    pub ttl_period: i32,
}
/// Request Message for `Chat_Delete` RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatDeleteRequest {
    #[prost(int64, tag = "1")]
    pub id: i64,
}
/// Schema for ChatGroup
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatGroup {
    #[prost(int64, tag = "1")]
    pub id: i64,
    #[prost(int64, tag = "2")]
    pub owner_id: i64,
    #[prost(string, optional, tag = "3")]
    pub username: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, tag = "4")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "5")]
    pub about: ::core::option::Option<::prost::alloc::string::String>,
    /// TODO: ChatPhoto type
    #[prost(string, optional, tag = "6")]
    pub photo_url: ::core::option::Option<::prost::alloc::string::String>,
    /// Date when user join the channel -- or if user is not a member, channel creation date
    #[prost(message, optional, tag = "7")]
    pub date: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(int64, optional, tag = "8")]
    pub pinned_msg_id: ::core::option::Option<i64>,
    /// Todo: ChatReactions
    #[prost(string, repeated, tag = "9")]
    pub available_reactions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int32, tag = "10")]
    pub ttl_period: i32,
    /// Invite Link
    ///
    /// TODO: ExportedChatInvite type
    #[prost(string, tag = "13")]
    pub invite_link: ::prost::alloc::string::String,
}
/// Inputs are the request messages/objects that the client sends to the server as part of the service call requests.
/// Request Message for `ChatGroup_Create` RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatGroupCreateRequest {
    #[prost(string, tag = "1")]
    pub unique_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub owner_id: i64,
    #[prost(string, tag = "3")]
    pub title: ::prost::alloc::string::String,
    #[prost(int32, tag = "4")]
    pub ttl_period: i32,
    #[prost(int64, repeated, tag = "5")]
    pub user_ids: ::prost::alloc::vec::Vec<i64>,
}
/// Request Message for `ChatGroup_UpdateTitle` RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatGroupUpdateTitleRequest {
    #[prost(int64, tag = "1")]
    pub id: i64,
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
}
/// Request Message for `ChatGroup_UpdateAbout` RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatGroupUpdateAboutRequest {
    #[prost(int64, tag = "1")]
    pub id: i64,
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
}
/// Request Message for `ChatGroup_AddUser` RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatGroupAddUserRequest {
    #[prost(int64, tag = "1")]
    pub id: i64,
    #[prost(int64, tag = "2")]
    pub user_id: i64,
    #[prost(int64, optional, tag = "3")]
    pub first_msg_id: ::core::option::Option<i64>,
}
/// Request Message for `ChatGroup_RemoveUser` RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatGroupRemoveUserRequest {
    #[prost(int64, tag = "1")]
    pub id: i64,
    #[prost(int64, tag = "2")]
    pub user_id: i64,
}
/// Request Message for `ChatGroup_EditAdmin` RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatGroupEditAdminRequest {
    #[prost(int64, tag = "1")]
    pub id: i64,
    #[prost(int64, tag = "2")]
    pub user_id: i64,
    #[prost(bool, tag = "3")]
    pub is_admin: bool,
}
/// Request Message for `ChatGroup_Delete` RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatGroupDeleteRequest {
    #[prost(int64, tag = "1")]
    pub id: i64,
}
/// The `Peer` type defines an entity in the chat system. It can be
/// a user, chatgroup, topic, forum? or channel
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PeerInput {
    #[prost(oneof = "peer_input::Peer", tags = "1, 2, 3, 4, 5")]
    pub peer: ::core::option::Option<peer_input::Peer>,
}
/// Nested message and enum types in `PeerInput`.
pub mod peer_input {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Peer {
        #[prost(message, tag = "1")]
        User(super::PeerUser),
        #[prost(message, tag = "2")]
        Chat(super::PeerChat),
        #[prost(message, tag = "3")]
        ChatGroup(super::PeerChatGroup),
        #[prost(message, tag = "4")]
        Topic(super::PeerTopic),
        #[prost(message, tag = "5")]
        Channel(super::PeerChannel),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PeerUser {
    #[prost(int64, tag = "1")]
    pub user_id: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PeerChat {
    #[prost(int64, tag = "1")]
    pub chat_id: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PeerChatGroup {
    #[prost(int64, tag = "1")]
    pub chat_group_id: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PeerTopic {
    #[prost(int64, tag = "1")]
    pub topic_id: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PeerChannel {
    #[prost(int64, tag = "1")]
    pub channel_id: i64,
}
/// The `MessageEntity` type defines styling, tagging, and mentioning in messages.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageEntity {
    #[prost(
        oneof = "message_entity::Entity",
        tags = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15"
    )]
    pub entity: ::core::option::Option<message_entity::Entity>,
}
/// Nested message and enum types in `MessageEntity`.
pub mod message_entity {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Entity {
        /// Styling entities
        #[prost(message, tag = "1")]
        Bold(super::Entity),
        #[prost(message, tag = "2")]
        Italic(super::Entity),
        #[prost(message, tag = "3")]
        BoldItalic(super::Entity),
        #[prost(message, tag = "4")]
        Underline(super::Entity),
        #[prost(message, tag = "5")]
        Strike(super::Entity),
        #[prost(message, tag = "6")]
        BlockQuote(super::Entity),
        #[prost(message, tag = "7")]
        CustomEmoji(super::EmojiEntity),
        /// Tagging entities
        #[prost(message, tag = "8")]
        SelfMention(super::UserEntity),
        #[prost(message, tag = "9")]
        Mention(super::UserEntity),
        #[prost(message, tag = "10")]
        Hashtag(super::Entity),
        /// Linking entities
        #[prost(message, tag = "11")]
        TextUrl(super::StringEntity),
        #[prost(message, tag = "12")]
        Url(super::Entity),
        #[prost(message, tag = "13")]
        Email(super::Entity),
        #[prost(message, tag = "14")]
        Cashtag(super::Entity),
        #[prost(message, tag = "15")]
        Phone(super::Entity),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Entity {
    #[prost(int32, tag = "1")]
    pub offset: i32,
    #[prost(int32, tag = "2")]
    pub length: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StringEntity {
    #[prost(int32, tag = "1")]
    pub offset: i32,
    #[prost(int32, tag = "2")]
    pub length: i32,
    #[prost(string, tag = "3")]
    pub text: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserEntity {
    #[prost(int32, tag = "1")]
    pub offset: i32,
    #[prost(int32, tag = "2")]
    pub length: i32,
    #[prost(int64, tag = "3")]
    pub user_id: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmojiEntity {
    #[prost(int32, tag = "1")]
    pub offset: i32,
    #[prost(int32, tag = "2")]
    pub length: i32,
    #[prost(int64, tag = "3")]
    pub document_id: i64,
}
/// Inputs are the request messages/objects that the client sends to the server as part of the service call requests.
/// Request Message for `Message_Send` RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageSendRequest {
    /// Unique client message ID required to prevent message resending.
    /// Every messsage should be sent with a new unique ID.
    /// format should be a Nanoid string 12-16 characters in length.
    #[prost(string, tag = "1")]
    pub unique_id: ::prost::alloc::string::String,
    /// ID of the user sending the message. TODO: Convert the source of this to the JWT token.
    #[prost(int64, tag = "2")]
    pub owner_id: i64,
    /// owning container of the message -- channel, topic or chat_group
    #[prost(message, optional, tag = "3")]
    pub input_peer: ::core::option::Option<PeerInput>,
    /// The actual message
    #[prost(string, tag = "4")]
    pub message: ::prost::alloc::string::String,
    /// ID of the message being forwarded
    #[prost(int64, optional, tag = "5")]
    pub fwd_from_msg_id: ::core::option::Option<i64>,
    /// ID of the message being replied to
    #[prost(int64, optional, tag = "6")]
    pub reply_to_msg_id: ::core::option::Option<i64>,
    /// This field is required for channel posts. The top_msg_id is the
    /// channel post message id that owns the message thread for replies
    /// For topics and chat groups, this field is not applicable
    #[prost(int64, optional, tag = "7")]
    pub top_msg_id: ::core::option::Option<i64>,
    #[prost(message, repeated, tag = "8")]
    pub entities: ::prost::alloc::vec::Vec<MessageEntity>,
    /// flag indicating the sender of the message is a channel
    #[prost(message, optional, tag = "9")]
    pub send_as: ::core::option::Option<PeerInput>,
    /// Send this message silently (no notifications for the receivers)
    #[prost(bool, tag = "10")]
    pub silent: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterUserRequest {
    #[prost(string, tag = "2")]
    pub user_name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterUserResponse {
    /// UUID
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub user_name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Topic {
    /// UUID
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// UUID
    #[prost(string, tag = "2")]
    pub channel_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub desc: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub tags: ::prost::alloc::string::String,
    /// TODO: Convert to Date
    #[prost(string, tag = "6")]
    pub created_date: ::prost::alloc::string::String,
    /// UUID
    #[prost(string, tag = "7")]
    pub owner_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JoinTopicRequest {
    /// UUID
    #[prost(string, tag = "1")]
    pub topic_id: ::prost::alloc::string::String,
    /// UUID
    #[prost(string, tag = "2")]
    pub user_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TopicResponse {
    /// UUID
    #[prost(string, tag = "1")]
    pub msg_id: ::prost::alloc::string::String,
    /// UUID
    #[prost(string, tag = "2")]
    pub topic_id: ::prost::alloc::string::String,
    /// UUID
    #[prost(string, tag = "3")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(oneof = "topic_response::Msg", tags = "5, 6, 7")]
    pub msg: ::core::option::Option<topic_response::Msg>,
}
/// Nested message and enum types in `TopicResponse`.
pub mod topic_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Msg {
        /// for returning existing Comments
        #[prost(message, tag = "5")]
        Comment(super::CommentResponse),
        /// for returning real-time updates
        #[prost(message, tag = "6")]
        Reaction(super::ReactionResponse),
        /// for returning real-time is_typing messages
        #[prost(message, tag = "7")]
        UserIsTyping(super::UserIsTypingResponse),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LeaveTopicRequest {
    /// UUID
    #[prost(string, tag = "1")]
    pub topic_id: ::prost::alloc::string::String,
    /// UUID
    #[prost(string, tag = "2")]
    pub user_id: ::prost::alloc::string::String,
}
/// **************************************************
/// Topic level messages that are sent in from clients
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Comment {
    /// UUID
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// UUIDa
    #[prost(string, tag = "2")]
    pub topic_id: ::prost::alloc::string::String,
    /// UUID
    #[prost(string, tag = "3")]
    pub user_id: ::prost::alloc::string::String,
    /// UUID or Empty string
    #[prost(string, tag = "4")]
    pub replyto_comment_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub comment: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Reaction {
    /// UUID
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// UUID // NOT REALLY NEEDED -- TO BE REMOVED
    #[prost(string, tag = "2")]
    pub topic_id: ::prost::alloc::string::String,
    /// UUID
    #[prost(string, tag = "3")]
    pub user_id: ::prost::alloc::string::String,
    /// UUID
    #[prost(string, tag = "4")]
    pub comment_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub reaction: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserIsTyping {
    /// UUID
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub topic_id: ::prost::alloc::string::String,
}
/// *******************************************
/// Topic level messages that pushed to clients
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommentResponse {
    /// UUID
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// UUID
    #[prost(string, tag = "2")]
    pub topic_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub user_name: ::prost::alloc::string::String,
    /// UUID or Empty string
    #[prost(string, tag = "4")]
    pub replyto_comment_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub comment: ::prost::alloc::string::String,
    #[prost(bool, tag = "6")]
    pub this_users_comment: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReactionResponse {
    /// UUID
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// UUID // NOT REALLY NEEDED -- TO BE REMOVED
    #[prost(string, tag = "2")]
    pub topic_id: ::prost::alloc::string::String,
    /// UUID
    #[prost(string, tag = "3")]
    pub user_name: ::prost::alloc::string::String,
    /// UUID
    #[prost(string, tag = "4")]
    pub comment_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub reaction: ::prost::alloc::string::String,
    #[prost(bool, tag = "6")]
    pub this_users_reaction: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserIsTypingResponse {
    /// UUID
    #[prost(string, tag = "1")]
    pub user_name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub topic_id: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod chat_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Interface exported by the server.
    #[derive(Debug, Clone)]
    pub struct ChatServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ChatServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> ChatServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> ChatServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            ChatServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// ****************************** Chats ******************************
        /// Creates a new chat
        pub async fn chat_create(
            &mut self,
            request: impl tonic::IntoRequest<super::ChatCreateRequest>,
        ) -> std::result::Result<tonic::Response<super::Chat>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/chat.ChatService/Chat_Create",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("chat.ChatService", "Chat_Create"));
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a chat
        pub async fn chat_delete(
            &mut self,
            request: impl tonic::IntoRequest<super::ChatDeleteRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/chat.ChatService/Chat_Delete",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("chat.ChatService", "Chat_Delete"));
            self.inner.unary(req, path, codec).await
        }
        /// ****************************** ChatGroups ******************************
        /// Creates a new chat group
        pub async fn chat_group_create(
            &mut self,
            request: impl tonic::IntoRequest<super::ChatGroupCreateRequest>,
        ) -> std::result::Result<tonic::Response<super::ChatGroup>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/chat.ChatService/ChatGroup_Create",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("chat.ChatService", "ChatGroup_Create"));
            self.inner.unary(req, path, codec).await
        }
        /// Updates the title of a given chat group
        pub async fn chat_group_update_title(
            &mut self,
            request: impl tonic::IntoRequest<super::ChatGroupUpdateTitleRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/chat.ChatService/ChatGroup_UpdateTitle",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("chat.ChatService", "ChatGroup_UpdateTitle"));
            self.inner.unary(req, path, codec).await
        }
        /// Updates the About of a given chat group
        pub async fn chat_group_update_about(
            &mut self,
            request: impl tonic::IntoRequest<super::ChatGroupUpdateAboutRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/chat.ChatService/ChatGroup_UpdateAbout",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("chat.ChatService", "ChatGroup_UpdateAbout"));
            self.inner.unary(req, path, codec).await
        }
        /// Adds a user to a chat group
        pub async fn chat_group_add_user(
            &mut self,
            request: impl tonic::IntoRequest<super::ChatGroupAddUserRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/chat.ChatService/ChatGroup_AddUser",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("chat.ChatService", "ChatGroup_AddUser"));
            self.inner.unary(req, path, codec).await
        }
        /// Removes a user to a chat group
        pub async fn chat_group_remove_user(
            &mut self,
            request: impl tonic::IntoRequest<super::ChatGroupRemoveUserRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/chat.ChatService/ChatGroup_RemoveUser",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("chat.ChatService", "ChatGroup_RemoveUser"));
            self.inner.unary(req, path, codec).await
        }
        /// Sets/unsets user as admin for a chat group
        pub async fn chat_group_edit_admin(
            &mut self,
            request: impl tonic::IntoRequest<super::ChatGroupEditAdminRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/chat.ChatService/ChatGroup_EditAdmin",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("chat.ChatService", "ChatGroup_EditAdmin"));
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a chat group
        pub async fn chat_group_delete(
            &mut self,
            request: impl tonic::IntoRequest<super::ChatGroupDeleteRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/chat.ChatService/ChatGroup_Delete",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("chat.ChatService", "ChatGroup_Delete"));
            self.inner.unary(req, path, codec).await
        }
        /// ******************************* Messages *******************************
        /// Sends a new message to a chat
        pub async fn message_send(
            &mut self,
            request: impl tonic::IntoRequest<super::MessageSendRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/chat.ChatService/Message_Send",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("chat.ChatService", "Message_Send"));
            self.inner.unary(req, path, codec).await
        }
        /// Temporary function to register a user until proper integration
        /// w/ user auth system and user lookup server in place.
        pub async fn temp_register_user(
            &mut self,
            request: impl tonic::IntoRequest<super::RegisterUserRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RegisterUserResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/chat.ChatService/TempRegisterUser",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("chat.ChatService", "TempRegisterUser"));
            self.inner.unary(req, path, codec).await
        }
        /// ******************************** Channels *********************************
        /// Creates a new top-level channel
        pub async fn channel_create(
            &mut self,
            request: impl tonic::IntoRequest<super::ChannelCreateRequest>,
        ) -> std::result::Result<tonic::Response<super::Channel>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/chat.ChatService/Channel_Create",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("chat.ChatService", "Channel_Create"));
            self.inner.unary(req, path, codec).await
        }
        /// Update the admin rights for a specific user on a channel
        pub async fn channel_edit_admin(
            &mut self,
            request: impl tonic::IntoRequest<super::ChannelEditAdminRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/chat.ChatService/Channel_EditAdmin",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("chat.ChatService", "Channel_EditAdmin"));
            self.inner.unary(req, path, codec).await
        }
        /// Assigns this channel to a new owner
        pub async fn channel_edit_owner(
            &mut self,
            request: impl tonic::IntoRequest<super::ChannelEditOwnerRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/chat.ChatService/Channel_EditOwner",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("chat.ChatService", "Channel_EditOwner"));
            self.inner.unary(req, path, codec).await
        }
        /// Updates a participants rights on a channel
        pub async fn channel_edit_participant(
            &mut self,
            request: impl tonic::IntoRequest<super::ChannelEditParticipantRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/chat.ChatService/Channel_EditParticipant",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("chat.ChatService", "Channel_EditParticipant"));
            self.inner.unary(req, path, codec).await
        }
        /// Updates the photo for a channel
        pub async fn channel_edit_photo(
            &mut self,
            request: impl tonic::IntoRequest<super::ChannelEditPhotoRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/chat.ChatService/Channel_EditPhoto",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("chat.ChatService", "Channel_EditPhoto"));
            self.inner.unary(req, path, codec).await
        }
        /// Updates the title for a channel
        pub async fn channel_edit_title(
            &mut self,
            request: impl tonic::IntoRequest<super::ChannelEditTitleRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/chat.ChatService/Channel_EditTitle",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("chat.ChatService", "Channel_EditTitle"));
            self.inner.unary(req, path, codec).await
        }
        /// Updates the about for a channel
        pub async fn channel_edit_about(
            &mut self,
            request: impl tonic::IntoRequest<super::ChannelEditAboutRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/chat.ChatService/Channel_EditAbout",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("chat.ChatService", "Channel_EditAbout"));
            self.inner.unary(req, path, codec).await
        }
        /// Disables a top-level channel
        /// TODO: require permissions to be able to call this method.
        pub async fn channel_delete(
            &mut self,
            request: impl tonic::IntoRequest<super::ChannelDeleteRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/chat.ChatService/Channel_Delete",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("chat.ChatService", "Channel_Delete"));
            self.inner.unary(req, path, codec).await
        }
        /// Disables a top-level channel
        pub async fn channel_disable(
            &mut self,
            request: impl tonic::IntoRequest<super::ChannelDisableRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/chat.ChatService/Channel_Disable",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("chat.ChatService", "Channel_Disable"));
            self.inner.unary(req, path, codec).await
        }
        /// Channels ::  Settings Methods
        /// Enable Prehistor for new users/participants
        pub async fn channel_enable_prehistory(
            &mut self,
            request: impl tonic::IntoRequest<super::ChannelEnablePrehistoryRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/chat.ChatService/Channel_EnablePrehistory",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("chat.ChatService", "Channel_EnablePrehistory"));
            self.inner.unary(req, path, codec).await
        }
        /// Enable Signatures for posts
        pub async fn channel_show_signatures(
            &mut self,
            request: impl tonic::IntoRequest<super::ChannelShowSignaturesRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/chat.ChatService/Channel_ShowSignatures",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("chat.ChatService", "Channel_ShowSignatures"));
            self.inner.unary(req, path, codec).await
        }
        /// Flag channel as must join to send messages
        pub async fn channel_set_join_to_send(
            &mut self,
            request: impl tonic::IntoRequest<super::ChannelSetJoinToSendRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/chat.ChatService/Channel_SetJoinToSend",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("chat.ChatService", "Channel_SetJoinToSend"));
            self.inner.unary(req, path, codec).await
        }
        /// Flag channel as requiring approval to join
        pub async fn channel_set_join_approval_required(
            &mut self,
            request: impl tonic::IntoRequest<
                super::ChannelSetJoinApprovalRequiredRequest,
            >,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/chat.ChatService/Channel_SetJoinApprovalRequired",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "chat.ChatService",
                        "Channel_SetJoinApprovalRequired",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Channels :: Joining/Leaving Methods
        /// Join a channel
        pub async fn channel_join(
            &mut self,
            request: impl tonic::IntoRequest<super::ChannelJoinRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ChannelJoinResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/chat.ChatService/Channel_Join",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("chat.ChatService", "Channel_Join"));
            self.inner.unary(req, path, codec).await
        }
        /// Leave a channel
        pub async fn channel_leave(
            &mut self,
            request: impl tonic::IntoRequest<super::ChannelLeaveRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/chat.ChatService/Channel_Leave",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("chat.ChatService", "Channel_Leave"));
            self.inner.unary(req, path, codec).await
        }
        /// Approve/Deny a user's request to join a channel
        pub async fn channel_action_join_request(
            &mut self,
            request: impl tonic::IntoRequest<super::ChannelActionJoinRequestRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/chat.ChatService/Channel_ActionJoinRequest",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("chat.ChatService", "Channel_ActionJoinRequest"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// ******************************** Topics *********************************
        /// TO BE deprecated -- re-written
        /// Creates a topic within a channel
        pub async fn create_topic(
            &mut self,
            request: impl tonic::IntoRequest<super::Topic>,
        ) -> std::result::Result<tonic::Response<super::Topic>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/chat.ChatService/CreateTopic",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("chat.ChatService", "CreateTopic"));
            self.inner.unary(req, path, codec).await
        }
        /// Delete a topic within a channel
        pub async fn delete_topic(
            &mut self,
            request: impl tonic::IntoRequest<super::Topic>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/chat.ChatService/DeleteTopic",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("chat.ChatService", "DeleteTopic"));
            self.inner.unary(req, path, codec).await
        }
        /// Joins a user to a topic by registering the user on the topic queue
        pub async fn join_topic(
            &mut self,
            request: impl tonic::IntoRequest<super::JoinTopicRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::TopicResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/chat.ChatService/JoinTopic",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("chat.ChatService", "JoinTopic"));
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn leave_topic(
            &mut self,
            request: impl tonic::IntoRequest<super::LeaveTopicRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/chat.ChatService/LeaveTopic",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("chat.ChatService", "LeaveTopic"));
            self.inner.unary(req, path, codec).await
        }
        /// ***** Comments ******* //
        /// Creates a comment within a topic
        pub async fn add_comment(
            &mut self,
            request: impl tonic::IntoRequest<super::Comment>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/chat.ChatService/AddComment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("chat.ChatService", "AddComment"));
            self.inner.unary(req, path, codec).await
        }
        /// ***** Reactions ******* //
        /// React to a comment
        pub async fn send_reaction(
            &mut self,
            request: impl tonic::IntoRequest<super::Reaction>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/chat.ChatService/SendReaction",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("chat.ChatService", "SendReaction"));
            self.inner.unary(req, path, codec).await
        }
        /// ***** Typing Indicator ******* //
        /// Ephemeral Event to push out showing a user is typing
        pub async fn send_user_is_typing(
            &mut self,
            request: impl tonic::IntoRequest<super::UserIsTyping>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/chat.ChatService/SendUserIsTyping",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("chat.ChatService", "SendUserIsTyping"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod chat_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with ChatServiceServer.
    #[async_trait]
    pub trait ChatService: Send + Sync + 'static {
        /// ****************************** Chats ******************************
        /// Creates a new chat
        async fn chat_create(
            &self,
            request: tonic::Request<super::ChatCreateRequest>,
        ) -> std::result::Result<tonic::Response<super::Chat>, tonic::Status>;
        /// Deletes a chat
        async fn chat_delete(
            &self,
            request: tonic::Request<super::ChatDeleteRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        /// ****************************** ChatGroups ******************************
        /// Creates a new chat group
        async fn chat_group_create(
            &self,
            request: tonic::Request<super::ChatGroupCreateRequest>,
        ) -> std::result::Result<tonic::Response<super::ChatGroup>, tonic::Status>;
        /// Updates the title of a given chat group
        async fn chat_group_update_title(
            &self,
            request: tonic::Request<super::ChatGroupUpdateTitleRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        /// Updates the About of a given chat group
        async fn chat_group_update_about(
            &self,
            request: tonic::Request<super::ChatGroupUpdateAboutRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        /// Adds a user to a chat group
        async fn chat_group_add_user(
            &self,
            request: tonic::Request<super::ChatGroupAddUserRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        /// Removes a user to a chat group
        async fn chat_group_remove_user(
            &self,
            request: tonic::Request<super::ChatGroupRemoveUserRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        /// Sets/unsets user as admin for a chat group
        async fn chat_group_edit_admin(
            &self,
            request: tonic::Request<super::ChatGroupEditAdminRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        /// Deletes a chat group
        async fn chat_group_delete(
            &self,
            request: tonic::Request<super::ChatGroupDeleteRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        /// ******************************* Messages *******************************
        /// Sends a new message to a chat
        async fn message_send(
            &self,
            request: tonic::Request<super::MessageSendRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        /// Temporary function to register a user until proper integration
        /// w/ user auth system and user lookup server in place.
        async fn temp_register_user(
            &self,
            request: tonic::Request<super::RegisterUserRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RegisterUserResponse>,
            tonic::Status,
        >;
        /// ******************************** Channels *********************************
        /// Creates a new top-level channel
        async fn channel_create(
            &self,
            request: tonic::Request<super::ChannelCreateRequest>,
        ) -> std::result::Result<tonic::Response<super::Channel>, tonic::Status>;
        /// Update the admin rights for a specific user on a channel
        async fn channel_edit_admin(
            &self,
            request: tonic::Request<super::ChannelEditAdminRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        /// Assigns this channel to a new owner
        async fn channel_edit_owner(
            &self,
            request: tonic::Request<super::ChannelEditOwnerRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        /// Updates a participants rights on a channel
        async fn channel_edit_participant(
            &self,
            request: tonic::Request<super::ChannelEditParticipantRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        /// Updates the photo for a channel
        async fn channel_edit_photo(
            &self,
            request: tonic::Request<super::ChannelEditPhotoRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        /// Updates the title for a channel
        async fn channel_edit_title(
            &self,
            request: tonic::Request<super::ChannelEditTitleRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        /// Updates the about for a channel
        async fn channel_edit_about(
            &self,
            request: tonic::Request<super::ChannelEditAboutRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        /// Disables a top-level channel
        /// TODO: require permissions to be able to call this method.
        async fn channel_delete(
            &self,
            request: tonic::Request<super::ChannelDeleteRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        /// Disables a top-level channel
        async fn channel_disable(
            &self,
            request: tonic::Request<super::ChannelDisableRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        /// Channels ::  Settings Methods
        /// Enable Prehistor for new users/participants
        async fn channel_enable_prehistory(
            &self,
            request: tonic::Request<super::ChannelEnablePrehistoryRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        /// Enable Signatures for posts
        async fn channel_show_signatures(
            &self,
            request: tonic::Request<super::ChannelShowSignaturesRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        /// Flag channel as must join to send messages
        async fn channel_set_join_to_send(
            &self,
            request: tonic::Request<super::ChannelSetJoinToSendRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        /// Flag channel as requiring approval to join
        async fn channel_set_join_approval_required(
            &self,
            request: tonic::Request<super::ChannelSetJoinApprovalRequiredRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        /// Channels :: Joining/Leaving Methods
        /// Join a channel
        async fn channel_join(
            &self,
            request: tonic::Request<super::ChannelJoinRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ChannelJoinResponse>,
            tonic::Status,
        >;
        /// Leave a channel
        async fn channel_leave(
            &self,
            request: tonic::Request<super::ChannelLeaveRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        /// Approve/Deny a user's request to join a channel
        async fn channel_action_join_request(
            &self,
            request: tonic::Request<super::ChannelActionJoinRequestRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        /// ******************************** Topics *********************************
        /// TO BE deprecated -- re-written
        /// Creates a topic within a channel
        async fn create_topic(
            &self,
            request: tonic::Request<super::Topic>,
        ) -> std::result::Result<tonic::Response<super::Topic>, tonic::Status>;
        /// Delete a topic within a channel
        async fn delete_topic(
            &self,
            request: tonic::Request<super::Topic>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        /// Server streaming response type for the JoinTopic method.
        type JoinTopicStream: futures_core::Stream<
                Item = std::result::Result<super::TopicResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// Joins a user to a topic by registering the user on the topic queue
        async fn join_topic(
            &self,
            request: tonic::Request<super::JoinTopicRequest>,
        ) -> std::result::Result<tonic::Response<Self::JoinTopicStream>, tonic::Status>;
        async fn leave_topic(
            &self,
            request: tonic::Request<super::LeaveTopicRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        /// ***** Comments ******* //
        /// Creates a comment within a topic
        async fn add_comment(
            &self,
            request: tonic::Request<super::Comment>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        /// ***** Reactions ******* //
        /// React to a comment
        async fn send_reaction(
            &self,
            request: tonic::Request<super::Reaction>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        /// ***** Typing Indicator ******* //
        /// Ephemeral Event to push out showing a user is typing
        async fn send_user_is_typing(
            &self,
            request: tonic::Request<super::UserIsTyping>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
    }
    /// Interface exported by the server.
    #[derive(Debug)]
    pub struct ChatServiceServer<T: ChatService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: ChatService> ChatServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ChatServiceServer<T>
    where
        T: ChatService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/chat.ChatService/Chat_Create" => {
                    #[allow(non_camel_case_types)]
                    struct Chat_CreateSvc<T: ChatService>(pub Arc<T>);
                    impl<
                        T: ChatService,
                    > tonic::server::UnaryService<super::ChatCreateRequest>
                    for Chat_CreateSvc<T> {
                        type Response = super::Chat;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ChatCreateRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).chat_create(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = Chat_CreateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/chat.ChatService/Chat_Delete" => {
                    #[allow(non_camel_case_types)]
                    struct Chat_DeleteSvc<T: ChatService>(pub Arc<T>);
                    impl<
                        T: ChatService,
                    > tonic::server::UnaryService<super::ChatDeleteRequest>
                    for Chat_DeleteSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ChatDeleteRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).chat_delete(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = Chat_DeleteSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/chat.ChatService/ChatGroup_Create" => {
                    #[allow(non_camel_case_types)]
                    struct ChatGroup_CreateSvc<T: ChatService>(pub Arc<T>);
                    impl<
                        T: ChatService,
                    > tonic::server::UnaryService<super::ChatGroupCreateRequest>
                    for ChatGroup_CreateSvc<T> {
                        type Response = super::ChatGroup;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ChatGroupCreateRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).chat_group_create(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ChatGroup_CreateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/chat.ChatService/ChatGroup_UpdateTitle" => {
                    #[allow(non_camel_case_types)]
                    struct ChatGroup_UpdateTitleSvc<T: ChatService>(pub Arc<T>);
                    impl<
                        T: ChatService,
                    > tonic::server::UnaryService<super::ChatGroupUpdateTitleRequest>
                    for ChatGroup_UpdateTitleSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ChatGroupUpdateTitleRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).chat_group_update_title(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ChatGroup_UpdateTitleSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/chat.ChatService/ChatGroup_UpdateAbout" => {
                    #[allow(non_camel_case_types)]
                    struct ChatGroup_UpdateAboutSvc<T: ChatService>(pub Arc<T>);
                    impl<
                        T: ChatService,
                    > tonic::server::UnaryService<super::ChatGroupUpdateAboutRequest>
                    for ChatGroup_UpdateAboutSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ChatGroupUpdateAboutRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).chat_group_update_about(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ChatGroup_UpdateAboutSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/chat.ChatService/ChatGroup_AddUser" => {
                    #[allow(non_camel_case_types)]
                    struct ChatGroup_AddUserSvc<T: ChatService>(pub Arc<T>);
                    impl<
                        T: ChatService,
                    > tonic::server::UnaryService<super::ChatGroupAddUserRequest>
                    for ChatGroup_AddUserSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ChatGroupAddUserRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).chat_group_add_user(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ChatGroup_AddUserSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/chat.ChatService/ChatGroup_RemoveUser" => {
                    #[allow(non_camel_case_types)]
                    struct ChatGroup_RemoveUserSvc<T: ChatService>(pub Arc<T>);
                    impl<
                        T: ChatService,
                    > tonic::server::UnaryService<super::ChatGroupRemoveUserRequest>
                    for ChatGroup_RemoveUserSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ChatGroupRemoveUserRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).chat_group_remove_user(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ChatGroup_RemoveUserSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/chat.ChatService/ChatGroup_EditAdmin" => {
                    #[allow(non_camel_case_types)]
                    struct ChatGroup_EditAdminSvc<T: ChatService>(pub Arc<T>);
                    impl<
                        T: ChatService,
                    > tonic::server::UnaryService<super::ChatGroupEditAdminRequest>
                    for ChatGroup_EditAdminSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ChatGroupEditAdminRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).chat_group_edit_admin(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ChatGroup_EditAdminSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/chat.ChatService/ChatGroup_Delete" => {
                    #[allow(non_camel_case_types)]
                    struct ChatGroup_DeleteSvc<T: ChatService>(pub Arc<T>);
                    impl<
                        T: ChatService,
                    > tonic::server::UnaryService<super::ChatGroupDeleteRequest>
                    for ChatGroup_DeleteSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ChatGroupDeleteRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).chat_group_delete(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ChatGroup_DeleteSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/chat.ChatService/Message_Send" => {
                    #[allow(non_camel_case_types)]
                    struct Message_SendSvc<T: ChatService>(pub Arc<T>);
                    impl<
                        T: ChatService,
                    > tonic::server::UnaryService<super::MessageSendRequest>
                    for Message_SendSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MessageSendRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).message_send(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = Message_SendSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/chat.ChatService/TempRegisterUser" => {
                    #[allow(non_camel_case_types)]
                    struct TempRegisterUserSvc<T: ChatService>(pub Arc<T>);
                    impl<
                        T: ChatService,
                    > tonic::server::UnaryService<super::RegisterUserRequest>
                    for TempRegisterUserSvc<T> {
                        type Response = super::RegisterUserResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RegisterUserRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).temp_register_user(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = TempRegisterUserSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/chat.ChatService/Channel_Create" => {
                    #[allow(non_camel_case_types)]
                    struct Channel_CreateSvc<T: ChatService>(pub Arc<T>);
                    impl<
                        T: ChatService,
                    > tonic::server::UnaryService<super::ChannelCreateRequest>
                    for Channel_CreateSvc<T> {
                        type Response = super::Channel;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ChannelCreateRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).channel_create(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = Channel_CreateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/chat.ChatService/Channel_EditAdmin" => {
                    #[allow(non_camel_case_types)]
                    struct Channel_EditAdminSvc<T: ChatService>(pub Arc<T>);
                    impl<
                        T: ChatService,
                    > tonic::server::UnaryService<super::ChannelEditAdminRequest>
                    for Channel_EditAdminSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ChannelEditAdminRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).channel_edit_admin(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = Channel_EditAdminSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/chat.ChatService/Channel_EditOwner" => {
                    #[allow(non_camel_case_types)]
                    struct Channel_EditOwnerSvc<T: ChatService>(pub Arc<T>);
                    impl<
                        T: ChatService,
                    > tonic::server::UnaryService<super::ChannelEditOwnerRequest>
                    for Channel_EditOwnerSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ChannelEditOwnerRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).channel_edit_owner(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = Channel_EditOwnerSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/chat.ChatService/Channel_EditParticipant" => {
                    #[allow(non_camel_case_types)]
                    struct Channel_EditParticipantSvc<T: ChatService>(pub Arc<T>);
                    impl<
                        T: ChatService,
                    > tonic::server::UnaryService<super::ChannelEditParticipantRequest>
                    for Channel_EditParticipantSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ChannelEditParticipantRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).channel_edit_participant(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = Channel_EditParticipantSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/chat.ChatService/Channel_EditPhoto" => {
                    #[allow(non_camel_case_types)]
                    struct Channel_EditPhotoSvc<T: ChatService>(pub Arc<T>);
                    impl<
                        T: ChatService,
                    > tonic::server::UnaryService<super::ChannelEditPhotoRequest>
                    for Channel_EditPhotoSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ChannelEditPhotoRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).channel_edit_photo(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = Channel_EditPhotoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/chat.ChatService/Channel_EditTitle" => {
                    #[allow(non_camel_case_types)]
                    struct Channel_EditTitleSvc<T: ChatService>(pub Arc<T>);
                    impl<
                        T: ChatService,
                    > tonic::server::UnaryService<super::ChannelEditTitleRequest>
                    for Channel_EditTitleSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ChannelEditTitleRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).channel_edit_title(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = Channel_EditTitleSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/chat.ChatService/Channel_EditAbout" => {
                    #[allow(non_camel_case_types)]
                    struct Channel_EditAboutSvc<T: ChatService>(pub Arc<T>);
                    impl<
                        T: ChatService,
                    > tonic::server::UnaryService<super::ChannelEditAboutRequest>
                    for Channel_EditAboutSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ChannelEditAboutRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).channel_edit_about(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = Channel_EditAboutSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/chat.ChatService/Channel_Delete" => {
                    #[allow(non_camel_case_types)]
                    struct Channel_DeleteSvc<T: ChatService>(pub Arc<T>);
                    impl<
                        T: ChatService,
                    > tonic::server::UnaryService<super::ChannelDeleteRequest>
                    for Channel_DeleteSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ChannelDeleteRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).channel_delete(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = Channel_DeleteSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/chat.ChatService/Channel_Disable" => {
                    #[allow(non_camel_case_types)]
                    struct Channel_DisableSvc<T: ChatService>(pub Arc<T>);
                    impl<
                        T: ChatService,
                    > tonic::server::UnaryService<super::ChannelDisableRequest>
                    for Channel_DisableSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ChannelDisableRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).channel_disable(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = Channel_DisableSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/chat.ChatService/Channel_EnablePrehistory" => {
                    #[allow(non_camel_case_types)]
                    struct Channel_EnablePrehistorySvc<T: ChatService>(pub Arc<T>);
                    impl<
                        T: ChatService,
                    > tonic::server::UnaryService<super::ChannelEnablePrehistoryRequest>
                    for Channel_EnablePrehistorySvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ChannelEnablePrehistoryRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).channel_enable_prehistory(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = Channel_EnablePrehistorySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/chat.ChatService/Channel_ShowSignatures" => {
                    #[allow(non_camel_case_types)]
                    struct Channel_ShowSignaturesSvc<T: ChatService>(pub Arc<T>);
                    impl<
                        T: ChatService,
                    > tonic::server::UnaryService<super::ChannelShowSignaturesRequest>
                    for Channel_ShowSignaturesSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ChannelShowSignaturesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).channel_show_signatures(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = Channel_ShowSignaturesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/chat.ChatService/Channel_SetJoinToSend" => {
                    #[allow(non_camel_case_types)]
                    struct Channel_SetJoinToSendSvc<T: ChatService>(pub Arc<T>);
                    impl<
                        T: ChatService,
                    > tonic::server::UnaryService<super::ChannelSetJoinToSendRequest>
                    for Channel_SetJoinToSendSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ChannelSetJoinToSendRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).channel_set_join_to_send(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = Channel_SetJoinToSendSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/chat.ChatService/Channel_SetJoinApprovalRequired" => {
                    #[allow(non_camel_case_types)]
                    struct Channel_SetJoinApprovalRequiredSvc<T: ChatService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ChatService,
                    > tonic::server::UnaryService<
                        super::ChannelSetJoinApprovalRequiredRequest,
                    > for Channel_SetJoinApprovalRequiredSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ChannelSetJoinApprovalRequiredRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).channel_set_join_approval_required(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = Channel_SetJoinApprovalRequiredSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/chat.ChatService/Channel_Join" => {
                    #[allow(non_camel_case_types)]
                    struct Channel_JoinSvc<T: ChatService>(pub Arc<T>);
                    impl<
                        T: ChatService,
                    > tonic::server::UnaryService<super::ChannelJoinRequest>
                    for Channel_JoinSvc<T> {
                        type Response = super::ChannelJoinResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ChannelJoinRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).channel_join(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = Channel_JoinSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/chat.ChatService/Channel_Leave" => {
                    #[allow(non_camel_case_types)]
                    struct Channel_LeaveSvc<T: ChatService>(pub Arc<T>);
                    impl<
                        T: ChatService,
                    > tonic::server::UnaryService<super::ChannelLeaveRequest>
                    for Channel_LeaveSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ChannelLeaveRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).channel_leave(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = Channel_LeaveSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/chat.ChatService/Channel_ActionJoinRequest" => {
                    #[allow(non_camel_case_types)]
                    struct Channel_ActionJoinRequestSvc<T: ChatService>(pub Arc<T>);
                    impl<
                        T: ChatService,
                    > tonic::server::UnaryService<super::ChannelActionJoinRequestRequest>
                    for Channel_ActionJoinRequestSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ChannelActionJoinRequestRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).channel_action_join_request(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = Channel_ActionJoinRequestSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/chat.ChatService/CreateTopic" => {
                    #[allow(non_camel_case_types)]
                    struct CreateTopicSvc<T: ChatService>(pub Arc<T>);
                    impl<T: ChatService> tonic::server::UnaryService<super::Topic>
                    for CreateTopicSvc<T> {
                        type Response = super::Topic;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Topic>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).create_topic(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateTopicSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/chat.ChatService/DeleteTopic" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteTopicSvc<T: ChatService>(pub Arc<T>);
                    impl<T: ChatService> tonic::server::UnaryService<super::Topic>
                    for DeleteTopicSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Topic>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).delete_topic(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteTopicSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/chat.ChatService/JoinTopic" => {
                    #[allow(non_camel_case_types)]
                    struct JoinTopicSvc<T: ChatService>(pub Arc<T>);
                    impl<
                        T: ChatService,
                    > tonic::server::ServerStreamingService<super::JoinTopicRequest>
                    for JoinTopicSvc<T> {
                        type Response = super::TopicResponse;
                        type ResponseStream = T::JoinTopicStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::JoinTopicRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).join_topic(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = JoinTopicSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/chat.ChatService/LeaveTopic" => {
                    #[allow(non_camel_case_types)]
                    struct LeaveTopicSvc<T: ChatService>(pub Arc<T>);
                    impl<
                        T: ChatService,
                    > tonic::server::UnaryService<super::LeaveTopicRequest>
                    for LeaveTopicSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::LeaveTopicRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).leave_topic(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = LeaveTopicSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/chat.ChatService/AddComment" => {
                    #[allow(non_camel_case_types)]
                    struct AddCommentSvc<T: ChatService>(pub Arc<T>);
                    impl<T: ChatService> tonic::server::UnaryService<super::Comment>
                    for AddCommentSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Comment>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).add_comment(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AddCommentSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/chat.ChatService/SendReaction" => {
                    #[allow(non_camel_case_types)]
                    struct SendReactionSvc<T: ChatService>(pub Arc<T>);
                    impl<T: ChatService> tonic::server::UnaryService<super::Reaction>
                    for SendReactionSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Reaction>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).send_reaction(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SendReactionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/chat.ChatService/SendUserIsTyping" => {
                    #[allow(non_camel_case_types)]
                    struct SendUserIsTypingSvc<T: ChatService>(pub Arc<T>);
                    impl<T: ChatService> tonic::server::UnaryService<super::UserIsTyping>
                    for SendUserIsTypingSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UserIsTyping>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).send_user_is_typing(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SendUserIsTypingSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: ChatService> Clone for ChatServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: ChatService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ChatService> tonic::server::NamedService for ChatServiceServer<T> {
        const NAME: &'static str = "chat.ChatService";
    }
}
/// External Error Response
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Error {
    #[prost(int32, tag = "1")]
    pub code: i32,
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
    #[prost(int32, tag = "3")]
    pub status: i32,
}
/// Internal/NATS Error Response
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ErrorReply {
    #[prost(int32, tag = "1")]
    pub code: i32,
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
    #[prost(int32, tag = "3")]
    pub status: i32,
    #[prost(message, repeated, tag = "4")]
    pub details: ::prost::alloc::vec::Vec<ErrorDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ErrorDetails {
    #[prost(string, tag = "1")]
    pub reason: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub domain: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub metadata: ::prost::alloc::vec::Vec<MetaData>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetaData {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Metadata {
    #[prost(message, repeated, tag = "1")]
    pub headers: ::prost::alloc::vec::Vec<MetadataMap>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetadataMap {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub value: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// ********** Channel_Create ********** //
/// Backend NATS messaging for `Channel_Create`: Request to create a new chat group
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NatsChannelCreateRequest {
    #[prost(message, repeated, tag = "1")]
    pub headers: ::prost::alloc::vec::Vec<MetadataMap>,
    #[prost(message, optional, tag = "2")]
    pub data: ::core::option::Option<ChannelCreateRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NatsChannelCreateResponse {
    #[prost(oneof = "nats_channel_create_response::Msg", tags = "1, 2")]
    pub msg: ::core::option::Option<nats_channel_create_response::Msg>,
}
/// Nested message and enum types in `NatsChannelCreateResponse`.
pub mod nats_channel_create_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Msg {
        #[prost(message, tag = "1")]
        Data(super::Channel),
        #[prost(message, tag = "2")]
        Error(super::ErrorReply),
    }
}
/// ********** Channel_EditAdmin ********** //
/// Backend NATS messaging for `Channel_EditAdmin`: Edits admin rights of the channel
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NatsChannelEditAdminRequest {
    #[prost(message, repeated, tag = "1")]
    pub headers: ::prost::alloc::vec::Vec<MetadataMap>,
    #[prost(message, optional, tag = "2")]
    pub data: ::core::option::Option<ChannelEditAdminRequest>,
}
/// ********** Channel_EditOwner ********** //
/// Backend NATS messaging for `Channel_EditOwner`: Assigns channel ownership to another user
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NatsChannelEditOwnerRequest {
    #[prost(message, repeated, tag = "1")]
    pub headers: ::prost::alloc::vec::Vec<MetadataMap>,
    #[prost(message, optional, tag = "2")]
    pub data: ::core::option::Option<ChannelEditOwnerRequest>,
}
/// ********** Channel_EditParticipant ********** //
/// Backend NATS messaging for `Channel_EditParticipant`: Edits participant rights of the channel
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NatsChannelEditParticipantRequest {
    #[prost(message, repeated, tag = "1")]
    pub headers: ::prost::alloc::vec::Vec<MetadataMap>,
    #[prost(message, optional, tag = "2")]
    pub data: ::core::option::Option<ChannelEditParticipantRequest>,
}
/// ********** Channel_EditPhoto ********** //
/// Backend NATS messaging for `Channel_EditPhoto`: Edits the photo of the channel
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NatsChannelEditPhotoRequest {
    #[prost(message, repeated, tag = "1")]
    pub headers: ::prost::alloc::vec::Vec<MetadataMap>,
    #[prost(message, optional, tag = "2")]
    pub data: ::core::option::Option<ChannelEditPhotoRequest>,
}
/// ********** Channel_EditTitle ********** //
/// Backend NATS messaging for `Channel_EditTitle`: Edits the title of the channel
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NatsChannelEditTitleRequest {
    #[prost(message, repeated, tag = "1")]
    pub headers: ::prost::alloc::vec::Vec<MetadataMap>,
    #[prost(message, optional, tag = "2")]
    pub data: ::core::option::Option<ChannelEditTitleRequest>,
}
/// ********** Channel_EditAbout ********** //
/// Backend NATS messaging for `Channel_EditAbout`: Edits the about of the channel
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NatsChannelEditAboutRequest {
    #[prost(message, repeated, tag = "1")]
    pub headers: ::prost::alloc::vec::Vec<MetadataMap>,
    #[prost(message, optional, tag = "2")]
    pub data: ::core::option::Option<ChannelEditAboutRequest>,
}
/// ********** Channel_Delete ********** //
/// Backend NATS messaging for `Channel_Delete`: Deletes a channel
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NatsChannelDeleteRequest {
    #[prost(message, repeated, tag = "1")]
    pub headers: ::prost::alloc::vec::Vec<MetadataMap>,
    #[prost(message, optional, tag = "2")]
    pub data: ::core::option::Option<ChannelDeleteRequest>,
}
/// ********** Channel_Disable ********** //
/// Backend NATS messaging for `Channel_Disable`: Disables/Enables a channel
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NatsChannelDisableRequest {
    #[prost(message, repeated, tag = "1")]
    pub headers: ::prost::alloc::vec::Vec<MetadataMap>,
    #[prost(message, optional, tag = "2")]
    pub data: ::core::option::Option<ChannelDisableRequest>,
}
/// ********** Channel_EnablePrehistory ********** //
/// Backend NATS messaging for `Channel_EnablePrehistory`: Enables prehistory for a channel
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NatsChannelEnablePrehistoryRequest {
    #[prost(message, repeated, tag = "1")]
    pub headers: ::prost::alloc::vec::Vec<MetadataMap>,
    #[prost(message, optional, tag = "2")]
    pub data: ::core::option::Option<ChannelEnablePrehistoryRequest>,
}
/// ********** Channel_ShowSignatures ********** //
/// Backend NATS messaging for `Channel_ShowSignatures`: Shows signatures for a channel
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NatsChannelShowSignaturesRequest {
    #[prost(message, repeated, tag = "1")]
    pub headers: ::prost::alloc::vec::Vec<MetadataMap>,
    #[prost(message, optional, tag = "2")]
    pub data: ::core::option::Option<ChannelShowSignaturesRequest>,
}
/// ********** Channel_SetJoinToSend ********** //
/// Backend NATS messaging for `Channel_SetJoinToSend`: Sets join to send for a channel
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NatsChannelSetJoinToSendRequest {
    #[prost(message, repeated, tag = "1")]
    pub headers: ::prost::alloc::vec::Vec<MetadataMap>,
    #[prost(message, optional, tag = "2")]
    pub data: ::core::option::Option<ChannelSetJoinToSendRequest>,
}
/// ********** Channel_SetJoinToApprove ********** //
/// Backend NATS messaging for `Channel_SetJoinToApprove`: Sets join to approve for a channel
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NatsChannelSetJoinApprovalRequiredRequest {
    #[prost(message, repeated, tag = "1")]
    pub headers: ::prost::alloc::vec::Vec<MetadataMap>,
    #[prost(message, optional, tag = "2")]
    pub data: ::core::option::Option<ChannelSetJoinApprovalRequiredRequest>,
}
/// ********** Channel_Join ********** //
/// Backend NATS messaging for `Channel_Join`: Joins a channel
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NatsChannelJoinRequest {
    #[prost(message, repeated, tag = "1")]
    pub headers: ::prost::alloc::vec::Vec<MetadataMap>,
    #[prost(message, optional, tag = "2")]
    pub data: ::core::option::Option<ChannelJoinRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NatsChannelJoinResponse {
    #[prost(oneof = "nats_channel_join_response::Msg", tags = "1, 2")]
    pub msg: ::core::option::Option<nats_channel_join_response::Msg>,
}
/// Nested message and enum types in `NatsChannelJoinResponse`.
pub mod nats_channel_join_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Msg {
        #[prost(message, tag = "1")]
        Data(super::ChannelJoinResponse),
        #[prost(message, tag = "2")]
        Error(super::ErrorReply),
    }
}
/// ********** Channel_Leave ********** //
/// Backend NATS messaging for `Channel_Leave`: Leaves a channel
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NatsChannelLeaveRequest {
    #[prost(message, repeated, tag = "1")]
    pub headers: ::prost::alloc::vec::Vec<MetadataMap>,
    #[prost(message, optional, tag = "2")]
    pub data: ::core::option::Option<ChannelLeaveRequest>,
}
/// ********** Channel_ActionJoinRequest ********** //
/// Backend NATS messaging for `Channel_ActionJoinRequest`: Approves or rejects a join request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NatsChannelActionJoinRequestRequest {
    #[prost(message, repeated, tag = "1")]
    pub headers: ::prost::alloc::vec::Vec<MetadataMap>,
    #[prost(message, optional, tag = "2")]
    pub data: ::core::option::Option<ChannelActionJoinRequestRequest>,
}
/// ********** Chat_Create ********** //
/// Backend NATS messaging for `Chat_Create`: Request to create a new chat
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NatsChatCreateRequest {
    #[prost(message, repeated, tag = "1")]
    pub headers: ::prost::alloc::vec::Vec<MetadataMap>,
    #[prost(message, optional, tag = "2")]
    pub data: ::core::option::Option<ChatCreateRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NatsChatCreateResponse {
    #[prost(oneof = "nats_chat_create_response::Msg", tags = "1, 2")]
    pub msg: ::core::option::Option<nats_chat_create_response::Msg>,
}
/// Nested message and enum types in `NatsChatCreateResponse`.
pub mod nats_chat_create_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Msg {
        #[prost(message, tag = "1")]
        Data(super::Chat),
        #[prost(message, tag = "2")]
        Error(super::ErrorReply),
    }
}
/// ********** Chat_Delete ********** //
/// Backend NATS messaging for `Chat_Delete`: Deletes a chat
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NatsChatDeleteRequest {
    #[prost(message, repeated, tag = "1")]
    pub headers: ::prost::alloc::vec::Vec<MetadataMap>,
    #[prost(message, optional, tag = "2")]
    pub data: ::core::option::Option<ChatDeleteRequest>,
}
/// Schema for UserProfileMin
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserProfileMinArray {
    #[prost(message, repeated, tag = "1")]
    pub user_profiles: ::prost::alloc::vec::Vec<UserProfileMin>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserProfileMin {
    #[prost(int64, tag = "1")]
    pub id: i64,
    #[prost(int64, tag = "2")]
    pub access_hash: i64,
    #[prost(string, tag = "3")]
    pub username: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "4")]
    pub color: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "5")]
    pub photo_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "6")]
    pub emoji_status: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "7")]
    pub last_seen_date: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(bool, tag = "8")]
    pub is_deleted: bool,
    #[prost(bool, tag = "9")]
    pub is_scam: bool,
    #[prost(bool, tag = "10")]
    pub is_fake: bool,
    #[prost(bool, tag = "11")]
    pub is_banned: bool,
}
/// Inputs are the request messages/objects that the client sends to the server as part of the service call requests.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserProfilesMinRequest {
    #[prost(int64, repeated, tag = "1")]
    pub user_ids: ::prost::alloc::vec::Vec<i64>,
}
/// ********** Get UserProfilesMin ********** //
/// Backend NATS messaging for `Chat_Create`: Request to retrieve a list of user profiles.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NatsGetUserProfilesMinRequest {
    #[prost(message, repeated, tag = "1")]
    pub headers: ::prost::alloc::vec::Vec<MetadataMap>,
    #[prost(message, optional, tag = "2")]
    pub data: ::core::option::Option<UserProfilesMinRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NatsGetUserProfilesMinResponse {
    #[prost(oneof = "nats_get_user_profiles_min_response::Msg", tags = "1, 2")]
    pub msg: ::core::option::Option<nats_get_user_profiles_min_response::Msg>,
}
/// Nested message and enum types in `NatsGetUserProfilesMinResponse`.
pub mod nats_get_user_profiles_min_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Msg {
        #[prost(message, tag = "1")]
        Data(super::UserProfileMinArray),
        #[prost(message, tag = "2")]
        Error(super::ErrorReply),
    }
}
/// ********** ChatGroup_Create ********** //
/// Backend NATS messaging for `ChatGroup_Create`: Request to create a new chat group
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NatsChatGroupCreateRequest {
    #[prost(message, repeated, tag = "1")]
    pub headers: ::prost::alloc::vec::Vec<MetadataMap>,
    #[prost(message, optional, tag = "2")]
    pub data: ::core::option::Option<ChatGroupCreateRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NatsChatGroupCreateResponse {
    #[prost(oneof = "nats_chat_group_create_response::Msg", tags = "1, 2")]
    pub msg: ::core::option::Option<nats_chat_group_create_response::Msg>,
}
/// Nested message and enum types in `NatsChatGroupCreateResponse`.
pub mod nats_chat_group_create_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Msg {
        #[prost(message, tag = "1")]
        Data(super::ChatGroup),
        #[prost(message, tag = "2")]
        Error(super::ErrorReply),
    }
}
/// ********** ChatGroup_UpdateTitle ********** //
/// Backend NATS messaging for `ChatGroup_UpdateTitle` and `ChatGroup_UpdateAbout`: Requests to update the title or about of a chat group
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NatsChatGroupUpdateTitleRequest {
    #[prost(message, repeated, tag = "1")]
    pub headers: ::prost::alloc::vec::Vec<MetadataMap>,
    #[prost(message, optional, tag = "2")]
    pub data: ::core::option::Option<ChatGroupUpdateTitleRequest>,
}
/// ********** ChatGroup_UpdateAbout ********** //
/// Backend NATS messaging for `ChatGroup_UpdateTitle` and `ChatGroup_UpdateAbout`: Requests to update the title or about of a chat group
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NatsChatGroupUpdateAboutRequest {
    #[prost(message, repeated, tag = "1")]
    pub headers: ::prost::alloc::vec::Vec<MetadataMap>,
    #[prost(message, optional, tag = "2")]
    pub data: ::core::option::Option<ChatGroupUpdateAboutRequest>,
}
/// ********** ChatGroup_AddUser ********** //
/// Backend NATS messaging for `ChatGroup_AddUser`: Request to add a user from a chat group
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NatsChatGroupAddUserRequest {
    #[prost(message, repeated, tag = "1")]
    pub headers: ::prost::alloc::vec::Vec<MetadataMap>,
    #[prost(message, optional, tag = "2")]
    pub data: ::core::option::Option<ChatGroupAddUserRequest>,
}
/// ********** ChatGroup_RemoveUser ********** //
/// Backend NATS messaging for `ChatGroup_RemoveUser`: Request to remove a user from a chat group
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NatsChatGroupRemoveUserRequest {
    #[prost(message, repeated, tag = "1")]
    pub headers: ::prost::alloc::vec::Vec<MetadataMap>,
    #[prost(message, optional, tag = "2")]
    pub data: ::core::option::Option<ChatGroupRemoveUserRequest>,
}
/// ********** ChatGroup_EditAdmin ********** //
/// Backend NATS messaging for `ChatGroup_EditAdmin`: Request to set/unset a chat participant as admin
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NatsChatGroupEditAdminRequest {
    #[prost(message, repeated, tag = "1")]
    pub headers: ::prost::alloc::vec::Vec<MetadataMap>,
    #[prost(message, optional, tag = "2")]
    pub data: ::core::option::Option<ChatGroupEditAdminRequest>,
}
/// ********** ChatGroup_Delete ********** //
/// Backend NATS messaging for `ChatGroup_Delete`: Deletes a chat group
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NatsChatGroupDeleteRequest {
    #[prost(message, repeated, tag = "1")]
    pub headers: ::prost::alloc::vec::Vec<MetadataMap>,
    #[prost(message, optional, tag = "2")]
    pub data: ::core::option::Option<ChatGroupDeleteRequest>,
}
/// Schema for Message
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Message {
    #[prost(int64, tag = "1")]
    pub id: i64,
    #[prost(int64, tag = "2")]
    pub peer_id: i64,
    #[prost(int64, tag = "3")]
    pub user_id: i64,
    #[prost(int64, tag = "4")]
    pub replyto_message_id: i64,
    #[prost(string, tag = "5")]
    pub message: ::prost::alloc::string::String,
}
/// ********** Message_Send ********** //
/// Backend NATS messaging for `Message_Send`: Request to send a new message is a specified chat
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NatsMessageSendRequest {
    #[prost(message, repeated, tag = "1")]
    pub headers: ::prost::alloc::vec::Vec<MetadataMap>,
    #[prost(message, optional, tag = "2")]
    pub data: ::core::option::Option<MessageSendRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NatsEmptyResponse {
    #[prost(oneof = "nats_empty_response::Msg", tags = "1, 2")]
    pub msg: ::core::option::Option<nats_empty_response::Msg>,
}
/// Nested message and enum types in `NatsEmptyResponse`.
pub mod nats_empty_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Msg {
        #[prost(bool, tag = "1")]
        Success(bool),
        #[prost(message, tag = "2")]
        Error(super::ErrorReply),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserHeartbeatRequest {
    #[prost(bool, tag = "1")]
    pub is_online: bool,
    #[prost(message, optional, tag = "2")]
    pub date: ::core::option::Option<::prost_types::Timestamp>,
}
/// Inputs are the request messages/objects that the client sends to the server as part of the service call requests.
/// Request Message for `Chat_Create` RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserCreateRequest {}
/// Request Message for `Chat_Delete` RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserDeleteRequest {
    #[prost(int64, tag = "1")]
    pub id: i64,
}
/// Schema for User
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct User {
    #[prost(int64, tag = "1")]
    pub id: i64,
    /// Date when user was created
    #[prost(message, optional, tag = "6")]
    pub date: ::core::option::Option<::prost_types::Timestamp>,
}
/// ********** User_Hearbeat ********** //
/// Backend NATS messaging for persisting user hearbeat
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NatsUserHeartbeatRequest {
    #[prost(message, repeated, tag = "1")]
    pub headers: ::prost::alloc::vec::Vec<MetadataMap>,
    #[prost(message, optional, tag = "2")]
    pub data: ::core::option::Option<UserHeartbeatRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NatsUserCreateResponse {
    #[prost(oneof = "nats_user_create_response::Msg", tags = "1, 2")]
    pub msg: ::core::option::Option<nats_user_create_response::Msg>,
}
/// Nested message and enum types in `NatsUserCreateResponse`.
pub mod nats_user_create_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Msg {
        #[prost(message, tag = "1")]
        Data(super::User),
        #[prost(message, tag = "2")]
        Error(super::ErrorReply),
    }
}
/// ********** User_Create ********** //
/// Backend NATS messaging for `User_Create`: Request to create a new chat
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NatsUserCreateRequest {
    #[prost(message, repeated, tag = "1")]
    pub headers: ::prost::alloc::vec::Vec<MetadataMap>,
    #[prost(message, optional, tag = "2")]
    pub data: ::core::option::Option<UserCreateRequest>,
}
/// ********** User_Delete ********** //
/// Backend NATS messaging for `User_Delete`: Deletes a chat
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NatsUserDeleteRequest {
    #[prost(message, repeated, tag = "1")]
    pub headers: ::prost::alloc::vec::Vec<MetadataMap>,
    #[prost(message, optional, tag = "2")]
    pub data: ::core::option::Option<UserDeleteRequest>,
}
