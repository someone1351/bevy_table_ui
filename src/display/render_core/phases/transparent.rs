
use core::ops::Range;


use bevy::ecs::prelude::*;
use bevy::math::FloatOrd;
use bevy::render::{
    render_phase::{
        CachedRenderPipelinePhaseItem, DrawFunctionId,
        PhaseItem, PhaseItemExtraIndex, SortedPhaseItem,
    },
    render_resource::CachedRenderPipelineId,
    sync_world::MainEntity,
};


/// Transparent 2D [`SortedPhaseItem`]s.
pub struct TransparentMy {
    pub sort_key: FloatOrd,
    pub entity: (Entity, MainEntity),
    // pub entity: Entity,
    pub pipeline: CachedRenderPipelineId,
    pub draw_function: DrawFunctionId,
    pub batch_range: Range<u32>,
    // pub extracted_index: usize,
    pub extra_index: PhaseItemExtraIndex,
    // Whether the mesh in question is indexed (uses an index buffer in
    // addition to its vertex buffer).
    // pub indexed: bool,
}

impl PhaseItem for TransparentMy {
    #[inline]
    fn entity(&self) -> Entity {
        self.entity.0
        // self.entity
    }

    #[inline]
    fn main_entity(&self) -> MainEntity {
        self.entity.1
        // MainEntity::from(Entity::PLACEHOLDER)
    }

    #[inline]
    fn draw_function(&self) -> DrawFunctionId {
        self.draw_function
    }

    #[inline]
    fn batch_range(&self) -> &Range<u32> {
        &self.batch_range
    }

    #[inline]
    fn batch_range_mut(&mut self) -> &mut Range<u32> {
        &mut self.batch_range
    }

    #[inline]
    fn extra_index(&self) -> PhaseItemExtraIndex {
        self.extra_index.clone()
    }

    #[inline]
    fn batch_range_and_extra_index_mut(&mut self) -> (&mut Range<u32>, &mut PhaseItemExtraIndex) {
        (&mut self.batch_range, &mut self.extra_index)
    }
}

impl SortedPhaseItem for TransparentMy {
    type SortKey = FloatOrd;

    #[inline]
    fn sort_key(&self) -> Self::SortKey {
        self.sort_key
    }

    #[inline]
    fn sort(items: &mut [Self]) {
        // radsort is a stable radix sort that performed better than `slice::sort_by_key` or `slice::sort_unstable_by_key`.
        radsort::sort_by_key(items, |item| item.sort_key().0);
    }

    fn indexed(&self) -> bool {
        // self.indexed
        false
    }
}

impl CachedRenderPipelinePhaseItem for TransparentMy {
    #[inline]
    fn cached_pipeline(&self) -> CachedRenderPipelineId {
        self.pipeline
    }
}

