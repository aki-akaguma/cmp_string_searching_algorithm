# cmp_string_match
research: comparing string match of rust

## The Naive string-searching algorithm is faster on the modern computer.

- rustc 1.52.0 (88f19c6da 2021-05-03)

|         `name`          | `bench:en`  | `bench:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| cmp-my-naive-optmc-1st  |    5.299 uc |   88.435 uc |    6.940 uc |   73.004 uc |
| cmp-my-naive-optmc-last |    5.559 uc |    5.564 uc |    6.089 uc |    6.053 uc |
| cmp-ahocorasick-find    |   14.264 uc |   15.168 uc |   15.353 uc |   15.275 uc |
| cmp-libc-memmem         |   15.449 uc |   19.288 uc |   17.923 uc |   21.641 uc |
| cmp-twowaysearcher      |   22.881 uc |   17.634 uc |   22.915 uc |   17.894 uc |
| cmp-my-naive-opt-last   |   35.623 uc |   35.538 uc |   36.662 uc |   38.172 uc |
| cmp-my-naive-opt-1st    |   36.143 uc |   90.012 uc |   37.367 uc |   81.878 uc |
| cmp-twoway-find-str     |   40.841 uc |   33.649 uc |   39.138 uc |   33.029 uc |
| cmp-std-str-find        |   41.122 uc |   34.682 uc |   41.599 uc |   33.400 uc |
| cmp-my-naive-classic    |  336.520 uc |  358.240 uc |  202.980 uc |  227.540 uc |

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
