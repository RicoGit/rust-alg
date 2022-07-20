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


use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;


enum NodeType {
    Sum(f32, f32), Mul(f32), Pow(f32),
    Sin, Cos,
    Variable
}

/// Computational node
struct GraphNode {
    /// Computed cached value
    val: Option<f32>,
    /// Performed operation
    kind: NodeType,
    /// Dependent children nodes
    next_nodes: Vec<Rc<RefCell<GraphNode>>>,
}

impl GraphNode {

    fn input<I: Into<f32>>(val: I) -> GraphNode {
        GraphNode {
            val,
            kind: NodeType::Variable,
            next_nodes
        }
    }

    fn sum(op1: GraphNode, op2: GraphNode) -> GraphNode {
        GraphNode {
            val: None,
            kind: NodeType::Sum(op2),
            // todo!
        }
    }


    fn add(&mut self, node: GraphNode) {
        self.next_nodes.push(Rc::new(RefCell::new(node)))
    }


}

struct Graph {
    /// Graph inputs
    input: Vec<GraphNode>,
}

impl Graph {

    // fn builder<I: Into<f32>>(input: Vec<I>) -> Graph {
    //     todo!()
    // }




    fn compute(&self) -> f32 {
        0.0
    }


}

struct GraphBuilder {
    /// ($x1 + $x2) * $x3
}

#[cfg(test)]
mod tests {

    #[test]
    fn test() {
        // x1, x2, x3 are input nodes of the computational graph:
        let x1 = create_input("x1");
        let x2 = create_input("x2");
        let x3 = create_input("x3");
        // graph variable is the output node of the graph:
        let graph = add(
            x1.clone(),
            mul(
                x2.clone(),
                sin(
                    add(
                        x2.clone(),
                        pow_f32(x3.clone(), 3f32)
                    )
                )
            )
        );
        x1.set(1f32);
        x2.set(2f32);
        x3.set(3f32);
        let mut result = graph.compute();
        result = round(result, 5);
        println!("Graph output = {}", result);
        assert_eq!(round(result, 5), -0.32727);
        x1.set(2f32);
        x2.set(3f32);
        x3.set(4f32);
        result = graph.compute();
        result = round(result, 5);
        println!("Graph output = {}", result);
        assert_eq!(round(result, 5), -0.56656);

    }
}