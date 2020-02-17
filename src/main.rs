#![allow(non_snake_case)]
use CalcArc::GroupOfNode;
use CalcArc::GroupOfBeam;
fn main() {
    let node1 = GroupOfNode::createNode(0., 0.);
    let node2 = GroupOfNode::createNode(3., 4.);
    println!("distance is {}", node1.getDistanceTo(&node2));

    let beam = GroupOfBeam::createBeam(node1, node2);
    println!("distance is {}" , beam.getLength())
}
