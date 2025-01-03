use crate::{Error, Session};
use fetcher::{fetch, Method, Request, Url};

/// Retrieve the CAS URL from the instance URL.
#[cfg_attr(feature = "ffi", uniffi::export)]
#[cfg_attr(target_arch = "wasm32", wasm::append_fetcher, wasm::export)]
pub async fn get_cas_url(instance_url: &str) -> Result<String, Error> {
  let mut url = Url::parse(instance_url).unwrap();
  url.set_path("/services/doAuth.php");

  let request = Request {
    url,
    method: Method::GET,
    headers: Default::default(),
    follow: false,
    body: None,
  };

  #[cfg(target_arch = "wasm32")]
  let response = fetch(request, &fetcher).await?;

  #[cfg(not(target_arch = "wasm32"))]
  let response = fetch(request).await?;

  let headers = response.headers;
  let location = headers.get("location").ok_or(Error::InvalidRedirection())?;

  Ok(location.to_str().unwrap().into())
}

/// Process the CAS ticket to get the session.
#[cfg_attr(feature = "ffi", uniffi::export)]
#[cfg_attr(target_arch = "wasm32", wasm::append_fetcher, wasm::export)]
pub async fn get_session_from_cas_ticket(
  instance_url: &str,
  ticket: &str,
) -> Result<Session, Error> {
  let mut url = Url::parse(instance_url).unwrap();
  url.set_path("/services/doAuth.php");
  url
    .query_pairs_mut()
    .clear()
    .append_pair("href", instance_url)
    .append_pair("ticket", ticket);

  let request = Request {
    url,
    method: Method::GET,
    headers: Default::default(),
    follow: false,
    body: None,
  };

  #[cfg(target_arch = "wasm32")]
  let response = fetch(request, &fetcher).await?;

  #[cfg(not(target_arch = "wasm32"))]
  let response = fetch(request).await?;

  if response.status != 302 {
    return Err(Error::InvalidCasTicket());
  }

  let headers = response.headers;
  let set_cookie = headers.get("set-cookie").ok_or(Error::InvalidCasTicket())?;
  let cookie = set_cookie.to_str().unwrap().split(';').next().unwrap();
  let php_sess_id = cookie.split('=').last().unwrap();

  #[cfg(target_arch = "wasm32")]
  let session = Session::new(instance_url, php_sess_id, fetcher);

  #[cfg(not(target_arch = "wasm32"))]
  let session = Session::new(instance_url, php_sess_id);

  Ok(session)
}
