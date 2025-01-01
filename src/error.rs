use thiserror::Error;

#[derive(Error, Debug)]
#[cfg_attr(feature = "ffi", derive(uniffi::Error))]
#[cfg_attr(target_arch = "wasm32", derive(wasm::Error))]
pub enum Error {
  #[error("no redirection was made, make sure the instance URL is correct")]
  InvalidRedirection(),
  #[error("the given CAS ticket is invalid")]
  InvalidCasTicket(),
  #[error("session expired, you need to authenticate again")]
  ExpiredSession(),
  #[error("server replied with an error ({0})")]
  ServerError(String)
}
