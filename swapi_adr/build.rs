//Android specific
use rust_swig::{JavaConfig, LanguageConfig};
use std::{path::Path};

fn main() {
    setup_swig();
}

fn setup_swig() {
    println!("cargo:warning=Build: Android setup");
    let out_dir = Path::new("src/swig/");
    let in_src = Path::new("src/swig/").join("java_glue.rs.in");
    let out_src = Path::new(&out_dir).join("java_glue.rs");
    //ANCHOR: config
    let swig_gen = rust_swig::Generator::new(LanguageConfig::JavaConfig(
        JavaConfig::new(
            Path::new("..")
                .join("SwapiAndroidApp")
                .join("app")
                .join("src")
                .join("main")
                .join("java")
                .join("com")
                .join("rust")
                .join("app")
                .join("swapiclient")
                .join("swapi"),
            "com.rust.app.swapiclient.swapi".into(),
        )
            .use_null_annotation_from_package("androidx.annotation".into()),
    ))
        .rustfmt_bindings(true);

    swig_gen.expand("android bindings", &in_src, &out_src);

    println!("cargo:rerun-if-changed={}", in_src.display());
}