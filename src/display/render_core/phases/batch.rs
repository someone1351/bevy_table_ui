

use bevy::render::render_phase::PhaseItemBatchSetKey;


/// 2D meshes aren't currently multi-drawn together, so this batch set key only
/// stores whether the mesh is indexed.
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct BatchSetKeyMy {
    /// True if the mesh is indexed.
    pub indexed: bool,
}

impl PhaseItemBatchSetKey for BatchSetKeyMy {
    fn indexed(&self) -> bool {
        self.indexed
    }
}

