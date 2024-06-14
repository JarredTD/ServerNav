#[derive(Debug)]
pub struct BoundedVec<T> {
    vec: Vec<T>,
    max_capacity: usize,
}

impl<T> BoundedVec<T> {
    pub fn new(max_capacity: usize) -> Self {
        Self {
            vec: Vec::with_capacity(max_capacity),
            max_capacity,
        }
    }

    pub fn push(&mut self, value: T) {
        while self.vec.len() >= self.max_capacity {
            self.vec.remove(0);
        }
        self.vec.push(value);
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }

    pub fn capacity(&self) -> usize {
        self.max_capacity
    }

    pub fn is_empty(&self) -> bool {
        self.vec.len() == 0
    }

    pub fn is_full(&self) -> bool {
        self.vec.len() >= self.max_capacity
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.vec.get(index)
    }
}

impl<T> std::ops::Deref for BoundedVec<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.vec
    }
}

impl<T> std::ops::DerefMut for BoundedVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.vec
    }
}
