use std::ops::Mul;

use bevy::{reflect::Reflect, prelude::Vec2};


// //todo remove and use Rect
// #[derive(Reflect,Copy, Clone, PartialEq, Debug, Default)]
// pub struct UiRect {
//     // pub left : f32,
//     // pub right : f32,
//     // pub top : f32,
//     // pub bottom : f32,

//     pub min:Vec2,
//     pub max:Vec2,

// }


// impl UiRect {
//     pub fn init(x:f32) -> Self {
//         Self {
//             min: Vec2::new(x, x),
//             max: Vec2::new(x, x),
//             // left:x,right:x,top:x,bottom:x,

//         }
//     }

//     // pub fn min(&self) -> Vec2 {
//     //     Vec2::new(self.min.x,self.min.y)
//     // }

//     // pub fn max(&self) -> Vec2 {
//     //     Vec2::new(self.max.x,self.max.y)
//     // }

//     // pub fn clamp(&self,other:UiRect) -> UiRect {
//     //     // UiRect {
//     //     //     left:self.min.x.clamp(other.min.x, other.max.x),
//     //     //     top:self.min.y.clamp(other.min.y, other.max.y), //y is down

//     //     //     right:self.max.x.clamp(other.min.x, other.max.x),
//     //     //     bottom:self.max.y.clamp(other.min.y, other.max.y), //y is down
//     //     // }

//     //     UiRect {
//     //         min:self.min.clamp(other.min, other.max),
//     //         max:self.max.clamp(other.min, other.max),
//     //     }
//     // }
//     pub fn contains(&self, point:Vec2) -> bool {
//         point.x >= self.min.x && point.x <=self.max.x &&
//             point.y >= self.min.y && point.y <=self.max.y //y is down
//     }

//     pub fn width(&self) -> f32 {
//         self.max.x-self.min.x //y is down
//     }

//     pub fn height(&self) -> f32 {
//         self.max.y-self.min.y //y is down
//     }

//     // pub fn expand_by(&self,other: UiRect) -> UiRect {
//     //     // UiRect {
//     //     //     left : self.min.x - other.min.x,
//     //     //     top : self.min.y - other.min.y, //y is down
//     //     //     right :self.max.x + other.max.x,
//     //     //     bottom : self.max.y + other.max.y, //y is down
//     //     // }

//     //     UiRect {
//     //         min:self.min - other.min,
//     //         max :self.max + other.max,
//     //     }
//     // }

//     // pub fn is_zero(&self) -> bool {
//     //     self.min.x==0.0 && self.max.x==0.0 && self.min.y==0.0 && self.max.y==0.0
//     // }
//     // pub fn left_top(&self) -> Vec2 {
//     //     // Vec2::new(self.min.x,self.min.y)
//     //     self.min
//     // }
//     // pub fn left_bottom(&self) -> Vec2 {
//     //     Vec2::new(self.min.x,self.max.y)
//     // }
//     // pub fn right_bottom(&self) -> Vec2 {
//     //     // Vec2::new(self.max.x,self.max.y)
//     //     self.max
//     // }
//     // pub fn right_top(&self) -> Vec2 {
//     //     Vec2::new(self.max.x,self.min.y)
//     // }
//     // pub fn left(&self) -> f32 {
//     //     self.min.x
//     // }
//     // pub fn top(&self) -> f32 {
//     //     self.min.y
//     // }
//     // pub fn right(&self) -> f32 {
//     //     self.max.x
//     // }
//     // pub fn bottom(&self) -> f32 {
//     //     self.max.y
//     // }
//     // pub fn hsum(&self) -> f32 {
//     //     self.min.x+self.max.x
//     // }
//     // pub fn vsum(&self) -> f32 {
//     //     self.min.y+self.max.y
//     // }

//     // pub fn sum(&self) -> Vec2 {
//     //     Vec2::new(self.hsum(),self.vsum())
//     // }
//     pub fn size(&self) -> Vec2 {
//         Vec2::new(self.width(),self.height())
//     }

//     // pub fn intersects(&self,other:&Self) -> bool {
//     //     !(other.min.x > self.max.x || other.max.x < self.min.x || other.max.y < self.min.y || other.min.y > self.max.y)
//     // }
// }

// impl std::ops::Add<UiRect> for UiRect {
//     type Output = UiRect;

//     fn add(self, rhs: UiRect) -> UiRect {
//         UiRect{
//             min:self.min+rhs.min,
//             max:self.max+rhs.max,
//         }
//         // UiRect {
//         //     left : self.min.x + rhs.min.x,
//         //     right :self.max.x + rhs.max.x,
//         //     top : self.min.y + rhs.min.y, //y is down
//         //     bottom : self.max.y + rhs.max.y, //y is down
//         // }
//     }
// }

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
    pub fn new_px(all:f32) ->Self {
        Self::new_scalar(UiVal::Px(all))
    }
    pub fn new_scale(all:f32) ->Self {
        Self::new_scalar(UiVal::Scale(all))
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

impl UiVal {
    pub fn is_none(&self) -> bool {
        if let Self::None = self {
            true
        } else {
            false
        }
    }
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
