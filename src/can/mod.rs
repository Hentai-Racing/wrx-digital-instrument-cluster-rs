pub mod can_backend;
pub mod can_mux_parser;
pub mod util;

pub mod emulators {
    include!(concat!(env!("OUT_DIR"), "/proj_gen/emulators/mod.rs"));
}

pub mod messages {
    include!(concat!(env!("OUT_DIR"), "/proj_gen/can/messages/mod.rs"));
}
