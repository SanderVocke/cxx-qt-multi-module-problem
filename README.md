# cxx-qt multi-module problem

Given this cxx-qt crate, observe the following:

- `cargo build` succeeds. Static library is generated.
- `cargo test` fails with several multiple definition errors and several undefined symbol errors.

Now, comment out one of the modules, both in `build.rs` (the .rs file) and in `lib.rs` (the library file). Example:

lib.rs
```
pub mod module_a;
// pub mod module_b;
```

build.rs
```
use cxx_qt_build::{CxxQtBuilder, QmlModule};

fn main() {
    CxxQtBuilder::new()
        .cc_builder(|cc| {
            cc.include("src/cxx");
        })
        .qt_module("Network")
        .qml_module(QmlModule {
            uri: "qml_module",
            rust_files: &[
                "src/module_a.rs",
                // "src/module_b.rs",
            ],
            qml_files: &[] as &[&'static str],
            ..Default::default()
        })
        .build();
}
```

Now, `cargo test` succeeds.

# Question

How to modify this structure such that both modules can be compiled into the same static library and `cargo test` can run all of the tests together?