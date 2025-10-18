

// #![allow(unused_mut)]
// #![allow(dead_code)]
// #![allow(unused_variables)]
// #![allow(unused_imports)]
// #![allow(unused_assignments)]
// #[allow(unused_parens)]

mod layout;
mod interact;
mod display;
// mod affect;

pub use layout::{plugin::*,components::*,values::*,};
pub use interact::{plugin::*,components::*,resources::*,messages::*};
pub use display::{plugin::*,components::*,values::*};
// pub use affect::{plugin::*,components::*};

pub use display::render_core::core_my::CameraUi; //mesh
pub use display::TestRenderComponent;

/*
DONE

* remove window from lib
** make root nodes size None, and have user manually set them
** either add extra value non scaled px
** or have UiRoot(w,h)
*** doesn't set the size of the node, but ...
*** could add scaling to it too
*** allows ui nodes to be stored under other non ui entities
*** makes things clearer,
*** can store a popup inside a ui node, but the popup would be outside the cur ui node
*** dont need to add extra value entry
*** can require computed on it
**** for children, can auto add computed?

* auto add computed to nodes that use ui component,
** if entity used as child, but has no ui component, could add temp computed, stored in temp hashmap?
*** what about later if only recomputing based on modification
*** or if missing ui component, then don't use as child, any descendents thast do have uicomputed, ignore

* handle render layers

* instead of positioning camera at 0,0,far-1, set near to -1000, and far to 0/1, then can position things closer to camera at neg/less


* handling scaling, for px, font size, scale (no but resize window size when used)?
** multiply val::px by scaling
** font handled by scaling input var
** what about images? yes?
** have scaling component

* for root, add offset for position?


TODO

* only use transparent pipeline for colours with alpha != 1, fonts, images with transparent flag = true, need to add opaque and alpha key pipelines

* fix colour bleeding on edges

* probably want to allow different ui's on dif monitors, where cursor can move across and click on them?

TODO
* replace all floats with ints and fractions
*/

