pub trait AliRequest {
    fn should_verify_signature(&self) -> bool {
        false
    }

    fn method(&self) -> &str;
}
