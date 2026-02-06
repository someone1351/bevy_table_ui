
use std::fmt::Display;

use bevy::{ecs::prelude::*, math::Vec2};

// use crate::DeviceType;

/*
TODO
* add device to focus init/left/right/up/down/prev/next/enter/exit ?
* add button to focus press begin/end/cancel?

*/

// #[derive(Debug,Message,Clone)]
// pub enum UiInteractInputFocusMessage {
//     FocusBegin{entity:Entity,device:i32},
//     FocusEnd{entity:Entity,device:i32},
//     Input(UiInteractInputMessage),
// }

#[derive(Debug,Message,Clone,Copy)]
pub enum UiInteractInputMessage {
    FocusOn{entity:Entity,device:i32},
    // FocusClear{root_entity:Entity, group:i32,device:i32},

    //add device to focus, so can have multiple users selecting from same nodes
    FocusInit{root_entity:Entity, group:i32,device:i32},
    FocusLeft{root_entity:Entity, group:i32,device:i32},
    FocusRight{root_entity:Entity, group:i32,device:i32},
    FocusUp{root_entity:Entity, group:i32,device:i32},
    FocusDown{root_entity:Entity, group:i32,device:i32},
    FocusPrev{root_entity:Entity, group:i32,device:i32},
    FocusNext{root_entity:Entity, group:i32,device:i32},
    FocusEnter{root_entity:Entity, group:i32,device:i32},
    FocusExit{root_entity:Entity, group:i32,device:i32},

    FocusPressBegin{root_entity:Entity,group:i32,device:i32,button:i32},
    FocusPressEnd{root_entity:Entity,device:i32,button:i32}, //why does this lack group? group is set by begin?
    FocusPressCancel{root_entity:Entity,device:i32,button:i32}, //why does this lack group?

    //need page up/down, to work on top of focus stk, and output should be client clamped size?
    //  or do with press or special press {root_entity:Entity,device:i32,button:i32}

    CursorPressBegin{root_entity:Entity,device:i32,button:i32},
    CursorPressEnd{root_entity:Entity,device:i32,button:i32},
    CursorPressCancel{root_entity:Entity,device:i32,button:i32},
    CursorMoveTo{root_entity:Entity,device:i32,cursor:Option<Vec2>},
    CursorScroll{root_entity:Entity,device:i32,axis:i32, scroll:f32},

    //add DragBegin/DragEnd/DragMoveTo ? so can do the mmb click to toggle scroll,

    // // Custom(Entity,String,Vec<script_lang::Value>), //root_entity,custom_event_name, params
    // Custom2{
    //     name:String,
    //     root_entity:Entity,
    //     entity:Option<Entity>,
    //     // unlocked:bool,
    //     params:Vec<script_lang::Value>,
    // },
}

impl UiInteractInputMessage {
    pub fn root_entity(&self) -> Option<Entity> {
        match *self {
            UiInteractInputMessage::FocusOn { .. } => None,
            UiInteractInputMessage::FocusInit { root_entity, ..} => Some(root_entity),
            UiInteractInputMessage::FocusLeft { root_entity, ..} => Some(root_entity),
            UiInteractInputMessage::FocusRight { root_entity, ..} => Some(root_entity),
            UiInteractInputMessage::FocusUp { root_entity, ..} => Some(root_entity),
            UiInteractInputMessage::FocusDown { root_entity, ..} => Some(root_entity),
            UiInteractInputMessage::FocusPrev { root_entity, ..} => Some(root_entity),
            UiInteractInputMessage::FocusNext { root_entity, ..} => Some(root_entity),
            UiInteractInputMessage::FocusEnter { root_entity, ..} => Some(root_entity),
            UiInteractInputMessage::FocusExit { root_entity, ..} => Some(root_entity),
            UiInteractInputMessage::FocusPressBegin { root_entity, ..} => Some(root_entity),
            UiInteractInputMessage::FocusPressEnd { root_entity, ..} => Some(root_entity),
            UiInteractInputMessage::FocusPressCancel { root_entity, ..} => Some(root_entity),
            UiInteractInputMessage::CursorPressBegin { root_entity, ..} => Some(root_entity),
            UiInteractInputMessage::CursorPressEnd { root_entity, ..} => Some(root_entity),
            UiInteractInputMessage::CursorPressCancel { root_entity, ..} => Some(root_entity),
            UiInteractInputMessage::CursorMoveTo { root_entity, ..} => Some(root_entity),
            UiInteractInputMessage::CursorScroll { root_entity, ..} => Some(root_entity),
        }
    }

    pub fn focus_group(&self) -> Option<i32> {
        match *self {
            UiInteractInputMessage::FocusOn { .. } => None,
            UiInteractInputMessage::FocusInit { group, .. } => Some(group),
            UiInteractInputMessage::FocusLeft { group, .. } => Some(group),
            UiInteractInputMessage::FocusRight { group, .. } => Some(group),
            UiInteractInputMessage::FocusUp { group, .. } => Some(group),
            UiInteractInputMessage::FocusDown { group, .. } => Some(group),
            UiInteractInputMessage::FocusPrev { group, .. } => Some(group),
            UiInteractInputMessage::FocusNext { group, .. } => Some(group),
            UiInteractInputMessage::FocusEnter { group, .. } => Some(group),
            UiInteractInputMessage::FocusExit { group, .. } => Some(group),
            UiInteractInputMessage::FocusPressBegin { group, .. } => Some(group),
            UiInteractInputMessage::FocusPressEnd { .. } => None,
            UiInteractInputMessage::FocusPressCancel { .. } => None,
            UiInteractInputMessage::CursorPressBegin { .. } => None,
            UiInteractInputMessage::CursorPressEnd { .. } => None,
            UiInteractInputMessage::CursorPressCancel { .. } => None,
            UiInteractInputMessage::CursorMoveTo { .. } => None,
            UiInteractInputMessage::CursorScroll { .. } => None,
        }
    }
    pub fn device(&self) -> i32 {
        match *self {
            UiInteractInputMessage::FocusOn { device, .. } => device,
            UiInteractInputMessage::FocusInit { device, .. } => device,
            UiInteractInputMessage::FocusLeft { device, .. } => device,
            UiInteractInputMessage::FocusRight { device, .. } => device,
            UiInteractInputMessage::FocusUp { device, .. } => device,
            UiInteractInputMessage::FocusDown { device, .. } => device,
            UiInteractInputMessage::FocusPrev { device, .. } => device,
            UiInteractInputMessage::FocusNext { device, .. } => device,
            UiInteractInputMessage::FocusEnter { device, .. } => device,
            UiInteractInputMessage::FocusExit { device, .. } => device,
            UiInteractInputMessage::FocusPressBegin { device, .. } => device,
            UiInteractInputMessage::FocusPressEnd { device, .. } => device,
            UiInteractInputMessage::FocusPressCancel { device, .. } => device,
            UiInteractInputMessage::CursorPressBegin { device, .. } => device,
            UiInteractInputMessage::CursorPressEnd { device, .. } => device,
            UiInteractInputMessage::CursorPressCancel { device, .. } => device,
            UiInteractInputMessage::CursorMoveTo { device, .. } => device,
            UiInteractInputMessage::CursorScroll { device, .. } => device,
        }
    }
    pub fn is_focus_move(&self) -> bool {
        match *self {
            UiInteractInputMessage::FocusOn {..} => true,
            UiInteractInputMessage::FocusInit {..} => true,
            UiInteractInputMessage::FocusLeft {..} => true,
            UiInteractInputMessage::FocusRight {..} => true,
            UiInteractInputMessage::FocusUp {..} => true,
            UiInteractInputMessage::FocusDown {..} => true,
            UiInteractInputMessage::FocusPrev {..} => true,
            UiInteractInputMessage::FocusNext {..} => true,
            UiInteractInputMessage::FocusEnter {..} => true,
            UiInteractInputMessage::FocusExit {..} => true,
            UiInteractInputMessage::FocusPressBegin {..} => false,
            UiInteractInputMessage::FocusPressEnd {..} => false,
            UiInteractInputMessage::FocusPressCancel {..} => false,
            UiInteractInputMessage::CursorPressBegin {..} => false,
            UiInteractInputMessage::CursorPressEnd {..} => false,
            UiInteractInputMessage::CursorPressCancel {..} => false,
            UiInteractInputMessage::CursorMoveTo {..} => false,
            UiInteractInputMessage::CursorScroll {..} => false,
        }
    }
    pub fn is_focus(&self) -> bool {
        match *self {
            UiInteractInputMessage::FocusOn {..} => true,
            UiInteractInputMessage::FocusInit {..} => true,
            UiInteractInputMessage::FocusLeft {..} => true,
            UiInteractInputMessage::FocusRight {..} => true,
            UiInteractInputMessage::FocusUp {..} => true,
            UiInteractInputMessage::FocusDown {..} => true,
            UiInteractInputMessage::FocusPrev {..} => true,
            UiInteractInputMessage::FocusNext {..} => true,
            UiInteractInputMessage::FocusEnter {..} => true,
            UiInteractInputMessage::FocusExit {..} => true,
            UiInteractInputMessage::FocusPressBegin {..} => true,
            UiInteractInputMessage::FocusPressEnd {..} => true,
            UiInteractInputMessage::FocusPressCancel {..} => true,
            UiInteractInputMessage::CursorPressBegin {..} => false,
            UiInteractInputMessage::CursorPressEnd {..} => false,
            UiInteractInputMessage::CursorPressCancel {..} => false,
            UiInteractInputMessage::CursorMoveTo {..} => false,
            UiInteractInputMessage::CursorScroll {..} => false,
        }
    }
    // pub fn device_type(&self) -> DeviceType {
    //     match *self {
    //         UiInteractInputMessage::FocusOn {device,..} => DeviceType::Focus(device),
    //         UiInteractInputMessage::FocusInit {device,..} => DeviceType::Focus(device),
    //         UiInteractInputMessage::FocusLeft {device,..} => DeviceType::Focus(device),
    //         UiInteractInputMessage::FocusRight {device,..} => DeviceType::Focus(device),
    //         UiInteractInputMessage::FocusUp {device,..} => DeviceType::Focus(device),
    //         UiInteractInputMessage::FocusDown {device,..} => DeviceType::Focus(device),
    //         UiInteractInputMessage::FocusPrev {device,..} => DeviceType::Focus(device),
    //         UiInteractInputMessage::FocusNext {device,..} => DeviceType::Focus(device),
    //         UiInteractInputMessage::FocusEnter {device,..} => DeviceType::Focus(device),
    //         UiInteractInputMessage::FocusExit {device,..} => DeviceType::Focus(device),
    //         UiInteractInputMessage::FocusPressBegin {device,..} => DeviceType::Focus(device),
    //         UiInteractInputMessage::FocusPressEnd {device,..} => DeviceType::Focus(device),
    //         UiInteractInputMessage::FocusPressCancel {device,..} => DeviceType::Focus(device),
    //         UiInteractInputMessage::CursorPressBegin {device,..} => DeviceType::Cursor(device),
    //         UiInteractInputMessage::CursorPressEnd {device,..} => DeviceType::Cursor(device),
    //         UiInteractInputMessage::CursorPressCancel {device,..} => DeviceType::Cursor(device),
    //         UiInteractInputMessage::CursorMoveTo {device,..} => DeviceType::Cursor(device),
    //     }
    // }
}
// impl Clone for UiInteractInputEvent {
//     fn clone(&self) -> Self {
//         match self {
//             Self::FocusInit(root_entity,focus_group) => Self::FocusInit(*root_entity,*focus_group),
//             Self::FocusLeft(root_entity,focus_group) => Self::FocusLeft(*root_entity,*focus_group),
//             Self::FocusRight(root_entity,focus_group) => Self::FocusRight(*root_entity,*focus_group),
//             Self::FocusUp(root_entity,focus_group) => Self::FocusUp(*root_entity,*focus_group),
//             Self::FocusDown(root_entity,focus_group) => Self::FocusDown(*root_entity,*focus_group),
//             Self::FocusPrev(root_entity,focus_group) => Self::FocusPrev(*root_entity,*focus_group),
//             Self::FocusNext(root_entity,focus_group) => Self::FocusNext(*root_entity,*focus_group),
//             Self::FocusEnter(root_entity,focus_group) => Self::FocusEnter(*root_entity,*focus_group),
//             Self::FocusExit(root_entity,focus_group) => Self::FocusExit(*root_entity,*focus_group),

//             Self::FocusPressBegin(root_entity, focus_group, device) => Self::FocusPressBegin(*root_entity, *focus_group, *device),
//             Self::FocusPressEnd(root_entity,focus_group) => Self::FocusPressEnd(*root_entity,*focus_group),
//             Self::FocusPressCancel(root_entity,focus_group) => Self::FocusPressCancel(*root_entity,*focus_group),

//             Self::CursorPressBegin(root_entity,focus_group) => Self::CursorPressBegin(*root_entity,*focus_group),
//             Self::CursorPressEnd(root_entity,focus_group) => Self::CursorPressEnd(*root_entity,*focus_group),
//             Self::CursorPressCancel(root_entity,focus_group) => Self::CursorPressCancel(*root_entity,*focus_group),
//             Self::CursorMoveTo(root_entity,focus_group,cursor) => Self::CursorMoveTo(*root_entity,*focus_group,*cursor),

//             // // Self::Custom(root_entity,name,params) => Self::Custom(
//             // //     *root_entity,
//             // //     name.clone(),
//             // //     params.iter().map(|x|x.clone_as_is()).collect::<Vec<_>>(),
//             // // ),

//             // Self::Custom2 { name, root_entity, entity,
//             //     // unlocked,
//             //     params,
//             // } => Self::Custom2 {
//             //     name:name.clone(), root_entity:*root_entity, entity:*entity,
//             //     //unlocked:*unlocked,
//             //     params: params.iter().map(|x|x.clone_as_is()).collect::<Vec<_>>(), //used clone_as_is instead of clone_root, so if user incorrectly doesn't use the clone_as_root elsewhere, then the error wont be hidden when passed through here
//             // },
//         }
//     }
// }

/*
TODO
* have focus/cursor press begin/end/click?
*/

#[derive(Debug,Clone,Copy)] //
pub enum UiInteractMessageType {
    CursorHoverBegin{device:i32,cursor:Vec2,}, //don't really need device? like press?
    CursorHoverEnd{device:i32,},
    CursorPressBegin{device:i32,button:i32,first:bool,cursor:Vec2,}, //,is_cursor:bool //might need hashset of devices?
    CursorPressEnd{device:i32,button:i32,last:bool,},
    CursorClick{device:i32,button:i32,
        // times:u32, //how many times within the frame, most times will be 0, as press/release happens over multiple frames
    },

    //add drag_begin/drag_end
    CursorDragX{dist:f32,delta:f32,device:i32,button:i32,}, //scale:f32
    CursorDragY{dist:f32,delta:f32,device:i32,button:i32,}, //scale:f32

    CursorDragBegin{device:i32,button:i32,outer_offset:Vec2,inner_offset:Vec2,cursor:Vec2,},
    CursorDragEnd{device:i32,button:i32},
    // CursorDrag{dist:f32,delta:f32,device:i32,button:i32,axis:i32,},
    CursorScroll{scroll:f32,device:i32,axis:i32,},


    FocusPressBegin{device:i32,button:i32},
    FocusPressEnd{device:i32,button:i32},
    FocusClick{device:i32,button:i32,},

    //DragCursorX
    //DragCursorY
    //DragScrollX
    //DragScrollY

    SelectBegin,
    SelectEnd,
    FocusBegin{group:i32,device:i32,},
    FocusEnd{group:i32,device:i32,},

    // FocusLeft{group:i32,moved:bool},
    // FocusRight{group:i32,moved:bool},
    // FocusUp{group:i32,moved:bool},
    // FocusDown{group:i32,moved:bool},
    // FocusPrev{group:i32,moved:bool},
    // FocusNext{group:i32,moved:bool},
    // Custom{event_name:String,params:Vec<script_lang::Value>},
    // FocusCancel,
    // Char(char),
    // Update,
}

// impl Clone for UiEventType {
//     fn clone(&self) -> Self {
//         match self {
//             Self::Custom{event_name,params} => Self::Custom{
//                 event_name:event_name.clone(),
//                 params:params.iter().map(|x|x.clone_as_is()).collect::<Vec<_>>(),
//             },
//             Self::HoverBegin => Self::HoverBegin,
//             Self::HoverEnd => Self::HoverEnd,
//             Self::PressBegin => Self::PressBegin,
//             Self::PressEnd => Self::PressEnd,
//             Self::Click => Self::Click,
//             Self::DragBegin => Self::DragBegin,
//             Self::DragEnd => Self::DragEnd,
//             Self::DragMove{ h_px,v_px, h_scale,v_scale, } => Self::DragMove{
//                 h_px:*h_px,
//                 v_px:*v_px,
//                 h_scale : *h_scale,
//                 v_scale:*v_scale,
//             },
//             Self::SelectBegin => Self::SelectBegin,
//             Self::SelectEnd => Self::SelectEnd,
//             Self::FocusBegin{group} => Self::FocusBegin{group:*group},
//             Self::FocusEnd{group} => Self::FocusEnd{group:*group},

//         }
//     }
// }

// impl UiInteractMessageType {
//     pub fn name(&self) -> &'static str {
//         match self {
//             Self::HoverBegin{..} => "hover_begin",
//             Self::HoverEnd{..} => "hover_end",
//             Self::CursorPressBegin{..} => "press_begin",
//             Self::CursorPressEnd{..} => "press_end",
//             Self::CursorClick{..} => "click",
//             // Self::DragX{..}=>"drag_x",
//             // Self::DragY{..}=>"drag_y",
//             Self::CursorDrag { .. } => "drag",
//             Self::SelectBegin => "select_begin",
//             Self::SelectEnd => "select_end",
//             Self::FocusBegin{..} => "focus_begin",
//             Self::FocusEnd{..} => "focus_end",

//             Self::FocusPressBegin {.. } => "focus_press_begin",
//             Self::FocusPressEnd {.. } => "focus_press_end",
//             Self::FocusClick {.. } => "focus_click",

//         }
//     }
// }

// #[derive(Debug)]
// pub enum UiEventEntity {
//     Global(Entity), //root_entity
//     Entity(Entity),
// }

#[derive(Debug,Message,Clone)]
pub struct UiInteractEvent {
    pub entity : Entity,
    // pub node : UiNode,
    pub event_type : UiInteractMessageType,
    // pub ids : Vec<String>,
}

impl Display for UiInteractEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} : {:?}", self.entity,self.event_type)
    }
}