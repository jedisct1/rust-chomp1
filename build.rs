/// This build script checks if we can use `#![feature(specialization)]`.
extern crate rustc_version;

use rustc_version::Channel;

const SPECIALIZATION_CFG: &str = "has_specialization";

fn main() {
    assert!(rustc_version::version().unwrap().major >= 1);

    // Prevent rebuilds of build.rs if other files change
    println!("cargo:rerun-if-changed=build.rs");

    let version = rustc_version::version_meta().unwrap();

    if version.channel == Channel::Nightly {
        if let Some(ref date) = version.commit_date {
            // year, month, day
            let ndate = date
                .splitn(3, '-')
                .map(str::parse)
                .collect::<Result<Vec<i32>, _>>()
                .unwrap();

            // specialization is available from nightly 2016-3-15
            if ndate >= vec![2016, 3, 15] {
                println!("cargo:rustc-cfg={}", SPECIALIZATION_CFG);
            }
        }
    }
}
