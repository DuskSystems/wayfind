#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Key {
    index: usize,
    generation: usize,
}

#[derive(Debug, Clone)]
struct Slot<T> {
    data: Option<T>,
    generation: usize,
}

#[derive(Debug, Clone)]
pub struct Storage<T> {
    data: Vec<Slot<T>>,
    free: Vec<usize>,
}

impl<T> Storage<T> {
    pub const fn new() -> Self {
        Self {
            data: Vec::new(),
            free: Vec::new(),
        }
    }

    pub fn insert(&mut self, data: T) -> Key {
        if let Some(index) = self.free.pop() {
            let slot = &mut self.data[index];
            slot.data = Some(data);

            Key {
                index,
                generation: slot.generation,
            }
        } else {
            let index = self.data.len();
            self.data.push(Slot {
                data: Some(data),
                generation: 0,
            });

            Key {
                index,
                generation: 0,
            }
        }
    }

    pub fn get(&self, key: Key) -> Option<&T> {
        let slot = self.data.get(key.index)?;
        if slot.generation == key.generation {
            slot.data.as_ref()
        } else {
            None
        }
    }

    pub fn remove(&mut self, key: Key) -> Option<T> {
        let slot = self.data.get_mut(key.index)?;
        if slot.generation != key.generation {
            return None;
        }

        if let Some(data) = slot.data.take() {
            slot.generation += 1;
            self.free.push(key.index);
            Some(data)
        } else {
            None
        }
    }
}
