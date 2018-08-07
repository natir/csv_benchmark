use std::process::Command;

fn main() {
    let output = Command::new("g++").args(&["src/lib.cpp", "-Wredundant-decls", "-Wcast-align", "-Wmissing-declarations", "-Wmissing-include-dirs", "-Wswitch-enum", "-Wswitch-default", "-Wextra", "-Wall", "-Werror", "-Winvalid-pch", "-Wredundant-decls", "-Wformat=2", "-Wmissing-format-attribute", "-Wformat-nonliteral", "-O3", "-flto", "-march=native", "-mtune=native", "-I", "src/", "-lpthread", "-o"])
        .arg("target/cpp_version")
        .output()
        .expect("failled to build");

    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    println!("cargo:rerun-if-changed=src/lib.cpp");
}
