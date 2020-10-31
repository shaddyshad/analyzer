use std::collections::VecDeque;

#[derive(Debug)]
pub struct Stack<T: Sized>{
     items: VecDeque<T>
}

impl <T> Stack<T> {
    pub fn new() -> Self {
        Self {
            items: VecDeque::new()
        }
    }

    pub fn push(&mut self, item: T){
        self.items.push_back(item);
    }

    pub fn pop(&mut self) -> Option<T>{
        self.items.pop_front()
    }

    pub fn last(&self) -> Option<&T>{
        self.items.back()
    }
}