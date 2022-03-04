pub trait CodegenDerive {
    fn _derive(&mut self, name: &str);

    fn derive_multiple(&mut self, names: &[&str]) {
        for name in names {
            self._derive(name);
        }
    }
}

impl CodegenDerive for codegen::Enum {
    fn _derive(&mut self, name: &str) {
        self.derive(name);
    }
}

impl CodegenDerive for codegen::Struct {
    fn _derive(&mut self, name: &str) {
        self.derive(name);
    }
}