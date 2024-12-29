use thiserror::Error;

#[derive(Error, Debug)]
#[cfg_attr(feature = "ffi", derive(uniffi::Error))]
#[cfg_attr(target_arch = "wasm32", derive(wasm::Error))]
pub enum Error {
  #[error("no redirection was made, make sure the instance URL is correct")]
  InvalidRedirection(),
}
