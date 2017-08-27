use std::rc::Rc;
use std::fmt;
use std::ops::Deref;

#[derive(Debug, Clone)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Operator::Add => write!(f, "+"),
            &Operator::Subtract => write!(f, "-"),
            &Operator::Multiply => write!(f, "*"),
            &Operator::Divide => write!(f, "/")
        }
    }
}

#[derive(Debug, Clone)]
enum Operation {
    Push(i32),
    Pop(Operator)
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Operation::Push(number) => write!(f, "{}", number),
            &Operation::Pop(ref operator) => write!(f, "{}", operator)
        }
    }
}

#[derive(Debug)]
struct NodeData {
    parent: Option<Node>,
    op: Operation,
    stack: Vec<i32>,
    numbers: Vec<i32>
}

#[derive(Clone, Debug)]
struct Node(Rc<NodeData>);

impl Deref for Node {
    type Target = NodeData;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ref parent) = self.parent {
            write!(f, "{} ", parent)?;
        }
        write!(f, "{}", self.op)
    }
}

impl Node {
    fn new(numbers: Vec<i32>) -> Node {
        Node(Rc::new(NodeData {
            parent: Option::None,
            op: Operation::Push(0),
            stack: Vec::new(),
            numbers: numbers
        }))
    }

    fn push(&self, number_index: usize) -> Node {
        let mut numbers = self.numbers.clone();
        let number = numbers.remove(number_index);

        let mut stack = self.stack.clone();
        stack.push(number);

        Node(Rc::new(NodeData {
            parent: Option::Some(self.clone()),
            op: Operation::Push(number),
            stack: stack,
            numbers: numbers
        }))
    }

    fn pop(&self, operator: Operator, stack: &Vec<i32>, result: i32) -> Node {
        let mut new_stack = stack.clone();
        new_stack.push(result);

        Node(Rc::new(NodeData {
            parent: Option::Some(self.clone()),
            op: Operation::Pop(operator),
            stack: new_stack,
            numbers: self.numbers.clone()
        }))
    }
}

fn explore_node(node: &Node) -> Vec<Node> {
    let mut children = Vec::new();

    // Pop
    if node.stack.len() >= 2 {
        let mut stack = node.stack.clone();
        let right = stack.pop().unwrap();
        let left = stack.pop().unwrap();

        children.push(node.pop(Operator::Add, &stack, left + right));

        if left - right >= 0 {
            children.push(node.pop(Operator::Subtract, &stack, left - right));
        }

        children.push(node.pop(Operator::Multiply, &stack, left * right));

        if right != 0 && left % right == 0 {
            children.push(node.pop(Operator::Divide, &stack, left / right));
        }
    }

    // Push
    for (i, _) in node.numbers.iter().enumerate() {
        children.push(node.push(i));
    }

    children
}

fn main() {
    let numbers = vec![50, 100, 9, 3, 8, 4];
//    let target = 857;
    let root = Node::new(numbers);

    println!("{}", &root);
    for child in explore_node(&root) {
        println!("|- {}", &child);
        for grandchild in explore_node(&child) {
            println!("  |- {}", &grandchild);
        }
    }
}