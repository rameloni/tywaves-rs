/// Trait to describe objects with a pointer to a trace name.
pub trait TracePointer {
    fn get_trace_name(&self) -> String;
    fn get_trace_path(&self) -> String {
        self.get_trace_name()
    }
}
