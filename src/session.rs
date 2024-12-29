#[derive(Debug, Clone)]
#[cfg_attr(feature = "ffi", derive(uniffi::Object))]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
pub struct Session {
  instance_url: String,
  php_sess_id: String
}

#[cfg_attr(feature = "ffi", uniffi::export)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
impl Session {
  /// When you need to reuse a logged in session, you can provide `php_sess_id`
  /// which corresponds to the `PHPSESSID` cookie value.
  ///
  /// You can obtain this value by calling the [`process_cas_ticket`] function.
  ///
  /// ## Examples
  ///
  /// ```rust
  /// let session = Session::new("https://scodoc.notes.local/", None);
  /// let cas_url = retrieve_cas_url(&session).await?; // You'll have to authenticate yourself to grab the ticket.
  /// let session = process_cas_ticket(&session, "ST-123456789-SOMELONGSTRING-vmjava-pcas2").await?;
  /// // You can now do authenticated requests with this session.
  ///
  /// let session = Session::new("https://scodoc.notes.local/", Some("some_php_sess_id".into()));
  /// // You can now do authenticated requests with this session.
  /// ```
  #[cfg_attr(feature = "ffi", uniffi::constructor)]
  #[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen(constructor))]
  pub fn new(instance_url: &str, php_sess_id: Option<String>) -> Self {
    Self {
      instance_url: instance_url.to_string(),
      php_sess_id: php_sess_id.unwrap_or_default()
    }
  }

  #[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen(getter = instanceURL))]
  pub fn instance_url(&self) -> String {
    self.instance_url.clone()
  }

  #[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen(getter = phpSessId))]
  pub fn php_sess_id(&self) -> String {
    self.php_sess_id.clone()
  }
}
