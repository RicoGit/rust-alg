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

use std::cell::{Ref, RefCell, RefMut};
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;

#[derive(Clone)]
pub struct Dag(Rc<RefCell<GraphNode>>);

impl Dag {
    pub fn constant<I: Into<f32>>(val: I) -> Dag {
        GraphNode::new(Some(val.into()), NodeType::Constant).wrap()
    }

    pub fn input<I: Into<f32>>(val: I) -> Dag {
        let mut node = GraphNode::new(Some(val.into()), NodeType::Input);
        node.back_ref_required = true;
        node.wrap()
    }

    pub fn sum(op1: Dag, op2: Dag) -> Dag {
        let node = GraphNode::new(None, NodeType::Sum(op1.clone(), op2.clone())).wrap();
        Self::add_back_refs_to_operand(&node, op1);
        Self::add_back_refs_to_operand(&node, op2);
        node
    }

    pub fn mul(op1: Dag, op2: Dag) -> Dag {
        let node = GraphNode::new(None, NodeType::Mul(op1.clone(), op2.clone())).wrap();
        Self::add_back_refs_to_operand(&node, op1);
        Self::add_back_refs_to_operand(&node, op2);
        node
    }

    pub fn pow(base: Dag, exponent: Dag) -> Dag {
        let node = GraphNode::new(None, NodeType::Pow(base.clone(), exponent.clone())).wrap();
        Self::add_back_refs_to_operand(&node, base);
        Self::add_back_refs_to_operand(&node, exponent);
        node
    }

    pub fn sin(op: Dag) -> Dag {
        let node = GraphNode::new(None, NodeType::Sin(op.clone())).wrap();
        Self::add_back_refs_to_operand(&node, op);
        node
    }

    /// Calculates result and fills cache.
    /// Returns from cache if it's present.
    pub fn compute(&self) -> f32 {
        fn dfs(node: &Dag) -> f32 {
            let mut node = node.node_mut();
            if let &Some(cache) = &node.cache {
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
                NodeType::Pow(base, exp) => {
                    let res = dfs(base).powf(dfs(exp));
                    node.cache.replace(res);
                    res
                }
                NodeType::Sin(op) => {
                    let res = dfs(op).sin();
                    node.cache.replace(res);
                    res
                }
            }
        }

        dfs(self)
    }

    /// Updates input variable with a new value and returns old value.
    /// If specified node in not a Input returns Err.
    pub fn set<I: Into<f32>>(&self, new_val: I) -> Result<f32, ()> {
        /// Traverse to the root node via back references and clears caches during the way
        fn clear_path_to_root(node: &Dag) {
            let mut node = node.node_mut();
            let _ = node.cache.take();
            for child in &node.back_ref {
                clear_path_to_root(child)
            }
        }

        let mut input_node = self.node_mut();
        if let NodeType::Input = input_node.node_type {
            // save old value
            let old_val = input_node.cache.take().unwrap();
            drop(input_node);
            // clear path to the Root
            clear_path_to_root(&self);
            // update Input node with new value
            self.node_mut().cache.replace(new_val.into());
            Ok(old_val)
        } else {
            Err(())
        }
    }

    fn add_back_refs_to_operand(&self, op: Dag) {
        let mut op = op.node_mut();
        if op.back_ref_required {
            op.back_ref.push(self.clone());
            self.node_mut().back_ref_required = true;
        }
    }

    pub fn node(&self) -> Ref<GraphNode> {
        self.0.as_ref().borrow()
    }

    pub fn node_mut(&self) -> RefMut<GraphNode> {
        self.0.as_ref().borrow_mut()
    }
}

#[derive(Debug)]
enum NodeType {
    /// Input variable, can be changed
    Input,
    Constant,
    Sum(Dag, Dag),
    Mul(Dag, Dag),
    Pow(Dag, Dag),
    Sin(Dag),
}

/// Computational node
pub struct GraphNode {
    /// Cached value (computed in a first pass)
    cache: Option<f32>,
    /// Performed operation
    node_type: NodeType,
    /// Dependent nodes (back reference for recalculating when variable changed)
    back_ref: Vec<Dag>,
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

    fn wrap(self) -> Dag {
        Dag(Rc::new(RefCell::new(self)))
    }
}

impl Debug for Dag {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.node())
    }
}

impl Debug for GraphNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.node_type {
            NodeType::Constant => {
                write!(f, "{:?}", self.cache.unwrap())
            }
            NodeType::Sum(op1, op2) => {
                write!(f, "({:?}+{:?})", op1.node(), op2.node())
            }
            NodeType::Mul(op1, op2) => {
                write!(f, "({:?}*{:?})", op1.node(), op2.node())
            }
            NodeType::Pow(op1, op2) => {
                write!(f, "({:?}^{:?})", op1.node(), op2.node())
            }
            NodeType::Sin(op) => {
                write!(f, "sin({:?})", op.node())
            }
            NodeType::Input => {
                write!(f, "[{:?}]", self.cache.unwrap())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::computational_graph::graph::*;

    /// Returns cache path from input node to the root
    fn computational_path(input: &Dag) -> Vec<Option<f32>> {
        let mut caches = vec![];

        fn traverse(input: &Dag, result: &mut Vec<Option<f32>>) {
            let node = input.node();
            result.push(node.cache.clone());
            for child in &node.back_ref {
                traverse(child, result)
            }
        }
        traverse(input, &mut caches);
        caches
    }

    /// Round to decimal digits
    fn round(x: f32, precision: u32) -> f32 {
        let m = 10i32.pow(precision) as f32;
        (x * m).round() / m
    }

    #[test]
    fn build_graph_test() {
        let x1 = Dag::input(1.0);
        let x2 = Dag::input(2.0);
        let x3 = Dag::input(3.0);

        let mul1 = Dag::sum(x1.clone(), Dag::constant(0.5));
        let sum1 = Dag::sum(x3.clone(), Dag::constant(-3.0));
        let sum2 = Dag::sum(x2.clone(), sum1);
        let sum3 = Dag::sum(x1.clone(), sum2);
        let root = Dag::mul(mul1, sum3);

        let expression = format!("{:?}", root);
        assert_eq!(expression, "(([1.0]+0.5)*([1.0]+([2.0]+([3.0]+-3.0))))");
    }

    #[test]
    fn set_and_compute_small_test() {
        let x1 = Dag::input(1.0);
        let root = Dag::sum(x1.clone(), Dag::input(2.0));
        assert_eq!(format!("{:?}", root), "([1.0]+[2.0])");

        let res = Dag::compute(&root);
        assert_eq!(res, 3.0);
        assert_eq!(root.node().cache, Some(res));

        Dag::set(&x1, 11.0).unwrap();
        assert_eq!(format!("{:?}", root), "([11.0]+[2.0])");
        assert_eq!(root.node().cache, None);
    }

    #[test]
    fn err_when_set_to_not_input_node() {
        let node = Dag::constant(42.0);
        assert_eq!(Dag::set(&node, 0.0), Err(()))
    }

    #[test]
    fn clear_and_save_cache() {
        let x1 = Dag::input(1.0);
        let x2 = Dag::input(2.0);
        let x3 = Dag::input(3.0);
        let root = Dag::sum(
            x1.clone(),
            Dag::sum(x2.clone(), Dag::sum(x3.clone(), Dag::input(2.0))),
        );

        assert_eq!(Dag::compute(&root), 8.0);

        assert_eq!(computational_path(&x1), vec![Some(1.0), Some(8.0)]);
        assert_eq!(
            computational_path(&x2),
            vec![Some(2.0), Some(7.0), Some(8.0)]
        );
        assert_eq!(
            computational_path(&x3),
            vec![Some(3.0), Some(5.0), Some(7.0), Some(8.0)]
        );

        Dag::set(&x1, 11.0).unwrap();
        assert_eq!(computational_path(&x1), vec![Some(11.0), None]);
        assert_eq!(computational_path(&x2), vec![Some(2.0), Some(7.0), None]);
        assert_eq!(
            computational_path(&x3),
            vec![Some(3.0), Some(5.0), Some(7.0), None]
        );

        assert_eq!(Dag::compute(&root), 18.0);
        assert_eq!(computational_path(&x1), vec![Some(11.0), Some(18.0)]);
        assert_eq!(
            computational_path(&x2),
            vec![Some(2.0), Some(7.0), Some(18.0)]
        );
        assert_eq!(
            computational_path(&x3),
            vec![Some(3.0), Some(5.0), Some(7.0), Some(18.0)]
        );

        Dag::set(&x3, -10.0).unwrap();
        assert_eq!(computational_path(&x1), vec![Some(11.0), None]);
        assert_eq!(computational_path(&x2), vec![Some(2.0), None, None]);
        assert_eq!(computational_path(&x3), vec![Some(-10.0), None, None, None]);

        assert_eq!(Dag::compute(&root), 5.0);
        assert_eq!(computational_path(&x1), vec![Some(11.0), Some(5.0)]);
        assert_eq!(
            computational_path(&x2),
            vec![Some(2.0), Some(-6.0), Some(5.0)]
        );
        assert_eq!(
            computational_path(&x3),
            vec![Some(-10.0), Some(-8.0), Some(-6.0), Some(5.0)]
        );
    }

    #[test]
    fn big_complex_test() {
        let x1 = Dag::input(1.0);
        let x2 = Dag::input(2.0);
        let x3 = Dag::input(3.0);
        let x4 = Dag::input(4.0004);

        let mul1 = Dag::mul(x1.clone(), Dag::constant(0.5));
        let sum1 = Dag::sum(x1.clone(), Dag::constant(-3.0));
        let sum2 = Dag::sum(x2.clone(), sum1);
        let sum3 = Dag::sum(x3.clone(), sum2);
        let sum4 = Dag::sum(sum3, x4.clone());
        let root = Dag::mul(mul1, sum4);

        assert_eq!(
            format!("{:?}", root.node()),
            "(([1.0]*0.5)*(([3.0]+([2.0]+([1.0]+-3.0)))+[4.0004]))"
        );

        assert_eq!(Dag::compute(&root), 3.5002);
        assert_eq!(
            computational_path(&x1),
            vec![
                Some(1.0),
                Some(0.5),
                Some(3.5002),
                Some(-2.0),
                Some(0.0),
                Some(3.0),
                Some(7.0004),
                Some(3.5002)
            ]
        );
        assert_eq!(
            computational_path(&x2),
            vec![Some(2.0), Some(0.0), Some(3.0), Some(7.0004), Some(3.5002)]
        );
        assert_eq!(
            computational_path(&x3),
            vec![Some(3.0), Some(3.0), Some(7.0004), Some(3.5002)]
        );
        assert_eq!(
            computational_path(&x4),
            vec![Some(4.0004), Some(7.0004), Some(3.5002)]
        );

        Dag::set(&x2, 42.0).unwrap();
        assert_eq!(
            format!("{:?}", root.node()),
            "(([1.0]*0.5)*(([3.0]+([42.0]+([1.0]+-3.0)))+[4.0004]))"
        );
        assert_eq!(
            computational_path(&x2),
            vec![Some(42.0), None, None, None, None]
        );

        assert_eq!(Dag::compute(&root), 23.5002);
        assert_eq!(
            computational_path(&x2),
            vec![
                Some(42.0),
                Some(40.0),
                Some(43.0),
                Some(47.0004),
                Some(23.5002)
            ]
        );
    }

    #[test]
    fn pow_test() {
        let x1 = Dag::input(1.1);
        let x2 = Dag::input(2.2);
        let x3 = Dag::input(3.3);
        let x4 = Dag::input(4.4);

        let root = Dag::pow(
            Dag::pow(Dag::pow(x1.clone(), x2.clone()), x3.clone()),
            x4.clone(),
        );
        assert_eq!(
            format!("{:?}", root.node()),
            "((([1.1]^[2.2])^[3.3])^[4.4])"
        );

        assert_eq!(
            round(Dag::compute(&root), 5),
            round(f32::powf(f32::powf(f32::powf(1.1, 2.2), 3.3), 4.4), 5)
        );
        assert_eq!(
            computational_path(&x1),
            vec![Some(1.1), Some(1.2332864), Some(1.9976113), Some(21.001406)]
        );
    }

    #[test]
    fn sin_test() {
        let x1 = Dag::input(1.1);

        let root = Dag::sin(Dag::sin(x1.clone()));
        assert_eq!(format!("{:?}", root.node()), "sin(sin([1.1]))");

        assert_eq!(round(Dag::compute(&root), 5), round(1.1_f32.sin().sin(), 5));
        assert_eq!(
            computational_path(&x1),
            vec![Some(1.1), Some(0.8912074), Some(0.77783114)]
        );
    }
}
