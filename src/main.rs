#![allow(non_snake_case)]
mod Node;
fn main() {
    let node1 = Node::Node{x:0., y:0.};
    let node2 = Node::Node{x:3., y:4.};
    println!("distance is {}", node1.getDistanceTo(&node2));
}
