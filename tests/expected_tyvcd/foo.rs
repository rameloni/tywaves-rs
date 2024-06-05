use tywaves_rs::tyvcd::trace_pointer::{ConstValue, TraceGetter, TraceValue};

// External variable that will be captured.

use crate::tyvcd::spec::*;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

// Create the TyVcd for the [[tests/inputs/tyvcd/foo/foo_no_types.dd]] file.
pub fn create_foo_single_no_types() -> TyVcd {
    let mut scopes = HashMap::new();
    scopes.insert(
        String::from("Foo"),
        Rc::new(RefCell::new(Scope::empty(
            String::from("Foo"),
            String::from("Foo"),
            // type info na, use the target language one
            TypeInfo::new("Foo".to_string(), Vec::new()),
        ))),
    );
    // inA
    scopes
        .get("Foo")
        .unwrap()
        .borrow_mut()
        .variables
        .push(Variable::new(
            TraceValue::RefTraceName("a".to_string()),
            String::from("inA"),
            // type info na, use the target language one
            TypeInfo::new("logic".to_string(), Vec::new()),
            VariableKind::Ground(32),
        ));
    // inB
    scopes
        .get("Foo")
        .unwrap()
        .borrow_mut()
        .variables
        .push(Variable::new(
            TraceValue::RefTraceName("b".to_string()),
            String::from("outB"),
            // type info na, use the target language one
            TypeInfo::new("logic".to_string(), Vec::new()),
            VariableKind::Ground(32),
        ));
    // var1 => const
    let var1_val = String::from("00101010");
    scopes
        .get("Foo")
        .unwrap()
        .borrow_mut()
        .variables
        .push(Variable::new(
            TraceValue::Constant(ConstValue::FourValue(var1_val.into_bytes(), 8)),
            String::from("var1"),
            // type info na, use the target language one
            TypeInfo::new("logic".to_string(), Vec::new()),
            VariableKind::Ground(8),
        ));

    // instances
    scopes.get("Foo").unwrap().borrow_mut().subscopes.insert(
        String::from("b0"),
        Rc::new(RefCell::new(Scope::empty(
            String::from("b0"),
            String::from("Bar"),
            // type info na, use the target language one
            TypeInfo::new("Bar".to_string(), Vec::new()),
        ))),
    );

    scopes.get("Foo").unwrap().borrow_mut().subscopes.insert(
        String::from("b1"),
        Rc::new(RefCell::new(Scope::empty(
            String::from("b1"),
            String::from("Bar"),
            // type info na, use the target language one
            TypeInfo::new("Bar".to_string(), Vec::new()),
        ))),
    );

    TyVcd { scopes }
}

// Create the TyVcd for the [[tests/inputs/tyvcd/foo/foo.dd]] file.
pub fn create_foo_single() -> TyVcd {
    let mut scopes = HashMap::new();
    scopes.insert(
        String::from("Foo"),
        Rc::new(RefCell::new(Scope::empty(
            String::from("Foo"),
            String::from("Foo"),
            TypeInfo::new("Foo".to_string(), Vec::new()),
        ))),
    );
    // inA
    scopes
        .get("Foo")
        .unwrap()
        .borrow_mut()
        .variables
        .push(Variable::new(
            TraceValue::RefTraceName("a".to_string()),
            String::from("inA"),
            TypeInfo::new("SInt<32>".to_string(), Vec::new()),
            VariableKind::Ground(32),
        ));
    // inB
    scopes
        .get("Foo")
        .unwrap()
        .borrow_mut()
        .variables
        .push(Variable::new(
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
        ));
    // var1 => const
    let var1_val = String::from("00101010");
    scopes
        .get("Foo")
        .unwrap()
        .borrow_mut()
        .variables
        .push(Variable::new(
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
        ));

    // var2
    let var2_val = String::from("00010101");
    scopes
        .get("Foo")
        .unwrap()
        .borrow_mut()
        .variables
        .push(Variable::new(
            TraceValue::Constant(ConstValue::FourValue(var2_val.into_bytes(), 8)),
            String::from("var2"),
            // type info na, use the target language one
            TypeInfo::new("logic".to_string(), Vec::new()),
            VariableKind::Ground(8),
        ));

    // instances
    scopes.get("Foo").unwrap().borrow_mut().subscopes.insert(
        String::from("b0"),
        Rc::new(RefCell::new(Scope::empty(
            String::from("b0"),
            String::from("Bar"),
            // type info na, use the target language one
            TypeInfo::new("Bar".to_string(), Vec::new()),
        ))),
    );

    scopes.get("Foo").unwrap().borrow_mut().subscopes.insert(
        "b1".to_string(),
        Rc::new(RefCell::new(Scope::empty(
            String::from("b1"),
            String::from("Bar"),
            // type info na, use the target language one
            TypeInfo::new("Bar".to_string(), Vec::new()),
        ))),
    );

    TyVcd { scopes }
}

// Create the TyVcd for the [[tests/inputs/tyvcd/foo]] directory.
pub fn create_foo() -> TyVcd {
    let mut foo = create_foo_single();
    let bar = super::bar::create_bar_single();

    for (_, subscope_to_update) in &mut foo.scopes.get("Foo").unwrap().borrow_mut().subscopes {
        let new_scope = Scope::from_other(
            &bar.scopes.get("Bar").unwrap().borrow().clone(),
            subscope_to_update
                .borrow()
                .get_trace_name()
                .unwrap()
                .clone(),
        );

        *subscope_to_update = Rc::new(RefCell::new(new_scope));
    }
    foo
}
