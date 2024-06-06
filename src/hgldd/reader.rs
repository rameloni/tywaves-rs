use crate::hgldd::spec::{Hgldd, Instance, Object, ObjectKind};
use std::path::Path;

/// The extension used for HGLDD files.
const HGLDD_EXTENSION: &str = "dd";

/// Remove comments (if any) from the HGLDD content.
#[inline]
pub fn drop_comments(hgldd_str: &str) -> String {
    hgldd_str
        .lines()
        .filter(|line| !line.trim_start().starts_with("//"))
        .collect::<Vec<&str>>()
        .join("\n")
}

// Only for test and doctest
#[cfg(test)]
pub(crate) fn parse_hgldds_pub(hgldd_str: &str) -> Vec<Hgldd> {
    parse_hgldds(hgldd_str)
}

// Parse an HGLDD string with multiple HGLDDs in it
#[inline]
pub fn parse_hgldds(hgldd_str: &str) -> Vec<Hgldd> {
    // TODO: add error handling
    // Skip the comment line (if any)
    let hgldd_str = drop_comments(hgldd_str);
    let deserializer = serde_json::Deserializer::from_reader(hgldd_str.as_bytes());
    let iterator = deserializer.into_iter::<serde_json::Value>();
    iterator
        .map(|x| serde_json::from_value(x.unwrap()).unwrap())
        .collect()
}

/// Parse single HGLDD file.
/// Return a vector of the [Hgldd] definitions present in a file.
#[inline]
pub fn parse_hgldd_file(hgldd_path: &Path) -> Vec<Hgldd> {
    // TODO: add error handling
    let hgldd_str = std::fs::read_to_string(hgldd_path).unwrap();
    parse_hgldds(&hgldd_str)
}

#[inline]
/// Parse a directory containing multiple HGLDD files.
/// Return a vector of the [Hgldd] definitions present in the directory.
pub fn parse_hgldd_dir(hgldd_dir_path: &Path) -> Vec<Hgldd> {
    // TODO: add error handling

    // Read the directory and parse all the files
    let files = std::fs::read_dir(hgldd_dir_path).unwrap();
    let mut hgldds = Vec::new();
    for file in files {
        let file = file.unwrap();
        let path = file.path();
        // Check if the file is an HGLDD file
        if path.is_file() && path.extension().unwrap() == HGLDD_EXTENSION {
            hgldds.append(&mut parse_hgldd_file(&path));
        }
    }
    hgldds
}

/// Add extra modules to the HGLDDs.
/// It will replace the top module HGLDD with the a new **hierarchy** version of the top module.
///
/// # Example
/// ```
/// use tywaves_rs::hgldd::reader::{parse_hgldds, add_extra_modules};
///
/// fn main() {
///     let hgldd_str = r#"
///         {
///             "HGLDD": { "file_info": [], "version": "0.1.0" },
///             "objects": [
///                 {
///                     "obj_name": "Bar",
///                     "module_name": "Bar",
///                     "kind": "module",
///                     "children": [{ "name": "HGLme", "hdl_obj_name": "HDL", "obj_name": "HGLme", "module_name": "HDL" }],
///                     "port_vars": []
///                 }
///             ]
///         }"#;
///     let hgldds = parse_hgldds(hgldd_str);
///     let hgldds = add_extra_modules(hgldds, vec!["TOP_TB".to_string(), "DUT".to_string()], &"TOP_MODULE".to_string());
///     println!("{}", serde_json::to_string_pretty(&hgldds).unwrap());
/// }
/// ```
/// It will:
/// - Update the [Hgldd] with `module_name` as "TOP_MODULE" with the `module_name` as "DUT".
/// - A new [Hgldd] object with the top module name as "TOP_MODULE" and the old top module as a child.
pub fn add_extra_modules(
    hgldds: Vec<Hgldd>,
    // TODO: performance improvement, use a slice and str instead of String
    extra_modules: Vec<String>,
    top_module_name: &String,
) -> Vec<Hgldd> {
    // Get a copy of the input vectors by moving them (it should be more efficient than cloning them)
    let mut hgldds = hgldds;
    let mut extra_modules = extra_modules;

    // Get the header of the HGLDDs
    let hgldd_header = if let Some(hgldd_header) = hgldds.first() {
        hgldd_header.hgldd.clone()
    } else {
        return hgldds;
    };

    // 1. Update the top module name with the new hgldd name
    let mut top_obj = None;

    for hgldd in hgldds.iter_mut() {
        for object in hgldd.objects.iter_mut() {
            if object.module_name.as_ref() == Some(top_module_name)
                || object.obj_name == *top_module_name
            {
                // Replace the top module with the new top module
                object.module_name = extra_modules.pop();
                // Save the top HGLDD and break
                top_obj = Some(object);
                break;
            }
        }
    }

    // 2. For all the remaining extra modules, create a new hierachy
    if let Some(top_obj) = top_obj {
        let mut inst_name_hgl = top_obj.obj_name.clone(); // HGL
        let mut inst_name_hdl = if let Some(module_name) = &top_obj.module_name {
            module_name.clone()
        } else {
            top_obj.obj_name.clone()
        }; // HDL

        // Create a new HGLDD for each extra module (in reverse order)
        while let Some(extra_module_name) = extra_modules.pop() {
            let new_obj =
                Object::new(extra_module_name.clone(), ObjectKind::Module).with_children(vec![
                    Instance::new(
                        inst_name_hdl.clone(), // Instance names
                        inst_name_hdl.clone(), // Type instance names
                        inst_name_hgl.clone(), // hgl type name
                        inst_name_hdl.clone(),
                        // inst_name_hdl.clone(),
                    ),
                ]);

            let new_hgldd = Hgldd {
                hgldd: hgldd_header.clone(),
                objects: vec![new_obj],
            };
            // Update the instance names
            inst_name_hgl = extra_module_name.clone();
            inst_name_hdl = extra_module_name;

            // Update the HGLDDs
            hgldds.push(new_hgldd);
        }
    }
    // Return the updated hgldds
    hgldds
}

#[cfg(test)]
mod tests {

    #[test]
    fn add_extra_modules_works() {
        use super::*;

        // Input HGLDD
        let hgldd_str = r#"
        {
            "HGLDD": { "file_info": [], "version": "0.1.0" },
            "objects": [
                {
                    "obj_name": "Bar",
                    "module_name": "Bar",
                    "kind": "module",
                    "children": [{ "name": "HGLme", "hdl_obj_name": "HDL", "obj_name": "HGLme", "module_name": "HDL" }],
                    "port_vars": []
                }
            ]
        }"#;

        // Updated HGLDD
        let hgldd_out = r#"
        {
            "HGLDD": { "file_info": [], "version": "0.1.0" },
            "objects": [
                {
                    "obj_name": "Bar",
                    "module_name": "b0",
                    "kind": "module",
                    "children": [{ "name": "HGLme", "hdl_obj_name": "HDL", "obj_name": "HGLme", "module_name": "HDL" }],
                    "port_vars": []
                }
            ]
        }
        {
            "HGLDD": { "file_info": [], "version": "0.1.0" },
            "objects": [
                {
                    "obj_name": "TOP_TB",
                    "kind": "module",
                    "children": [{ "name": "b0", "hdl_obj_name": "b0", "obj_name": "Bar", "module_name": "b0" }],
                    "port_vars": []
                }
            ]
        }
        "#;
        // Parse the input
        let hgldds = parse_hgldds(hgldd_str);
        assert_eq!(hgldds.len(), 1);
        assert_eq!(hgldds[0].objects.len(), 1);
        assert_eq!(hgldds[0].objects[0].obj_name, "Bar");
        assert_eq!(hgldds[0].objects[0].module_name, Some("Bar".to_string()));

        // Replace Bar with TOP_TB -> b0
        let hgldds = add_extra_modules(
            hgldds,
            vec!["TOP_TB".to_string(), "b0".to_string()],
            &"Bar".to_string(),
        );
        assert_eq!(hgldds.len(), 2);
        assert_eq!(hgldds[0].objects.len(), 1);
        assert_eq!(hgldds[0].objects[0].obj_name, "Bar");
        assert_eq!(hgldds[0].objects[0].module_name, Some("b0".to_string()));

        let hgldds_expected = parse_hgldds(hgldd_out);
        assert_json_diff::assert_json_eq!(&hgldds, &hgldds_expected);
    }
}
