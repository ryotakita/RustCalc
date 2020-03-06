#![allow(non_snake_case)]
use CalcArc::GroupOfNode;
use CalcArc::GroupOfBeam;
use CalcArc::RCParse;
fn main() {

    let fileInput = "input.txt";
    let mut nodeGroup = GroupOfNode::createNodeGroup();

    let lstGirderInput = RCParse::parseOfGirder(fileInput);
    for ParamGirder in lstGirderInput{
        let BeamXY: Vec<&str> = ParamGirder.split(',').collect();
        GroupOfNode::createNode(BeamXY[0].parse().unwrap(), BeamXY[1].parse().unwrap(), &mut nodeGroup);
        GroupOfNode::createNode(BeamXY[2].parse().unwrap(), BeamXY[3].parse().unwrap(), &mut nodeGroup);
    }

    nodeGroup.showGroup();
    println!("{}",nodeGroup.findNode().getDistanceTo(nodeGroup.findNode()))

    
}
