use crate::common::Ascii;

pub struct Node {
    pub value: u8,
    pub children: Vec<Node>,
    pub is_last: bool,
}
impl Node {
    pub fn new(key: u8) -> Self {
        Node {
            value: key,
            children: vec![],
            is_last: false,
        }
    }

    fn index_of_child(&self, value: u8) -> Option<usize> {
        self.children.iter().position(|node| node.value == value)
    }

    fn find_or_create_child(&mut self, value: u8) -> &mut Node {
        let child = match self.index_of_child(value) {
            Some(i) => self.children.get_mut(i).unwrap(),
            None => {
                let node = Node::new(value);
                self.children.push(node);
                self.children.last_mut().unwrap()
            }
        };
        child
    }
}

pub struct Trie {
    pub root: Node,
}
impl Trie {
    pub fn new() -> Self {
        Trie { root: Node::new(0) }
    }

    pub fn insert(&mut self, value: &str) {
        let mut current = &mut self.root;
        let ascii_string = Ascii::from(value);
        for key in ascii_string {
            current = current.find_or_create_child(key);
        }
        current.is_last = true;
    }

    pub fn search_all(&mut self, word: &str) -> Vec<Ascii> {
        let mut suggestions: Vec<Ascii> = vec![];
        let mut current = &self.root;
        let ascii_word = Ascii::from(word);
        for c in ascii_word.clone() {
            match current.index_of_child(c) {
                Some(i) => current = current.children.get(i).unwrap(),
                None => return suggestions,
            }
        }
        let new_word = self.get_words_with_prefix(current, ascii_word);
        suggestions.extend(new_word);
        suggestions
    }

    fn get_words_with_prefix(&self, node: &Node, prefix: Ascii) -> Vec<Ascii> {
        let mut words: Vec<Ascii> = vec![];
        let query = prefix;
        for child in &node.children {
            if child.is_last == true {
                let mut new_word = query.clone();
                new_word.push(child.value);
                words.push(new_word)
            } else {
                let mut new_word = query.clone();
                new_word.push(child.value);
                words.extend(self.get_words_with_prefix(child, new_word));
            }
        }

        words
    }
}
