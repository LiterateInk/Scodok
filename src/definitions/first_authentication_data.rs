use super::{Config, ErrorResponse, RedirectResponse, Semester, Summary, UserStatus};
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
    relevé: Summary,
  },
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct FirstAuthenticationDataAuth {
  pub session: String,
  pub name: String, // "Doe John"
  pub statut: UserStatus,
}
