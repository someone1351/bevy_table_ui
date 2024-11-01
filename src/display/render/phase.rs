
use bevy::math::FloatOrd;
use bevy::prelude::*;

// use bevy::core::FloatOrd;

use bevy::render::render_phase::*;
use bevy::render::render_resource::CachedRenderPipelineId;
// use bevy::utils::FloatOrd;
// use bevy::utils::nonmax::NonMaxU32;

use core::ops::Range;

pub struct MyTransparentUi {
    // pub sort_key: (FloatOrd,u32),
    pub sort_key: FloatOrd,
    pub entity: Entity,
    pub pipeline: CachedRenderPipelineId,
    pub draw_function: DrawFunctionId,
    /// Range in the vertex buffer of this item
    pub batch_range: Range<u32>, //Option<>,
    // pub dynamic_offset: Option<NonMaxU32>,
    pub extra_index: PhaseItemExtraIndex,
}

impl PhaseItem for MyTransparentUi {
    // // type SortKey = (FloatOrd,u32);
    // type SortKey = FloatOrd;
    // const AUTOMATIC_BATCHING: bool = false;

    #[inline]
    fn entity(&self) -> Entity {
        self.entity
    }

    // #[inline]
    // fn sort_key(&self) -> Self::SortKey {
    //     self.sort_key
    // }

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

    // #[inline]
    // fn dynamic_offset(&self) -> Option<NonMaxU32> {
    //     self.dynamic_offset
    // }

    // #[inline]
    // fn dynamic_offset_mut(&mut self) -> &mut Option<NonMaxU32> {
    //     &mut self.dynamic_offset
    // }

    #[inline]
    fn extra_index(&self) -> PhaseItemExtraIndex {
        self.extra_index
    }

    #[inline]
    fn batch_range_and_extra_index_mut(&mut self) -> (&mut Range<u32>, &mut PhaseItemExtraIndex) {
        (&mut self.batch_range, &mut self.extra_index)
    }
}

// impl EntityPhaseItem for MyTransparentUi {
//     #[inline]
//     fn entity(&self) -> Entity {
//         self.entity
//     }
// }

impl CachedRenderPipelinePhaseItem for MyTransparentUi {
    #[inline]
    fn cached_pipeline(&self) -> CachedRenderPipelineId {
        self.pipeline
    }
}

// impl BatchedPhaseItem for MyTransparentUi {
//     fn batch_range(&self) -> &Option<Range<u32>> {
//         &self.batch_range
//     }

//     fn batch_range_mut(&mut self) -> &mut Option<Range<u32>> {
//         &mut self.batch_range
//     }
// }

impl SortedPhaseItem for MyTransparentUi {
    // type SortKey = (FloatOrd, u32);
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