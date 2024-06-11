use std::sync::{Arc, RwLock};

/// Trait with methods to return the name and path of an object in the trace file.
///
/// Any type that implements this trait is able to return a [TraceValue].
pub trait TraceGetter {
    /// Return the name of the object in the trace file (it does not consider the path).
    fn get_trace_name(&self) -> Option<&String> {
        match self.get_trace_value() {
            TraceValue::RefTraceName(name) => Some(name),
            _ => None,
        }
    }
    /// Return the path of the object in the trace file (the path is a sequence of scopes + the trace_name).
    fn get_trace_path(&self) -> Vec<&String>;

    fn get_trace_value(&self) -> &TraceValue;

    fn as_any(&self) -> &dyn std::any::Any;
}

/// Trait to find a trace path in a data structure.
///
/// Any type that implements this trait is able to find a [TraceGetter] given a `path`: a sequence of [TraceId].
///
/// **Important**: The returned reference is a reference counted with interior mutability.
/// This means that preventing to internal changes is left to the user.
pub trait TraceFinder {
    /// Return the element pointin to the trace path.
    // fn find_trace<'a>(&'a self, path: &[String]) -> Option<Ref<'a, dyn TraceGetter>>;
    fn find_trace(&self, path: &[String]) -> Option<Arc<RwLock<dyn TraceGetter>>>;
}

#[derive(Debug, Clone, PartialEq)]
pub enum TraceValue {
    /// The trace value has a reference name.
    /// It means that the actual value is stored in a reference indicated by a trace name.
    RefTraceName(String),
    /// The value is a constant value.
    /// The trace value contains the value itself.
    Constant(ConstValue),
    /// A reference to multiple trace values.
    /// The actual value is the result of an operation between multiple trace values.
    RefTraceValues(Vec<TraceValue>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConstValue {
    Binary(Vec<u8>, u32),
    FourValue(Vec<u8>, u32),
    String(String),
    Real(f64),
}

// impl PartialEq for TraceValue<'_> {
//     fn eq(&self, other: &Self) -> bool {
//         match (self, other) {
//             (TraceValue::RefTraceName(a), TraceValue::RefTraceName(b)) => a == b,
//             (TraceValue::Constant(a), TraceValue::Constant(b)) => match a {
//                 wellen::SignalValue::Binary(va, sa) => match b {
//                     wellen::SignalValue::Binary(vb, sb) => va == vb && sa == sb,
//                     _ => false,
//                 },
//                 wellen::SignalValue::FourValue(va, sa) => match b {
//                     wellen::SignalValue::FourValue(vb, sb) => va == vb && sa == sb,
//                     _ => false,
//                 },
//                 wellen::SignalValue::NineValue(va, sa) => match b {
//                     wellen::SignalValue::NineValue(vb, sb) => va == vb && sa == sb,
//                     _ => false,
//                 },
//                 wellen::SignalValue::String(sa) => match b {
//                     wellen::SignalValue::String(sb) => sa == sb,
//                     _ => false,
//                 },
//                 wellen::SignalValue::Real(va) => match b {
//                     wellen::SignalValue::Real(vb) => va == vb,
//                     _ => false,
//                 },
//             },

//             (TraceValue::RefTraceValues(a), TraceValue::RefTraceValues(b)) => a == b,
//             _ => false,
//         }
//     }
// }
