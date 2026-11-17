pub fn matches_filter(line_bytes: &[u8], filter_bytes: Option<&[u8]>) -> bool {
    match filter_bytes {
        Some(pattern) => line_bytes.windows(pattern.len()).any(|window| window == pattern),
        None => true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_matches_successfully() {
        let line = b"Nov  6 10:00:00 server-01 nginx[123]: ERROR Connection lost";
        let pattern = b"ERROR";
        assert!(matches_filter(line, Some(pattern)));
    }

    #[test]
    fn test_filter_does_not_match() {
        let line = b"Nov  6 10:01:00 server-01 nginx[123]: INFO System running fine";
        let pattern = b"ERROR";
        assert!(!matches_filter(line, Some(pattern)));
    }

    #[test]
    fn test_no_filter_provided() {
        let line = b"Nov  6 10:02:00 server-01 nginx[123]: DEBUG Some debug message";
        assert!(matches_filter(line, None));
    }

    #[test]
    fn test_case_sensitivity() {
        let line = b"Nov  6 10:03:00 server-01 nginx[123]: error lowercase error";
        let pattern = b"ERROR";
        assert!(!matches_filter(line, Some(pattern)));
    }
}
