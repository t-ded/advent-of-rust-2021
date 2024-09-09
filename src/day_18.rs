use std::cmp::max;
use std::ops::Add;
use std::str::Chars;

#[derive(Debug, Clone)]
struct SnailfishNumberNode{
    regular_number_value: Option<usize>,
    left: Option<Box<SnailfishNumberNode>>,
    right: Option<Box<SnailfishNumberNode>>,
    height: usize,
}

impl SnailfishNumberNode {
    fn leaf_from_regular_value(value: usize) -> Self {
        Self {
            regular_number_value: Some(value),
            left: None,
            right: None,
            height: 0,
        }
    }

    fn node_with_children(left: Box<SnailfishNumberNode>, right: Box<SnailfishNumberNode>) -> Self {
        let new_node = Self {
            height: max(left.height, right.height) + 1,
            regular_number_value: None,
            left: Some(left),
            right: Some(right),
        };
        new_node
    }

    fn magnitude(&self) -> usize {
        if let Some(value) = self.regular_number_value {
            value
        }
        else {
            3 * self.left.as_ref().unwrap().magnitude() + 2 * self.right.as_ref().unwrap().magnitude()
        }
    }

    fn is_leaf(&self) -> bool {
        self.regular_number_value.is_some()
    }

    fn get_leftmost_leaf(&mut self) -> &mut SnailfishNumberNode {
        if self.left.is_some() {
            self.left.as_mut().unwrap().get_leftmost_leaf()
        } else {
            self
        }
    }

    fn get_rightmost_leaf(&mut self) -> &mut SnailfishNumberNode {
        if self.right.is_some() {
            self.right.as_mut().unwrap().get_rightmost_leaf()
        } else {
            self
        }
    }

    fn increase_regular_value(&mut self, to_add: usize) {
        if let Some(regular_value) = self.regular_number_value.as_mut() {
            *regular_value += to_add;
        }
    }

    fn split(&mut self) -> bool {
        if let Some(val) = self.regular_number_value {
            if val >= 10 {
                self.left = Some(Box::new(SnailfishNumberNode::leaf_from_regular_value(self.regular_number_value.unwrap() / 2)));
                self.right = Some(Box::new(SnailfishNumberNode::leaf_from_regular_value((self.regular_number_value.unwrap() + 1) / 2)));
                self.regular_number_value = None;
                self.height = 1;
                return true
            }
        } else {
            if let Some(left) = self.left.as_mut() {
                if left.split() {
                    self.height = max(left.height + 1, self.height);
                    return true
                }
            }
            if let Some(right) = self.right.as_mut() {
                if right.split() {
                    self.height = max(right.height + 1, self.height);
                    return true
                }
            }
        }
        false
    }

    fn explode(&mut self) -> (bool, usize, bool, usize) {
        if self.height == 1 {
            let left_value = self.left.as_ref().unwrap().regular_number_value.unwrap();
            let right_value = self.right.as_ref().unwrap().regular_number_value.unwrap();
            self.regular_number_value = Some(0);
            self.left = None;
            self.right = None;
            self.height = 0;
            // println!("TADA VALUES TO EXPLODE: {left_value}, {right_value}");
            return (false, left_value, false, right_value)
        }
        let left = self.left.as_mut().unwrap();
        let right = self.right.as_mut().unwrap();
        if left.height == self.height - 1 {
            let (left_assigned, left_value, mut right_assigned, right_value) = left.explode();
            if !right_assigned {
                let leftmost_in_right_subtree = right.get_leftmost_leaf();
                leftmost_in_right_subtree.increase_regular_value(right_value);
                right_assigned = true;
            }
            self.height = max(left.height, right.height) + 1;
            // println!("Recursion values: {left_assigned}, {left_value}, {right_assigned}, {right_value}");
            (left_assigned, left_value, right_assigned, right_value)
        } else {
            let (mut left_assigned, left_value, right_assigned, right_value) = right.explode();
            if !left_assigned {
                let rightmost_in_left_subtree = left.get_rightmost_leaf();
                rightmost_in_left_subtree.increase_regular_value(left_value);
                left_assigned = true;
            }
            self.height = max(left.height, right.height) + 1;
            // println!("Recursion values: {left_assigned}, {left_value}, {right_assigned}, {right_value}");
            (left_assigned, left_value, right_assigned, right_value)
        }
    }

    fn to_string(&self) -> String {
        if let Some(value) = self.regular_number_value {
            value.to_string()
        } else {
            let left_str = self.left.as_ref().unwrap().to_string();
            let right_str = self.right.as_ref().unwrap().to_string();
            format!("[{},{}]", left_str, right_str)
        }
    }
}

#[derive(Debug, Clone)]
struct SnailfishNumberTree {
    root: Box<SnailfishNumberNode>,
}

impl SnailfishNumberTree {
    fn from_root(root: SnailfishNumberNode) -> Self {
        Self { root: Box::new(root) }
    }

    fn from_input(input: &str) -> Self {
        let mut chars = input.chars();
        let root = SnailfishNumberTree::parse_node(&mut chars);
        SnailfishNumberTree { root }
    }

    fn parse_node(chars: &mut Chars) -> Box<SnailfishNumberNode> {
        let current_char = chars.next();

        match current_char {
            Some('[') => {
                let left = SnailfishNumberTree::parse_node(chars);
                assert_eq!(chars.next(), Some(','));
                let right = SnailfishNumberTree::parse_node(chars);
                assert_eq!(chars.next(), Some(']'));
                let node = Box::new(SnailfishNumberNode::node_with_children(left, right));
                node
            }
            Some(digit) if digit.is_digit(10) => {
                let mut num_str = digit.to_string();
                while let Some(c) = chars.clone().next() {
                    if c.is_digit(10) {
                        num_str.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                let number = num_str.parse::<usize>().unwrap();
                Box::new(SnailfishNumberNode::leaf_from_regular_value(number))
            }
            _ => panic!("Unexpected char in SnailfishNumberTree"),
        }
    }

    fn magnitude(&self) -> usize {
        self.root.magnitude()
    }

    fn reduce(&mut self) {
        loop {
            if self.root.height >= 5 {
                self.root.explode();
                // println!("After explode: {}", self.to_string());
                // println!("Root height: {}", self.root.height);
                continue
            }
            if self.root.split() {
                // println!("After split: {}", self.to_string());
                // println!("Root height: {}", self.root.height);
                continue
            }
            break
        }
    }

    fn to_string(&self) -> String {
        self.root.to_string()
    }
}

impl Add<SnailfishNumberTree> for SnailfishNumberTree {
    type Output = SnailfishNumberTree;

    fn add(self, rhs: SnailfishNumberTree) -> Self::Output {
        let res_root = SnailfishNumberNode  {
            regular_number_value: None,
            height: max(self.root.height, rhs.root.height) + 1,
            left: Some(Box::new(*self.root)),
            right: Some(Box::new(*rhs.root)),
        };

        let mut res = SnailfishNumberTree::from_root(res_root);
        // println!("After addition: {}", res.to_string());
        // println!("Root height: {}", res.root.height);
        res.reduce();
        res
    }
}

pub fn part_1(input: &str) -> usize {
    let mut summands = input.lines();
    let mut sum = SnailfishNumberTree::from_input(summands.next().unwrap());
    for line in summands {
        // println!("{}", line);
        // println!("{}", sum.to_string());
        // println!("{:?}", SnailfishNumberTree::from_input(&line).to_string());
        sum = sum + SnailfishNumberTree::from_input(&line);
        // println!("{:?}", sum.to_string());
        // println!("{:?}", sum.magnitude());
        // println!()
    }
    sum.magnitude()
}

pub fn part_2(input: &str) -> usize {
    let mut max_magnitude = 0;
    let snailfish_numbers: Vec<SnailfishNumberTree> = input.lines().map(|line| SnailfishNumberTree::from_input(line)).collect();
    for i in 0..snailfish_numbers.len() {
        for j in 0..snailfish_numbers.len() {
            let sum = snailfish_numbers[i].clone() + snailfish_numbers[j].clone();
            max_magnitude = max(max_magnitude, sum.magnitude());
        }
    }
    max_magnitude
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::day_18::{part_1, part_2};

    #[test]
    fn example_magnitudes() {
        assert_eq!(SnailfishNumberTree::from_input("[[1,2],[[3,4],5]]").magnitude(), 143);
        assert_eq!(SnailfishNumberTree::from_input("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").magnitude(), 1_384);
        assert_eq!(SnailfishNumberTree::from_input("[[[[1,1],[2,2]],[3,3]],[4,4]]").magnitude(), 445);
        assert_eq!(SnailfishNumberTree::from_input("[[[[3,0],[5,3]],[4,4]],[5,5]]").magnitude(), 791);
        assert_eq!(SnailfishNumberTree::from_input("[[[[5,0],[7,4]],[5,5]],[6,6]]").magnitude(), 1_137);
        assert_eq!(SnailfishNumberTree::from_input("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]").magnitude(), 3_488);
    }

    #[test]
    fn simple_addition() {
        let a = SnailfishNumberTree::from_input("[1,2]");
        let b = SnailfishNumberTree::from_input("[[3,4],5]");
        assert_eq!((a + b).magnitude(), 143);

        let a = SnailfishNumberTree::from_input("[[[[4,3],4],4],[7,[[8,4],9]]]");
        let b = SnailfishNumberTree::from_input("[1,1]");
        assert_eq!((a + b).magnitude(), 1_384);
    }

    #[test]
    fn simple_explodes() {
        let mut a = SnailfishNumberTree::from_input("[[[[[9,8],1],2],3],4]");
        a.reduce();
        println!("{:?}", a);
    }

    #[test]
    fn aoc_examples_work() {
        assert_eq!(part_1(
            "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]"), 3_488);
        assert_eq!(part_1(
            "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"), 4_140);
    }

    #[test]
    fn pt_1_works() {
        assert_eq!(part_1("[[[2,[3,5]],[8,7]],[[9,3],2]]
[[3,[3,7]],[[3,6],[[1,1],7]]]
[8,[[5,5],[2,9]]]
[[5,[3,5]],[[2,1],[[7,1],[7,7]]]]
[[[[3,3],0],[[0,3],0]],[[8,[2,2]],[[0,4],3]]]
[3,6]
[[5,[[4,2],1]],[[6,[0,3]],[4,[7,7]]]]
[[6,5],[2,[3,6]]]
[[[[0,1],0],[[7,4],5]],[[6,2],[4,[0,8]]]]
[[[[4,7],3],8],[[7,[0,4]],[7,[1,4]]]]
[[[0,[9,8]],[2,9]],[[[6,4],[4,0]],4]]
[2,[[4,[8,5]],[6,8]]]
[[[0,7],[5,[3,0]]],[[[6,4],[3,2]],[[4,7],[9,6]]]]
[[[[0,6],[0,7]],[8,0]],[8,[4,8]]]
[[[[9,9],2],[[6,2],[2,2]]],[[5,[8,8]],6]]
[[0,[[4,6],7]],[[7,[4,8]],9]]
[[0,5],[[5,3],[[3,9],4]]]
[2,[[[9,4],[8,8]],1]]
[5,[[[2,3],6],[2,[7,0]]]]
[[7,[[8,6],3]],[2,[2,7]]]
[6,[[2,4],[[9,7],[5,9]]]]
[[[9,[2,1]],9],1]
[[[6,9],[2,[2,5]]],[[[4,4],0],7]]
[1,[[[3,9],[6,1]],[4,0]]]
[[[3,8],[3,[2,7]]],[[[9,2],2],6]]
[6,[[8,[3,1]],7]]
[[[9,9],7],[[[3,1],[8,4]],[0,0]]]
[[[1,[7,8]],[4,2]],2]
[[9,7],6]
[[6,[4,8]],[[[8,6],[0,1]],[[0,4],[8,4]]]]
[[[[1,8],[8,6]],[9,[2,0]]],[5,[2,[7,2]]]]
[1,9]
[[8,[9,[9,3]]],[[[1,1],8],[[1,5],[8,6]]]]
[[[3,[4,4]],3],[[7,0],[6,0]]]
[[[6,[6,3]],[6,7]],[1,[8,0]]]
[[[9,7],[1,7]],8]
[[8,[[4,6],[4,8]]],8]
[[[1,9],6],1]
[[[[0,5],[0,0]],7],[4,8]]
[[[[6,0],[4,2]],[8,[5,1]]],[[0,[4,8]],[[3,2],8]]]
[[[[5,9],[5,8]],[9,[0,1]]],[[[8,6],[3,1]],[[9,8],0]]]
[0,[[9,9],[6,2]]]
[[[[7,9],[9,1]],[[1,0],[6,4]]],[4,[[2,1],2]]]
[4,2]
[[[6,5],[[0,6],2]],[[[1,2],0],[[8,9],8]]]
[[8,[[4,1],0]],[[[1,5],[3,5]],3]]
[[[8,3],[[9,1],[8,1]]],[[9,9],3]]
[[2,7],[[[3,9],[2,3]],9]]
[[2,[[7,3],[1,6]]],[[4,4],[2,7]]]
[[[5,6],[3,[5,3]]],[[[2,8],0],[4,[8,8]]]]
[[[1,2],[4,[5,8]]],[8,[8,[9,0]]]]
[[[[0,5],[8,1]],0],[[[5,4],[6,9]],[[7,5],[4,9]]]]
[[9,[2,1]],[[[3,8],[9,5]],[[4,4],4]]]
[[[5,9],[[1,1],[8,9]]],[[1,9],8]]
[[[8,8],[3,9]],[[[2,1],0],9]]
[[[[7,8],2],[5,[3,9]]],[6,1]]
[[[[2,4],[9,1]],[[9,8],[4,4]]],[0,1]]
[[[[8,8],0],9],4]
[[[8,[1,5]],0],[[[8,5],4],[[7,3],[9,5]]]]
[[[5,4],[[5,1],2]],[[[6,8],6],[[3,6],[1,9]]]]
[[[3,[2,5]],[6,[6,2]]],[[0,7],[3,9]]]
[3,[[2,9],8]]
[[[[3,7],[1,6]],[[9,9],[0,3]]],[[[7,3],8],[[3,1],6]]]
[[[[7,1],4],[[4,0],[4,5]]],[8,[[5,3],[4,6]]]]
[[[[0,8],1],[7,9]],[[7,5],[[1,0],[0,9]]]]
[[[9,7],[0,[7,8]]],2]
[[[5,2],5],[0,[[1,6],[2,0]]]]
[[[[3,9],7],7],[[3,[3,4]],[0,[5,9]]]]
[[[[2,5],[9,9]],[1,[6,5]]],6]
[[[1,[5,9]],[[1,1],1]],[5,[[0,4],[9,0]]]]
[[[5,8],[0,7]],[3,[2,[8,6]]]]
[[[[0,7],[7,9]],[[8,4],[8,7]]],[0,[[3,7],9]]]
[[[5,[5,5]],[[9,5],8]],[[[2,1],5],9]]
[5,[4,[[3,6],[3,2]]]]
[[[9,4],3],[[[8,7],[7,5]],[8,[7,7]]]]
[9,[[[9,2],0],[[9,9],[4,3]]]]
[[[4,[7,2]],[[7,9],[5,4]]],1]
[[[[4,9],5],7],[[5,6],0]]
[[[5,[3,1]],[8,1]],[8,[7,0]]]
[[5,6],[6,[[0,5],0]]]
[[[5,[4,5]],9],6]
[[[9,[7,0]],6],[2,[1,6]]]
[[[9,[8,4]],[7,[6,0]]],[[[4,6],[7,5]],[8,[0,8]]]]
[0,7]
[[3,[3,8]],[9,[[3,1],[4,4]]]]
[[6,7],[8,9]]
[[[[9,8],[0,2]],[[4,0],[7,5]]],[[[5,0],1],2]]
[[[[1,2],[3,9]],1],[[5,1],[0,1]]]
[[[[5,8],0],6],[7,0]]
[[[8,[5,4]],[[3,0],7]],[[8,[7,5]],4]]
[[[[5,8],8],8],[[[0,4],[2,5]],0]]
[[[9,6],3],[[[3,3],1],[2,[9,2]]]]
[[[6,3],6],[[[4,1],8],[2,3]]]
[2,[[1,8],0]]
[5,[[[7,6],[1,9]],[4,[8,2]]]]
[[[[6,9],[0,7]],[[2,7],8]],[[6,0],[2,[1,6]]]]
[[[[7,8],[5,1]],[[2,9],2]],0]
[5,3]
[2,[7,[7,[5,8]]]]
[[3,3],[8,[2,6]]]"), 2_907);
    }

    #[test]
    fn pt_2_works() {
        assert_eq!(part_2("[[[2,[3,5]],[8,7]],[[9,3],2]]
[[3,[3,7]],[[3,6],[[1,1],7]]]
[8,[[5,5],[2,9]]]
[[5,[3,5]],[[2,1],[[7,1],[7,7]]]]
[[[[3,3],0],[[0,3],0]],[[8,[2,2]],[[0,4],3]]]
[3,6]
[[5,[[4,2],1]],[[6,[0,3]],[4,[7,7]]]]
[[6,5],[2,[3,6]]]
[[[[0,1],0],[[7,4],5]],[[6,2],[4,[0,8]]]]
[[[[4,7],3],8],[[7,[0,4]],[7,[1,4]]]]
[[[0,[9,8]],[2,9]],[[[6,4],[4,0]],4]]
[2,[[4,[8,5]],[6,8]]]
[[[0,7],[5,[3,0]]],[[[6,4],[3,2]],[[4,7],[9,6]]]]
[[[[0,6],[0,7]],[8,0]],[8,[4,8]]]
[[[[9,9],2],[[6,2],[2,2]]],[[5,[8,8]],6]]
[[0,[[4,6],7]],[[7,[4,8]],9]]
[[0,5],[[5,3],[[3,9],4]]]
[2,[[[9,4],[8,8]],1]]
[5,[[[2,3],6],[2,[7,0]]]]
[[7,[[8,6],3]],[2,[2,7]]]
[6,[[2,4],[[9,7],[5,9]]]]
[[[9,[2,1]],9],1]
[[[6,9],[2,[2,5]]],[[[4,4],0],7]]
[1,[[[3,9],[6,1]],[4,0]]]
[[[3,8],[3,[2,7]]],[[[9,2],2],6]]
[6,[[8,[3,1]],7]]
[[[9,9],7],[[[3,1],[8,4]],[0,0]]]
[[[1,[7,8]],[4,2]],2]
[[9,7],6]
[[6,[4,8]],[[[8,6],[0,1]],[[0,4],[8,4]]]]
[[[[1,8],[8,6]],[9,[2,0]]],[5,[2,[7,2]]]]
[1,9]
[[8,[9,[9,3]]],[[[1,1],8],[[1,5],[8,6]]]]
[[[3,[4,4]],3],[[7,0],[6,0]]]
[[[6,[6,3]],[6,7]],[1,[8,0]]]
[[[9,7],[1,7]],8]
[[8,[[4,6],[4,8]]],8]
[[[1,9],6],1]
[[[[0,5],[0,0]],7],[4,8]]
[[[[6,0],[4,2]],[8,[5,1]]],[[0,[4,8]],[[3,2],8]]]
[[[[5,9],[5,8]],[9,[0,1]]],[[[8,6],[3,1]],[[9,8],0]]]
[0,[[9,9],[6,2]]]
[[[[7,9],[9,1]],[[1,0],[6,4]]],[4,[[2,1],2]]]
[4,2]
[[[6,5],[[0,6],2]],[[[1,2],0],[[8,9],8]]]
[[8,[[4,1],0]],[[[1,5],[3,5]],3]]
[[[8,3],[[9,1],[8,1]]],[[9,9],3]]
[[2,7],[[[3,9],[2,3]],9]]
[[2,[[7,3],[1,6]]],[[4,4],[2,7]]]
[[[5,6],[3,[5,3]]],[[[2,8],0],[4,[8,8]]]]
[[[1,2],[4,[5,8]]],[8,[8,[9,0]]]]
[[[[0,5],[8,1]],0],[[[5,4],[6,9]],[[7,5],[4,9]]]]
[[9,[2,1]],[[[3,8],[9,5]],[[4,4],4]]]
[[[5,9],[[1,1],[8,9]]],[[1,9],8]]
[[[8,8],[3,9]],[[[2,1],0],9]]
[[[[7,8],2],[5,[3,9]]],[6,1]]
[[[[2,4],[9,1]],[[9,8],[4,4]]],[0,1]]
[[[[8,8],0],9],4]
[[[8,[1,5]],0],[[[8,5],4],[[7,3],[9,5]]]]
[[[5,4],[[5,1],2]],[[[6,8],6],[[3,6],[1,9]]]]
[[[3,[2,5]],[6,[6,2]]],[[0,7],[3,9]]]
[3,[[2,9],8]]
[[[[3,7],[1,6]],[[9,9],[0,3]]],[[[7,3],8],[[3,1],6]]]
[[[[7,1],4],[[4,0],[4,5]]],[8,[[5,3],[4,6]]]]
[[[[0,8],1],[7,9]],[[7,5],[[1,0],[0,9]]]]
[[[9,7],[0,[7,8]]],2]
[[[5,2],5],[0,[[1,6],[2,0]]]]
[[[[3,9],7],7],[[3,[3,4]],[0,[5,9]]]]
[[[[2,5],[9,9]],[1,[6,5]]],6]
[[[1,[5,9]],[[1,1],1]],[5,[[0,4],[9,0]]]]
[[[5,8],[0,7]],[3,[2,[8,6]]]]
[[[[0,7],[7,9]],[[8,4],[8,7]]],[0,[[3,7],9]]]
[[[5,[5,5]],[[9,5],8]],[[[2,1],5],9]]
[5,[4,[[3,6],[3,2]]]]
[[[9,4],3],[[[8,7],[7,5]],[8,[7,7]]]]
[9,[[[9,2],0],[[9,9],[4,3]]]]
[[[4,[7,2]],[[7,9],[5,4]]],1]
[[[[4,9],5],7],[[5,6],0]]
[[[5,[3,1]],[8,1]],[8,[7,0]]]
[[5,6],[6,[[0,5],0]]]
[[[5,[4,5]],9],6]
[[[9,[7,0]],6],[2,[1,6]]]
[[[9,[8,4]],[7,[6,0]]],[[[4,6],[7,5]],[8,[0,8]]]]
[0,7]
[[3,[3,8]],[9,[[3,1],[4,4]]]]
[[6,7],[8,9]]
[[[[9,8],[0,2]],[[4,0],[7,5]]],[[[5,0],1],2]]
[[[[1,2],[3,9]],1],[[5,1],[0,1]]]
[[[[5,8],0],6],[7,0]]
[[[8,[5,4]],[[3,0],7]],[[8,[7,5]],4]]
[[[[5,8],8],8],[[[0,4],[2,5]],0]]
[[[9,6],3],[[[3,3],1],[2,[9,2]]]]
[[[6,3],6],[[[4,1],8],[2,3]]]
[2,[[1,8],0]]
[5,[[[7,6],[1,9]],[4,[8,2]]]]
[[[[6,9],[0,7]],[[2,7],8]],[[6,0],[2,[1,6]]]]
[[[[7,8],[5,1]],[[2,9],2]],0]
[5,3]
[2,[7,[7,[5,8]]]]
[[3,3],[8,[2,6]]]"), 4_690);
    }
}
