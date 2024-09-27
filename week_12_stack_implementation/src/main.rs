struct Stack<T> {
    data: Vec<T>,
    max_size: usize
}

fn main() {}

impl<T> Stack<T> {
    fn new(max_size: usize) -> Self {
        Stack {
            data: Vec::with_capacity(max_size),
            max_size
        }
    }

    fn push(&mut self, element: T) -> Result<(), &'static str> {
        if self.data.len() == self.max_size {
            Err("Stack Overflow")
        } else {
            self.data.push(element);
            Ok(())
        }
    }

    fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}