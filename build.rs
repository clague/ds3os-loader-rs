use cc;

fn main() {
    cc::Build::new()
        .file("tea32.c")
        .opt_level_str("fast")
        .compile("tea32")
}