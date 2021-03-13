pub fn do_search_std_str_find(texts: &[&str], pattern: &str) -> anyhow::Result<usize> {
    let mut found: usize = 0;
    for line in texts {
        if let Some(_n) = line.find(pattern) {
            found += 1;
        }
    }
    Ok(found)
}

pub fn do_search_libc_memmem(texts: &[&str], pattern: &str) -> anyhow::Result<usize> {
    use libc::c_void;
    use libc::memmem;
    let needle_ptr = pattern.as_bytes().as_ptr() as *const c_void;
    let needle_len = pattern.as_bytes().len();
    let mut found: usize = 0;
    for line in texts {
        let haystack = line.as_bytes();
        let haystack_ptr = haystack.as_ptr() as *const c_void;
        let haystack_len = haystack.len();
        unsafe {
            let p = memmem(haystack_ptr, haystack_len, needle_ptr, needle_len);
            if !p.is_null() {
                found += 1;
            }
        }
    }
    Ok(found)
}

pub fn do_search_twoway_find_str(texts: &[&str], pattern: &str) -> anyhow::Result<usize> {
    let mut found: usize = 0;
    for line in texts {
        if let Some(_n) = twoway::find_str(line, pattern) {
            found += 1;
        }
    }
    Ok(found)
}

pub fn do_search_twowaysearcher_search_in(
    texts: &[&str],
    pat: &memmem::TwoWaySearcher,
) -> anyhow::Result<usize> {
    use memmem::Searcher;
    let mut found: usize = 0;
    for line in texts {
        if let Some(_n) = pat.search_in(line.as_bytes()) {
            found += 1;
        }
    }
    Ok(found)
}

pub fn do_search_ahocorasick_find(
    texts: &[&str],
    pat: &aho_corasick::AhoCorasick,
) -> anyhow::Result<usize> {
    let mut found: usize = 0;
    for line in texts {
        if let Some(_n) = pat.find(line) {
            found += 1;
        }
    }
    Ok(found)
}

mod naive_classic;
pub fn do_search_my_naive_classic(texts: &[&str], pattern: &str) -> anyhow::Result<usize> {
    use naive_classic::my_naive_classic;
    let mut found: usize = 0;
    for line in texts {
        if let Some(_n) = my_naive_classic(line, pattern) {
            found += 1;
        }
    }
    Ok(found)
}

mod naive_opt_1st;
pub fn do_search_my_naive_opt_1st(texts: &[&str], pattern: &str) -> anyhow::Result<usize> {
    use naive_opt_1st::my_naive_opt_1st;
    let mut found: usize = 0;
    for line in texts {
        if let Some(_n) = my_naive_opt_1st(line, pattern) {
            found += 1;
        }
    }
    Ok(found)
}

mod naive_opt_last;
pub fn do_search_my_naive_opt_last(texts: &[&str], pattern: &str) -> anyhow::Result<usize> {
    use naive_opt_last::my_naive_opt_last;
    let mut found: usize = 0;
    for line in texts {
        if let Some(_n) = my_naive_opt_last(line, pattern) {
            found += 1;
        }
    }
    Ok(found)
}

pub fn do_search_memchr(texts: &[&str], pattern: &str) -> anyhow::Result<usize> {
    let mut found: usize = 0;
    for line in texts {
        if let Some(_n) = memchr_find_str(line, pattern) {
            found += 1;
        }
    }
    Ok(found)
}

fn memchr_find_str(haystack: &str, needle: &str) -> Option<usize> {
    let hay_bytes = haystack.as_bytes();
    let pat_bytes = needle.as_bytes();
    let hay_len = hay_bytes.len();
    let pat_len = pat_bytes.len();
    //
    // utf8 byte sequence:
    //     1 bytes: 7F
    //     2 bytes: DF BF
    //     3 bytes: EF BF BF
    //     4 bytes: F4 8F BF BF
    // 1st byte (of 2..4 bytes seq) is likely to be repeated.
    // I think it is stochastically effective to use the last byte.
    //
    let last_idx = pat_len - 1;
    let last_byte = pat_bytes[last_idx];
    if hay_len <= last_idx {
        return None;
    }
    for m in memchr::memchr_iter(last_byte, &hay_bytes[last_idx..]) {
        let st = m;
        let ed = st + pat_len;
        if ed >= hay_len {
            break;
        }
        if pat_bytes == &hay_bytes[st..ed] {
            return Some(st);
        }
    }
    None
}
