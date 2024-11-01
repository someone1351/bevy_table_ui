

#![allow(unused_parens)]


use bevy::ecs::prelude::*;
use bevy::render::render_asset::RenderAssets;
use bevy::render::render_phase::*;
use bevy::render::mesh::GpuBufferInfo;
use bevy::render::view::ViewUniformOffset;
use bevy::ecs::system::{lifetimeless::*, SystemParamItem};

use super::components::*;
use super::resources::*;

pub struct SetViewBindGroup<const I: usize>;

impl<P: PhaseItem, const I: usize> RenderCommand<P> for SetViewBindGroup<I> {
    type Param = ();//SQuery<(Read<ViewUniformOffset>, Read<MyViewBindGroup>)>;
    type ViewQuery = (Read<ViewUniformOffset>, Read<MyViewBindGroup>);//Read<ViewUniformOffset>;
    type ItemQuery = ();
    #[inline]
    fn render<'w>(
        _item: &P,
        (view_uniform,mesh2d_view_bind_group): (&'w ViewUniformOffset,&'w MyViewBindGroup), //view
        _entity: Option<()>, //item
        view_query: SystemParamItem<'w, '_, Self::Param>, //param
        pass: &mut TrackedRenderPass<'w>,
        
    ) -> RenderCommandResult {
        // let (view_uniform, mesh2d_view_bind_group) = view_query.get_inner(view).unwrap();
        pass.set_bind_group(
            I, 
            &mesh2d_view_bind_group.value,
            &[view_uniform.offset]
        );
        RenderCommandResult::Success
    }
}


pub struct SetColorTextureBindGroup<const I: usize>;

#[allow(unused_parens)]
impl<P: PhaseItem, const I: usize> RenderCommand<P> for SetColorTextureBindGroup<I> {
    type Param = SRes<MyUiImageBindGroups>;//(, SQuery<>);
    type ViewQuery = ();
    type ItemQuery = Read<MyUiBatch>;

    fn render<'w>(
        _item: &P,
        _view: (), //view
        batch : Option<&'w MyUiBatch>, //item
        image_bind_groups : SystemParamItem<'w, '_, Self::Param>, //param
        // (image_bind_groups,query_batch): SystemParamItem<'w, '_, Self::Param>, //s, query_batch
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        let Some(batch) = batch else {
            return RenderCommandResult::Failure;
        };

        // let sprite_batch = query_batch.get(item).unwrap();
        // let batch = query_batch.get(_item).unwrap();
        //let image_bind_group = image_bind_group.into_inner();
        
        let image_bind_groups = image_bind_groups.into_inner();
        //let bind_group = image_bind_groups.values.get(&batch.image_handle.id()).unwrap();
        let Some(bind_group) = image_bind_groups.values.get(&batch.image_handle.as_ref().map(|x|x.id())) else {
            return RenderCommandResult::Success;
        };
        
        // let bind_group = image_bind_groups.values.get(&batch.image_handle).unwrap();
        // if let Some(bind_group) = image_bind_group.0 {

        // }
        // let bind_group = &image_bind_group.value;
        //;
        pass.set_bind_group( I, bind_group, &[], );
        RenderCommandResult::Success
    }
}


pub struct SetDrawBuf;

impl<P: PhaseItem> RenderCommand<P> for SetDrawBuf {
    type Param = SRes<MyUiMeta> ;//(, SQuery<>);
    type ViewQuery = ();
    type ItemQuery = Read<MyUiBatch>;

    #[inline]
    fn render<'w>(
        item: &P,
        _view : (), //view
        batch : Option<&'w MyUiBatch>, //item
        ui_meta : SystemParamItem<'w, '_, Self::Param>, //param
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        let Some(batch) = batch else {
            return RenderCommandResult::Failure;
        };
        
        // let batch = query_batch.get(item).unwrap();
        // println!("{:?}",(
        //     item.entity(),
        //     &batch.range,
        //     &batch.image_handle,
        // ));
        //println!("ee {:?}",item);

        pass.set_vertex_buffer(0, ui_meta.into_inner().vertices.buffer().unwrap().slice(..));

        // let Some(verts)=ui_meta.into_inner().vertices.buffer() else {
        //     println!("fail");
        //     return RenderCommandResult::Failure;
        // };

        // pass.set_vertex_buffer(0, verts.slice(..));
        pass.draw(batch.range.clone(), 0..1);
        
        RenderCommandResult::Success     
    }
}

// This specifies how to render a colored 2d mesh
pub type DrawMesh = (
    bevy::render::render_phase::SetItemPipeline, // Set the pipeline
    SetViewBindGroup<0>, // Set the view uniform as bind group 0
    SetColorTextureBindGroup<1>,
    // SetDrawMesh, // Draw the mesh
    SetDrawBuf,
);