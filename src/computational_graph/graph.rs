//!
//! Computational graph is a direct acyclic graph, where each node takes one or multiple inputs
//! and produces one or more outputs. All inputs and outputs are floating point values (`f32`).
//!
//! # For example:
//!
//! `y = x1 + x2 * sin(x2 + pow(x3, 3))`
//!
//! where `x1`, `x2`, `x3` are computation graph inputs and y is a graph output. 3 is the exponent for
//! power function.
//!


use std::cell::RefCell;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;

type Node = Rc<RefCell<GraphNode>>;

#[derive(Debug)]
enum NodeType {
    Constant,
    Sum(Node, Node),
    Mul(Node, Node),
    // Pow(f32, f32),
    // Sin(f32),
    // Cos(f32),
    /// Input variable, can be changed
    Input
}

impl NodeType {

    /// Checks whether a Node has at least one back reference
    fn has_back_ref(&self) -> bool {

        fn has_ref(node: &Node) -> bool {
            node.as_ref().borrow().back_ref.is_empty()
        }

        match self {
            NodeType::Sum(op1, op2) => { has_ref(op1) || has_ref(op2) }
            _ => { false }
        }
    }
}

/// Computational node
pub struct GraphNode {
    /// Cached value (computed in a first pass)
    cache: Option<f32>,
    /// Performed operation
    operation: NodeType,
    /// Dependent nodes (back reference for recalculating when variable changed)
    back_ref: Vec<Node>,
}

impl GraphNode {


    pub fn constant<I: Into<f32>>(val: I) -> Node {
        GraphNode {
            cache: Some(val.into()),
            operation: NodeType::Constant,
            back_ref: vec![]
        }.wrap()
    }

    pub fn input<I: Into<f32>>(val: I) -> Node {
        GraphNode {
            cache: Some(val.into()),
            operation: NodeType::Input,
            back_ref: vec![]
        }.wrap()
    }

    pub fn sum(op1: Node, op2: Node) -> Node {

        let node = GraphNode {
            cache: None,
            operation: NodeType::Sum(op1.clone(), op2.clone()),
            back_ref: vec![]
        }.wrap();

        Self::add_back_refs_to_operand(&node, op1);
        Self::add_back_refs_to_operand(&node, op2);

        node
    }

    pub fn mul(op1: Node, op2: Node) -> Node {

        let node = GraphNode {
            cache: None,
            operation: NodeType::Mul(op1.clone(), op2.clone()),
            back_ref: vec![]
        }.wrap();

        Self::add_back_refs_to_operand(&node, op1);
        Self::add_back_refs_to_operand(&node, op2);

        node
    }

    /// Calculates result and fills cache.
    /// If input didn't get changed returns old value.
    /// If input was changed recalculates only affected nodes
    pub fn compute(&self) -> f32 {
        0.0
    }

    fn add_back_refs_to_operand(current: &Node, op: Node) {
        let mut op= op.borrow_mut();
        if let NodeType::Input = op.operation {
            op.back_ref.push(current.clone())
        }
        if op.operation.has_back_ref() {
            op.back_ref.push(current.clone())
        }
    }

    fn wrap(self) -> Node {
        Rc::new(RefCell::new(self))
    }

}

impl Debug for GraphNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.operation {
            NodeType::Constant => {  write!(f, "{:?}", self.cache.unwrap()) }
            NodeType::Sum(op1, op2) => { write!(f, "({:?}+{:?})", op1.borrow(), op2.borrow()) }
            NodeType::Mul(op1, op2) => { write!(f, "({:?}*{:?})", op1.borrow(), op2.borrow()) }
            NodeType::Input => { write!(f, "[{:?}]", self.cache.unwrap()) }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::{Debug, Formatter};

    use crate::computational_graph::graph::GraphNode;

    #[test]
    fn build_graph_test() {

        let x1 = GraphNode::input(1.0);
        let x2 = GraphNode::input(2.0);
        let x3 = GraphNode::input(3.0);

        let mul1 = GraphNode::sum(x1.clone(), GraphNode::constant(0.5));
        let sum1 = GraphNode::sum(x3.clone(), GraphNode::constant(-3.0));
        let sum2 = GraphNode::sum(x2.clone(), sum1);
        let sum3 = GraphNode::sum(x1.clone(), sum2);
        let root = GraphNode::mul(mul1, sum3);

        let expression = format!("{:?}", root.borrow());
        assert_eq!(expression, "(([1.0]+0.5)*([1.0]+([2.0]+([3.0]+-3.0))))")
    }
}