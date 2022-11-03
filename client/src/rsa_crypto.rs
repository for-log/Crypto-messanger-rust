use wasm_bindgen::JsValue;
use web_sys::CryptoKey;

#[derive(Clone, Debug)]
pub struct RsaCrypto {
    private_key: CryptoKey,
    public_key: CryptoKey,
    is_null: bool
}

impl RsaCrypto {
    pub fn new() -> Self {
        Self { private_key: JsValue::NULL.into(), public_key: JsValue::NULL.into(), is_null: true }
    }
    pub fn set_private(&mut self, value: CryptoKey) -> &mut Self {
        self.is_null = false;
        self.private_key = value;
        self
    }
    pub fn set_public(&mut self, value: CryptoKey) -> &mut Self {
        self.is_null = false;
        self.public_key = value;
        self
    }
    pub fn get_private(&self) -> Result<&CryptoKey, ()> {
        if self.is_null { Err(()) } else { Ok(&self.private_key) }
    }
    pub fn get_public(&self) -> Result<&CryptoKey, ()> {
        if self.is_null { Err(()) } else { Ok(&self.public_key) }
    }
}