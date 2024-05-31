use tywaves_rs::tyvcd::trace_pointer::TraceGetter;

// External variable that will be captured.

use crate::tyvcd::spec::*;

use std::collections::HashMap;

// Create the TyVcd for the [[tests/inputs/tyvcd/foo/foo_no_types.dd]] file.
pub fn create_foo_single_no_types() -> TyVcd {
    let mut scopes = HashMap::new();
    scopes.insert(
        String::from("Foo"),
        Scope::empty(
            String::from("Foo"),
            String::from("Foo"),
            // type info na, use the target language one
            TypeInfo::new("Foo".to_string(), Vec::new()),
        ),
    );
    // inA
    scopes.get_mut("Foo").unwrap().variables.push(Variable::new(
        String::from("a"),
        String::from("inA"),
        // type info na, use the target language one
        TypeInfo::new("logic".to_string(), Vec::new()),
        VariableKind::Ground,
    ));
    // inB
    scopes.get_mut("Foo").unwrap().variables.push(Variable::new(
        String::from("b"),
        String::from("outB"),
        // type info na, use the target language one
        TypeInfo::new("logic".to_string(), Vec::new()),
        VariableKind::Ground,
    ));
    // var1 => const
    scopes.get_mut("Foo").unwrap().variables.push(Variable::new(
        String::from("00101010"),
        String::from("var1"),
        // type info na, use the target language one
        TypeInfo::new("logic".to_string(), Vec::new()),
        VariableKind::Ground,
    ));

    // instances
    scopes.get_mut("Foo").unwrap().subscopes.push(Scope::empty(
        String::from("b0"),
        String::from("Bar"),
        // type info na, use the target language one
        TypeInfo::new("Bar".to_string(), Vec::new()),
    ));

    scopes.get_mut("Foo").unwrap().subscopes.push(Scope::empty(
        String::from("b1"),
        String::from("Bar"),
        // type info na, use the target language one
        TypeInfo::new("Bar".to_string(), Vec::new()),
    ));

    TyVcd { scopes }
}

// Create the TyVcd for the [[tests/inputs/tyvcd/foo/foo.dd]] file.
pub fn create_foo_single() -> TyVcd {
    let mut scopes = HashMap::new();
    scopes.insert(
        String::from("Foo"),
        Scope::empty(
            String::from("Foo"),
            String::from("Foo"),
            TypeInfo::new("Foo".to_string(), Vec::new()),
        ),
    );
    // inA
    scopes.get_mut("Foo").unwrap().variables.push(Variable::new(
        String::from("a"),
        String::from("inA"),
        TypeInfo::new("SInt<32>".to_string(), Vec::new()),
        VariableKind::Ground,
    ));
    // inB
    scopes.get_mut("Foo").unwrap().variables.push(Variable::new(
        String::from("b"),
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
        VariableKind::Ground,
    ));
    // var1 => const
    scopes.get_mut("Foo").unwrap().variables.push(Variable::new(
        String::from("00101010"),
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
        VariableKind::Ground,
    ));

    // var2
    scopes.get_mut("Foo").unwrap().variables.push(Variable::new(
        String::from("00010101"),
        String::from("var2"),
        // type info na, use the target language one
        TypeInfo::new("logic".to_string(), Vec::new()),
        VariableKind::Ground,
    ));

    // instances
    scopes.get_mut("Foo").unwrap().subscopes.push(Scope::empty(
        String::from("b0"),
        String::from("Bar"),
        // type info na, use the target language one
        TypeInfo::new("Bar".to_string(), Vec::new()),
    ));

    scopes.get_mut("Foo").unwrap().subscopes.push(Scope::empty(
        String::from("b1"),
        String::from("Bar"),
        // type info na, use the target language one
        TypeInfo::new("Bar".to_string(), Vec::new()),
    ));

    TyVcd { scopes }
}

// Create the TyVcd for the [[tests/inputs/tyvcd/foo]] directory.
pub fn create_foo() -> TyVcd {
    let mut foo = create_foo_single();
    let bar = super::bar::create_bar_single();

    for subscope_to_update in &mut foo.scopes.get_mut("Foo").unwrap().subscopes {
        let new_scope = Scope::from_other(
            bar.scopes.get("Bar").unwrap(),
            subscope_to_update.get_trace_name().clone(),
        );

        *subscope_to_update = new_scope;
    }
    foo
}
