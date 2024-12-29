#[cfg_attr(feature = "ffi", derive(uniffi::Object))]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
pub struct Session {
  instance_url: String
}

#[cfg_attr(feature = "ffi", uniffi::export)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
impl Session {
  #[cfg_attr(feature = "ffi", uniffi::constructor)]
  #[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen(constructor))]
  pub fn new(instance_url: &str) -> Self {
    Self {
      instance_url: instance_url.to_string()
    }
  }

  #[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen(getter = instanceURL))]
  pub fn instance_url(&self) -> String {
    self.instance_url.clone()
  }
}
