use std::{marker::PhantomData, os::raw::c_void, panic::AssertUnwindSafe};

use crate::abi::ffi_safe::StringSlice;

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
                Ok(Some(arg)) => {
                    crate::abi::ffi_safe::Result::Ok(StringSlice::from_str(arg))
                }
                Ok(None) => {
                    crate::abi::ffi_safe::Result::Ok(StringSlice::from_str(""))
                }
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
    fn get_argument(&self, name: &str) -> Option<&str> {
        unsafe {
            let Context {
                user_data,
                get_argument,
                _lifetime,
            } = *self;

            let name = StringSlice::from_str(name);

            match name.trim().is_empty() {
                true => None,
                false => match get_argument(user_data, name) {
                    super::ffi_safe::Result::Ok(other) => {
                        match other.is_empty() {
                            true => None,
                            false => Some(other.into_str()),
                        }
                    }
                    super::ffi_safe::Result::Err(_) => None,
                },
            }
        }
    }
}
