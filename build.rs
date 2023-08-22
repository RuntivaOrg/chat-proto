use std::{env, error::Error, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let iface_files = &[
        "proto/chat_svc.proto",
        // Channels
        "proto/chat-svc/v1/channel/entities.proto",
        "proto/chat-svc/v1/channel/service_messages.proto",
        "proto/chat-svc/v1/channel/channel_shared.proto",
        // Chats
        "proto/chat/v1/chat.proto",
        "proto/chat/v1/chat_inputs.proto",
        // Chatgroups
        "proto/chatgroup/v1/entities.proto",
        "proto/chatgroup/v1/service_messages.proto",
        // Messages
        "proto/message/v1/message.proto",
        "proto/message/v1/message_entity.proto",
        "proto/message/v1/message_inputs.proto",
        // NATS specific messages and entities
        "proto/nats/v1/common.proto",
        "proto/nats/v1/enums.proto",
        "proto/nats/v1/channel.proto",
        "proto/nats/v1/chat.proto",
        "proto/nats/v1/chatgroup.proto",
        "proto/nats/v1/message.proto",
        "proto/nats/v1/user.proto",
        // Common
        "proto/common_peer.proto",
        // Users
        "proto/user/user_inputs.proto",
        "proto/user/user.proto",
        // UserProfiles
        "proto/userprofile/userprofile.proto",
        "proto/userprofile/userprofile_inputs.proto",
        "proto/userprofile/userprofile_nats.proto",
        // UpdatesService
        "proto/updates_svc.proto",
        "proto/updates/updates_inputs.proto",
        "proto/updates/updates_nats.proto",
        // Chatgroups
        "proto/updates/chatgroups/chatgroup.proto",
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
