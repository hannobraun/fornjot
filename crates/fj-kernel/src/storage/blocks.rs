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

    pub fn reserve(&mut self) -> (Index, *const Option<T>) {
        let mut current_block = match self.inner.pop() {
            Some(block) => block,
            None => Block::new(self.block_size),
        };

        let ret = loop {
            match current_block.reserve() {
                Ok((object_index, ptr)) => {
                    let block_index = BlockIndex(self.inner.len());
                    break (
                        Index {
                            block_index,
                            object_index,
                        },
                        ptr,
                    );
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

    pub fn insert(&mut self, index: Index, object: T) -> *const Option<T> {
        let block = &mut self.inner[index.block_index.0];
        block.insert(index.object_index, object)
    }

    pub fn get_and_inc(&self, index: &mut Index) -> Option<&Option<T>> {
        let block = self.inner.get(index.block_index.0)?;
        let object = block.get(index.object_index);

        index.inc(block);

        Some(object)
    }

    #[cfg(test)]
    pub fn iter(&self) -> impl Iterator<Item = &T> + '_ {
        self.inner.iter().flat_map(|block| block.iter())
    }
}

#[derive(Debug)]
pub struct Block<T> {
    objects: Box<[Option<T>]>,
    next: ObjectIndex,
}

impl<T> Block<T> {
    pub fn new(size: usize) -> Self {
        let vec = iter::repeat_with(|| None)
            .take(size)
            .collect::<Vec<Option<T>>>();
        let objects = vec.into_boxed_slice();

        Self {
            objects,
            next: ObjectIndex(0),
        }
    }

    pub fn reserve(&mut self) -> Result<(ObjectIndex, *const Option<T>), ()> {
        if self.next.0 >= self.objects.len() {
            return Err(());
        }

        let index = self.next;
        let ptr = &mut self.objects[self.next.0];
        self.next.0 += 1;

        Ok((index, ptr))
    }

    pub fn insert(
        &mut self,
        index: ObjectIndex,
        object: T,
    ) -> *const Option<T> {
        self.objects[index.0] = Some(object);
        &self.objects[index.0]
    }

    pub fn get(&self, index: ObjectIndex) -> &Option<T> {
        &self.objects[index.0]
    }

    pub fn len(&self) -> usize {
        self.next.0
    }

    #[cfg(test)]
    pub fn iter(&self) -> impl Iterator<Item = &T> + '_ {
        let mut i = 0;
        iter::from_fn(move || {
            if i >= self.len() {
                return None;
            }

            let object = self.get(ObjectIndex(i)).as_ref()?;
            i += 1;

            Some(object)
        })
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Index {
    pub block_index: BlockIndex,
    pub object_index: ObjectIndex,
}

impl Index {
    pub fn zero() -> Self {
        Self {
            block_index: BlockIndex(0),
            object_index: ObjectIndex(0),
        }
    }

    pub fn inc<T>(&mut self, block: &Block<T>) {
        self.object_index.0 += 1;
        if self.object_index.0 >= block.len() {
            self.block_index.0 += 1;
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct BlockIndex(pub usize);

#[derive(Clone, Copy, Debug)]
pub struct ObjectIndex(pub usize);

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
