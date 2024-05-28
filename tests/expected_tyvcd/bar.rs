use tywaves_rs::tyvcd::spec::*;
use std::collections::HashMap;

pub fn create_bar_single() -> TyVcd {
    let mut scopes = HashMap::new();
    scopes.insert(
        String::from("Bar"),
        Scope::empty(
            String::from("Bar"),
            String::from("Bar"),
            TypeInfo::new("todo".to_string(), Vec::new()),
        ),
    );
    // inA
    scopes.get_mut("Bar").unwrap().variables.push(Variable::new(
        String::from("x"),
        String::from("inX"),
        TypeInfo::new("todo".to_string(), Vec::new()),
        VariableKind::Ground,
    ));
    // inB
    scopes.get_mut("Bar").unwrap().variables.push(Variable::new(
        String::from(""),
        String::from("outY"),
        TypeInfo::new("todo".to_string(), Vec::new()),
        VariableKind::Ground,
    ));
    // var1 => const
    scopes.get_mut("Bar").unwrap().variables.push(Variable::new(
        String::from(""),
        String::from("varZ"),
        TypeInfo::new("todo".to_string(), Vec::new()),
        VariableKind::Ground,
    ));

    scopes.get_mut("Bar").unwrap().variables.push(Variable::new(
        String::from(""),
        String::from("add"),
        TypeInfo::new("todo".to_string(), Vec::new()),
        VariableKind::Ground,
    ));


    TyVcd { scopes }
}