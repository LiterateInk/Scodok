use super::{Config, ErrorResponse, RedirectResponse, Semester, UserStatus};
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(untagged)]
pub enum FirstAuthenticationDataResponse {
  #[allow(dead_code)]
  Redirect(RedirectResponse),
  Error(ErrorResponse),
  Data {
    config: Box<Config>,
    auth: FirstAuthenticationDataAuth,
    semestres: Vec<Semester>,
  },
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct FirstAuthenticationDataAuth {
  pub session: String,
  pub name: String, // "Doe John"
  pub statut: UserStatus,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct FirstAuthenticationDataSummary {
  pub version: String, // "0"
  pub r#type: String,  // NOTE: "BUT", probably an `enum`
  pub date: String,    // "2025-01-01T02:50:07.364238+01:00"
  pub publie: bool,
  pub etat_inscription: String, // NOTE: "I", probably an `enum`
}
