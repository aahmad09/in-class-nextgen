use std::cell::RefCell;
use std::rc::Rc;

//creating a directed graph where each node can have multiple children
//using Rc<RefCell<T>> to allow shared ownership of nodes
#[derive(Debug)]
struct GraphNode<T> {
    value: T,
    children: Vec<Rc<RefCell<GraphNode<T>>>>,
}

impl<T> GraphNode<T> {
    fn new(value: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(GraphNode {
            value,
            children: vec![],
        }))
    }

    fn add_child(node: &Rc<RefCell<Self>>, child: Rc<RefCell<GraphNode<T>>>) {
        node.borrow_mut().children.push(child);
    }
}

fn main() {
    let node1 = GraphNode::new("Node 1");
    let node2 = GraphNode::new("Node 2");
    let node3 = GraphNode::new("Node 3");

    GraphNode::add_child(&node1, node2.clone());
    GraphNode::add_child(&node1, node3.clone());

    let node4 = GraphNode::new("Node 4");
    GraphNode::add_child(&node2, node4.clone());

    println!("Graph Structure: {:?}", node1.borrow());
}
