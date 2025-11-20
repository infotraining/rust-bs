fn main() {
    cc::Build::new()
        .cpp(true)
        .file("src/rectangle.cpp")
        .file("src/rectangle_wrapper.cpp")
        .include("./include")
        .flag_if_supported("-std=c++17")
        .compile("rectangle");

    let bindings = bindgen::Builder::default()
        .header("include/rectangle_wrapper.hpp")
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file("src/bindings.rs")
        .expect("Couldn't write bindings!");

    println!("cargo:rerun-if-changed=include/rectangle_wrapper.hpp");
    println!("cargo:rerun-if-changed=src/rectangle_wrapper.cpp");
    println!("cargo:rerun-if-changed=include/rectangle.hpp");

}
