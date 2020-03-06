#![allow(non_snake_case)]
/// ```
/// use CalcArc::GroupOfNode;
/// let nodeThis = GroupOfNode::createNode(0., 0.);
/// let nodeTar = GroupOfNode::createNode(3., 4.);
/// let Dis = GroupOfNode::calcDistance(&nodeThis, &nodeTar);
/// ````
pub fn calcDistance(node1: &Node, node2: &Node) -> f64
{
    let disX = (node1.x - node2.x).abs();
    let disY = (node1.y - node2.y).abs();
    (disX.powf(2.) + disY.powf(2.)).powf(0.5) 
}

/// createNode
/// ```
/// use CalcArc::GroupOfNode;
/// let node = GroupOfNode::createNode(0., 0.);
/// ```
pub fn createNode(xtmp: f64, ytmp: f64, nodeGroup: &mut NodeGroup)
{
    let node = Node{x:xtmp, y:ytmp};
    nodeGroup.addGroup(node);
}

/// Node has X & Y
pub struct Node {
    x: f64,
    y: f64
}

impl Node{
    /// calculate distance to other node
    /// ```
    /// use CalcArc::GroupOfNode;
    /// let nodeThis = GroupOfNode::createNode(0., 0.);
    /// let nodeTarget = GroupOfNode::createNode(3., 4.);
    /// let dis = nodeThis.getDistanceTo(&nodeTarget);
    /// ```
    pub fn getDistanceTo(&self, nodeTar: &Node) -> f64
    {
        ((self.x - nodeTar.x).abs().powf(2.) + (self.y - nodeTar.y).abs().powf(2.)).powf(0.5)
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
    nodeGroup: Vec<Node> 
}

pub fn createNodeGroup() -> NodeGroup{
    NodeGroup{nodeGroup: Vec::new()}
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
        self.nodeGroup.push(node);
    }

    pub fn showGroup(&self){
        for i in &self.nodeGroup{
            println!("x:{}, y:{}",i.x, i.y);
        }
    }

    pub fn findNode(&self) -> &Node{
        self.nodeGroup.get(0).unwrap()
    }
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