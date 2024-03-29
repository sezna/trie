#![feature(test)]
#[cfg(test)]
mod tests;
/**
 * A minimal Trie that only works for str's intended for use as autocomplete for a dictionary website.
 */
pub struct Trie {
    children: Vec<Trie>,
    is_end_of_word: bool,
    value: char,
}

impl Trie {
    pub fn new() -> Trie {
        Trie {
            children: Vec::new(),
            is_end_of_word: false,
            value: '$',
        }
    }
    // Mutates the TrieNode to add a word to it.
    pub fn add_word_mut(&mut self, word: &str) {
        if word.len() == 0 {
            self.is_end_of_word = true;
            return;
        }
        let first_char = word.chars().next().unwrap();
        // See if we already have this at the current node level
        match self.children.iter().find(|x| x.value == first_char) {
            // Yes, this letter exists in this node...
            Some(_) => (),
            // No, it doesn't, and we need to add it in.
            None => {
                self.children.push(Trie {
                    children: Vec::new(),
                    is_end_of_word: false,
                    value: first_char,
                });
            }
        }
        self.children
            .iter_mut()
            .find(|x| x.value == first_char)
            .unwrap()
            .add_word_mut(get_tail(word));
    }

    /**
     * returns `true` if query is contained in the Trie,
     * returns `false` if not.  
     */
    pub fn search(&self, query: &str) -> bool {
        if query.len() == 0 && self.is_end_of_word {
            return true;
        }
        let first_char = query.chars().next().unwrap();
        match self.children.iter().find(|x| x.value == first_char) {
            Some(node) => node.search(get_tail(query)),
            None => {
                return false;
            }
        }
    }

    /** Takes a query and returns all strings contained in the Trie which
     *  start with that query
     */
    pub fn predict(&self, query: &str) -> Vec<String> {
        if let Some(tree) = self.get_subtree(query) {
            tree.predict_helper(query)
        }
        else {
            Vec::new()
        }
    }

    fn predict_helper(&self, string_so_far: &str) -> Vec<String> {
        let mut to_return: Vec<String> = Vec::new();
        if self.is_end_of_word {
            to_return.push(String::from(string_so_far));
        }
        for child in self.children.iter() {
            let mut owned_string = String::from(string_so_far);
            owned_string.push(child.value);
            to_return.append(&mut child.predict_helper(&owned_string));
        }
        return to_return;
    }
    fn get_subtree(&self, query: &str) -> Option<&Trie> {
        let mut owned_query = query;
        let mut rover = self;
        while owned_query.len() > 0 {
            let first_char = owned_query.chars().next().unwrap();
            owned_query = get_tail(owned_query);
            match rover.children.iter().find(|x| x.value == first_char) {
                Some(node) => rover = node,
                None => return None,
            }
        }
        return Some(rover);
    }
}

/// In the implementation of this trie, I have to drop the first character of the query
/// in multiple places. The index must be calculated manually since the string is
/// UTF-8 and the characters will have variable length in bytes. This is the
/// most optimal way of performing this operation that I have found so far.
fn get_tail(query: &str) -> &str {
    let option = query.char_indices().nth(1);
    let idx = match option {
        Some((idx, _)) => idx,
        None => return "",
    };
    return &query[idx..];
}
