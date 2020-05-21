#![allow(non_snake_case)]
use plotlib::page::Page;
use std::io;
use plotlib::scatter::{Scatter, Style};
use plotlib::style::Point;
use plotlib::view::ContinuousView;
use ARCS::RCParse;
use ARCS::GroupOfBeam;
use ARCS::GroupOfNode;
use ARCS::GroupOfExternalForce;

fn main() {

    let fileInput = "input.txt";
    let mut nodeGroup = GroupOfNode::createNodeGroup();
    let mut beamGroup = GroupOfBeam::createBeamGroup();
    let mut forceGroup = GroupOfExternalForce::createForceGroup();

    let lstGirderInput = RCParse::parseOfGirder(fileInput);
    let mut parthOfGirder = |vec: &Vec<&str>| {
        let Ix = vec[0].parse().unwrap();
        let Iy = vec[1].parse().unwrap();
        let Jx = vec[2].parse().unwrap();
        let Jy = vec[3].parse().unwrap();
        let ptI = GroupOfNode::createPt(Ix, Iy);
        let ptJ = GroupOfNode::createPt(Jx, Jy);
        GroupOfBeam::createBeam(ptI, ptJ, &mut beamGroup, &mut nodeGroup);
    };
    let mut parthOfForce = |vec: &Vec<&str>| {
        let x = match vec[0].parse(){
            Ok(F) => F,
            Err(error) => 
                panic!("error:{}", error),
        };
        let y = match vec[1].parse(){
            Ok(F) => F,
            Err(error) => 
                panic!("error:{}", error),
        };
        let f = match vec[2].parse(){
            Ok(F) => F,
            Err(error) => 
                panic!("error:{}", error),
        };

        GroupOfExternalForce::createForce(x, y, f, &mut forceGroup);
    };

    for ParamGirder in lstGirderInput{
        let BeamXY: Vec<&str> = ParamGirder.split(',').collect();
        let lenParth = BeamXY.len();
        match lenParth{
            3 => parthOfForce(&BeamXY),
            4 => parthOfGirder(&BeamXY),
            _ => panic!{"input size is not collect. size : {}", lenParth}
        }
    }

    // エラーチェック
    for force in forceGroup.getForceGroup(){
        match nodeGroup.isExistNode(force.getPt()){
            Err(err) => panic!("force is exist in not Node. ThisForceIs:{:?}", err),
            Ok(())     => ()
        };
        
    }

    



    nodeGroup.showGroup();
    beamGroup.showGroup();
    forceGroup.showGroup();
    let mut guess = String::new();
    io::stdin().read_line(&mut guess);
    //nodeGroup.createSVG();
    //beamGroup.createSVG();

    
}
