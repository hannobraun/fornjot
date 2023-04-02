use std::{marker::PhantomData, os::raw::c_void, panic::AssertUnwindSafe};

use crate::{abi::ffi_safe::StringSlice, models::Error};

#[repr(C)]
pub struct Context<'a> {
    user_data: *const c_void,
    get_argument: unsafe extern "C" fn(
        *const c_void,
        StringSlice,
    ) -> crate::abi::ffi_safe::Result<
        StringSlice,
        crate::abi::ffi_safe::String,
    >,
    _lifetime: PhantomData<&'a ()>,
}

impl<'a> From<&'a &dyn crate::models::Context> for Context<'a> {
    fn from(ctx: &'a &dyn crate::models::Context) -> Self {
        unsafe extern "C" fn get_argument(
            user_data: *const c_void,
            name: StringSlice,
        ) -> crate::abi::ffi_safe::Result<
            StringSlice,
            crate::abi::ffi_safe::String,
        > {
            let ctx = &*(user_data as *const &dyn crate::models::Context);

            match std::panic::catch_unwind(AssertUnwindSafe(|| {
                ctx.get_argument(&name)
            })) {
                Ok(result) => match result {
                    crate::abi::ffi_safe::Result::Ok(option) => match option {
                        Some(arg) => crate::abi::ffi_safe::Result::Ok(
                            StringSlice::from_str(arg),
                        ),
                        None => crate::abi::ffi_safe::Result::Ok(
                            StringSlice::from_str(""),
                        ),
                    },
                    crate::abi::ffi_safe::Result::Err(_) => {
                        crate::abi::ffi_safe::Result::Err(
                            String::from("Problem handling model arguments")
                                .into(),
                        )
                    }
                    //TODO: I'm not familiar enough with the codebase yet to know the best way to
                    // handle this error, but it will almost certainly need to be handled better
                    // than this in the future
                },
                Err(payload) => crate::abi::ffi_safe::Result::Err(
                    crate::abi::on_panic(payload),
                ),
            }
        }

        Context {
            user_data: ctx as *const &dyn crate::models::Context
                as *const c_void,
            get_argument,
            _lifetime: PhantomData,
        }
    }
}

impl crate::models::Context for Context<'_> {
    fn get_argument(
        &self,
        name: &str,
    ) -> super::ffi_safe::Result<Option<&str>, Error> {
        unsafe {
            let Context {
                user_data,
                get_argument,
                _lifetime,
            } = *self;

            let name = StringSlice::from_str(name);

            match name.trim().is_empty() {
                true => super::ffi_safe::Result::Ok(None),
                false => match get_argument(user_data, name) {
                    super::ffi_safe::Result::Ok(other) => {
                        match other.is_empty() {
                            true => Ok(None).into(),
                            false => Ok(Some(other.into_str())).into(),
                        }
                    }
                    _ => Err("Problem handling model arguments".into()).into(),
                },
            }
        }
    }
}
