//! Custom test runner

fn main() -> anyhow::Result<()> {
    for test in fj::tests::all() {
        test();
    }

    Ok(())
}
