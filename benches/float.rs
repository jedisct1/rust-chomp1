use std::str;
use std::str::FromStr;

use benchmark_simple::*;
use chomp1::ascii;
use chomp1::primitives::IntoInner;
use chomp1::types::Buffer;

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

const PI_SLICE: &[u8] = b"3.14159265358979323846264338327950288419716939937510582097494459230781640628620899862803482534211706798";
const PI_F64: f64 = 3.141_592_653_589_793;
const PI_F32: f32 = 3.141_592_7;

pub fn bench() {
    match_float();
    float_f64();
    float_f32();
    float_small_f64();
    float_small_f32();
    float_no_conversion();
    float_small_no_conversion();
    from_str_f32();
    from_str_f64();
    from_str_small_f32();
    from_str_small_f64();
}

fn match_float() {
    let res = Bench::new().run(&Options::default(), || ascii::match_float(PI_SLICE));
    println!("{}: {}", function_name!(), res);
}

fn float_f64() {
    assert_eq!(
        ascii::float::<_, f64>(PI_SLICE).into_inner(),
        (&b""[..], Ok(PI_F64))
    );

    let res = Bench::new().run(&Options::default(), || ascii::float::<_, f64>(PI_SLICE));
    println!("{}: {}", function_name!(), res);
}

fn float_f32() {
    assert_eq!(
        ascii::float::<_, f32>(PI_SLICE).into_inner(),
        (&b""[..], Ok(PI_F32))
    );

    let res = Bench::new().run(&Options::default(), || ascii::float::<_, f32>(PI_SLICE));
    println!("{}: {}", function_name!(), res);
}

fn float_small_f64() {
    assert_eq!(
        ascii::float::<_, f64>(&b"1"[..]).into_inner(),
        (&b""[..], Ok(1.0))
    );

    let res = Bench::new().run(&Options::default(), || ascii::float::<_, f64>(&b"1"[..]));
    println!("{}: {}", function_name!(), res);
}

fn float_small_f32() {
    assert_eq!(
        ascii::float::<_, f32>(&b"1"[..]).into_inner(),
        (&b""[..], Ok(1.0))
    );

    let res = Bench::new().run(&Options::default(), || ascii::float::<_, f32>(&b"1"[..]));
    println!("{}: {}", function_name!(), res);
}

/// The purpose of this test is to measure the time Chomp uses to parse and
/// allocate the vector required to pass the data on to Rust's `FromStr`
/// implementation for `f32` and `f64`.
fn float_no_conversion() {
    let res = Bench::new().run(&Options::default(), || {
        ascii::match_float(PI_SLICE).map(|b| b.into_vec())
    });
    println!("{}: {}", function_name!(), res);
}

fn float_small_no_conversion() {
    let res = Bench::new().run(&Options::default(), || {
        ascii::match_float(&b"1"[..]).map(|b| b.into_vec())
    });
    println!("{}: {}", function_name!(), res);
}

/// Reference, 32-bit
fn from_str_f32() {
    assert_eq!(
        FromStr::from_str(unsafe { str::from_utf8_unchecked(PI_SLICE) }),
        Ok(PI_F32)
    );

    let res = Bench::new().run(&Options::default(), || {
        let f: Result<f32, _> = FromStr::from_str(unsafe { str::from_utf8_unchecked(PI_SLICE) });
        f
    });
    println!("{}: {}", function_name!(), res);
}

/// Reference, 64-bit
fn from_str_f64() {
    assert_eq!(
        FromStr::from_str(unsafe { str::from_utf8_unchecked(PI_SLICE) }),
        Ok(PI_F64)
    );

    let res = Bench::new().run(&Options::default(), || {
        let f: Result<f64, _> = FromStr::from_str(unsafe { str::from_utf8_unchecked(PI_SLICE) });
        f
    });
    println!("{}: {}", function_name!(), res);
}

/// Reference, 32-bit, small
fn from_str_small_f32() {
    assert_eq!(
        FromStr::from_str(unsafe { str::from_utf8_unchecked(&b"1"[..]) }),
        Ok(1.0f32)
    );

    let res = Bench::new().run(&Options::default(), || {
        let f: Result<f32, _> = FromStr::from_str(unsafe { str::from_utf8_unchecked(&b"1"[..]) });
        f
    });
    println!("{}: {}", function_name!(), res);
}

/// Reference, 64-bit
fn from_str_small_f64() {
    assert_eq!(
        FromStr::from_str(unsafe { str::from_utf8_unchecked(&b"1"[..]) }),
        Ok(1.0f64)
    );
    let res = Bench::new().run(&Options::default(), || {
        let f: Result<f64, _> = FromStr::from_str(unsafe { str::from_utf8_unchecked(&b"1"[..]) });
        f
    });
    println!("{}: {}", function_name!(), res);
}
