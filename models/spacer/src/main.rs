use fj::prelude::*;
use serde::Deserialize;

fn main() -> anyhow::Result<()> {
    fj::run_model(Spacer)
}

struct Spacer;

impl fj::Model for Spacer {
    type Params = Params;
    type Ty = fj::Difference<fj::Cylinder, fj::Cylinder>;

    fn instantiate(&self, params: Self::Params) -> Self::Ty {
        let outer = params.outer.unwrap_or(50.0);
        let inner = params.inner.unwrap_or(25.0);
        let height = params.height.unwrap_or(25.0);

        let outer = fj::Cylinder::new().with_radius(outer).with_height(height);
        let inner = fj::Cylinder::new().with_radius(inner).with_height(height);

        let spacer = (outer, inner).difference();

        spacer
    }
}

#[derive(Deserialize)]
struct Params {
    outer: Option<f32>,
    inner: Option<f32>,
    height: Option<f32>,
}
