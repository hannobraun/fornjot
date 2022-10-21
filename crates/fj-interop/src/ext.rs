//! Extension traits for standard library types

/// Extension trait for arrays
pub trait ArrayExt<T, const N: usize> {
    /// Stable replacement for `each_ref`
    ///
    /// <https://doc.rust-lang.org/std/primitive.array.html#method.each_ref>
    fn each_ref_ext(&self) -> [&T; N];

    /// Stable replacement for `try_map`
    ///
    /// <https://doc.rust-lang.org/std/primitive.array.html#method.try_map>
    fn try_map_ext<F, U, E>(self, f: F) -> Result<[U; N], E>
    where
        F: FnMut(T) -> Result<U, E>;

    /// Stable replacement for `zip`
    ///
    /// <https://doc.rust-lang.org/std/primitive.array.html#method.zip>
    fn zip_ext<U>(self, rhs: [U; N]) -> [(T, U); N];
}

impl<T> ArrayExt<T, 2> for [T; 2] {
    fn each_ref_ext(&self) -> [&T; 2] {
        let [a, b] = self;
        [a, b]
    }

    fn try_map_ext<F, U, E>(self, f: F) -> Result<[U; 2], E>
    where
        F: FnMut(T) -> Result<U, E>,
    {
        let [a, b] = self.map(f);
        Ok([a?, b?])
    }

    fn zip_ext<U>(self, rhs: [U; 2]) -> [(T, U); 2] {
        let [a, b] = self;
        let [c, d] = rhs;

        [(a, c), (b, d)]
    }
}

impl<T> ArrayExt<T, 4> for [T; 4] {
    fn each_ref_ext(&self) -> [&T; 4] {
        let [a, b, c, d] = self;
        [a, b, c, d]
    }

    fn try_map_ext<F, U, E>(self, f: F) -> Result<[U; 4], E>
    where
        F: FnMut(T) -> Result<U, E>,
    {
        let [a, b, c, d] = self.map(f);
        Ok([a?, b?, c?, d?])
    }

    fn zip_ext<U>(self, rhs: [U; 4]) -> [(T, U); 4] {
        let [a, b, c, d] = self;
        let [e, f, g, h] = rhs;

        [(a, e), (b, f), (c, g), (d, h)]
    }
}
