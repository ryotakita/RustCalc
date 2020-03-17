#![allow(non_snake_case)]
use plotlib::page::Page;
use plotlib::scatter::{Scatter, Style};
use plotlib::style::Point;
use plotlib::view::ContinuousView;
use CalcArc::GroupOfNode;
use CalcArc::GroupOfBeam;
use CalcArc::RCParse;
fn main() {

    let fileInput = "input.txt";
    let mut nodeGroup = GroupOfNode::createNodeGroup();
    let mut beamGroup = GroupOfBeam::createBeamGroup();

    let lstGirderInput = RCParse::parseOfGirder(fileInput);
    for ParamGirder in lstGirderInput{
        let BeamXY: Vec<&str> = ParamGirder.split(',').collect();
        let Ix = BeamXY[0].parse().unwrap();
        let Iy = BeamXY[1].parse().unwrap();
        let Jx = BeamXY[2].parse().unwrap();
        let Jy = BeamXY[3].parse().unwrap();
        GroupOfNode::createNode(Ix, Iy, &mut nodeGroup);
        GroupOfNode::createNode(Jx, Jy, &mut nodeGroup);
        GroupOfBeam::createBeam(Ix, Iy, Jx, Jy, &mut beamGroup);
    }

    nodeGroup.showGroup();
    //nodeGroup.createSVG();
    //beamGroup.createSVG();

    
}
