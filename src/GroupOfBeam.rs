#![allow(non_snake_case)]
use crate::GroupOfNode::*; 

/// create Beam component
/// ```
/// let beam = createBeam(nodeI, nodeJ)
/// ````
pub fn createBeam(nodeI: Node, nodeJ: Node) -> Beam
{
    Beam{ nodeI:  nodeI
        , nodeJ:  nodeJ
        , Weight: std::collections::HashMap::new()
        }
}

/// beam component has node-I & node-J
pub struct Beam{
    nodeI: Node,
    nodeJ: Node,
    Weight: std::collections::HashMap<i32, f64>
}

impl Beam{
    pub fn getLength(&self) -> f64{
       self.nodeI.getDistanceTo(&self.nodeJ)
    } 

    pub fn addWeight(mut self, weight: (i32, f64)){
        self.Weight.insert(weight.0, weight.1);
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