use std::collections::VecDeque;
use tendril::StrTendril;
use super::TokenSet;

/// input buffer 
#[derive(Debug)]
pub struct InputBuffer {
    buffers: VecDeque<StrTendril>
}

// for results of pop_except_from 
#[derive(Debug)]
pub enum SetResult {
    FromSet(char),
    NotFromSet(StrTendril)
}

impl InputBuffer {
    pub fn new() -> Self {
        Self {
            buffers: VecDeque::with_capacity(16)
        }
    }

    // push to the end of a buffer 
    pub fn push(&mut self, buf: StrTendril){
        if buf.len32() == 0{
            return;
        }

        self.buffers.push_back(buf)
    }

    // check if empty 
    pub fn is_empty(&self) -> bool {
        self.buffers.is_empty()
    }

    /// get the length of buffer 
    pub fn len(&self) -> usize {
        self.buffers.len()
    }

    /// retrieve the next character from the buffer 
    pub fn next(&mut self) -> Option<char>{
        let (result, now_empty) = match self.buffers.front_mut(){
            None => (None, false),
            Some(buf) => {
                let c = buf.pop_front_char().expect("empty buffer in the queue");
                (Some(c), buf.is_empty())
            }
        };

        if now_empty{
            self.buffers.pop_front();
        }

        result 
    }

    /// pop from a set 
    pub fn pop_except_from(&mut self, set: TokenSet) -> Option<SetResult> {
        let (result, empty) = match self.buffers.front_mut(){
            None => (None, false),
            Some(buf) => {
                let n = set.nonmember_prefix_len(&buf);

                if n > 0 {
                    let out;

                    unsafe {
                        out = buf.unsafe_subtendril(0, n);
                        buf.unsafe_pop_front(n);
                    }

                    (Some(SetResult::NotFromSet(out)), buf.is_empty())
                }else{
                    let c = buf.pop_front_char().expect("empty buffer in the queue");
                    (Some(SetResult::FromSet(c)), buf.is_empty())
                }
            }
        };

        if empty {
            self.buffers.pop_front();
        }

        result

    }
}

