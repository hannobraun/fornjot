use fj_interop::ext::ArrayExt;

use super::{HasPartial, MaybePartial};

pub fn merge_options<T>(a: Option<T>, b: Option<T>) -> Option<T>
where
    T: Eq,
{
    if a == b {
        return a;
    }

    // We know that `a != b`, or we wouldn't have made it here.
    if a.is_some() && b.is_some() {
        panic!("Can't merge `Option`s if both are defined");
    }

    a.xor(b)
}

pub fn merge_arrays<T: HasPartial>(
    a: [MaybePartial<T>; 2],
    b: [MaybePartial<T>; 2],
) -> [MaybePartial<T>; 2] {
    a.zip_ext(b).map(|(a, b)| a.merge_with(b))
}
