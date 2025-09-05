#![no_std]

/// Returns a slice with the prefix removed.
///
/// If `data` starts with `prefix`, returns the remaining sequence.
///
/// If `data` does not start with this prefix, `None` is returned.
///
/// ```
/// use strip_slice_prefix::strip_prefix;
///
/// let auth_header = b"Basic VGhlIGFuc3dlciBpcyA0Mi4K";
/// let credentials = strip_prefix(auth_header, b"Basic ").unwrap();
/// assert_eq!(credentials, b"VGhlIGFuc3dlciBpcyA0Mi4K");
/// ```
pub fn strip_prefix<'a, T>(data: &'a [T], prefix: &[T]) -> Option<&'a [T]>
where
    T: Eq,
{
    let mut remaining = data;
    for prefix_char in prefix {
        if prefix_char == remaining.first()? {
            remaining = &remaining[1..];
        } else {
            return None;
        }
    }
    Some(remaining)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_suffix() {
        let result = strip_prefix(b"Hello", b"Hello").unwrap();
        assert_eq!(result, b"");
    }

    #[test]
    fn test_nonempty_suffix() {
        let result = strip_prefix(b"Hello world", b"Hello").unwrap();
        assert_eq!(result, b" world");
    }

    #[test]
    fn test_too_long_prefix() {
        let result = strip_prefix(b"Hell", b"Hello");
        assert!(result.is_none());
    }

    #[test]
    fn test_wrong_prefix() {
        let result = strip_prefix(b"Hello", b"world");
        assert!(result.is_none());
    }

    #[test]
    fn test_empty_prefix() {
        let result = strip_prefix(b"Hello", b"").unwrap();
        assert_eq!(result, b"Hello");
    }
}
