use std::iter;

#[derive(Debug)]
pub struct Blocks<T> {
    inner: Vec<Block<T>>,
    block_size: usize,
}

impl<T> Blocks<T> {
    pub fn new(block_size: usize) -> Self {
        Self {
            inner: Vec::new(),
            block_size,
        }
    }

    pub fn push(&mut self, object: T) -> *const Option<T> {
        let (index, _) = self.reserve();
        self.insert(index, object)
    }

    pub fn reserve(&mut self) -> ((usize, usize), *mut Option<T>) {
        let mut current_block = match self.inner.pop() {
            Some(block) => block,
            None => Block::new(self.block_size),
        };

        let ret = loop {
            match current_block.reserve() {
                Ok((object_index, ptr)) => {
                    let block_index = self.inner.len();
                    break ((block_index, object_index), ptr);
                }
                Err(()) => {
                    // Block is full. Need to create a new one and retry.
                    self.inner.push(current_block);
                    current_block = Block::new(self.block_size);
                }
            }
        };

        self.inner.push(current_block);

        ret
    }

    pub fn insert(
        &mut self,
        (block_index, object_index): (usize, usize),
        object: T,
    ) -> *const Option<T> {
        let block = &mut self.inner[block_index];
        block.insert(object_index, object)
    }

    pub fn get(&self, index: usize) -> Option<&Block<T>> {
        self.inner.get(index)
    }

    #[cfg(test)]
    pub fn iter(&self) -> impl Iterator<Item = &T> + '_ {
        self.inner.iter().flat_map(|block| block.iter())
    }
}

#[derive(Debug)]
pub struct Block<T> {
    objects: Box<[Option<T>]>,
    next: usize,
}

impl<T> Block<T> {
    pub fn new(size: usize) -> Self {
        let vec = iter::repeat_with(|| None)
            .take(size)
            .collect::<Vec<Option<T>>>();
        let objects = vec.into_boxed_slice();

        Self { objects, next: 0 }
    }

    pub fn reserve(&mut self) -> Result<(usize, *mut Option<T>), ()> {
        if self.next >= self.objects.len() {
            return Err(());
        }

        let index = self.next;
        let ptr = &mut self.objects[self.next];
        self.next += 1;

        Ok((index, ptr))
    }

    pub fn insert(&mut self, index: usize, object: T) -> *const Option<T> {
        self.objects[index] = Some(object);
        &self.objects[index]
    }

    pub fn get(&self, index: usize) -> &Option<T> {
        &self.objects[index]
    }

    pub fn len(&self) -> usize {
        self.next
    }

    #[cfg(test)]
    pub fn iter(&self) -> impl Iterator<Item = &T> + '_ {
        let mut i = 0;
        iter::from_fn(move || {
            if i >= self.len() {
                return None;
            }

            let object = self.get(i).as_ref()?;
            i += 1;

            Some(object)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Blocks;

    #[test]
    fn push() {
        let mut blocks = Blocks::new(1);

        blocks.push(0);
        blocks.push(1);

        let objects = blocks.iter().copied().collect::<Vec<_>>();
        assert_eq!(objects, [0, 1]);
    }
}
