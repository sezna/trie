#![feature(test)]
#[cfg(test)]
mod tests;
/**
 * A minimal Trie that only works for str's intended for use as autocomplete for a dictionary website.
 */
pub mod trie {
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
            let first_char = word.chars().collect::<Vec<char>>()[0];
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
            let word_tail: String = word.chars().skip(1).collect();
            self.children
                .iter_mut()
                .find(|x| x.value == first_char)
                .unwrap()
                .add_word_mut(&word_tail);
        }

        /**
         * returns `true` if query is contained in the Trie,
         * returns `false` if not.  
         */
        pub fn search(&self, query: &str) -> bool {
            if query.len() == 0 && self.is_end_of_word {
                return true;
            }
            let first_char = query.chars().collect::<Vec<char>>()[0];
            let query_tail: String = query.chars().skip(1).collect();
            match self.children.iter().find(|x| x.value == first_char) {
                Some(node) => node.search(&query_tail),
                None => {
                    return false;
                }
            }
        }

        /** Takes a query and returns all strings contained in the Trie which
         *  start with that query
         */
        pub fn predict(&self, query: &str) -> Vec<String> {
            let starting_point = self.get_subtree(query);
            starting_point.predict_helper(query)
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

        fn get_subtree(&self, query: &str) -> &Trie {
            let mut owned_query = String::from(query);
            let mut rover = self;
            while owned_query.len() > 0 {
                let first_char = owned_query.chars().collect::<Vec<char>>()[0];
                owned_query = owned_query.chars().skip(1).collect();
                match rover.children.iter().find(|x| x.value == first_char) {
                    Some(node) => rover = node,
                    None => (),
                }
            }
            return rover;
        }
    }
}
