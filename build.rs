use cc;

fn main() {
    cc::Build::new()
        .file("tea32.c")
        .compile("tea32.o")
}