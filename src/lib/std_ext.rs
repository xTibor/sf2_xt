pub mod str {
    #[derive(Debug)]
    pub struct AsciiError;

    impl std::fmt::Display for AsciiError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ASCII error")
        }
    }

    impl std::error::Error for AsciiError {}

    pub fn from_ascii(v: &[u8]) -> Result<&str, AsciiError> {
        if v.iter().all(u8::is_ascii) {
            Ok(unsafe { std::mem::transmute(v) })
        } else {
            Err(AsciiError)
        }
    }
}
