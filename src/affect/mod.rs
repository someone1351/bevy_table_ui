
pub mod components;
pub mod systems;
pub mod plugin;

/*
TODO
* allow press, hover state etc to have Option<Device>, so that dif devices can for example have their own press colour, or provide None, as have a default colour for unspeified devices
* could allow user to specify priority for the state eg Hover(priority)
- could allow same state of dif priorities to average their vals?
*/