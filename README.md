# cmp_string_match

![Rust Version][rustc-image]
![Apache2/MIT licensed][license-image]

research: comparing string match of rust

## The Naive string-searching algorithm is faster on the modern computer.

- rustc 1.56.1 (59eed8a2a 2021-11-01)

|         `name`          | `bench:en`  | `bench:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| cmp-my-naive-optmc-1st  |    5.572 uc |   88.334 uc |    5.499 uc |   87.254 uc |
| cmp-my-naive-optmc-last |    5.585 uc |    5.572 uc |    5.460 uc |    5.444 uc |
| cmp-libc-memmem         |    6.723 uc |    6.619 uc |    6.522 uc |    6.531 uc |
| cmp-ahocorasick-find    |   14.159 uc |   15.124 uc |   14.118 uc |   15.081 uc |
| cmp-twowaysearcher      |   24.017 uc |   17.509 uc |   23.482 uc |   17.817 uc |
| cmp-my-naive-opt-last   |   35.716 uc |   35.771 uc |   35.715 uc |   35.746 uc |
| cmp-my-naive-opt-1st    |   36.229 uc |   93.927 uc |   36.217 uc |   83.240 uc |
| cmp-std-str-find        |   38.367 uc |   32.371 uc |   38.208 uc |   32.634 uc |
| cmp-twoway-find-str     |   44.234 uc |   33.062 uc |   43.240 uc |   33.130 uc |
| cmp-my-naive-classic    |  524.750 uc |  524.720 uc |  334.920 uc |  334.210 uc |


- rustc 1.53.0 (53cb7b09b 2021-06-17)

|         `name`          | `bench:en`  | `bench:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| cmp-my-naive-optmc-1st  |    5.432 uc |   81.083 uc |    5.458 uc |   87.334 uc |
| cmp-my-naive-optmc-last |    5.517 uc |    5.522 uc |    5.516 uc |    5.611 uc |
| cmp-libc-memmem         |    6.614 uc |    6.639 uc |    6.673 uc |    6.652 uc |
| cmp-ahocorasick-find    |   14.271 uc |   15.194 uc |   14.344 uc |   15.211 uc |
| cmp-twowaysearcher      |   23.902 uc |   17.608 uc |   23.327 uc |   17.705 uc |
| cmp-my-naive-opt-last   |   35.722 uc |   35.737 uc |   35.734 uc |   35.722 uc |
| cmp-my-naive-opt-1st    |   36.384 uc |   88.302 uc |   36.387 uc |   88.314 uc |
| cmp-twoway-find-str     |   39.334 uc |   33.993 uc |   40.386 uc |   33.805 uc |
| cmp-std-str-find        |   41.349 uc |   35.003 uc |   40.935 uc |   35.952 uc |
| cmp-my-naive-classic    |  311.940 uc |  311.690 uc |  311.280 uc |  311.020 uc |


- rustc 1.52.0 (88f19c6da 2021-05-03)

|         `name`          | `bench:en`  | `bench:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| cmp-my-naive-optmc-1st  |    5.302 uc |   89.060 uc |    6.857 uc |   68.086 uc |
| cmp-my-naive-optmc-last |    5.566 uc |    5.586 uc |    6.060 uc |    6.164 uc |
| cmp-ahocorasick-find    |   14.251 uc |   15.160 uc |   15.473 uc |   15.309 uc |
| cmp-libc-memmem         |   15.398 uc |   19.288 uc |   18.756 uc |   21.259 uc |
| cmp-twowaysearcher      |   23.866 uc |   17.296 uc |   23.504 uc |   17.453 uc |
| cmp-my-naive-opt-last   |   35.687 uc |   35.703 uc |   36.127 uc |   35.979 uc |
| cmp-my-naive-opt-1st    |   36.271 uc |   90.692 uc |   36.759 uc |   81.594 uc |
| cmp-twoway-find-str     |   39.970 uc |   33.341 uc |   39.470 uc |   32.746 uc |
| cmp-std-str-find        |   43.438 uc |   33.927 uc |   41.697 uc |   33.914 uc |
| cmp-my-naive-classic    |  337.330 uc |  361.250 uc |  201.580 uc |  225.010 uc |

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

[//]: # (badges)

[rustc-image]: https://img.shields.io/badge/rustc-1.56+-blue.svg
[license-image]: https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg
