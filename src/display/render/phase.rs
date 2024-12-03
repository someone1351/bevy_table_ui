
use bevy::math::FloatOrd;
use bevy::prelude::*;
use bevy::render::render_phase::*;
use bevy::render::render_resource::CachedRenderPipelineId;
use bevy::render::sync_world::MainEntity;
use core::ops::Range;

pub struct MyTransparentUi {
    pub sort_key: FloatOrd,
    pub entity: Entity,
    pub pipeline: CachedRenderPipelineId,
    pub draw_function: DrawFunctionId,
    pub batch_range: Range<u32>, /// Range in the vertex buffer of this item
    pub extra_index: PhaseItemExtraIndex,
}

impl PhaseItem for MyTransparentUi {
    #[inline]
    fn entity(&self) -> Entity {
        self.entity
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
        self.extra_index
    }

    #[inline]
    fn batch_range_and_extra_index_mut(&mut self) -> (&mut Range<u32>, &mut PhaseItemExtraIndex) {
        (&mut self.batch_range, &mut self.extra_index)
    }
    
    fn main_entity(&self) -> MainEntity {
        MainEntity::from(Entity::PLACEHOLDER) 
    }
}

impl CachedRenderPipelinePhaseItem for MyTransparentUi {
    #[inline]
    fn cached_pipeline(&self) -> CachedRenderPipelineId {
        self.pipeline
    }
}

impl SortedPhaseItem for MyTransparentUi {
    type SortKey = FloatOrd;

    #[inline]
    fn sort_key(&self) -> Self::SortKey {
        self.sort_key
    }

    #[inline]
    fn sort(items: &mut [Self]) {
        items.sort_by_key(|item| item.sort_key());
    }
}