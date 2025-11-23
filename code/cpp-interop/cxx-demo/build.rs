fn main() {
    cxx_build::bridge("src/main.rs")
        .file("src/rectangle.cpp")
        .include("include")
        .flag_if_supported("-std=c++17") // Use C++17 standard
        .std("c++17")
        .compile("cxxbridge-demo");

    println!("cargo:rerun-if-changed=src/rectangle.cpp");
    println!("cargo:rerun-if-changed=include/rectangle.hpp");
}
