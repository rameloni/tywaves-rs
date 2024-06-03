use std::collections::HashMap;
use tywaves_rs::tyvcd::{spec::*, trace_pointer::TraceValue};

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
        TraceValue::RefTraceName("x".to_string()),
        String::from("inX"),
        TypeInfo::new("logic".to_string(), Vec::new()),
        VariableKind::Ground(32),
    ));
    // inB
    scopes.get_mut("Bar").unwrap().variables.push(Variable::new(
        // op: *
        TraceValue::RefTraceValues(vec![
            TraceValue::RefTraceName("x".to_string()),
            TraceValue::RefTraceName("x".to_string()),
        ]),
        String::from("outY"),
        TypeInfo::new("logic".to_string(), Vec::new()),
        VariableKind::Ground(32),
    ));
    // var1 => const
    scopes.get_mut("Bar").unwrap().variables.push(Variable::new(
        // op: *
        TraceValue::RefTraceValues(vec![
            TraceValue::RefTraceName("x".to_string()),
            TraceValue::RefTraceName("x".to_string()),
        ]),
        String::from("varZ"),
        TypeInfo::new("logic".to_string(), Vec::new()),
        VariableKind::Ground(32),
    ));

    scopes.get_mut("Bar").unwrap().variables.push(Variable::new(
        // op: +
        TraceValue::RefTraceValues(vec![
            // op: *
            TraceValue::RefTraceValues(vec![
                TraceValue::RefTraceName("x".to_string()),
                TraceValue::RefTraceName("x".to_string()),
            ]),
            TraceValue::RefTraceName("x".to_string()),
        ]),
        String::from("add"),
        TypeInfo::new("logic".to_string(), Vec::new()),
        VariableKind::Ground(32),
    ));

    TyVcd { scopes }
}
