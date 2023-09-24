use std::{env, error::Error, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let iface_files = &[
        // *************************************************************************************
        // Chat Service
        "proto/chat_svc.proto",
        // Chats
        "proto/chat-svc/v1/chat/chat.proto",
        "proto/chat-svc/v1/chat/service_messages.proto",
        // Chatgroups
        "proto/chat-svc/v1/chatgroup/entities.proto",
        "proto/chat-svc/v1/chatgroup/service_messages.proto",
        // Chatgroups
        //"proto/updates/chatgroups/chatgroup.proto",
        // Channels
        "proto/chat-svc/v1/channel/entities.proto",
        "proto/chat-svc/v1/channel/service_messages.proto",
        "proto/chat-svc/v1/channel/channel_shared.proto",
        // Messages
        "proto/chat-svc/v1/message/message.proto",
        "proto/chat-svc/v1/message/message_inputs.proto",
        // *************************************************************************************
        // User Service
        // Users
        "proto/user-svc/v1/user/user_inputs.proto",
        "proto/user-svc/v1/user/user.proto",
        // UserProfiles
        "proto/user-svc/v1/userprofile/userprofile.proto",
        "proto/user-svc/v1/userprofile/userprofile_inputs.proto",
        // *************************************************************************************
        // NATS specific messages and entities
        "proto/nats/v1/common.proto",
        "proto/nats/v1/enums.proto",
        "proto/nats/v1/channel.proto",
        "proto/nats/v1/chat.proto",
        "proto/nats/v1/chatgroup.proto",
        "proto/nats/v1/message.proto",
        "proto/nats/v1/state.proto",
        "proto/nats/v1/user.proto",
        "proto/nats/v1/userprofile.proto",
        // *************************************************************************************
        // Common
        "proto/common/v1/common_peer.proto",
        "proto/common/v1/chatgroup.proto",
        "proto/common/v1/message_entity.proto",
        "proto/common/v1/notification_settings.proto",
        // *************************************************************************************
        // Persist Service
        "proto/persist-svc/v1/chatgroup_events.proto",
        // *************************************************************************************
        // Distro Messaging
        "proto/distro/v1/chatgroup.proto",
        // *************************************************************************************
        // Connections Service
        "proto/connections_svc.proto",
        "proto/connections/updates_inputs.proto",
        "proto/connections/updates_nats.proto",
        // *************************************************************************************
        // Updates Service
        "proto/updates_svc.proto",
    ];
    let dirs = &["proto"];

    tonic_build::configure()
        .protoc_arg("--experimental_allow_proto3_optional")
        .file_descriptor_set_path(out_dir.join("chatserver.bin"))
        .build_server(true)
        .build_client(true)
        .compile(iface_files, dirs)?;

    // recompile protobufs only if any of the proto files changes.
    for file in iface_files {
        println!("cargo:rerun-if-changed={}", file);
    }

    Ok(())
}
