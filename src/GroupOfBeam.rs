#![allow(non_snake_case)]
use crate::GroupOfNode::*; 
use plotlib::page::Page;
use plotlib::scatter::{Scatter, Style};
use plotlib::line::Line;
use plotlib::style::Point;
use plotlib::view::ContinuousView;

/// create Beam component
/// ```
/// let beam = createBeam(nodeI, nodeJ)
/// ````
pub fn createBeam(IxIn: f64, IyIn: f64, JxIn: f64, JyIn: f64, grp: &mut BeamGroup) {
    let beam = Beam{ Ix: IxIn , Iy: IyIn, Jx:JxIn, Jy:JyIn };
    grp.addGroup(beam);
}

/// beam component has node-I & node-J
pub struct Beam{
    Ix : f64,
    Iy : f64,
    Jx : f64,
    Jy : f64
}

impl Beam{
}

pub fn createBeamGroup() -> BeamGroup{
    BeamGroup{beamGroup: Vec::new()}
}

pub struct BeamGroup{
    beamGroup: Vec< Beam > 
}

impl BeamGroup{
    /// calculate distance to other node
    /// ```
    /// use CalcArc::GroupOfBeam;
    /// let nodeThis = GroupOfBeam::createBeam(0., 0.);
    /// let nodeTarget = GroupOfBeam::createBeam(3., 4.);
    /// let dis = nodeThis.getDistanceTo(&nodeTarget);
    /// ```
    pub fn addGroup(&mut self, beam: Beam) 
    {
        self.beamGroup.push(beam);
    }

    pub fn showGroup(&self){
        for i in &self.beamGroup{
            println!("Ix:{}, Iy:{}, Jx:{}, Jy:{}",i.Ix, i.Iy, i.Jx, i.Jy);
        }
    }

    pub fn getBeamGroup(&self) -> &Vec<Beam>{
        &self.beamGroup
    }
    pub fn createSVG(&self){
        let vec = self.getBeamGroup();
        let mut vec2 :Vec<(f64, f64)> = vec![];
        for i in vec{
            vec2.push((i.Ix, i.Iy));
            vec2.push((i.Jx, i.Jy));
        }
        let s = Line::new(&vec2[..]);

        let v = ContinuousView::new()
            .add(&s)
            .x_range(-5., 10.)
            .y_range(-2., 6.)
            .x_label("x")
            .y_label("y");

        Page::single(&v).save("Beam.svg").unwrap();
    }
}

#[test]
fn createBeam_test()
{
    let nodeI = createNode(0., 0.);
    let nodeJ = createNode(100., 200.);
    let beam = createBeam(nodeI, nodeJ);
    assert_eq!(beam.getLength(), (100.0_f64.powf(2.) + 200.0_f64.powf(2.)).powf(0.5));
}