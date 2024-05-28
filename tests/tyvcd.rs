mod expected_tyvcd;

use std::collections::HashMap;
use std::path::Path;

use test_case::test_case;

use tywaves_rs::hgldd;
use tywaves_rs::tyvcd;

use expected_tyvcd::foo;

#[test_case("tests/inputs/hgldd/foo.dd", 1; "Test foo.dd")]
#[test_case("tests/inputs/hgldd/bar.dd", 1; "Test bar.dd")]
#[test_case("tests/inputs/gcd.dd", 1; "Test gcd.dd")]
#[test_case("tests/inputs/hgldd/global.dd", 1; "Test global.dd")]
#[test_case("tests/inputs/hgldd", 4; "Test directory project")]

fn test_hgldd_tyvcd_builder_success(file_path: &str, exp_hgldd_len: usize) {
    // Read the hgldd file
    let hgldd_file = Path::new(file_path);
    // Check if it is a directory

    let hgldd = if hgldd_file.is_dir() {
        hgldd::reader::parse_hgldd_dir(hgldd_file)
    } else {
        hgldd::reader::parse_hgldd_file(hgldd_file)
    };

    assert_eq!(hgldd.len(), exp_hgldd_len);

    for hgldd in hgldd {
        let _tyvcd = tyvcd::builder::from_hgldd(&hgldd);
    }
}
