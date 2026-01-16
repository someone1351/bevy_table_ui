
#![allow(dead_code)]
use core::ops::Range;

use bevy::asset::UntypedAssetId;

use bevy::ecs::prelude::*;
use bevy::render::{
    render_phase::{
        BinnedPhaseItem, CachedRenderPipelinePhaseItem, DrawFunctionId,
        PhaseItem, PhaseItemExtraIndex,
    },
    render_resource::{
        BindGroupId, CachedRenderPipelineId,
    },
    sync_world::MainEntity,
};

use super::BatchSetKeyMy;



/// Opaque 2D [`BinnedPhaseItem`]s.
pub struct OpaqueMy {
    /// Determines which objects can be placed into a *batch set*.
    ///
    /// Objects in a single batch set can potentially be multi-drawn together,
    /// if it's enabled and the current platform supports it.
    pub batch_set_key: BatchSetKeyMy,
    /// The key, which determines which can be batched.
    pub bin_key: OpaqueMyBinKey,
    /// An entity from which data will be fetched, including the mesh if
    /// applicable.
    pub representative_entity: (Entity, MainEntity),
    /// The ranges of instances.
    pub batch_range: Range<u32>,
    /// An extra index, which is either a dynamic offset or an index in the
    /// indirect parameters list.
    pub extra_index: PhaseItemExtraIndex,
}

/// Data that must be identical in order to batch phase items together.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OpaqueMyBinKey {
    /// The identifier of the render pipeline.
    pub pipeline: CachedRenderPipelineId,
    /// The function used to draw.
    pub draw_function: DrawFunctionId,
    /// The asset that this phase item is associated with.
    ///
    /// Normally, this is the ID of the mesh, but for non-mesh items it might be
    /// the ID of another type of asset.
    pub asset_id: UntypedAssetId,
    /// The ID of a bind group specific to the material.
    pub material_bind_group_id: Option<BindGroupId>,
}

impl PhaseItem for OpaqueMy {
    #[inline]
    fn entity(&self) -> Entity {
        self.representative_entity.0
    }

    fn main_entity(&self) -> MainEntity {
        self.representative_entity.1
    }

    #[inline]
    fn draw_function(&self) -> DrawFunctionId {
        self.bin_key.draw_function
    }

    #[inline]
    fn batch_range(&self) -> &Range<u32> {
        &self.batch_range
    }

    #[inline]
    fn batch_range_mut(&mut self) -> &mut Range<u32> {
        &mut self.batch_range
    }

    fn extra_index(&self) -> PhaseItemExtraIndex {
        self.extra_index.clone()
    }

    fn batch_range_and_extra_index_mut(&mut self) -> (&mut Range<u32>, &mut PhaseItemExtraIndex) {
        (&mut self.batch_range, &mut self.extra_index)
    }
}

impl BinnedPhaseItem for OpaqueMy {
    // Since 2D meshes presently can't be multidrawn, the batch set key is
    // irrelevant.
    type BatchSetKey = BatchSetKeyMy;

    type BinKey = OpaqueMyBinKey;

    fn new(
        batch_set_key: Self::BatchSetKey,
        bin_key: Self::BinKey,
        representative_entity: (Entity, MainEntity),
        batch_range: Range<u32>,
        extra_index: PhaseItemExtraIndex,
    ) -> Self {
        OpaqueMy {
            batch_set_key,
            bin_key,
            representative_entity,
            batch_range,
            extra_index,
        }
    }
}

impl CachedRenderPipelinePhaseItem for OpaqueMy {
    #[inline]
    fn cached_pipeline(&self) -> CachedRenderPipelineId {
        self.bin_key.pipeline
    }
}
