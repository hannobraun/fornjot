use fj::prelude::*;
use serde::Deserialize;

fn main() -> anyhow::Result<()> {
    fj::run_model(Spacer)
}

struct Spacer;

impl fj::Model for Spacer {
    type Params = Params;
    type Ty = fj::Difference<fj::Cylinder, fj::Cylinder>;

    fn instantiate(&self) -> Self::Ty {
        // TASK: Make it possible to pass those parameters in from the outside,
        //       for example via command-line arguments.
        let params = Params {
            outer: 50.0,
            inner: 25.0,
            height: 25.0,
        };

        let outer = fj::Cylinder::new()
            .with_radius(params.outer)
            .with_height(params.height);
        let inner = fj::Cylinder::new()
            .with_radius(params.inner)
            .with_height(params.height);

        let spacer = (outer, inner).difference();

        spacer
    }
}

#[derive(Deserialize)]
struct Params {
    outer: f32,
    inner: f32,
    height: f32,
}
