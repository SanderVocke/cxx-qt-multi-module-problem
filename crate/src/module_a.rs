#[cxx_qt::bridge]
pub mod qobject {
    unsafe extern "RustQt" {
        #[qobject]
        type ModuleA = super::ModuleARust;
    }

    unsafe extern "RustQt" {
        #[qinvokable]
        fn module_a_fn(self: &ModuleA);
    }

    unsafe extern "C++" {
        include!("my_includes/make_unique.h");

        #[rust_name = "make_unique_module_a"]
        fn make_unique() -> UniquePtr<ModuleA>;
    }
}

use std::env;

#[derive(Default)]
pub struct ModuleARust {}

impl qobject::ModuleA {
    pub fn module_a_fn(self: &qobject::ModuleA) {
        println!("Hello world module A!");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::qobject::make_unique_module_a;

    #[test]
    fn test_hello_world_module_a() {
        let obj = make_unique_module_a();
        obj.module_a_fn();
    }
}