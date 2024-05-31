mod expected_tyvcd;

use std::collections::HashMap;
use std::path::Path;

use test_case::test_case;

use tywaves_rs::hgldd;
use tywaves_rs::tyvcd::spec::*;
use tywaves_rs::tyvcd::trace_pointer::*;
use tywaves_rs::tyvcd::{self, builder::GenericBuilder};

use expected_tyvcd::*;

use pretty_assertions::assert_eq;

#[test_case("tests/inputs/hgldd/foo.dd", 1; "Test foo.dd")]
#[test_case("tests/inputs/hgldd/bar.dd", 1; "Test bar.dd")]
#[test_case("tests/inputs/gcd.dd", 1; "Test gcd.dd")]
#[test_case("tests/inputs/hgldd/global.dd", 1; "Test global.dd")]
#[test_case("tests/inputs/hgldd", 7; "Test directory project")]

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

    let mut builder = tyvcd::builder::TyVcdBuilder::init(hgldd);
    builder.build();
}

#[test_case("tests/inputs/tyvcd/foo_no_types.dd", foo::create_foo_single_no_types; "Test foo_no_types.dd")]
#[test_case("tests/inputs/tyvcd/foo/foo.dd", foo::create_foo_single; "Test foo.dd")]
#[test_case("tests/inputs/tyvcd/foo/bar.dd", bar::create_bar_single; "Test bar.dd")]
#[test_case("tests/inputs/tyvcd/foo", foo::create_foo; "Test directory foo")]
#[test_case("tests/inputs/tyvcd/withBundlesAndVecs.dd", with_bundles_and_vecs::create_with_bundles_and_vecs; "Test with bundles and vecs")]
fn test_tyvcd_single_file_assertions(
    file_path: &str,
    create_expected_output: fn() -> tyvcd::spec::TyVcd,
) {
    // Read the hgldd file
    let hgldd_file = Path::new(file_path);
    // Check if it is a directory

    let hgldd = if hgldd_file.is_dir() {
        hgldd::reader::parse_hgldd_dir(hgldd_file)
    } else {
        hgldd::reader::parse_hgldd_file(hgldd_file)
    };

    let mut builder = tyvcd::builder::TyVcdBuilder::init(hgldd);
    builder.build();
    let tyvcd = builder.get_copy().unwrap();

    let expected_tyvcd = create_expected_output();
    assert_eq!(tyvcd, expected_tyvcd);
}

#[test]
fn test_trace_pointer() {
    let tyvcd_foo = foo::create_foo_single();
    assert!(tyvcd_foo.find_trace(&["unknown".to_string()]).is_none());
    assert!(tyvcd_foo.find_trace(&[]).is_none());

    let foo = tyvcd_foo
        .find_trace(&["Foo".to_string()])
        .and_then(|trace| trace.as_any().downcast_ref::<Scope>())
        .expect("failed to downcast");

    assert_eq!(foo.get_trace_name(), "Foo");
    // assert_eq!(foo.get_trace_path(), vec!["Foo"]);

    let bar0 = tyvcd_foo
        .find_trace(&["Foo".to_string(), "b0".to_string()])
        .and_then(|trace| trace.as_any().downcast_ref::<Scope>())
        .expect("failed to downcast");

    assert_eq!(bar0.get_trace_name(), "b0");
    // assert_eq!(bar.get_trace_path(), vec!["Foo", "b0"]);

    let bar1 = tyvcd_foo
        .find_trace(&["Foo".to_string(), "b1".to_string()])
        .and_then(|trace| trace.as_any().downcast_ref::<Scope>())
        .expect("failed to downcast");

    assert_eq!(bar1.get_trace_name(), "b1");

    let a = tyvcd_foo
        .find_trace(&["Foo".to_string(), "a".to_string()])
        .and_then(|trace| trace.as_any().downcast_ref::<Variable>())
        .expect("failed to downcast");
    assert_eq!(a.name, "inA");

    let bar_b = tyvcd_foo.find_trace(&["Foo".to_string(), "b0".to_string(), "b".to_string()]);
    // .unwrap()
    // .as_any()
    // .downcast_ref::<Variable>()
    // .expect("failed to downcast");
    assert!(bar_b.is_none());

    let tyvcd_foo = foo::create_foo();
    let bar_x = tyvcd_foo
        .find_trace(&["Foo".to_string(), "b0".to_string(), "x".to_string()])
        .and_then(|trace| trace.as_any().downcast_ref::<Variable>())
        .expect("failed to downcast");

    assert_eq!(bar_x.name, "inX");

    let tyvcd_with_bundles_and_vecs = with_bundles_and_vecs::create_with_bundles_and_vecs();
    println!("{:#?}", tyvcd_with_bundles_and_vecs);
    let bar_x = tyvcd_with_bundles_and_vecs
        .find_trace(&["WithBundlesAndVecs".to_string(), "io_a_0".to_string()])
        .and_then(|trace| trace.as_any().downcast_ref::<Variable>())
        .expect("failed to downcast");
    assert_eq!(
        *bar_x,
        Variable::new(
            String::from("io_a_0"),
            String::from("a"),
            TypeInfo::new("UInt<32>".to_string(), Vec::new()),
            VariableKind::Ground,
        )
    );
}