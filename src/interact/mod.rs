pub mod components;
pub mod systems;
pub mod resources;
pub mod plugin;
pub mod messages;
// use bevy::app::PostUpdate;
/*
TODO
* move UiLock to UiInteractLock
* add UiInteractComputed component
* * have unlocked
* * have pressed,released ?
* * have selected ?


TODO

* add buttons(i32) for drag, focusing, pressing

* have dist and delta for drag

TODO
* handle when event begins and ends in same frame?
** add bool to end message?
*** can implement using a bool that flips every frame, check if matches begin message?
*/

