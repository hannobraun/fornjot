use crate::Parameters;

pub struct Host<'a> {
    args: &'a Parameters,
    model: Option<Box<dyn fj::models::Model>>,
}

impl<'a> Host<'a> {
    pub fn new(parameters: &'a Parameters) -> Self {
        Self {
            args: parameters,
            model: None,
        }
    }

    pub fn take_model(&mut self) -> Option<Box<dyn fj::models::Model>> {
        self.model.take()
    }
}

impl<'a> fj::models::Host for Host<'a> {
    fn register_boxed_model(&mut self, model: Box<dyn fj::models::Model>) {
        self.model = Some(model);
    }
}

impl<'a> fj::models::Context for Host<'a> {
    fn get_argument(&self, name: &str) -> Option<&str> {
        self.args.get(name).map(|s| s.as_str())
    }
}
