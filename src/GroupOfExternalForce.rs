
#![allow(non_snake_case)]
use crate::GroupOfNode::*;

/// 外力を生成します。
pub fn createForce(x: i64, y: i64, f: f64, grp: &mut ForceGroup) {
    let pt = createPt(x, y);
    let force = Force{pt: pt, power: f};
    grp.addGroup(force);
}

/// 外力のクラス
/// ## Note
/// - 外力は上向き生として値を与えられます。
/// - 単位はkNです。
#[derive(Debug)]
pub struct Force {
    pt: Pt,
    power: f64
}

impl Force{
    pub fn getPt(&self) -> Pt{
        self.pt
    }
}

/// 外力の管理クラスです。
pub struct ForceGroup{
    forceGroup: Vec< Force > 
}

/// 外力の管理クラスを生成します。
pub fn createForceGroup() -> ForceGroup{
    ForceGroup{forceGroup: Vec::new()}
}

impl ForceGroup{

    /// 外力管理クラスに外力を追加します。
    pub fn addGroup(&mut self, force: Force) 
    {
        self.forceGroup.push(force)
    }

    pub fn getForceGroup(&self) -> &Vec<Force>{
        &self.forceGroup
    }


    /// 節点の一覧を出力します。
    /// ## Assert
    /// - 現在は直接プリントしていますが、将来的にはリストを渡します。
    pub fn showGroup(&self){
        println!("外力の一覧を出力します。");
        for i in &self.forceGroup{
            println!("{:?}", i)
        }
    }


}


