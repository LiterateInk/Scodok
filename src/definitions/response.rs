//! This file defines the different general responses the API
//! can sometime return depending on the context.
use serde::Deserialize;

/// When this is returned, we need to throw an error
/// saying the session expired.
///
/// This is because a redirection means we are
/// required to login again because of a
/// session expiration.
#[derive(Deserialize)]
// We'll never read the properties
// of this because we know it's an expired session.
#[allow(dead_code)]
pub struct RedirectResponse {
  pub redirect: String,
}

/// When this is returned, we need to throw a
/// server error with the message provided.
///
/// It's kinda similar to a "Bad Request" HTTP error.
#[derive(Deserialize)]
pub struct ErrorResponse {
  pub erreur: String,
}
