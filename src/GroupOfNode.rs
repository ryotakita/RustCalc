#![allow(non_snake_case)]
use plotlib::page::Page;
use plotlib::scatter::{Scatter, Style};
use plotlib::style::Point;
use plotlib::view::ContinuousView;
use std::collections::HashMap;
/// ```
/// use CalcArc::GroupOfNode;
/// let nodeThis = GroupOfNode::createNode(0., 0.);
/// let nodeTar = GroupOfNode::createNode(3., 4.);
/// let Dis = GroupOfNode::calcDistance(&nodeThis, &nodeTar);
/// ````
pub fn calcDistance(node1: &Node, node2: &Node) -> f64
{
    let disX = (node1.pt.x - node2.pt.x).abs();
    let disY = (node1.pt.y - node2.pt.y).abs();
    let tmp = (disX.pow(2) + disY.pow(2)) as f64;
    tmp.powf(0.5)
}

/// createNode
/// ```
/// use CalcArc::GroupOfNode;
/// let node = GroupOfNode::createNode(0., 0.);
/// ```
pub fn createNode(xtmp: i64, ytmp: i64, nodeGroup: &mut NodeGroup)
{
    let pt = Pt{x:xtmp, y:ytmp};
    let node = Node{pt};
    nodeGroup.addGroup(node);
}

// Pt has X & Y
#[derive(PartialEq, Eq, Hash,Clone, Copy)]
pub struct Pt{
    x: i64,
    y: i64
}

/// Node has X & Y
pub struct Node {
    pt: Pt
}

impl Node{

    fn getPt(&self) -> Pt{
        self.pt
    }
    /// calculate distance to other node
    /// ```
    /// use CalcArc::GroupOfNode;
    /// let nodeThis = GroupOfNode::createNode(0., 0.);
    /// let nodeTarget = GroupOfNode::createNode(3., 4.);
    /// let dis = nodeThis.getDistanceTo(&nodeTarget);
    /// ```
    pub fn getDistanceTo(&self, nodeTar: &Node) -> f64
    {
        let temp = ((self.pt.x - nodeTar.pt.x).abs().pow(2) + (self.pt.y - nodeTar.pt.y).abs().pow(2)) as f64;
        temp.powf(0.5)
    }

    pub fn getX(&self) -> i64
    {
        self.pt.x
    }

    pub fn getY(&self) -> i64
    {
        self.pt.y
    }
}

/// manegeNodeGroup
/// ```
/// use CalcArc::GroupOfNode;
/// let nodeThis = GroupOfNode::createNode(0., 0.);
/// let nodeTarget = GroupOfNode::createNode(3., 4.);
/// let dis = nodeThis.getDistanceTo(&nodeTarget);
/// ```
pub struct NodeGroup{
    nodeGroup: HashMap<Pt, Node>
}

pub fn createNodeGroup() -> NodeGroup{
    NodeGroup{nodeGroup: HashMap::new()}
}

impl NodeGroup{
    /// calculate distance to other node
    /// ```
    /// use CalcArc::GroupOfNode;
    /// let nodeThis = GroupOfNode::createNode(0., 0.);
    /// let nodeTarget = GroupOfNode::createNode(3., 4.);
    /// let dis = nodeThis.getDistanceTo(&nodeTarget);
    /// ```
    pub fn addGroup(&mut self, node: Node) 
    {
        self.nodeGroup.entry(node.getPt()).or_insert(node);
    }

    pub fn showGroup(&self){
        for (k,v) in &self.nodeGroup{
            println!("x:{}, y:{}",k.x, k.y);
        }
    }

    //pub fn findNodeByPt(&self, x: f64, y:f64) -> &Node{
    //    self.nodeGroup.iter().find(|&node| node.getX() == x && node.getY() == y).unwrap()
    //}

    //pub fn getNodeGroup(&self) -> &Vec<Node>{
        //&self.nodeGroup
    //}
    //pub fn createSVG(&self){
        //let vec = self.getNodeGroup();
        //let mut vec2 :Vec<(f64, f64)> = vec![];
        //for i in vec{
            //vec2.push((i.getX(), i.getY()));
        //}
        //let s = Scatter::from_slice(&vec2).style(
            //Style::new().colour("#35C788"),
        //);

        //let v = ContinuousView::new()
            //.add(&s)
            //.x_range(-5., 10.)
            //.y_range(-2., 6.)
            //.x_label("x")
            //.y_label("y");

        //Page::single(&v).save("Node.svg").unwrap();
    //}

}


#[test]
fn structNode_test()
{
    let node = createNode(0., 0.);
    assert_eq!(node.x, 0.);
    assert_eq!(node.y, 0.);

    let nodeTar = createNode(3., 4.);
    assert_eq!(nodeTar.x, 3.);
    assert_eq!(nodeTar.y, 4.);

    assert_eq!(node.getDistanceTo(&nodeTar), 5.);
}

#[test]
fn calcDis_test()
{
    let node1 = Node{x:0.,y:0.};
    let mut node2 = Node{x:3.,y:4.};
    assert_eq!(calcDistance(&node1, &node2),5.);
    node2.x = 1.;
    node2.y = 1.;
    assert_eq!(calcDistance(&node1, &node2), 2.0_f64.powf(0.5));
}