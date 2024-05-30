/// Trait with methods to return the name and path of an object in the trace file.
pub trait TraceGetter {
    /// Return the name of the object in the trace file (it does not consider the path).
    fn get_trace_name(&self) -> &String;
    /// Return the path of the object in the trace file (the path is a sequence of scopes + the trace_name).
    fn get_trace_path(&self) -> Vec<&String> {
        vec![self.get_trace_name()]
    }
}

/// Trait to find a trace path in a data structure.
pub trait TraceFinder {
    /// Return the element pointin to the trace path.
    fn find_trace(&self, path: &[String]) -> Option<&dyn TraceGetter>;
}
