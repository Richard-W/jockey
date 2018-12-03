/// Implemented by types emittable in Arguments::emit_args().
pub trait Emittable {

    /// Return a vector containing the emitted arguments.
    fn emit_args(&self, long_option: String) -> Vec<String>;
}


impl Emittable for String {
    fn emit_args(&self, long_option: String) -> Vec<String> {
        vec![long_option, self.to_string()]
    }
}

impl Emittable for bool {
    fn emit_args(&self, long_option: String) -> Vec<String> {
        if self.clone() {
            vec![long_option]
        }
        else {
            vec![]
        }
    }
}

impl<T: Emittable> Emittable for Option<T> {
    fn emit_args(&self, long_option: String) -> Vec<String> {
        match self {
            Some(ref val) => val.emit_args(long_option),
            None => vec![],
        }
    }
}
