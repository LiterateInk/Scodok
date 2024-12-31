use fetcher::{fetch, Method, Request, Url};
use crate::{Session, Error};

#[cfg_attr(feature = "ffi", uniffi::export)]
#[cfg_attr(target_arch = "wasm32", wasm::append_fetcher, wasm::export)]
pub async fn get_session_from_token (instance_url: &str, token: &str) -> Result<(), Error> {
  // TODO: find a supported account...
  Ok(())
}
