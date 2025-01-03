use super::{ErrorResponse, RedirectResponse};
use serde::Deserialize;
use serde_repr::{Deserialize_repr, Serialize_repr};
use strum_macros::Display;

#[derive(Deserialize)]
#[serde(untagged)]
pub enum UserStatusResponse {
  #[allow(dead_code)]
  Redirect(RedirectResponse),
  Error(ErrorResponse),
  Data {
    statut: UserStatus,
  },
}

/// Status of a user, you can get it
/// with the `get_user_status` function.
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Display)]
#[repr(u8)]
#[cfg_attr(feature = "ffi", derive(uniffi::Enum))]
#[cfg_attr(target_arch = "wasm32", wasm::export)]
pub enum UserStatus {
  /// INCONNU
  Unknown = 0,
  /// ETUDIANT
  Student = 10,
  /// PERSONNEL
  Staff = 20,
  /// ADMINISTRATEUR
  Admin = 30,
  /// SUPERADMINISTRATEUR
  SuperAdmin = 40,
}
