use std::collections::HashSet;

pub struct TokenSet {
    set: HashSet<char>
}

impl TokenSet {
    pub fn new(chars: Vec<char>) -> Self {
        let set : HashSet<char> = chars.iter().cloned().collect();

        Self {
            set 
        }
    }

    ///  Check if the token set contains that character 
    pub fn contains(&self, c: char) -> bool {
        self.set.contains(&c)
    }

    // get the number of characters in the set 
    pub fn nonmember_prefix_len(&self, buf: &str) -> u32 {
        let mut  n = 0;

        for ch in buf.chars() {
            if !self.contains(ch){
                n += 1;
            }else{
                break;
            }
        }

        n
    }
}