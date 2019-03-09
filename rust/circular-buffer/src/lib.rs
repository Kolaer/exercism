pub struct CircularBuffer<T> {
    data: Vec<Option<T>>,
    size: usize,
    head: usize,
    tail: usize,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        // NOTE: could not use vec![None, capacity] because T is not Clone
        let mut data = Vec::with_capacity(capacity);

        for _ in 0..capacity {
            data.push(None);
        }

        CircularBuffer {
            data,
            size: 0,
            head: 0,
            tail: 0,
        }
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        let capacity = self.data.len();

        if self.size == capacity {
            Err(Error::FullBuffer)
        } else {
            self.data[self.tail] = Some(element);
            self.tail = (self.tail + 1) % capacity;
            self.size += 1;
            Ok(())
        }
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.size == 0 {
            Err(Error::EmptyBuffer)
        } else {
            let element = self.data[self.head].take().unwrap();

            let capacity = self.data.len();
            self.head = (self.head + 1) % capacity;

            self.size -= 1;

            Ok(element)
        }
    }

    pub fn clear(&mut self) {
        for el in &mut self.data {
            *el = None;
        }
        self.size = 0;
    }

    pub fn overwrite(&mut self, element: T) {
        let capacity = self.data.len();
        if self.size == capacity {
            let _ = self.read().unwrap();
        }

        self.write(element).unwrap();
    }
}
