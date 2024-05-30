use std::path::Path;

use test_case::test_case;
use tywaves_rs::hgldd::{self};

#[test]
fn random_hgldd_file() {
    let hgldd = hgldd::spec::Header {
        version: String::from("1.0"),
        file_info: Vec::new(),
        hdl_file_index: None,
    };
    let object = hgldd::spec::Object {
        kind: hgldd::spec::ObjectKind::Module,
        obj_name: String::from("top"),
        module_name: Some(String::from("top")),
        is_ext_module: None,
        hgl_loc: None,
        hdl_loc: None,
        port_vars: Vec::new(),
        children: Some(Vec::new()),
        source_lang_type_info: None,
    };
    println!(
        "{}",
        serde_json::to_string_pretty(&hgldd::spec::Hgldd {
            hgldd,
            objects: vec![object]
        })
        .unwrap()
    );
}

#[test]
fn test_parser() {
    let hgldd_file = Path::new("tests/inputs/hgldd/foo.dd");
    hgldd::reader::parse_hgldd_file(hgldd_file);
}
#[test_case("tests/inputs/hgldd/foo.dd"; "Test foo.dd")]
#[test_case("tests/inputs/hgldd/bar.dd"; "Test bar.dd")]
#[test_case("tests/inputs/hgldd/global.dd"; "Test global.dd")]
#[test_case("tests/inputs/hgldd/foo_with_source_lang_types.dd"; "Test source lang types on variables")]
#[test_case("tests/inputs/hgldd/global_with_source_lang_types.dd"; "Test source lang types on subfields")]
#[test_case("tests/inputs/hgldd/bar_with_source_lang_types.dd"; "Test source lang types on module")]
fn test_hgldd_parser(file_path: &str) {
    let hgldd_file = Path::new(file_path);

    let hgldd = hgldd::reader::parse_hgldd_file(hgldd_file);
    assert_eq!(hgldd.len(), 1);
    let hgldd = hgldd.first();

    // Check
    // Read the input value
    let hgldd_str = serde_json::to_string(&hgldd).unwrap();
    let value_from_parser = serde_json::from_str::<serde_json::Value>(&hgldd_str).unwrap();

    // Get the expected value
    let hgldd_str = std::fs::read_to_string(hgldd_file).unwrap();
    let expected_value = serde_json::from_str::<serde_json::Value>(&hgldd_str).unwrap();

    assert_json_diff::assert_json_eq!(value_from_parser, expected_value);
}

#[test]
fn test_hgldd_parser_with_comments() {
    let hgldd_file = Path::new("tests/inputs/hgldd/foo_with_comments.dd");

    let hgldd = hgldd::reader::parse_hgldd_file(hgldd_file);
    assert!(hgldd.len() == 1);
    let hgldd = hgldd.first();

    // Check
    // Read the input value
    let hgldd_str = serde_json::to_string(&hgldd).unwrap();
    let value_from_parser = serde_json::from_str::<serde_json::Value>(&hgldd_str).unwrap();

    // Get the expected value
    let hgldd_str = std::fs::read_to_string(hgldd_file).unwrap();
    let hgldd_str = hgldd::reader::drop_comments(&hgldd_str);
    let expected_value = serde_json::from_str::<serde_json::Value>(&hgldd_str).unwrap();

    assert_json_diff::assert_json_eq!(value_from_parser, expected_value);
}

#[test]
fn test_multi_hgldd_parser() {
    let hgldd_file = Path::new("tests/inputs/3_hgldds_in_single_file.dd");
    let hgldds = hgldd::reader::parse_hgldd_file(hgldd_file);
    assert_eq!(hgldds.len(), 3);
}

#[test]
fn test_hgldd_parser_dir() {
    let hgldd_dir = Path::new("tests/inputs/hgldd");
    let hgldds = hgldd::reader::parse_hgldd_dir(hgldd_dir);
    assert_eq!(hgldds.len(), 7);

    // Collect the file names from the dir
    let reference_files: Vec<String> = std::fs::read_dir(hgldd_dir)
        .unwrap()
        .map(|x| x.unwrap().file_name().into_string().unwrap())
        .collect();
    // Check
    // Zip file_names and hgldds
    for (reference_file, hgldd) in reference_files.iter().zip(hgldds.iter()) {
        // Read the input value
        let hgldd_str = serde_json::to_string(&hgldd).unwrap();
        let value_from_parser = serde_json::from_str::<serde_json::Value>(&hgldd_str).unwrap();

        // Get the expected value
        let hgldd_str = std::fs::read_to_string(hgldd_dir.join(reference_file)).unwrap();
        // Remove comments
        let hgldd_str = hgldd::reader::drop_comments(&hgldd_str);
        let expected_value = serde_json::from_str::<serde_json::Value>(&hgldd_str).unwrap();

        assert_json_diff::assert_json_eq!(value_from_parser, expected_value);
    }
}
