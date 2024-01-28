#[cfg(feature = "base64")]
#[derive(deref_derive::Deref)]
pub struct Base64Engine(base64::engine::GeneralPurpose);

#[cfg(feature = "base64")]
impl Default for Base64Engine {
    fn default() -> Self {
        Self(base64::engine::GeneralPurpose::new(
            &base64::alphabet::STANDARD,
            base64::engine::GeneralPurposeConfig::default(),
        ))
    }
}

#[cfg(feature = "base64")]
impl Base64Engine {
    pub fn decode_string(&self, encoded: &String) -> Option<String> {
        use base64::engine::Engine;
        String::from_utf8(self.decode(encoded).ok()?).ok()
    }
}
