#![deny(unsafe_code, unused_qualifications, trivial_casts)]
#![deny(clippy::all)]
// #![warn(clippy::pedantic)]

mod error;
pub use error::ChatProtoError;

pub mod messages;
pub mod proto_ext;
pub mod subjects;

pub use proto::*;

#[allow(unused_qualifications)]
#[allow(clippy::all)] // generated code - no need to clippy check
mod proto {

    // The order of these appear to be important. When common is moved up, there are compile errors

    pub mod updates_stream {
        tonic::include_proto!("updates_stream");
    }
    pub mod runtiva {
        // Primary chat service -- calls made by clients to chat-server
        pub mod chat {
            pub mod v1 {
                tonic::include_proto!("runtiva.chat.v1");
            }
        }

        // NATS calls - primarily made by chat-server to other services initiated from chat-svc related requests
        pub mod nats {
            pub mod v1 {
                tonic::include_proto!("runtiva.nats.v1");
            }
        }

        // Events raised by the chat-persist service once an incoming request has been persisted,
        // these messages are the external events that are created from the internal domain events
        pub mod persist {
            pub mod v1 {
                tonic::include_proto!("runtiva.persist.v1");
            }
        }

        // Primary user service -- calls made by clients to user-server?? as well as NATS user-related requests
        pub mod user {
            pub mod v1 {
                tonic::include_proto!("runtiva.user.v1");
            }
        }

        // Shared common messages/models
        pub mod common {
            pub mod v1 {
                tonic::include_proto!("runtiva.common.v1");
            }
        }
    }

    pub const CHATSERVER_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("chatserver");
}

pub use CHATSERVER_DESCRIPTOR_SET;

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
