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

// todo trait graph

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
    Input,
}

/// Computational node
pub struct GraphNode {
    /// Cached value (computed in a first pass)
    cache: Option<f32>,
    /// Performed operation
    node_type: NodeType,
    /// Dependent nodes (back reference for recalculating when variable changed)
    back_ref: Vec<Node>,
    /// For nodes with this flag should be added back_ref
    back_ref_required: bool,
}

impl GraphNode {
    fn new(cache: Option<f32>, node_type: NodeType) -> GraphNode {
        GraphNode {
            cache,
            node_type,
            back_ref: vec![],
            back_ref_required: false,
        }
    }

    pub fn constant<I: Into<f32>>(val: I) -> Node {
        GraphNode::new(Some(val.into()), NodeType::Constant).wrap()
    }

    pub fn input<I: Into<f32>>(val: I) -> Node {
        let mut node = GraphNode::new(Some(val.into()), NodeType::Input);
        node.back_ref_required = true;
        node.wrap()
    }

    pub fn sum(op1: Node, op2: Node) -> Node {
        let node = GraphNode::new(None, NodeType::Sum(op1.clone(), op2.clone())).wrap();
        Self::add_back_refs_to_operand(&node, op1);
        Self::add_back_refs_to_operand(&node, op2);
        node
    }

    pub fn mul(op1: Node, op2: Node) -> Node {
        let node = GraphNode::new(None, NodeType::Mul(op1.clone(), op2.clone())).wrap();
        Self::add_back_refs_to_operand(&node, op1);
        Self::add_back_refs_to_operand(&node, op2);
        node
    }

    /// Calculates result and fills cache.
    /// Returns from cache if it's present.
    pub fn compute(node: &Node) -> f32 {
        fn dfs(node: &Node) -> f32 {
            let mut node = node.borrow_mut();
            if let Some(cache) = node.cache {
                return cache;
            }

            match &node.node_type {
                NodeType::Constant | NodeType::Input => node.cache.unwrap(),
                NodeType::Sum(op1, op2) => {
                    let res = dfs(op1) + dfs(op2);
                    node.cache.replace(res);
                    res
                }
                NodeType::Mul(op1, op2) => {
                    let res = dfs(op1) * dfs(op2);
                    node.cache.replace(res);
                    res
                }
            }
        }

        dfs(node)
    }

    /// Updates input variable with a new value and returns old value.
    /// If specified node in not a Input returns Err.
    pub fn set<I: Into<f32>>(node: &Node, new_val: I) -> Result<f32, ()> {
        /// Traverse to the root node via back references and clears caches during the way
        fn clear_path_to_root(node: &Node) {
            let mut node = node.borrow_mut();
            let _ = node.cache.take();
            for child in &node.back_ref {
                clear_path_to_root(child)
            }
        }

        let mut input_node = node.borrow_mut();
        if let NodeType::Input = input_node.node_type {
            // save old value
            let old_val = input_node.cache.take().unwrap();
            drop(input_node);
            // clear path to the Root
            clear_path_to_root(&node);
            // update Input node with new value
            node.borrow_mut().cache.replace(new_val.into());
            Ok(old_val)
        } else {
            Err(())
        }
    }

    fn add_back_refs_to_operand(current: &Node, op: Node) {
        let mut op = op.borrow_mut();
        if op.back_ref_required {
            op.back_ref.push(current.clone());
            current.borrow_mut().back_ref_required = true;
        }
    }

    fn wrap(self) -> Node {
        Rc::new(RefCell::new(self))
    }
}

impl Debug for GraphNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.node_type {
            NodeType::Constant => {
                write!(f, "{}", self.cache.unwrap())
            }
            NodeType::Sum(op1, op2) => {
                write!(f, "({:?}+{:?})", op1.borrow(), op2.borrow())
            }
            NodeType::Mul(op1, op2) => {
                write!(f, "({:?}*{:?})", op1.borrow(), op2.borrow())
            }
            NodeType::Input => {
                write!(
                    f,
                    "[{}]",
                    self.cache.map_or(String::new(), |f| f.to_string())
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::computational_graph::graph::*;

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

        let expression = format!("{:?}", root.as_ref().borrow());
        assert_eq!(expression, "(([1]+0.5)*([1]+([2]+([3]+-3))))");

        assert_eq!(GraphNode::compute(&root), 4.5);
        assert_eq!(GraphNode::compute(&root), 4.5)
    }

    #[test]
    fn set_should_clear_path_to_root_node() {
        let x1 = GraphNode::input(1.0);
        let root = GraphNode::sum(x1.clone(), GraphNode::input(2.0));
        assert_eq!(format!("{:?}", root.as_ref().borrow()), "([1]+[2])");

        let res = GraphNode::compute(&root);
        assert_eq!(res, 3.0);
        assert_eq!(root.as_ref().borrow().cache, Some(res));

        GraphNode::set(&x1, 11.0).unwrap();
        assert_eq!(format!("{:?}", root.as_ref().borrow()), "([11]+[2])");
        assert_eq!(root.as_ref().borrow().cache, None);
    }

    #[test]
    fn err_when_set_to_not_input_node() {
        let node = GraphNode::constant(42.0);
        assert_eq!(GraphNode::set(&node, 0.0), Err(()))
    }

    #[test]
    fn clear_and_save_cache() {
        let x1 = GraphNode::input(1.0);
        let root = GraphNode::sum(x1.clone(), GraphNode::input(2.0));

        let res1 = GraphNode::compute(&root);
        assert_eq!(res1, 3.0);
        assert_eq!(root.as_ref().borrow().cache, Some(res1));

        GraphNode::set(&x1, 11.0).unwrap();
        assert_eq!(root.as_ref().borrow().cache, None);

        let res2 = GraphNode::compute(&root);
        assert_eq!(res2, 13.0);
        assert_eq!(root.as_ref().borrow().cache, Some(res2));
    }

    #[test]
    fn big_complex_test() {
        let x1 = GraphNode::input(1.0);
        let x2 = GraphNode::input(2.0);
        let x3 = GraphNode::input(3.0);
        let x4 = GraphNode::input(4.0004);

        let mul1 = GraphNode::mul(x1.clone(), GraphNode::constant(0.5));
        let sum1 = GraphNode::sum(x1.clone(), GraphNode::constant(-3.0));
        let sum2 = GraphNode::sum(x2.clone(), sum1);
        let sum3 = GraphNode::sum(x3.clone(), sum2);
        let sum4 = GraphNode::sum(sum3, x4.clone());
        let root = GraphNode::mul(mul1, sum4);

        assert_eq!(
            format!("{:?}", root.as_ref().borrow()),
            "(([1]*0.5)*(([3]+([2]+([1]+-3)))+[4.0004]))"
        );

        assert_eq!(GraphNode::compute(&root), 3.5002);
        assert_eq!(root.as_ref().borrow().cache, Some(3.5002));

        GraphNode::set(&x2, 42.0).unwrap();
        assert_eq!(
            format!("{:?}", root.as_ref().borrow()),
            "(([1]*0.5)*(([3]+([42]+([1]+-3)))+[4.0004]))"
        );
        assert_eq!(root.as_ref().borrow().cache, None);

        assert_eq!(GraphNode::compute(&root), 23.5002);
        assert_eq!(root.as_ref().borrow().cache, Some(23.5002));
    }
}
