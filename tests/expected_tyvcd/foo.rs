use tywaves_rs::tyvcd::trace_pointer::TracePointer;

// External variable that will be captured.
use crate::tyvcd::builder::*;
use crate::tyvcd::spec::*;

use super::*;
use std::{collections::HashMap, process::id};

// Create the TyVcd for the [[tests/inputs/tyvcd/foo/foo.dd]] file.
pub fn create_foo_single() -> TyVcd {
    let mut scopes = HashMap::new();
    scopes.insert(
        String::from("Foo"),
        Scope::empty(
            String::from("Foo"),
            String::from("Foo"),
            TypeInfo::new("todo".to_string(), Vec::new()),
        ),
    );
    // inA
    scopes.get_mut("Foo").unwrap().variables.push(Variable::new(
        String::from("a"),
        String::from("inA"),
        TypeInfo::new("todo".to_string(), Vec::new()),
        VariableKind::Ground,
    ));
    // inB
    scopes.get_mut("Foo").unwrap().variables.push(Variable::new(
        String::from("b"),
        String::from("outB"),
        TypeInfo::new("todo".to_string(), Vec::new()),
        VariableKind::Ground,
    ));
    // var1 => const
    scopes.get_mut("Foo").unwrap().variables.push(Variable::new(
        String::from("00101010"),
        String::from("var1"),
        TypeInfo::new("todo".to_string(), Vec::new()),
        VariableKind::Ground,
    ));

    // instances
    scopes.get_mut("Foo").unwrap().subscopes.push(Scope::empty(
        String::from("b0"),
        String::from("Bar"),
        TypeInfo::new("todo".to_string(), Vec::new()),
    ));

    scopes.get_mut("Foo").unwrap().subscopes.push(Scope::empty(
        String::from("b1"),
        String::from("Bar"),
        TypeInfo::new("todo".to_string(), Vec::new()),
    ));

    TyVcd { scopes }
}

// Create the TyVcd for the [[tests/inputs/tyvcd/foo]] directory.
pub fn create_foo() -> TyVcd {
    let mut foo = create_foo_single();
    let bar = super::bar::create_bar_single();

    for subscope_to_update in &mut foo.scopes.get_mut("Foo").unwrap().subscopes {
        let mut new_scope = Scope::from_other(
            bar.scopes.get("Bar").unwrap(),
            subscope_to_update.get_trace_name(),
        );

        *subscope_to_update = new_scope;
    }
    foo
}
