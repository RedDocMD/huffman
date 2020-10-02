use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn update_frequencies(word: &str, freqs: &mut HashMap<char, usize>) {
    for c in word.chars() {
        match freqs.get_mut(&c) {
            Some(val) => *val = *val + 1,
            None => {
                freqs.insert(c, 1);
            }
        }
    }
}

pub fn get_frequencies(filename: &str) -> Result<HashMap<char, usize>, Box<dyn Error>> {
    let mut freqs = HashMap::new();
    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    let lines = contents.split('\n');
    for line in lines {
        let line_string = String::from(line);
        let words = line_string.split(' ');
        for word in words {
            update_frequencies(&String::from(word).to_ascii_lowercase(), &mut freqs);
        }
    }
    Ok(freqs)
}

#[derive(PartialEq, Debug)]
enum Label {
    Character(char),
    Number(usize),
}

#[derive(Debug)]
struct HuffmanNode {
    frequency: usize,
    label: Label,
    left: Option<Box<HuffmanNode>>,
    right: Option<Box<HuffmanNode>>,
}

#[derive(Debug)]
pub struct HuffmanCodeTree {
    root: HuffmanNode,
}

impl HuffmanNode {
    fn new(label: char, frequency: usize) -> Self {
        Self {
            frequency,
            label: Label::Character(label),
            left: None,
            right: None,
        }
    }

    fn new_with_children(label: usize, left: HuffmanNode, right: HuffmanNode) -> Self {
        Self {
            frequency: left.frequency + right.frequency,
            label: Label::Number(label),
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        }
    }

    fn is_internal(&self) -> bool {
        self.left.is_some() && self.right.is_some()
    }
}

impl Ord for HuffmanNode {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.frequency.cmp(&other.frequency) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => match self.label {
                Label::Character(left_char) => match other.label {
                    Label::Character(right_char) => left_char.cmp(&right_char),
                    Label::Number(_) => Ordering::Greater,
                },
                Label::Number(left_num) => match other.label {
                    Label::Character(_) => Ordering::Less,
                    Label::Number(right_num) => left_num.cmp(&right_num),
                },
            },
        }
    }
}

impl PartialOrd for HuffmanNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.cmp(other) {
            Ordering::Greater => Some(Ordering::Less),
            Ordering::Less => Some(Ordering::Greater),
            Ordering::Equal => Some(Ordering::Equal),
        }
    }
}

impl PartialEq for HuffmanNode {
    fn eq(&self, other: &Self) -> bool {
        self.frequency == other.frequency
            && self.label == other.label
            && self.is_internal() == other.is_internal()
    }
}

impl Eq for HuffmanNode {}

fn generate_huffman_tree(freqs: &HashMap<char, usize>, symbols: &[char]) -> HuffmanCodeTree {
    let mut queue = BinaryHeap::new();

    for sym in symbols {
        let node = HuffmanNode::new(*sym, *freqs.get(sym).unwrap_or(&0));
        queue.push(node);
    }

    for num in 1..symbols.len() {
        let first = queue.pop().unwrap();
        let second = queue.pop().unwrap();
        let super_node = HuffmanNode::new_with_children(num, first, second);
        queue.push(super_node);
    }

    HuffmanCodeTree {
        root: queue.pop().unwrap(),
    }
}

fn huffman_code_recursive(
    root: &HuffmanNode,
    code_map: &mut HashMap<char, Vec<u8>>,
    partial_code: &mut Vec<u8>,
) {
    if root.is_internal() {
        partial_code.push(0);
        huffman_code_recursive(root.left.as_ref().unwrap(), code_map, partial_code);
        partial_code.pop();
        partial_code.push(1);
        huffman_code_recursive(root.right.as_ref().unwrap(), code_map, partial_code);
        partial_code.pop();
    } else {
        match root.label {
            Label::Character(ch) => {
                code_map.insert(ch, partial_code.clone());
            }
            Label::Number(_) => {
                panic!("Incorrect tree!");
            }
        }
    }
}

#[derive(Debug)]
pub struct HuffmanCode {
    tree: HuffmanCodeTree,
    code: HashMap<char, Vec<u8>>,
    symbols: Vec<char>,
}

impl HuffmanCode {
    pub fn new(tree: HuffmanCodeTree, code: HashMap<char, Vec<u8>>, symbols: Vec<char>) -> Self {
        HuffmanCode {
            code,
            tree,
            symbols,
        }
    }

    pub fn encode(&self, message: &str) -> Vec<u8> {
        let mut encoded = Vec::new();
        for symbol in message.chars() {
            encoded.append(&mut self.code[&symbol].clone());
        }
        encoded
    }

    pub fn encode_char(&self, sym: char) -> Vec<u8> {
        self.code[&sym].clone()
    }
}

impl fmt::Display for HuffmanCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for symbol in &self.symbols {
            let mut code = String::new();
            for val in &self.code[symbol] {
                if *val == 1 {
                    code.push('1');
                } else if *val == 0 {
                    code.push('0');
                }
            }
            writeln!(f, "{} = {}", *symbol, code)?;
        }
        Ok(())
    }
}

pub fn generate_huffman_code(freqs: &HashMap<char, usize>, symbols: &[char]) -> HuffmanCode {
    let tree = generate_huffman_tree(&freqs, symbols);
    let mut code_map = HashMap::new();
    huffman_code_recursive(&tree.root, &mut code_map, &mut Vec::new());
    let mut symbols_copy = Vec::new();
    symbols_copy.extend_from_slice(symbols);
    HuffmanCode::new(tree, code_map, symbols_copy)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_update_frequencies() {
        let mut freqs = HashMap::new();
        let word = "Hello";
        update_frequencies(word, &mut freqs);
        assert_eq!(freqs[&'H'], 1);
        assert_eq!(freqs[&'e'], 1);
        assert_eq!(freqs[&'l'], 2);
        assert_eq!(freqs[&'o'], 1);
    }

    #[test]
    fn test_generate_huffman_tree() {
        let mut freqs = HashMap::new();

        freqs.insert('a', 25);
        freqs.insert('e', 40);
        freqs.insert('i', 23);

        let symbols = vec!['a', 'e', 'i'];

        let a_node = HuffmanNode::new('a', 25);
        let e_node = HuffmanNode::new('e', 40);
        let i_node = HuffmanNode::new('i', 23);
        let int_node1 = HuffmanNode::new_with_children(1, i_node, a_node);
        let int_node2 = HuffmanNode::new_with_children(2, e_node, int_node1);
        let expected_tree = HuffmanCodeTree { root: int_node2 };

        let tree = generate_huffman_tree(&freqs, &symbols);
        println!("{:?}", tree.root);

        fn check_tree_equality(root1: &HuffmanNode, root2: &HuffmanNode) -> bool {
            if root1 == root2 {
                if root1.is_internal() {
                    return check_tree_equality(
                        root1.left.as_ref().unwrap(),
                        root2.left.as_ref().unwrap(),
                    ) && check_tree_equality(
                        root1.right.as_ref().unwrap(),
                        root2.right.as_ref().unwrap(),
                    );
                } else {
                    return true;
                }
            }
            false
        }

        assert_eq!(check_tree_equality(&tree.root, &expected_tree.root), true);
    }

    #[test]
    fn test_huffman_code() {
        let mut freqs = HashMap::new();

        freqs.insert('a', 25);
        freqs.insert('e', 40);
        freqs.insert('i', 23);
        freqs.insert('o', 11);
        freqs.insert('s', 26);
        freqs.insert('t', 27);

        let symbols = vec!['a', 'e', 'i', 'o', 's', 't'];

        let mut expected_codes = HashMap::<char, Vec<u8>>::new();
        expected_codes.insert('e', vec![1, 0]);
        expected_codes.insert('a', vec![1, 1, 0]);
        expected_codes.insert('i', vec![0, 1, 1]);
        expected_codes.insert('o', vec![0, 1, 0]);
        expected_codes.insert('s', vec![1, 1, 1]);
        expected_codes.insert('t', vec![0, 0]);

        let huffman_code = generate_huffman_code(&freqs, &symbols);

        assert_eq!(huffman_code.code, expected_codes);
    }

    #[test]
    fn test_huffman_encoding() {
        let mut freqs = HashMap::new();

        freqs.insert('a', 25);
        freqs.insert('e', 40);
        freqs.insert('i', 23);
        freqs.insert('o', 11);
        freqs.insert('s', 26);
        freqs.insert('t', 27);

        let symbols = vec!['a', 'e', 'i', 'o', 's', 't'];
        let huffman_code = generate_huffman_code(&freqs, &symbols);

        let message = "toastie";
        let encoded = huffman_code.encode(message);
        let expected_encoded = vec![0, 0, 0, 1, 0, 1, 1, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 0];

        assert_eq!(encoded, expected_encoded);
    }
}
