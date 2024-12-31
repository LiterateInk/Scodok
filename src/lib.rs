#[cfg(target_arch = "wasm32")]
wasm::setup_allocator!();

#[cfg(feature = "ffi")]
uniffi::setup_scaffolding!();

mod error;
pub use error::Error;

mod cas;
pub use cas::{get_cas_url, get_session_from_cas_ticket};

mod token;
pub use token::get_session_from_token;

mod data;
pub use data::get_profile_picture_bytes;

mod session;
pub use session::Session;
