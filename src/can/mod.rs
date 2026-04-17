pub mod can_backend;
pub mod mux_context;
pub mod parsers;
pub mod util;

pub mod emulators {
    include!(concat!(env!("OUT_DIR"), "/proj_gen/emulators/mod.rs"));
}

pub mod messages {
    include!(concat!(env!("OUT_DIR"), "/proj_gen/can/messages/mod.rs"));
}
