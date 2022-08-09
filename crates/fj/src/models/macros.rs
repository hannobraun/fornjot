/// Define the initialization routine used when registering models.
///
/// See the [`crate::model`] macro if your model can be implemented as a pure
/// function.
///
/// # Examples
///
/// ```rust
/// use fj::models::{*, internal::RResult};
///
/// fj::register_model!(|host: &mut dyn Host| {
///     host.register_model(MyModel::default());
///
///     Ok(Metadata::new(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")))
/// });
///
/// #[derive(Default)]
/// struct MyModel { }
///
/// impl Model for MyModel {
///     fn metadata(&self) -> ModelMetadata { todo!() }
///
///     fn shape(&self, ctx: fj::models::internal::Context) -> RResult<fj::Shape, Error> {
///         todo!()
///     }
/// }
/// ```
#[macro_export]
macro_rules! register_model {
    ($init:expr) => {
        #[no_mangle]
        unsafe extern "C" fn fj_model_init(
            mut host: $crate::models::internal::Host,
        ) -> $crate::models::internal::RResult<
            $crate::models::Metadata,
            $crate::models::Error,
        > {
            let init: fn(
                &mut dyn $crate::models::Host,
            ) -> Result<
                $crate::models::Metadata,
                $crate::models::Error,
            > = $init;

            match init(&mut host) {
                Ok(meta) => $crate::models::internal::ROk(meta.into()),
                Err(e) => $crate::models::internal::RErr(e.into()),
            }
        }
    };
}
