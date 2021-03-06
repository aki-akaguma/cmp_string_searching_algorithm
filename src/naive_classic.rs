/*
pub fn _my_naive_classic(haystack: &str, needle: &str) -> Option<usize> {
    let hay_len = haystack.len();
    let nee_len = needle.len();
    //
    if nee_len == 0 || hay_len < nee_len {
        return None;
    }
    let mut it = haystack.char_indices();
    while let Some((i,_c)) = it.next() {
        if i + nee_len > hay_len {
            break;
        }
        if &haystack[i..(i + nee_len)] == needle {
            return Some(i);
        }
    }
    None
}
*/

pub fn my_naive_classic(haystack: &str, needle: &str) -> Option<usize> {
    let hay_bytes = haystack.as_bytes();
    let nee_bytes = needle.as_bytes();
    let hay_len = hay_bytes.len();
    let nee_len = nee_bytes.len();
    //
    if nee_len == 0 || hay_len < nee_len {
        return None;
    }
    for i in 0..(hay_len - nee_len + 1) {
        if &hay_bytes[i..(i + nee_len)] == nee_bytes {
            return Some(i);
        }
    }
    None
}

#[cfg(test)]
mod test {
    use super::my_naive_classic;
    #[test]
    fn test_void_0() {
        let haystack = "";
        let needle = "";
        let r = my_naive_classic(haystack, needle);
        assert_eq!(r, None);
    }
    #[test]
    fn test_void_1() {
        let haystack = "1";
        let needle = "";
        let r = my_naive_classic(haystack, needle);
        assert_eq!(r, None);
    }
    #[test]
    fn test_none() {
        let haystack = "111 a 111b";
        let needle = "xxx";
        let r = my_naive_classic(haystack, needle);
        assert_eq!(r, None);
    }
    #[test]
    fn test_fit() {
        let haystack = "111 a 111b";
        let needle = "111 a 111b";
        let r = my_naive_classic(haystack, needle);
        assert_eq!(r, Some(0));
    }
    #[test]
    fn test_unfit() {
        let haystack = "111 a 111b";
        let needle = "111 a 1111";
        let r = my_naive_classic(haystack, needle);
        assert_eq!(r, None);
    }
    #[test]
    fn test_last() {
        let haystack = "111 a 111b";
        let needle = "b";
        let r = my_naive_classic(haystack, needle);
        assert_eq!(r, Some(9));
    }
    #[test]
    fn test_small_1() {
        let haystack = "111 a 111b";
        let needle = "a";
        let r = my_naive_classic(haystack, needle);
        assert_eq!(r, Some(4));
    }
    #[test]
    fn test_small_2() {
        let haystack = "111 a 111b";
        let needle = "a 111";
        let r = my_naive_classic(haystack, needle);
        assert_eq!(r, Some(4));
    }
}
