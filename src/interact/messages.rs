
use bevy::ecs::prelude::*;


#[derive(Debug,Message,Clone)]
pub enum UiInteractInputMessage {
    FocusInit{root_entity:Entity, group:i32},
    FocusLeft{root_entity:Entity, group:i32},
    FocusRight{root_entity:Entity, group:i32},
    FocusUp{root_entity:Entity, group:i32},
    FocusDown{root_entity:Entity, group:i32},
    FocusPrev{root_entity:Entity, group:i32},
    FocusNext{root_entity:Entity, group:i32},
    FocusEnter{root_entity:Entity, group:i32},
    FocusExit{root_entity:Entity, group:i32},

    FocusPressBegin{root_entity:Entity,group:i32,device:i32},
    FocusPressEnd{root_entity:Entity,device:i32}, //why does this lack group?
    FocusPressCancel{root_entity:Entity,device:i32}, //why does this lack group?

    CursorPressBegin{root_entity:Entity,device:i32},
    CursorPressEnd{root_entity:Entity,device:i32},
    CursorPressCancel{root_entity:Entity,device:i32},
    CursorMoveTo{root_entity:Entity,device:i32,cursor:Option<bevy::math::Vec2>},

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
    pub fn get_root_entity(&self) -> Entity {
        match & self {
            Self::FocusInit{root_entity,..}|
            Self::FocusLeft{root_entity,..}|
            Self::FocusRight{root_entity,..}|
            Self::FocusUp{root_entity,..}|
            Self::FocusDown{root_entity,..}|
            Self::FocusPrev{root_entity,..}|
            Self::FocusNext{root_entity,..}|
            Self::FocusEnter{root_entity,..}|
            Self::FocusExit{root_entity,..}|

            Self::FocusPressBegin{root_entity,..}|
            Self::FocusPressEnd{root_entity,..}|
            Self::FocusPressCancel{root_entity,..}|

            Self::CursorPressBegin{root_entity,..}|
            Self::CursorPressEnd{root_entity,..}|
            Self::CursorPressCancel{root_entity,..}|
            Self::CursorMoveTo{root_entity,..}
            => {
                *root_entity
            }
        }
    }
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

#[derive(Debug,Clone)] //
pub enum UiInteractMessageType {
    HoverBegin{device:i32,}, //don't really need device? like press?
    HoverEnd{device:i32,},
    PressBegin, //{device:i32,is_cursor:bool}, //might need hashset of devices?
    PressEnd, //{device:i32,is_cursor:bool},
    Click,
    // DragBegin,
    // DragEnd,
    // DragMove{ h_px:i32,v_px:i32, h_scale:f32,v_scale:f32, },

    DragX{px:f32,}, //scale:f32
    DragY{px:f32,}, //scale:f32
    SelectBegin,
    SelectEnd,
    FocusBegin{group:i32},
    FocusEnd{group:i32},

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

impl UiInteractMessageType {
    pub fn name(&self) -> &'static str {
        match self {
            Self::HoverBegin{..} => "hover_begin",
            Self::HoverEnd{..} => "hover_end",
            Self::PressBegin => "press_begin",
            Self::PressEnd => "press_end",
            Self::Click => "click",
            // Self::DragBegin => "drag_begin",
            // Self::DragEnd => "drag_end",
            // Self::DragMove{..} => "drag_move",
            Self::DragX{..}=>"drag_x",
            Self::DragY{..}=>"drag_y",
            Self::SelectBegin => "select_begin",
            Self::SelectEnd => "select_end",
            Self::FocusBegin{..} => "focus_begin",
            Self::FocusEnd{..} => "focus_end",

            // Self::FocusLeft{..} => "focus_left",
            // Self::FocusRight{..} => "focus_right",
            // Self::FocusUp{..} => "focus_up",
            // Self::FocusDown{..} => "focus_down",
            // Self::FocusPrev{..} => "focus_prev",
            // Self::FocusNext{..} => "focus_next",

            // Self::Custom { .. } => "custom",
            // Self::Update => "update",
            // Self::Char(_) => "char",
            // Self::FocusCancel => "focus_cancel",
        }
    }
}

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

