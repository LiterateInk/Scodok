#[derive(Debug, Clone)]
#[cfg_attr(feature = "ffi", derive(uniffi::Object))]
#[cfg_attr(target_arch = "wasm32", wasm::export)]
pub struct Session {
  instance_url: String,
  php_sess_id: String,
}

#[cfg_attr(feature = "ffi", uniffi::export)]
#[cfg_attr(target_arch = "wasm32", wasm::export)]
impl Session {
  #[cfg_attr(feature = "ffi", uniffi::constructor)]
  #[cfg_attr(target_arch = "wasm32", wasm_bindgen(constructor))]
  pub fn new(instance_url: &str, php_sess_id: &str) -> Self {
    Self {
      instance_url: instance_url.to_string(),
      php_sess_id: php_sess_id.to_string(),
    }
  }

  #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter = instanceUrl))]
  pub fn instance_url(&self) -> String {
    self.instance_url.clone()
  }

  #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter = phpSessId))]
  pub fn php_sess_id(&self) -> String {
    self.php_sess_id.clone()
  }
}
