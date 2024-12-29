use fetcher::{fetch, Method, Request, Url};
use crate::Session;

/// Retrieve the CAS URL from the instance URL.
///
/// You'll need to implement the CAS authentication for the provided URL yourself.
///
/// After this, you can call the [`process_cas_ticket`] function to
/// authenticate in Scodoc Notes.
#[cfg_attr(feature = "ffi", uniffi::export)]
#[cfg_attr(target_arch = "wasm32", wasm::api_method(retrieveCasUrl))]
pub async fn retrieve_cas_url (session: &Session) -> Result<String, crate::Error> {
  let base = Url::parse(&session.instance_url()).unwrap();
  let url = base.join("/services/doAuth.php").unwrap();

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
