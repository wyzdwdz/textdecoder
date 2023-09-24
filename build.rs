use build_target::*;

fn main() {
    let mut build = cxx_build::bridge("src/ced.rs");

    build
        .file("3rdparty/compact_enc_det/compact_enc_det/compact_enc_det.cc")
        .file("3rdparty/compact_enc_det/compact_enc_det/compact_enc_det_hint_code.cc")
        .file("3rdparty/compact_enc_det/util/encodings/encodings.cc")
        .file("3rdparty/compact_enc_det/util/languages/languages.cc")
        .include("3rdparty/compact_enc_det")
        .std("c++14")
        .flag_if_supported("-Wno-narrowing")
        .flag_if_supported("-Wno-c++11-narrowing");

    if target_os().unwrap() == Os::Windows {
        build
            .define("UNICODE", None)
            .define("_UNICODE", None)
            .define("STRICT", None)
            .define("NOMINMAX", None);
    }

    build.compile("ced");

    println!("cargo:rerun-if-changed=src/ced.rs");

    if target_os().unwrap() == Os::Windows && target_env().unwrap() == Env::GNU {
        println!("cargo:rustc-link-lib=static=stdc++");
    }
}
