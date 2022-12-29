use std::string::String;

/// StringBuilder type for ergonometric string construction
/// 
/// Various kinds of sequences can be appended to it, in a chain, then access the built String
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
///         .0;                         // access constructed string via `.0`
/// 
/// assert_eq!(s, "abcdefghijkl");
/// ```
/// 
/// Various ways to construct the builder:
/// ```rust
/// # use string_builder::StringBuilder;
/// let mut b = StringBuilder::new();   // Empty string, default capacity
/// let mut b2 = StringBuilder::from("abc");    // Start with known string
/// let mut b3 = StringBuilder::with_capacity(1000);    // Empty string, with estimate of ending size,
///                                                     // may avoid redundant reallocations
/// ```
/// 
/// Use builder pattern:
/// 
pub struct StringBuilder (pub String);

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
    /// Construct with pre-initialized value 
    pub fn from(from: &str) -> Self {
        Self(from.to_string())
    }

    /// Append a [std::str] or `&`[sdt::string::String]
    pub fn append(mut self, from: &str) -> Self {
        self.0.push_str(from);
        self
    }

    /// Infallible method for dealing with utf8 -- panics if [from] is actually malformed.
    pub fn append_bytes(mut self, from: &[u8]) -> Self {
        let from_bytes = std::str::from_utf8(from).unwrap();
        self.0.push_str(from_bytes);
        self
    }

    /// Fallible method for dealing with potentially mal-formed utf8 strings
    pub fn try_append_bytes(mut self, from: &[u8]) -> Result<Self, std::str::Utf8Error> {

        Ok(self)
    }


}
#[cfg(test)]
mod tests {
    use super::*;

}
