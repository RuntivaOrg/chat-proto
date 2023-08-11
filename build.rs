use std::{env, error::Error, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let iface_files = &[
        "proto/chat_svc.proto",
        "proto/error_reply.proto",
        "proto/channel/channel.proto",
        "proto/channel/channel_inputs.proto",
        "proto/channel/channel_nats.proto",
        "proto/channel/channel_shared.proto",
        "proto/chat/chat.proto",
        "proto/chat/chat_inputs.proto",
        "proto/chat/chat_nats.proto",
        "proto/userprofile/userprofile.proto",
        "proto/userprofile/userprofile_inputs.proto",
        "proto/userprofile/userprofile_nats.proto",
        "proto/chatgroup.proto",
        "proto/chatgroup_inputs.proto",
        "proto/chatgroup_nats.proto",
        "proto/message.proto",
        "proto/message_inputs.proto",
        "proto/message_nats.proto",
        "proto/nats_emptyresponse.proto",
        "proto/common_peer.proto",
        "proto/common_entities.proto",
        // Users
        "proto/user/user_inputs.proto",
        "proto/user/user_nats.proto",
        "proto/user/user.proto",
        // UpdatesService
        "proto/updates_svc.proto",
        "proto/updates/updates_inputs.proto",
        "proto/updates/updates_nats.proto",
        "proto/updates/nats_emptyresponse.proto",
        "proto/updates/nats_metadata.proto",
        "proto/updates/error_reply.proto",
        // rich error messages
        //"proto/status.proto",
        //"proto/error_details.proto",
    ];
    let dirs = &["proto"];

    tonic_build::configure()
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
