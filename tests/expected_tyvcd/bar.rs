use std::collections::HashMap;
use tywaves_rs::tyvcd::spec::*;

pub fn create_bar_single() -> TyVcd {
    let mut scopes = HashMap::new();
    scopes.insert(
        String::from("Bar"),
        Scope::empty(
            String::from("Bar"),
            String::from("Bar"),
            // TypeInfo::new("BarType".to_string(), Vec::new()),
            TypeInfo::new(
                "BarType".to_string(),
                vec![
                    ConstructorParams {
                        name: "size".to_string(),
                        tpe: "int".to_string(),
                        value: Some("32".to_string()),
                    },
                    ConstructorParams {
                        name: "other".to_string(),
                        tpe: "bool".to_string(),
                        value: None,
                    },
                ],
            ),
        ),
    );
    // inA
    scopes.get_mut("Bar").unwrap().variables.push(Variable::new(
        String::from("x"),
        String::from("inX"),
        TypeInfo::new("logic".to_string(), Vec::new()),
        VariableKind::Ground,
    ));
    // inB
    scopes.get_mut("Bar").unwrap().variables.push(Variable::new(
        String::from(""), // TODO: add trace name for this example (check an example vcd to check the correct one)
        String::from("outY"),
        TypeInfo::new("logic".to_string(), Vec::new()),
        VariableKind::Ground,
    ));
    // var1 => const
    scopes.get_mut("Bar").unwrap().variables.push(Variable::new(
        String::from(""),
        String::from("varZ"),
        TypeInfo::new("logic".to_string(), Vec::new()),
        VariableKind::Ground,
    ));

    scopes.get_mut("Bar").unwrap().variables.push(Variable::new(
        String::from(""),
        String::from("add"),
        TypeInfo::new("logic".to_string(), Vec::new()),
        VariableKind::Ground,
    ));

    TyVcd { scopes }
}
