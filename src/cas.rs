use fetcher::{fetch, Method, Request, Url};
use crate::Session;

/// Retrieve the CAS URL from the instance URL.
///
/// You'll need to implement the CAS authentication for the provided URL yourself.
///
/// After this, you can call the [`process_cas_ticket`] function to
/// authenticate in Scodoc Notes.
#[cfg_attr(feature = "ffi", uniffi::export)]
#[cfg_attr(target_arch = "wasm32", wasm::append_fetcher, wasm::export)]
pub async fn retrieve_cas_url (instance_url: &str) -> Result<String, crate::Error> {
  let mut url = Url::parse(instance_url).unwrap();
  url.set_path("/services/doAuth.php");

  let request = Request {
    url,
    method: Method::GET,
    headers: Default::default(),
    follow: Some(false),
    body: None,
  };

  let response = fetch!(request);
  let headers = response.headers();
  let location = headers.get("location").ok_or(crate::Error::InvalidRedirection())?;

  Ok(location.to_str().unwrap().into())
}

#[cfg_attr(feature = "ffi", uniffi::export)]
#[cfg_attr(target_arch = "wasm32", wasm::append_fetcher, wasm::export)]
pub async fn process_cas_ticket (instance_url: &str, ticket: &str) -> Result<Session, crate::Error> {
  let mut url = Url::parse(instance_url).unwrap();
  url.set_path("/services/doAuth.php");
  url.query_pairs_mut().clear()
    .append_pair("href", instance_url)
    .append_pair("ticket", ticket);

  let request = Request {
    url,
    method: Method::GET,
    headers: Default::default(),
    follow: Some(false),
    body: None,
  };

  let response = fetch!(request);

  if response.status != 302 {
    return Err(crate::Error::InvalidCasTicket());
  }

  let headers = response.headers();
  let set_cookie = headers.get("set-cookie").ok_or(crate::Error::InvalidCasTicket())?;
  let cookie = set_cookie.to_str().unwrap().split(';').next().unwrap();
  let php_sessid = cookie.split('=').last().unwrap();

  Ok(Session::new(instance_url, php_sessid))
}
