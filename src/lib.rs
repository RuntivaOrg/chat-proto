#![deny(unsafe_code, unused_qualifications, trivial_casts)]
#![deny(clippy::all)]
// #![warn(clippy::pedantic)]

// Excluding rich_error from the build for now.
//pub mod rich_error;

pub use self::proto::*;
pub mod proto_ext;

#[allow(unused_qualifications)]
#[allow(clippy::all)] // Cannot really check the sanity of generated code :shrugs:
mod proto {
    pub mod chat {
        // proto package names here
        tonic::include_proto!("chat");
    }
    pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("chatserver");
}

pub fn pb_elapsed(t0: std::time::Instant, t1: std::time::Instant) -> prost_types::Duration {
    pb_duration(t1 - t0)
}

/// Converts a Rust Duration to a Protobuf Duration.
pub fn pb_duration(d: std::time::Duration) -> prost_types::Duration {
    let seconds = if d.as_secs() > std::i64::MAX as u64 {
        std::i64::MAX
    } else {
        d.as_secs() as i64
    };

    let nanos = if d.subsec_nanos() > std::i32::MAX as u32 {
        std::i32::MAX
    } else {
        d.subsec_nanos() as i32
    };

    ::prost_types::Duration { seconds, nanos }
}
