//! This file defines the different general responses the API
//! can sometime return depending on the context.
use serde::Deserialize;

/// When this is returned, we need to throw an error
/// saying the session expired !
///
/// This is because a redirection means we are
/// required to login again because of a
/// session expiration.
#[derive(Deserialize)]
pub struct RedirectResponse {
  #[allow(dead_code)]
  pub redirect: String,
}

#[derive(Deserialize)]
pub struct ErrorResponse {
  pub erreur: String,
}

