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
                "src/module_b.rs",
            ],
            qml_files: &[] as &[&'static str],
            ..Default::default()
        })
        .build();
}