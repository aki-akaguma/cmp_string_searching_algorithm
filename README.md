# cmp_string_match
research: comparing string match of rust

## The Naive string-searching algorithm is faster on the modern computer.

|         `name`          | `bench:en`  | `bench:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| cmp-my-naive-optmc-1st  |    5.277 uc |   90.743 uc |    5.942 uc |   66.975 uc |
| cmp-my-naive-optmc-last |  `5.348` uc |  `5.380` uc |  `5.892` uc |  `5.851` uc |
| cmp-ahocorasick-find    |   14.357 uc |   15.865 uc |   14.220 uc |   15.231 uc |
| cmp-libc-memmem         |   15.432 uc |   19.282 uc |   19.022 uc |   21.410 uc |
| cmp-twowaysearcher      |   23.287 uc |   16.725 uc |   23.685 uc |   16.868 uc |
| cmp-my-naive-opt-last   |   35.616 uc |   35.592 uc |   36.565 uc |   36.554 uc |
| cmp-my-naive-opt-1st    |   36.691 uc |  100.590 uc |   36.840 uc |   84.621 uc |
| cmp-twoway-find-str     |   40.137 uc |   34.331 uc |   42.096 uc |   31.965 uc |
| cmp-std-str-find        |   45.038 uc |   35.259 uc |   44.258 uc |   34.838 uc |
| cmp-my-naive-classic    |  353.630 uc |  377.250 uc |  216.950 uc |  238.040 uc |

- rustc 1.50.0 (cb75ad5db 2021-02-10)

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
