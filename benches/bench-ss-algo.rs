use criterion::{criterion_group, criterion_main, Criterion};

fn process_search_std_str_find(texts: &[&str], pattern: &str) -> anyhow::Result<usize> {
    cmp_string_searching_algorithm::do_search_std_str_find(texts, pattern)
}

fn process_search_libc_memmem(texts: &[&str], pattern: &str) -> anyhow::Result<usize> {
    cmp_string_searching_algorithm::do_search_libc_memmem(texts, pattern)
}

fn process_search_twoway_find_str(texts: &[&str], pattern: &str) -> anyhow::Result<usize> {
    cmp_string_searching_algorithm::do_search_twoway_find_str(texts, pattern)
}

fn process_search_twowaysearcher_search_in(
    texts: &[&str],
    pat: &memmem::TwoWaySearcher,
) -> anyhow::Result<usize> {
    cmp_string_searching_algorithm::do_search_twowaysearcher_search_in(texts, pat)
}

fn process_search_ahocorasick_find(
    texts: &[&str],
    pat: &aho_corasick::AhoCorasick,
) -> anyhow::Result<usize> {
    cmp_string_searching_algorithm::do_search_ahocorasick_find(texts, pat)
}

fn process_search_my_naive_classic(texts: &[&str], pattern: &str) -> anyhow::Result<usize> {
    cmp_string_searching_algorithm::do_search_my_naive_classic(texts, pattern)
}

fn process_search_my_naive_opt_1st(texts: &[&str], pattern: &str) -> anyhow::Result<usize> {
    cmp_string_searching_algorithm::do_search_my_naive_opt_1st(texts, pattern)
}

fn process_search_my_naive_opt_last(texts: &[&str], pattern: &str) -> anyhow::Result<usize> {
    cmp_string_searching_algorithm::do_search_my_naive_opt_last(texts, pattern)
}

fn process_search_my_naive_opt_mc_1st(texts: &[&str], pattern: &str) -> anyhow::Result<usize> {
    cmp_string_searching_algorithm::do_search_my_naive_opt_mc_1st(texts, pattern)
}

fn process_search_my_naive_opt_mc_last(texts: &[&str], pattern: &str) -> anyhow::Result<usize> {
    cmp_string_searching_algorithm::do_search_my_naive_opt_mc_last(texts, pattern)
}

mod create_data;

fn criterion_benchmark(c: &mut Criterion) {
    memx_cdy::memx_init();
    //
    let (v, match_cnt, pat_string_s, _pat_regex_s, _pat_glob_s) = create_data::create_data();
    let vv: Vec<&str> = v.iter().map(|item| item.as_str()).collect();
    //let pattern = "Error";
    //let pattern = "夏目漱石";
    //
    let pat_aho = aho_corasick::AhoCorasick::new(&[pat_string_s]);
    let pat_memmem = memmem::TwoWaySearcher::new(pat_string_s.as_bytes());
    //
    match process_search_std_str_find(
        criterion::black_box(&vv),
        criterion::black_box(pat_string_s),
    ) {
        Ok(n) => {
            assert_eq!(n, match_cnt);
        }
        Err(err) => {
            eprintln!("{}", err);
            unreachable!();
        }
    }
    match process_search_libc_memmem(
        criterion::black_box(&vv),
        criterion::black_box(pat_string_s),
    ) {
        Ok(n) => {
            assert_eq!(n, match_cnt);
        }
        Err(err) => {
            eprintln!("{}", err);
            unreachable!();
        }
    }
    match process_search_twoway_find_str(
        criterion::black_box(&vv),
        criterion::black_box(pat_string_s),
    ) {
        Ok(n) => {
            assert_eq!(n, match_cnt);
        }
        Err(err) => {
            eprintln!("{}", err);
            unreachable!();
        }
    }
    match process_search_twowaysearcher_search_in(
        criterion::black_box(&vv),
        criterion::black_box(&pat_memmem),
    ) {
        Ok(n) => {
            assert_eq!(n, match_cnt);
        }
        Err(err) => {
            eprintln!("{}", err);
            unreachable!();
        }
    }
    match process_search_ahocorasick_find(criterion::black_box(&vv), criterion::black_box(&pat_aho))
    {
        Ok(n) => {
            assert_eq!(n, match_cnt);
        }
        Err(err) => {
            eprintln!("{}", err);
            unreachable!();
        }
    }
    match process_search_my_naive_classic(
        criterion::black_box(&vv),
        criterion::black_box(pat_string_s),
    ) {
        Ok(n) => {
            assert_eq!(n, match_cnt);
        }
        Err(err) => {
            eprintln!("{}", err);
            unreachable!();
        }
    }
    match process_search_my_naive_opt_1st(
        criterion::black_box(&vv),
        criterion::black_box(pat_string_s),
    ) {
        Ok(n) => {
            assert_eq!(n, match_cnt);
        }
        Err(err) => {
            eprintln!("{}", err);
            unreachable!();
        }
    }
    match process_search_my_naive_opt_last(
        criterion::black_box(&vv),
        criterion::black_box(pat_string_s),
    ) {
        Ok(n) => {
            assert_eq!(n, match_cnt);
        }
        Err(err) => {
            eprintln!("{}", err);
            unreachable!();
        }
    }
    match process_search_my_naive_opt_mc_1st(
        criterion::black_box(&vv),
        criterion::black_box(pat_string_s),
    ) {
        Ok(n) => {
            assert_eq!(n, match_cnt);
        }
        Err(err) => {
            eprintln!("{}", err);
            unreachable!();
        }
    }
    match process_search_my_naive_opt_mc_last(
        criterion::black_box(&vv),
        criterion::black_box(pat_string_s),
    ) {
        Ok(n) => {
            assert_eq!(n, match_cnt);
        }
        Err(err) => {
            eprintln!("{}", err);
            unreachable!();
        }
    }
    //
    c.bench_function("cmp-std-str-find", |b| {
        b.iter(|| {
            let _r = process_search_std_str_find(
                criterion::black_box(&vv),
                criterion::black_box(pat_string_s),
            );
        })
    });
    c.bench_function("cmp-libc-memmem", |b| {
        b.iter(|| {
            let _r = process_search_libc_memmem(
                criterion::black_box(&vv),
                criterion::black_box(pat_string_s),
            );
        })
    });
    c.bench_function("cmp-twoway-find-str", |b| {
        b.iter(|| {
            let _r = process_search_twoway_find_str(
                criterion::black_box(&vv),
                criterion::black_box(pat_string_s),
            );
        })
    });
    c.bench_function("cmp-twowaysearcher", |b| {
        b.iter(|| {
            let _r = process_search_twowaysearcher_search_in(
                criterion::black_box(&vv),
                criterion::black_box(&pat_memmem),
            );
        })
    });
    c.bench_function("cmp-ahocorasick-find", |b| {
        b.iter(|| {
            let _r = process_search_ahocorasick_find(
                criterion::black_box(&vv),
                criterion::black_box(&pat_aho),
            );
        })
    });
    c.bench_function("cmp-my-naive-classic", |b| {
        b.iter(|| {
            let _r = process_search_my_naive_classic(
                criterion::black_box(&vv),
                criterion::black_box(pat_string_s),
            );
        })
    });
    c.bench_function("cmp-my-naive-opt-1st", |b| {
        b.iter(|| {
            let _r = process_search_my_naive_opt_1st(
                criterion::black_box(&vv),
                criterion::black_box(pat_string_s),
            );
        })
    });
    c.bench_function("cmp-my-naive-opt-last", |b| {
        b.iter(|| {
            let _r = process_search_my_naive_opt_last(
                criterion::black_box(&vv),
                criterion::black_box(pat_string_s),
            );
        })
    });
    c.bench_function("cmp-my-naive-optmc-1st", |b| {
        b.iter(|| {
            let _r = process_search_my_naive_opt_mc_1st(
                criterion::black_box(&vv),
                criterion::black_box(pat_string_s),
            );
        })
    });
    c.bench_function("cmp-my-naive-optmc-last", |b| {
        b.iter(|| {
            let _r = process_search_my_naive_opt_mc_last(
                criterion::black_box(&vv),
                criterion::black_box(pat_string_s),
            );
        })
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .warm_up_time(std::time::Duration::from_millis(300))
        .measurement_time(std::time::Duration::from_millis(1500));
    targets = criterion_benchmark
}
//criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
