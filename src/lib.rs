use std::string::String;
#[allow(unused_imports)]
use std::str::Utf8Error;

/// StringBuilder type for ergonometric construction of a [String].
///
/// Various kinds of string and byte sequences can be appended to it, using the "builder" pattern.
/// 
/// # Example
/// ```rust
/// use string_builder::StringBuilder;
///
/// let s2 = "ghi".to_string();
///
/// let mut s = StringBuilder::new()    // receiver must be declared `mut`  
///         .append("abc")
///         .append("def")              // takes &str naturally
///         .append(&s2)                // needs `&` for strings
///         .append(&format!("jkl"))    // ... and for format! results
///         .to_string();               // access constructed string via .to_string()
///
/// assert_eq!(s, "abcdefghijkl");
/// ```
///
/// Various ways to construct the builder:
/// ```rust
/// # use string_builder::StringBuilder;
/// let mut b = StringBuilder::new();           // Empty string, default capacity
/// let mut b2 = StringBuilder::from("abc");    // Start with known string
/// let mut b3 = StringBuilder::with_capacity(1000);    // Empty string, with estimate of ending size,
///                                                     // may avoid redundant reallocations
/// ```
///
/// Use builder pattern:
///
pub struct StringBuilder(String);

impl StringBuilder {
    /// Construct with empty string of default capacity
    pub fn new() -> Self {
        Self(String::new())
    }
    /// Construct with empty string, but your estimated capacity
    ///
    /// A good guess can reduce number of intermediate buffer allocations and data moves.
    pub fn with_capacity(size: usize) -> Self {
        Self(String::with_capacity(size))
    }
    /// Construct with non-empty initial value
    pub fn from(from: &str) -> Self {
        Self(from.to_string())
    }

    /// Append a [std::str] or `&`[sdt::string::String]
    pub fn append(mut self, from: &str) -> Self {
        self.0.push_str(from);
        self
    }

    /// Append an array of bytes.  Panics if [from] is not well-formed utf8.
    pub fn append_bytes(mut self, from: &[u8]) -> Self {
        let from_bytes = std::str::from_utf8(from).unwrap();
        self.0.push_str(from_bytes);
        self
    }

    /// Fallible method for appending bytes.
    /// 
    /// If panic's not your style, you can handle the potential [Utf8Error] 
    /// 
    /// ```rust
    /// use string_builder::StringBuilder;
    /// use std::error::Error;
    /// 
    /// fn my_fn() -> Result<(), Box<dyn Error>> {
    ///     let some_bytes = "Pelé".as_bytes(); // last char is actually 2 bytes 0xc3_a9
    ///     let s = StringBuilder::new()
    ///             .try_append_bytes(&some_bytes[0..=2])?
    ///             .try_append_bytes(&some_bytes[3..])?
    ///             .to_string();
    ///     assert_eq!(s, "Pelé");
    ///     Ok(())
    /// }
    /// ```
    pub fn try_append_bytes(mut self, from: &[u8]) -> Result<Self, std::str::Utf8Error> {
        let from_bytes = std::str::from_utf8(from)?;
        self.0.push_str(from_bytes);

        Ok(self)
    }

    /// Extract newly-built [String] at end of chain.
    pub fn to_string(self) -> String {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::Utf8Error;

    #[test]
    fn build_with_str_and_string() {
        let s = StringBuilder::new()
            .append("")
            .append(&"".to_string())
            .append("abc")
            .append(&format!("def"))
            .to_string();

        assert_eq!(s, "abcdef");
    }
 
    // helper to provide the same well-instrumented utf8 data to many tests
    fn byte_data() -> (&'static str, &'static [u8]) {
        let sample = "„Pelé hat alles verändert.";
        // utf8        „      P  e  l  é       h  a  t     a  l  l  e  s     v  e  r  ä    n  d  e  r  t  ."
        // decode      e2809e,50,65,6c,c3a9,20,68,61,74,20,61,6c,6c,65,73,20,76,65,72,c3a4,6e,64,65,72,74,2e
        // vec index   0 0 0  0  0  0  0 0  0  0  1  1  1  1  1  1  1  1  1  1  2  2  2 2  2  2  2  2  2  2
        //             0 1 2  3  4  5  6 7  8  9  0  1  2  3  4  5  6  7  8  9  0  1  2 3  4  5  6  7  8  9
        // decode courtesy of https://planetcalc.com/9029/

        let sample_bytes = sample.as_bytes();

        (sample, sample_bytes)
    }
    #[test]
    fn build_with_bytes() {
        
        let (sample, sample_bytes) = byte_data();

        let mut vec = Vec::<u8>::new();
        vec.extend_from_slice(&sample_bytes[18..=29]);

        let s = StringBuilder::new()
            .append_bytes(&sample_bytes[0..9])
            .append_bytes(b"hat alles")
            .append_bytes(&vec)
            .to_string();

        assert_eq!(s, sample, "all append_bytes");

        let s2 = StringBuilder::new()
            .append(sample)
            .append_bytes(&sample_bytes[0..9])
            .append_bytes(b"hat alles")
            .append_bytes(&vec)
            .to_string();

        assert_eq!(
            s2,
            sample.to_string() + sample,
            "mixed append() append_bytes()"
        )
    }

    #[test]
    fn build_with_try_append_bytes() -> Result<(), Utf8Error> {
        
        let (sample, sample_bytes) = byte_data();

        let mut vec = Vec::<u8>::new();
        vec.extend_from_slice(&sample_bytes[18..=29]);

        let s = StringBuilder::new()
            .try_append_bytes(&sample_bytes[0..9])?
            .try_append_bytes(b"hat alles")?
            .try_append_bytes(&vec)?
            .to_string();

        assert_eq!(s, sample, "all append_bytes");

        Ok(())
    }
    #[test]
    #[should_panic(expected = "Utf8Error")]
    fn append_bytes_panic() {
        let (_sample, sample_bytes) = byte_data();

        let _s = StringBuilder::new()
            .append_bytes(&sample_bytes[0..=6])
            .append_bytes(&sample_bytes[7..9])
            .to_string();

        assert!(false, "test failed to panic");
    }

    #[test]
    fn try_append_bytes_utf8_error() {
        let (_sample, sample_bytes) = byte_data();

        fn inner(sample_bytes: &[u8]) -> Result<(), Utf8Error> {
            let _s = StringBuilder::new()
                .try_append_bytes(&sample_bytes[0..=6])?
                .try_append_bytes(&sample_bytes[7..9])?
                .to_string();

            Ok(())
        }

        let result = inner(sample_bytes);

        let expected_error_string = "incomplete utf-8 byte sequence";

        if let Err(e) = result {
            assert!(e.to_string().contains(expected_error_string));
        } else {
            panic!("Result was: {:?}, did not contain expected: {}", result, expected_error_string);
        };
    }

    
}
