//! This file contains requests that can be done over the
//! following endpoint : <https://github.com/SebL68/Scodoc_Notes/blob/main/html/services/data.php>
use fetcher::{fetch, Method, Request, Url};
use crate::{Session, Error, UserStatus, definitions};

/// # `getStudentPic`
///
/// Get the student picture from the ScoDoc instance using its NIP, or
/// current user if none is provided. Only users with a role above `STUDENT`
/// can access other students' pictures using NIP.
///
/// ## Image details
///
/// Will return a `image/jpeg` image, as a `.jpg` file if you're
/// a student, or if you requested a student picture.
///
/// If your role is above `STUDENT` and you request your own picture,
/// it will return an SVG string as `image/svg+xml`.
///
/// If you don't have any picture, it will also return an SVG string
/// as `image/svg+xml`.
///
#[cfg_attr(feature = "ffi", uniffi::export)]
#[cfg_attr(target_arch = "wasm32", wasm::export)]
pub async fn get_profile_picture_bytes (session: &Session, nip: Option<String>) -> Result<Vec<u8>, Error> {
  let mut url = Url::parse(session.instance_url()).unwrap();
  url.set_path("/services/data.php");
  url.query_pairs_mut().clear()
    .append_pair("q", "getStudentPic")
    .append_pair("nip", nip.as_deref().unwrap_or(""));

  let request = Request {
    url,
    method: Method::GET,
    headers: session.get_headers(),
    follow: false,
    body: None,
  };

  let response = fetch!(request, session.fetcher());

  Ok(response.bytes)
}

/// # `getStatut`
///
/// Get the status of a user from a ScoDoc instance
/// using their email address.
///
#[cfg_attr(feature = "ffi", uniffi::export)]
#[cfg_attr(target_arch = "wasm32", wasm::export)]
pub async fn get_user_status (session: &Session, email: String) -> Result<UserStatus, Error> {
  let mut url = Url::parse(session.instance_url()).unwrap();
  url.set_path("/services/data.php");
  url.query_pairs_mut().clear()
    .append_pair("q", "getStatut")
    .append_pair("user", &email);

  let request = Request {
    url,
    method: Method::GET,
    headers: session.get_headers(),
    follow: false,
    body: None,
  };

  #[cfg(target_arch = "wasm32")]
  let response = fetch(request, session.fetcher()).await;
  #[cfg(not(target_arch = "wasm32"))]
  let response = fetch(request).await;

  use definitions::UserStatusResponse;
  let response: UserStatusResponse = serde_json::from_str(&response.text()).unwrap();

  match response {
    UserStatusResponse::Redirect(_) => Err(Error::ExpiredSession()),
    UserStatusResponse::Error(response) => Err(Error::ServerError(response.erreur)),
    UserStatusResponse::Data { statut } => Ok(statut)
  }
}
