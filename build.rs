use std::{env, error::Error, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let iface_files = &[
        "proto/chat_svc.proto",
        "proto/error_reply.proto",
        "proto/channel.proto",
        "proto/channel_inputs.proto",
        "proto/channel_nats.proto",
        "proto/chatgroup.proto",
        "proto/chatgroup_inputs.proto",
        "proto/chatgroup_nats.proto",
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
