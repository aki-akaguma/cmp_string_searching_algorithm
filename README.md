# cmp_string_match
research: comparing string match of rust

## The Naive string-searching algorithm is faster on the modern computer.

|         `name`          | `bench:en`  | `bench:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| cmp-my-naive-optmc-1st  |    5.255 uc |   88.790 uc |    5.927 uc |   68.377 uc |
| cmp-my-naive-optmc-last |    5.362 uc |    5.354 uc |    5.939 uc |    5.951 uc |
| cmp-ahocorasick-find    |   14.206 uc |   15.029 uc |   14.323 uc |   16.047 uc |
| cmp-libc-memmem         |   15.408 uc |   19.411 uc |   17.812 uc |   21.992 uc |
| cmp-twowaysearcher      |   23.374 uc |   17.342 uc |   23.684 uc |   17.393 uc |
| cmp-my-naive-opt-last   |   35.604 uc |   35.602 uc |   36.445 uc |   36.379 uc |
| cmp-my-naive-opt-1st    |   36.106 uc |   92.085 uc |   36.918 uc |   90.525 uc |
| cmp-twoway-find-str     |   38.508 uc |   34.295 uc |   39.250 uc |   34.121 uc |
| cmp-std-str-find        |   43.911 uc |   35.294 uc |   43.881 uc |   35.217 uc |
| cmp-my-naive-classic    |  336.910 uc |  360.650 uc |  185.960 uc |  213.130 uc |

- rustc 1.51.0 (2fd73fabe 2021-03-23)

## This benchmark measures string search.

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

Measure the benchmark with what I wrote this part in other ways.

## The Naive string-searching algorithm

This is the classical algorithm, matching in order from the beginning.
Please see (wikipedia)[https://en.wikipedia.org/wiki/String-searching_algorithm].

## The optimization idea 1

The crate memchr is very faster. This use simd optimization.
Use this on 1st character matching.

## The optimization idea 2

The 2nd Optimization is that takes advantage of UTF8 features,
cause of the rust language use only UTF-8 character strings.
