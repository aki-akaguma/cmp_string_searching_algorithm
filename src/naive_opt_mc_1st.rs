pub fn my_naive_opt_mc_1st(haystack: &str, needle: &str) -> Option<usize> {
    let hay_bytes = haystack.as_bytes();
    let nee_bytes = needle.as_bytes();
    let hay_len = hay_bytes.len();
    let nee_len = nee_bytes.len();
    //
    if nee_len == 0 || hay_len < nee_len {
        return None;
    }
    let byte_1st = nee_bytes[0];
    for m in memchr::memchr_iter(byte_1st, hay_bytes) {
        let st = m;
        let ed = st + nee_len;
        if ed > hay_len {
            break;
        }
        if nee_bytes == &hay_bytes[st..ed] {
            return Some(st);
        }
    }
    None
}

#[cfg(test)]
mod test {
    use super::my_naive_opt_mc_1st;
    #[test]
    fn test_void_0() {
        let haystack = "";
        let needle = "";
        let r = my_naive_opt_mc_1st(haystack, needle);
        assert_eq!(r, None);
    }
    #[test]
    fn test_void_1() {
        let haystack = "1";
        let needle = "";
        let r = my_naive_opt_mc_1st(haystack, needle);
        assert_eq!(r, None);
    }
    #[test]
    fn test_none() {
        let haystack = "111 a 111b";
        let needle = "xxx";
        let r = my_naive_opt_mc_1st(haystack, needle);
        assert_eq!(r, None);
    }
    #[test]
    fn test_fit() {
        let haystack = "111 a 111b";
        let needle = "111 a 111b";
        let r = my_naive_opt_mc_1st(haystack, needle);
        assert_eq!(r, Some(0));
    }
    #[test]
    fn test_unfit() {
        let haystack = "111 a 111b";
        let needle = "111 a 1111";
        let r = my_naive_opt_mc_1st(haystack, needle);
        assert_eq!(r, None);
    }
    #[test]
    fn test_last() {
        let haystack = "111 a 111b";
        let needle = "b";
        let r = my_naive_opt_mc_1st(haystack, needle);
        assert_eq!(r, Some(9));
    }
    #[test]
    fn test_small_1() {
        let haystack = "111 a 111b";
        let needle = "a";
        let r = my_naive_opt_mc_1st(haystack, needle);
        assert_eq!(r, Some(4));
    }
    #[test]
    fn test_small_2() {
        let haystack = "111 a 111b";
        let needle = "a 111";
        let r = my_naive_opt_mc_1st(haystack, needle);
        assert_eq!(r, Some(4));
    }
}
