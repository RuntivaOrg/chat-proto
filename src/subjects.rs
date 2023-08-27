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
