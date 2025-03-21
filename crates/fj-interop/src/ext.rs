//! Extension traits for standard library types

/// Extension trait for arrays
pub trait ArrayExt<T, const N: usize> {
    /// Stable replacement for `each_mut`
    ///
    /// <https://doc.rust-lang.org/std/primitive.array.html#method.each_mut>
    fn each_mut_ext(&mut self) -> [&mut T; N];

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
    fn each_mut_ext(&mut self) -> [&mut T; 2] {
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
        let [q, r] = rhs;
        [(a, q), (b, r)]
    }
}

impl<T> ArrayExt<T, 3> for [T; 3] {
    fn each_mut_ext(&mut self) -> [&mut T; 3] {
        let [a, b, c] = self;
        [a, b, c]
    }

    fn try_map_ext<F, U, E>(self, f: F) -> Result<[U; 3], E>
    where
        F: FnMut(T) -> Result<U, E>,
    {
        let [a, b, c] = self.map(f);
        Ok([a?, b?, c?])
    }

    fn zip_ext<U>(self, rhs: [U; 3]) -> [(T, U); 3] {
        let [a, b, c] = self;
        let [q, r, s] = rhs;
        [(a, q), (b, r), (c, s)]
    }
}

impl<T> ArrayExt<T, 4> for [T; 4] {
    fn each_mut_ext(&mut self) -> [&mut T; 4] {
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
        let [q, r, s, t] = rhs;
        [(a, q), (b, r), (c, s), (d, t)]
    }
}

/// Extension trait for arrays
pub trait SliceExt<T> {
    /// Stable replacement for `array_chunks`
    ///
    /// <https://doc.rust-lang.org/std/primitive.slice.html#method.array_chunks>
    fn array_chunks_ext<const N: usize>(&self) -> ArrayChunks<T, N>;

    /// Stable replacement for `array_windows`
    ///
    /// <https://doc.rust-lang.org/std/primitive.slice.html#method.array_windows>
    fn array_windows_ext<const N: usize>(&self) -> ArrayWindows<T, N>;
}

impl<T> SliceExt<T> for &[T] {
    fn array_chunks_ext<const N: usize>(&self) -> ArrayChunks<T, N> {
        ArrayChunks {
            slice: self,
            index: 0,
        }
    }

    fn array_windows_ext<const N: usize>(&self) -> ArrayWindows<T, N> {
        ArrayWindows {
            slice: self,
            index: 0,
        }
    }
}

/// Returned by [`SliceExt::array_chunks_ext`]
pub struct ArrayChunks<'a, T: 'a, const N: usize> {
    slice: &'a [T],
    index: usize,
}

impl<'a, T, const N: usize> Iterator for ArrayChunks<'a, T, N> {
    type Item = &'a [T; N];

    fn next(&mut self) -> Option<Self::Item> {
        if self.index + N > self.slice.len() {
            return None;
        }

        let next = &self.slice[self.index..self.index + N];
        self.index += N;

        let next = next.try_into().unwrap();
        Some(next)
    }
}

/// Returned by [`SliceExt::array_windows_ext`]
pub struct ArrayWindows<'a, T: 'a, const N: usize> {
    slice: &'a [T],
    index: usize,
}

impl<'a, T, const N: usize> Iterator for ArrayWindows<'a, T, N> {
    type Item = &'a [T; N];

    fn next(&mut self) -> Option<Self::Item> {
        if self.index + N > self.slice.len() {
            return None;
        }

        let next = &self.slice[self.index..self.index + N];
        self.index += 1;

        let next = next.try_into().unwrap();
        Some(next)
    }
}
