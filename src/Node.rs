/// ```
/// let Dis = calcDistance(nodeThis, nodeTar);
/// ````
pub fn calcDistance(node1: &Node, node2: &Node) -> f64
{
    let disX = (node1.x - node2.x).abs();
    let disY = (node1.y - node2.y).abs();
    (disX.powf(2.) + disY.powf(2.)).powf(0.5) 
}

/// ```
/// let node = Node{x:0., y:0.};
/// ```
pub struct Node {
    pub x: f64,
    pub y: f64
}

impl Node{
    /// ```
    /// let dis = nodeThis.getDistanceTo(&nodeTarget);
    /// ```
    pub fn getDistanceTo(&self, nodeTar: &Node) -> f64
    {
        ((self.x - nodeTar.x).abs().powf(2.) + (self.y - nodeTar.y).abs().powf(2.)).powf(0.5)
    }
}

/// テスト実行
#[test]
fn structNode_test()
{
    let node = Node{x:0.,y:0.};
    assert_eq!(node.x, 0.);
    assert_eq!(node.y, 0.);

    let nodeTar = Node{x:3.,y:4.};
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