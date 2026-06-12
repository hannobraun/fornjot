//! Custom test runner

fn main() {
    for test in fj::tests::all() {
        test();
    }
}
