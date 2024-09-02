use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, VecDeque},
};

#[derive(Debug)]
pub struct HuffmanTree {
    /// Frequency counts of each char
    counts: HashMap<char, usize>,
}
impl HuffmanTree {
    pub const fn new(counts: HashMap<char, usize>) -> Self {
        Self { counts }
    }
    fn build_tree(&self) -> HuffmanNode {
        let mut min_heap: BinaryHeap<HuffmanNode> = self
            .counts
            .iter()
            .map(|(k, v)| HuffmanNode::Leaf(*k, *v))
            .collect();
        while min_heap.len() > 1 {
            let left = min_heap.pop().unwrap();
            let right = min_heap.pop().unwrap();
            min_heap.push(HuffmanNode::Internal(
                left.get_weight() + right.get_weight(),
                Box::new(left),
                Box::new(right),
            ));
        }
        min_heap.pop().unwrap()
    }
    pub fn get_huffman_codes(&self) -> HashMap<char, String> {
        let root = self.build_tree();
        let mut result = HashMap::new();
        // use BFS to touch every node iteratively and add their code
        let mut queue: VecDeque<(HuffmanNode, String)> = VecDeque::new();
        queue.push_back((root, String::new()));
        while !queue.is_empty() {
            let (node, mut prefix) = queue.pop_front().unwrap();
            match node {
                HuffmanNode::Leaf(c, _) => {
                    result.insert(c, prefix);
                }
                HuffmanNode::Internal(_, left, right) => {
                    let mut left_prefix = prefix.clone();
                    left_prefix.push('0');
                    prefix.push('1');
                    queue.push_back((*left, left_prefix));
                    queue.push_back((*right, prefix));
                }
            }
        }

        result
    }
}

#[derive(Debug, Eq)]
enum HuffmanNode {
    /// leaf node that corresponds to actual character
    Leaf(char, usize),
    /// internal node for determining prefix encoding bit
    Internal(usize, Box<HuffmanNode>, Box<HuffmanNode>),
}
impl PartialEq for HuffmanNode {
    fn eq(&self, other: &Self) -> bool {
        match (&self, &other) {
            (&Self::Leaf(c1, weight1), &Self::Leaf(c2, weight2)) => c1 == c2 && weight1 == weight2,
            (&Self::Internal(weight1, left1, right1), &Self::Internal(weight2, left2, right2)) => {
                weight1 == weight2 && left1 == left2 && right1 == right2
            }
            _ => false,
        }
    }
}

impl Ord for HuffmanNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_weight = self.get_weight();
        let other_weight = other.get_weight();
        // reverse comparison to implement a MinHeap
        other_weight.cmp(&self_weight).then_with(|| {
            match (&self, &other) {
                (&Self::Leaf(_, _), &Self::Leaf(_, _))
                | (&Self::Internal(_, _, _), &Self::Internal(_, _, _)) => Ordering::Equal,
                // internal nodes should be later than Leaf IF equal weight
                (&Self::Internal(_, _, _), _) => Ordering::Less,
                (_, &Self::Internal(_, _, _)) => Ordering::Greater,
            }
        })
    }
}
impl PartialOrd for HuffmanNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl HuffmanNode {
    const fn get_weight(&self) -> usize {
        match self {
            Self::Leaf(_, weight) | Self::Internal(weight, _, _) => *weight,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encodings() {
        let counts = HashMap::from_iter([('a', 3), ('b', 2), ('c', 1)]);
        let tree = HuffmanTree::new(counts);
        let encodings = tree.get_huffman_codes();
        assert_eq!(
            encodings,
            HashMap::from_iter([
                ('a', "0".to_string()),
                ('b', "11".to_string()),
                ('c', "10".to_string()),
            ])
        );
    }
}
