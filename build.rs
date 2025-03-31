fn main() {
   println!("cargo:rerun-if-changed=csrc/main.cpp");

   cc::Build::new()
      .cpp(true)
      .file("csrc/main.cpp")
      .compile("csrc");
}
