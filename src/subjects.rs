// **************************************************
// **************** Updates Service *****************

// ****************** Connections *******************
pub const SUB_UPDATES_CONNECT: &str = "updates_connect"; // NatsUpdatesConnectRequest
pub const SUB_UPDATES_CLOSE: &str = "updates_close"; // NatsUpdatesCloseRequest
pub const SUB_UPDATES_REGISTER_ON_PEER_LIST: &str = "register_on_peer_list"; // NatsRegisterOnPeerListRequest
pub const SUB_UPDATES_UNREGISTER_FROM_PEER_LIST: &str = "register_from_peer_list"; // NatsUnregisterFromPeerListRequest
pub const SUB_UPDATES_REGISTER_ON_PEER: &str = "register_on_peer"; // NatsRegisterOnPeerRequest
pub const SUB_UPDATES_UNREGISTER_FROM_PEER: &str = "unregister_from_peer"; // NatsUnregisterFromPeerRequest

// Queries
pub const SUB_UPDATES_GET_CONNECTIONS: &str = "get_connections"; // NatsGetConnectionsRequest

// Utility functions
pub fn get_presence_connection_command(sub: &str) -> String {
    format!("presence.connection.command.{}", sub)
}

pub fn get_presence_connection_query(sub: &str) -> String {
    format!("presence.connection.query.{}", sub)
}

// **************************************************
// **************** Chat Service ********************

pub fn get_chat_channel_command(sub: &str) -> String {
    format!("chat.channel.command.{}", sub)
}

pub fn get_chat_channel_query(sub: &str) -> String {
    format!("chat.channel.query.{}", sub)
}

pub fn get_chat_chatgroup_command(sub: &str) -> String {
    format!("chat.chatgroup.command.{}", sub)
}

pub fn get_chat_chatgroup_query(sub: &str) -> String {
    format!("chat.chatgroup.query.{}", sub)
}

// ************* ChatGroup Subjects *****************
pub const SUB_CHATGROUP_CREATE: &str = "create"; // NatsChatGroupCreateRequest
pub const SUB_CHATGROUP_UPDATE_ABOUT: &str = "update_about"; // NatsChatGroupUpdateAboutRequest
pub const SUB_CHATGROUP_UPDATE_TITLE: &str = "update_title"; // NatsChatGroupUpdateTitleRequest
pub const SUB_CHATGROUP_UPDATE_ICON: &str = "update_icon"; // NatsChatGroupUpdateIconRequest
pub const SUB_CHATGROUP_UPDATE_AVAILABLE_REACTIONS: &str = "update_available_reactions"; // NatsChatGroupUpdateAvailableReactionsRequest
pub const SUB_CHATGROUP_UPDATE_SETTINGS: &str = "update_settings"; // NatsChatGroupUpdateSettingsRequest
pub const SUB_CHATGROUP_UPDATE_TTL: &str = "update_ttl"; // NatsChatGroupUpdateTTLRequest
pub const SUB_CHATGROUP_UPDATE_NOTIFICATION_SETTINGS: &str = "update_notification_settings"; // NatsChatGroupUpdateNotificationSettingsRequest
pub const SUB_CHATGROUP_ADD_USER: &str = "add_user"; // NatsChatGroupAddUserRequest
pub const SUB_CHATGROUP_REMOVE_USER: &str = "remove_user"; // NatsChatGroupRemoveUserRequest
pub const SUB_CHATGROUP_EDIT_ADMIN: &str = "edit_admin"; // NatsChatGroupEditAdminRequest
pub const SUB_CHATGROUP_TOGGLE_DISABLE: &str = "toggle_disable"; // NatsChatGroupToggleDisableRequest
pub const SUB_CHATGROUP_DELETE: &str = "delete"; // NatsChatGroupDeleteRequest
