#![allow(non_snake_case)]
mod Node;
fn main() {
    let node1 = Node::createNode(0., 0.);
    let node2 = Node::createNode(3., 4.);
    println!("distance is {}", node1.getDistanceTo(&node2));
}
