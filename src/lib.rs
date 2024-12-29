#[cfg(target_arch = "wasm32")]
wasm::setup_allocator!();

#[cfg(feature = "ffi")]
uniffi::setup_scaffolding!();

mod error;
pub use error::Error;

mod cas;
pub use cas::retrieve_cas_url;

mod session;
pub use session::Session;
