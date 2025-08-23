// build.rs
fn main() {
    cc::Build::new()
        .compiler("avr-gcc")
        .flag("-mmcu=atmega2560")
        .include("C:/avr/avr/include")

        .file("src/main.c")
        .file("src/main.S")
        .compile("ffi_lib");
}
