# cmp_string_match
research: comparing string match of rust

rustc 1.50.0 (cb75ad5db 2021-02-10)

|         `name`         | `bench:en`  | `bench:ja`  |  `musl:en`  |  `musl:ja`  |
|:-----------------------|------------:|------------:|------------:|------------:|
| cmp-memchr             |    5.348 uc |    5.306 uc |    5.793 uc |    5.851 uc |
| cmp-ahocorasick-find   |   14.186 uc |   15.021 uc |   14.227 uc |   15.105 uc |
| cmp-libc-memmem        |   15.623 uc |   19.290 uc |   17.711 uc |   21.700 uc |
| cmp-twowaysearcher     |   23.007 uc |   16.541 uc |   22.438 uc |   16.414 uc |
| cmp-my-naive-opt-last  |   35.827 uc |   35.824 uc |   36.228 uc |   36.303 uc |
| cmp-my-naive-opt-1st   |   36.359 uc |   92.798 uc |   36.661 uc |   84.797 uc |
| cmp-twoway-find-str    |   38.472 uc |   34.161 uc |   41.488 uc |   34.100 uc |
| cmp-std-str-find       |   43.847 uc |   35.387 uc |   42.161 uc |   33.412 uc |
| cmp-my-naive-classic   |  355.890 uc |  377.100 uc |  200.950 uc |  228.480 uc |

This benchmark measures string search.
It is based on `std::str::find()`, which has the same functionality
as the `strstr()` and `memmem()` functions in C language.

Here is the code using `std::str::find()`:

```rust
pub fn do_search_std_str_find(texts: &[&str], pattern: &str) -> anyhow::Result<usize> {
    let mut found: usize = 0;
    for line in texts {
        if let Some(_n) = line.find(pattern) {
            found += 1;
        }
    }
    Ok(found)
}
```

Receives multiple strings in `texts` and checks if each string contains
a `pattern` string. It then returns the count of strings it contained.

Measure the benchmark with what I wrote this part in other ways,
