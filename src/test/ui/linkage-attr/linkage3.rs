// FIXME https://github.com/rust-lang/rust/issues/59774

// check-fail
// normalize-stderr-test "thread.*panicked.*Metadata module not compiled.*\n" -> ""
// normalize-stderr-test "note:.*RUST_BACKTRACE=1.*\n" -> ""

#![feature(linkage)]

extern "C" {
    #[linkage = "foo"]
    static foo: *const i32;
//~^ ERROR: invalid linkage specified
}

fn main() {
    println!("{:?}", unsafe { foo });
}
