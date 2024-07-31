#[cxx_qt::bridge]
pub mod qobject {
    unsafe extern "RustQt" {
        #[qobject]
        type ModuleB = super::ModuleBRust;
    }

    unsafe extern "RustQt" {
        #[qinvokable]
        fn module_b_fn(self: &ModuleB);
    }

    unsafe extern "C++" {
        include!("my_includes/make_unique.h");

        #[rust_name = "make_unique_module_b"]
        fn make_unique() -> UniquePtr<ModuleB>;
    }
}

use std::env;

#[derive(Default)]
pub struct ModuleBRust {}

impl qobject::ModuleB {
    pub fn module_b_fn(self: &qobject::ModuleB) {
        println!("Hello world module B!");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::qobject::make_unique_module_b;

    #[test]
    fn test_hello_world_module_b() {
        let obj = make_unique_module_b();
        obj.module_b_fn();
    }
}