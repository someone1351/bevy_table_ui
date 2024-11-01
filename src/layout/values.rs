use std::ops::Mul;

use bevy::{reflect::Reflect, prelude::Vec2};

#[derive(Reflect,Copy, Clone, PartialEq, Debug, Default)]
pub struct UiRect {
    pub left : f32,
    pub right : f32,
    pub top : f32,
    pub bottom : f32,
}

impl UiRect {
    pub fn init(x:f32) -> Self {
        Self {
            left:x,right:x,top:x,bottom:x,
        }
    }
    pub fn clamp(&self,other:UiRect) -> UiRect {
        UiRect {
            left:self.left.clamp(other.left, other.right),
            right:self.right.clamp(other.left, other.right),
            top:self.top.clamp(other.top, other.bottom), //y is down
            bottom:self.bottom.clamp(other.top, other.bottom), //y is down
        }
    }
    pub fn contains_point(&self, point:Vec2) -> bool {
        point.x >= self.left && point.x <=self.right && 
            point.y >= self.top && point.y <=self.bottom //y is down
    }

    pub fn width(&self) -> f32 {
        self.right-self.left //y is down
    }

    pub fn height(&self) -> f32 {
        self.bottom-self.top //y is down
    }

    pub fn expand_by(&self,other: UiRect) -> UiRect {
        UiRect {
            left : self.left - other.left,
            right :self.right + other.right,
            top : self.top - other.top, //y is down
            bottom : self.bottom + other.bottom, //y is down
        }
    }

    pub fn is_zero(&self) -> bool {
        self.left==0.0 && self.right==0.0 && self.top==0.0 && self.bottom==0.0
    }
    pub fn left_top(&self) -> Vec2 {
        Vec2::new(self.left,self.top)
    }
    pub fn left_bottom(&self) -> Vec2 {
        Vec2::new(self.left,self.bottom)
    }
    pub fn right_bottom(&self) -> Vec2 {
        Vec2::new(self.right,self.bottom)
    }    
    pub fn right_top(&self) -> Vec2 {
        Vec2::new(self.right,self.top)
    }

    pub fn hsum(&self) -> f32 {
        self.left+self.right
    }
    pub fn vsum(&self) -> f32 {
        self.top+self.bottom
    }
    
    pub fn sum(&self) -> Vec2 {
        Vec2::new(self.hsum(),self.vsum())
    }
    pub fn size(&self) -> Vec2 {
        Vec2::new(self.width(),self.height())
    }

    pub fn intersects(&self,other:&Self) -> bool {
        !(other.left > self.right || other.right < self.left || other.bottom < self.top || other.top > self.bottom)        
    }
}

impl std::ops::Add<UiRect> for UiRect {
    type Output = UiRect;

    fn add(self, rhs: UiRect) -> UiRect {
        UiRect {
            left : self.left + rhs.left,
            right :self.right + rhs.right,
            top : self.top + rhs.top, //y is down
            bottom : self.bottom + rhs.bottom, //y is down
        }
    }
}

#[derive(Reflect,Copy, Clone, PartialEq, Debug, Default)]
pub struct UiRectVal {
    pub left : UiVal,
    pub right : UiVal,
    pub top : UiVal,
    pub bottom : UiVal,
}

impl UiRectVal {
    pub fn new_scalar(all:UiVal) ->Self {
        Self {
            left:all,
            right:all,
            top:all,
            bottom:all,
        }
    }
    pub fn new_size(w:UiVal,h:UiVal) ->Self {
        Self {
            left:w,
            right:w,
            top:h,
            bottom:h,
        }
    }
}
#[derive(Reflect,Copy, Clone, PartialEq, Debug, Default)]
pub enum UiAspectType {
    #[default]
    None,
    Horizontal, //calc height from width
    Vertical, //calc width from height
}

#[derive(Reflect,Copy, Clone, PartialEq, Debug, Default)]
pub enum UiVal {
    #[default]
    None,
    Px(f32),
    Scale(f32),
}

// impl Default for Val {
//     fn default() -> Self { Self::None }
// }

// impl UiVal {
//     pub fn to
// }
// impl ToString for UiVal {
//     fn to_string(&self) -> String {
//         match self {
//             Self::None => "none".to_string(),
//             Self::Px(x) => (*x as i32).to_string(),
//             Self::Scale(x) => format!("{x:?}"),
//         }
//     }
// }


// impl std::str::FromStr for UiVal {
//     type Err = ();

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let s = s.trim();

//         if s.len() == 0 || s.eq("none") {
//             Ok(Self::None)
//         } else if s.ends_with("%") {
//             let num = s[0..s.len()-1].parse::<f32>().or(Err(()))?;
//             Ok(Self::Scale(num*0.01))
//         } else if let Ok(x)=s.parse::<i32>() {
//             Ok(Self::Px(x as f32))
//         } else if let Ok(x)=s.parse::<f32>() {
//             Ok(Self::Scale(x))
//         } else {
//             Ok(Self::None)
//         }
//     }
// }

impl Mul<f32> for UiVal {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        match self {
            UiVal::Scale(p)=>{UiVal::Scale(p*rhs)},
            UiVal::Px(p)=>{UiVal::Px(p*rhs)},
            UiVal::None => UiVal::None
        }
    }
}

// #[derive(Debug, Clone, Copy)]
// pub enum HFlowDir {
//     Left, Right
// }

// impl Default for HFlowDir { 
//     fn default() -> Self {
//         Self::Right
//     }
// }

// impl std::str::FromStr for HFlowDir {
//     type Err = ();

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let s = s.trim();

//         if s.len() == 0 {
//             Ok(Self::Right)
//         } else if s.eq("left") {
//             Ok(Self::Left)
//         } else if s.eq("right") {
//             Ok(Self::Right)
//         } else {
//             Err(())
//         }
//     }
// }

// #[derive(Debug, Clone, Copy)]
// pub enum VFlowDir {
//     Up, Down
// }

// impl Default for VFlowDir { 
//     fn default() -> Self {
//         Self::Down
//     }
// }

// impl std::str::FromStr for VFlowDir {
//     type Err = ();

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let s = s.trim();

//         if s.len() == 0 {
//             Ok(Self::Down)
//         } else if s.eq("up") {
//             Ok(Self::Up)
//         } else if s.eq("down") {
//             Ok(Self::Down)
//         } else {
//             Err(())
//         }
//     }
// }

// #[derive(Debug, Clone, Copy)]
// pub enum FlowVal {
//     Horizontal {h : HFlowDir, v : VFlowDir},
//     Vertical {v : VFlowDir, h : HFlowDir},
// }

// impl Default for FlowVal { 
//     fn default() -> Self {
//         Self::Horizontal {
//             h:HFlowDir::Right,
//             v:VFlowDir::Down,
//         }
//     }
// }

// struct FlowDir {
//     vertical : bool,
//     left : bool,
//     up : bool,
// }


// #[derive(Debug, Clone, Copy)]
// pub enum UiFlowVal {
//     LeftUp,LeftDown,RightUp,RightDown,UpLeft,UpRight,DownLeft,DownRight,
// }

// impl Default for UiFlowVal { 
//     fn default() -> Self {Self::RightDown} 
// }

// impl std::str::FromStr for UiFlowVal {
//     type Err = ();

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let s = s.trim();

//         if s.len() == 0 {
//             Ok(Self::RightDown)
//         } else if s.eq("left_up") {
//             Ok(Self::LeftUp)
//         } else if s.eq("left_down") {
//             Ok(Self::LeftDown)
//         } else if s.eq("right_up") {
//             Ok(Self::RightUp)
//         } else if s.eq("right_down") {
//             Ok(Self::RightDown)
//         } else if s.eq("up_left") {
//             Ok(Self::UpLeft)
//         } else if s.eq("up_right") {
//             Ok(Self::UpRight)
//         } else if s.eq("down_left") {
//             Ok(Self::DownLeft)
//         } else if s.eq("down_right") {
//             Ok(Self::DownRight)
//         } else {
//             Err(())
//         }
//     }
// }
