use tywaves_rs::tyvcd::trace_pointer::{ConstValue, TraceGetter, TraceValue};

// External variable that will be captured.

use crate::tyvcd::spec::*;

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

// Create the TyVcd for the [[tests/inputs/tyvcd/foo/foo_no_types.dd]] file.
pub fn create_foo_single_no_types() -> TyVcd {
    let mut scopes = HashMap::new();
    scopes.insert(
        String::from("Foo"),
        Arc::new(RwLock::new(Scope::empty(
            String::from("Foo"),
            String::from("Foo"),
            // type info na, use the target language one
            TypeInfo::new("Foo".to_string(), Vec::new()),
            &vec![],
        ))),
    );
    // inA
    scopes.get("Foo").unwrap().write().unwrap().variables.push(
        Variable::new(
            TraceValue::RefTraceName("a".to_string()),
            String::from("inA"),
            // type info na, use the target language one
            TypeInfo::new("logic".to_string(), Vec::new()),
            VariableKind::Ground(32),
        )
        .as_top(),
    );
    // inB
    scopes.get("Foo").unwrap().write().unwrap().variables.push(
        Variable::new(
            TraceValue::RefTraceName("b".to_string()),
            String::from("outB"),
            // type info na, use the target language one
            TypeInfo::new("logic".to_string(), Vec::new()),
            VariableKind::Ground(32),
        )
        .as_top(),
    );
    // var1 => const
    let var1_val = String::from("00101010");
    scopes.get("Foo").unwrap().write().unwrap().variables.push(
        Variable::new(
            TraceValue::Constant(ConstValue::FourValue(var1_val.into_bytes(), 8)),
            String::from("var1"),
            // type info na, use the target language one
            TypeInfo::new("logic".to_string(), Vec::new()),
            VariableKind::Ground(8),
        )
        .as_top(),
    );

    // instances
    scopes
        .get("Foo")
        .unwrap()
        .write()
        .unwrap()
        .subscopes
        .insert(
            String::from("b0"),
            Arc::new(RwLock::new(Scope::empty(
                String::from("b0"),
                String::from("Bar"),
                // type info na, use the target language one
                TypeInfo::new("Bar".to_string(), Vec::new()),
                &vec!["Foo".to_string()],
            ))),
        );

    scopes
        .get("Foo")
        .unwrap()
        .write()
        .unwrap()
        .subscopes
        .insert(
            String::from("b1"),
            Arc::new(RwLock::new(Scope::empty(
                String::from("b1"),
                String::from("Bar"),
                // type info na, use the target language one
                TypeInfo::new("Bar".to_string(), Vec::new()),
                &vec!["Foo".to_string()],
            ))),
        );

    TyVcd { scopes }
}

// Create the TyVcd for the [[tests/inputs/tyvcd/foo/foo.dd]] file.
pub fn create_foo_single() -> TyVcd {
    let mut scopes = HashMap::new();
    scopes.insert(
        String::from("Foo"),
        Arc::new(RwLock::new(Scope::empty(
            String::from("Foo"),
            String::from("Foo"),
            TypeInfo::new("Foo".to_string(), Vec::new()),
            &vec![],
        ))),
    );
    // inA
    scopes.get("Foo").unwrap().write().unwrap().variables.push(
        Variable::new(
            TraceValue::RefTraceName("a".to_string()),
            String::from("inA"),
            TypeInfo::new("SInt<32>".to_string(), Vec::new()),
            VariableKind::Ground(32),
        )
        .as_top(),
    );
    // inB
    scopes.get("Foo").unwrap().write().unwrap().variables.push(
        Variable::new(
            TraceValue::RefTraceName("b".to_string()),
            String::from("outB"),
            // type info na, use the target language one
            TypeInfo::new(
                "logic".to_string(),
                vec![ConstructorParams {
                    name: "size".to_string(),
                    tpe: "uint".to_string(),
                    value: Some("32".to_string()),
                }],
            ),
            VariableKind::Ground(32),
        )
        .as_top(),
    );
    // var1 => const
    let var1_val = String::from("00101010");
    scopes.get("Foo").unwrap().write().unwrap().variables.push(
        Variable::new(
            TraceValue::Constant(ConstValue::FourValue(var1_val.into_bytes(), 8)),
            String::from("var1"),
            // type info na, use the target language one
            TypeInfo::new(
                "UInt<8>".to_string(),
                vec![
                    ConstructorParams {
                        name: "n".to_string(),
                        tpe: "int".to_string(),
                        value: Some("42".to_string()),
                    },
                    ConstructorParams {
                        name: "a".to_string(),
                        tpe: "myType".to_string(),
                        value: None,
                    },
                ],
            ),
            VariableKind::Ground(8),
        )
        .as_top(),
    );

    // var2
    let var2_val = String::from("00010101");
    scopes.get("Foo").unwrap().write().unwrap().variables.push(
        Variable::new(
            TraceValue::Constant(ConstValue::FourValue(var2_val.into_bytes(), 8)),
            String::from("var2"),
            // type info na, use the target language one
            TypeInfo::new("logic".to_string(), Vec::new()),
            VariableKind::Ground(8),
        )
        .as_top(),
    );

    // instances
    scopes
        .get("Foo")
        .unwrap()
        .write()
        .unwrap()
        .subscopes
        .insert(
            String::from("b0"),
            Arc::new(RwLock::new(Scope::empty(
                String::from("b0"),
                String::from("Bar"),
                // type info na, use the target language one
                TypeInfo::new("Bar".to_string(), Vec::new()),
                &vec!["Foo".to_string()],
            ))),
        );

    scopes
        .get("Foo")
        .unwrap()
        .write()
        .unwrap()
        .subscopes
        .insert(
            "b1".to_string(),
            Arc::new(RwLock::new(Scope::empty(
                String::from("b1"),
                String::from("Bar"),
                // type info na, use the target language one
                TypeInfo::new("Bar".to_string(), Vec::new()),
                &vec!["Foo".to_string()],
            ))),
        );

    TyVcd { scopes }
}

// Create the TyVcd for the [[tests/inputs/tyvcd/foo]] directory.
pub fn create_foo() -> TyVcd {
    let foo = create_foo_single();
    let bar = super::bar::create_bar_single();

    for (_, subscope_to_update) in &mut foo.scopes.get("Foo").unwrap().write().unwrap().subscopes {
        let new_scope = Scope::from_other(
            &bar.scopes.get("Bar").unwrap().read().unwrap().clone(),
            subscope_to_update
                .read()
                .unwrap()
                .get_trace_name()
                .unwrap()
                .clone(),
        )
        .prepend_parent_scopes(vec!["Foo".to_string()]);

        *subscope_to_update = Arc::new(RwLock::new(new_scope));
    }
    foo
}
