use std::iter;

use benchmark_simple::*;
use chomp1;
use chomp1::buffer::InputBuf;
use chomp1::prelude::*;

macro_rules! function_name {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        match &name[..name.len() - 3].rfind(':') {
            Some(pos) => &name[pos + 1..name.len() - 3],
            None => &name[..name.len() - 3],
        }
    }};
}

pub fn bench() {
    count_vec_1k();
    count_vec_10k();
    count_vec_10k_maybe_incomplete();
    many_vec_1k();
    many_vec_10k();
    many_vec_10k_maybe_incomplete();
    many1_vec_1k();
    many1_vec_10k();
    many1_vec_10k_maybe_incomplete();
}

fn count_vec_1k() {
    let data = iter::repeat(b'a').take(1024).collect::<Vec<u8>>();

    fn count_vec<I: Input>(i: I) -> ParseResult<I, Vec<I::Token>, Error<I::Token>> {
        count(i, 1024, any)
    }

    let res = Bench::new().run(&Options::default(), || parse_only(count_vec, &data));
    println!("{}: {}", function_name!(), res);
}

fn count_vec_10k() {
    let data = iter::repeat(b'a').take(10240).collect::<Vec<u8>>();

    fn count_vec<I: Input>(i: I) -> ParseResult<I, Vec<I::Token>, Error<I::Token>> {
        count(i, 10240, any)
    }

    let res = Bench::new().run(&Options::default(), || parse_only(count_vec, &data));
    println!("{}: {}", function_name!(), res);
}

fn many_vec_1k() {
    let data = iter::repeat(b'a').take(1024).collect::<Vec<u8>>();

    fn many_vec<I: Input>(i: I) -> ParseResult<I, Vec<I::Token>, Error<I::Token>> {
        many(i, any)
    }

    let res = Bench::new().run(&Options::default(), || parse_only(many_vec, &data));
    println!("{}: {}", function_name!(), res);
}

fn many_vec_10k() {
    let data = iter::repeat(b'a').take(10024).collect::<Vec<u8>>();

    fn many_vec<I: Input>(i: I) -> ParseResult<I, Vec<I::Token>, Error<I::Token>> {
        many(i, any)
    }

    let res = Bench::new().run(&Options::default(), || parse_only(many_vec, &data));
    println!("{}: {}", function_name!(), res);
}

fn many1_vec_1k() {
    let data = iter::repeat(b'a').take(1024).collect::<Vec<u8>>();

    fn many1_vec<I: Input>(i: I) -> ParseResult<I, Vec<I::Token>, Error<I::Token>> {
        many1(i, any)
    }

    let res = Bench::new().run(&Options::default(), || parse_only(many1_vec, &data));
    println!("{}: {}", function_name!(), res);
}

fn many1_vec_10k() {
    let data = iter::repeat(b'a').take(10024).collect::<Vec<u8>>();

    fn many1_vec<I: Input>(i: I) -> ParseResult<I, Vec<I::Token>, Error<I::Token>> {
        many1(i, any)
    }

    let res = Bench::new().run(&Options::default(), || parse_only(many1_vec, &data));
    println!("{}: {}", function_name!(), res);
}

fn count_vec_10k_maybe_incomplete() {
    let data = iter::repeat(b'a').take(10024).collect::<Vec<u8>>();

    fn count_vec<I: Input>(i: I) -> ParseResult<I, Vec<I::Token>, Error<I::Token>> {
        count(i, 10024, any)
    }

    let res = Bench::new().run(&Options::default(), || count_vec(InputBuf::new(&data)));
    println!("{}: {}", function_name!(), res);
}

fn many_vec_10k_maybe_incomplete() {
    let data = iter::repeat(b'a').take(10024).collect::<Vec<u8>>();

    fn many_vec<I: Input>(i: I) -> ParseResult<I, Vec<I::Token>, Error<I::Token>> {
        many(i, any)
    }

    let res = Bench::new().run(&Options::default(), || many_vec(InputBuf::new(&data)));
    println!("{}: {}", function_name!(), res);
}

fn many1_vec_10k_maybe_incomplete() {
    let data = iter::repeat(b'a').take(10024).collect::<Vec<u8>>();

    fn many1_vec<I: Input>(i: I) -> ParseResult<I, Vec<I::Token>, Error<I::Token>> {
        many1(i, any)
    }

    let res = Bench::new().run(&Options::default(), || many1_vec(InputBuf::new(&data)));
    println!("{}: {}", function_name!(), res);
}
