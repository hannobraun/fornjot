use iter_fixed::IntoIteratorFixed;

use super::{HasPartial, MaybePartial, MergeWith};

pub fn merge_arrays<T: HasPartial>(
    a: [MaybePartial<T>; 2],
    b: [MaybePartial<T>; 2],
) -> [MaybePartial<T>; 2] {
    a.into_iter_fixed()
        .zip(b)
        .collect::<[_; 2]>()
        .map(|(a, b)| a.merge_with(b))
}
