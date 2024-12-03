/*
TODOS 1
* make gaps/borders/padding/margin each able to scale based on parent size
- or scale to an individual font char size
* make gaps/edges/size each able to use aspect(none,horizontal,vertical)

TODOS 2
* mark row/col not to use nodes past those, for internal size, 
* have max internal size, eg via px, scale (positive, ie from parent)

* need min size, so if window is shrunk too far, it won't shrink past min size

* have option for gaps to be on all sides, not just in between row/cols

* need option to keep aspect ratio
- repurpose neg values for aspect, eg if width is neg scale, then use that to keep it at that percent of height
- - then need enum to specify if w/h is calc'd from ancestor or descendant/internal?
- need option to keep aspect ratio of images, or to stretch to fit w/h of node
- - need to way for image to size it's node?
- need to specify aspect to keep horizontal or vertical ??? don't need to? just use neg?
- how does adjacent size work with aspect ratio?

* could get size by getting w/h size from individual font's char size

* remove margins/border/padding, replace with vec of edges
- add flag that allows edge to be set to back color if its own color is none
- - use "inside_start", provides index of edge where it is inside

* allow gaps/edges to be calc'd by parent size instead of just current node's size

TODO 3
* simplify
- base sizes only on px, parent percent, percent over inside?, px over inside?

TODO 4
* rename Float to Floating
* rename Hide to Hidden
* rename Disable to Disabled
* in addition to Disable (not used or shown or inputable) and Hidden (not shown or inputable?), add Locked (not inputable ie interaction)
* make any floating node's order after non floating (is it already this or not?)

TODO 5
* allow floating nodes to not be clamped by their parent?, instead be clamped by their parent's ancestor(s)?

TODO 6
* make size etc work for root
* * loop through entities parents instead of entities, so Option<Entity>, where root parent is None
* * 

TODO 7
* add UiPriority component for root/floating nodes, higher priority are ordered ahead/ontop of their siblings

TODO 8
* have ui h/v align allow neg values which then align from right/bottom


*/


/*
text need to clip where it goes out of bounds
*/



// use std::collections::HashSet;
// use std::f32::consts::E;

use bevy::math::Vec2;
// use bevy::render::view::window;
use bevy::utils::default;
use bevy::{
    // asset::prelude::*, 
    ecs::prelude::*, 
    hierarchy::prelude::*,
    window::prelude::*,
};


use super::values::*;
use super::components::*;
use super::utils::*;

pub fn ui_init_computeds(
    windows: Query<&Window>,
    mut computed_query: Query<&mut UiLayoutComputed>,

    root_query: Query<Entity,(Without<Parent>,With<UiLayoutComputed>)>,
    children_query: Query<&Children,With<UiLayoutComputed>>,
    parent_query: Query<&Parent,With<UiLayoutComputed>>,

    size_query: Query<&UiSize,>, //(With<Parent>,)

    hide_query: Query<&UiHide>,
    disable_query: Query<&UiDisable>,
    lock_query: Query<&UiLock>,

    edge_query: Query<&UiEdge,>, //,(With<Parent>)
    // aspect_query: Query<&UiAspect,>, //(With<Parent>,)
 
 
    // change_query: Query<&UiChange>,

    // edge_query: Query<&UiEdge,(With<Parent>,)>,
    // mut c:Local<usize>,
) {
    
    // println!("ui_init_computeds {}",*c);
    // *c+=1;

    let window_size=windows.get_single()
        .and_then(|window|Ok((window.width(),window.height())))
        .unwrap_or((0.0,0.0));

    //
    for mut computed in computed_query.iter_mut() {
        *computed=Default::default();
    }
    
    //
    let mut stk = root_query.iter().collect::<Vec<_>>();
    let mut order=0;

    while let Some(entity) = stk.pop() {
        let parent=parent_query.get(entity);
        let parent_computed = parent.and_then(|p|computed_query.get(p.get()))
            .cloned()
            .unwrap_or(UiLayoutComputed { 
                visible: true, 
                unlocked : true,
                size:Vec2::new(window_size.0, window_size.1),
                ..Default::default() 
            });
            // .unwrap_or_default();

        // let is_root=parent.is_err();


        //necessary, as  newly added ui child entities seem to lack their ui components, 
        // but also necessary for child entities that lack any ui component
        let Ok(mut computed) = computed_query.get_mut(entity) else {
            continue;
        };

        //
        // computed.init();
        // *computed=Default::default();

        //
        let hide = hide_query.get(entity).cloned().unwrap_or_default().hide;
        let disable = disable_query.get(entity).cloned().unwrap_or_default().disable;
        let lock = lock_query.get(entity).cloned().unwrap_or_default().lock;

        //
        if disable {
            continue;
        }

        //
        stk.extend(children_query.get(entity).map(|c|c.iter().rev()).unwrap_or_default());

        //

        computed.enabled=true;

        // if hide {
        //     continue;
        // }

        // computed.visible=true;

        // if !hide {
        //     // computed.visible=true;
        //     computed.visible=parent_computed.visible;
        // }

        computed.visible=!hide && parent_computed.visible;
        computed.unlocked=!lock && computed.visible && parent_computed.unlocked;

        // computed.visible=true;
        // println!("hide {hide:?}");

        // computed.visible=!hide;
        //calc depth
        // computed.depth=cur_depth;
        // cur_depth+=1;

        computed.order=order;
        order+=1;

        computed.depth=0;

        {
            let mut p=parent.ok().map(|p|p.get());

            while let Some(pp)=p {
                computed.depth+=1;
                p=parent_query.get(pp).ok().map(|p|p.get());
            }
        }

        //
        // if is_root {
        //     //init root
        //     computed.pos.x=0.0;
        //     computed.pos.y=0.0;
        //     computed.size.x=window_size.0;
        //     computed.size.y=window_size.1;
        //     // println!("{entity:?} {computed:?}");
        // } else 
        {
            // if is_root {
            //     //init root
            //     // computed.pos.x=0.0;
            //     // computed.pos.y=0.0;
            //     // computed.size.x=window_size.0;
            //     // computed.size.y=window_size.1;
            //     // println!("{entity:?} {computed:?}");
            // } 

            let size = size_query.get(entity).cloned().unwrap_or_default();
            let edge = edge_query.get(entity).cloned().unwrap_or_default();
            // let aspect = aspect_query.get(entity).cloned().unwrap_or_default();

            // // if is_root 
            // {
            //     println!("wh1 {:?} {size:?} {entity} {is_root} {:?}",computed.size,size_query.get(entity));
            // }
            //calc computed wh's for pos px sizes (entity order not important, ie using df_entities for convience)
            if let UiVal::Px(p) = size.width {
                if p >= 0.0 {
                    computed.size.x = p;
                }
            }
        
            if let UiVal::Px(p) = size.height {
                if p>=0.0 {
                    computed.size.y = p;
                }
            }

            //calc ancestor sizes and store as negative in computed w,h for none/percent vals (ie just computed wh that are unset aka <0)
            if computed.size.x < 0.0 { //is unset
                computed.size.x = -parent_computed.size.x.abs();
            }
            
            if computed.size.y < 0.0 { //is unset
                computed.size.y = -parent_computed.size.y.abs();          
            }

            
            //calc wh for positive percent size
            if let UiVal::Scale(s) = size.width {
                if s>=0.0 {
                    //make room for edges?
                    // computed.size.x = s*computed.size.x.abs();

                    //
                    let w = (s*computed.size.x.abs()-edge.h_px()).max(0.0);
                    computed.size.x = w/(edge.h_scale()+1.0);

                    //todo nedge
                    //computed.size.y*edge.h_transverse_scale();

                }
            }

            if let UiVal::Scale(s) = size.height {
                if s>=0.0 {
                    //make room for edges?
                    // computed.size.y = s*computed.size.y.abs();

                    //
                    let h = (s*computed.size.y.abs()-edge.v_px()).max(0.0);
                    computed.size.y = h/(edge.v_scale()+1.0);

                    //todo nedge
                    //computed.size.x*edge.v_transverse_scale();
                }
            }

            // if is_root {
            //     println!("wh2 {:?} {size:?}",computed.size);
            // }
        }
    }

}







pub fn ui_calc_computeds(
    windows: Query<&Window>,

    mut computed_query: Query<&mut UiLayoutComputed>,
    root_query: Query<Entity,(Without<Parent>,With<UiLayoutComputed>)>,
    children_query: Query<&Children,With<UiLayoutComputed>>,
    parent_query: Query<&Parent,With<UiLayoutComputed>>,

    //
    // span_query: Query<&UiSpan>,
    // gap_query: Query<&UiGap>,

    (span_query,gap_query): (Query<&UiSpan>,Query<&UiGap>),

    //only work with non root entities
    size_query: Query<&UiSize,>, //(With<Parent>,)
    inner_size_query: Query<&UiInnerSize,>, //(With<Parent>,)
    align_query: Query<&UiAlign,>, //(With<Parent>,)
    float_query: Query<&UiFloat,>, //(With<Parent>,)
    fill_query: Query<&UiFill,>, //(With<Parent>,)
    expand_query: Query<&UiExpand,>, //(With<Parent>,)
    scroll_query: Query<&UiScroll,>, //(With<Parent>,)
    edge_query: Query<&UiEdge,>, //(With<Parent>,)
    congruent_query: Query<&UiCongruent,>, //(With<Parent>,)    
    // mut c:Local<usize>,
) {
    
    // println!("ui_calc_computeds {}",*c);
    // *c+=1;
    
    // println!("2");
    // let now = std::time::Instant::now();
    // println!("ui_calc_computeds Elapsed: {}", now.elapsed().as_secs_f64());

    // let window_size = if let Some(window) = windows.get_primary() {(window.width(),window.height())} else {(0.0,0.0)};
    let window_size=windows.get_single().and_then(|window|Ok((window.width(),window.height()))).unwrap_or((0.0,0.0));

    // let inv_scale_factor = 1. / scale_factor;

    // println!("w {:?}",window_size);
    //init df
    // let mut df_entities = Vec::<Entity>::new();
    
    // {
    //     let mut stk = root_query.iter().collect::<Vec<_>>();
    
    //     while let Some(entity) = stk.pop() {
    //         let Ok(computed) = computed_query.get(entity) else {continue;};
            
    //         // if !computed.visible {continue;}
    //         if !computed.enabled {continue;}

    //         df_entities.push(entity);

    //         stk.extend(children_query.get(entity)
    //             .and_then(|c: &Children|Ok(c.iter().rev()))
    //             .unwrap_or_default()
    //         );
    //     }
    // }

    //calc row/cols

    //
    let mut stk = root_query.iter().map(|x|(x,false)).collect::<Vec<_>>();
    stk.reverse(); //not needed?

    while let Some((entity,b)) = stk.pop()
    // for &entity in df_entities.iter().rev() 
 
    {

        //so that entities are visited in reverse
        if !b {
        
            let Ok(computed) = computed_query.get(entity) else {
                continue;
            };

            if !computed.enabled {
                continue;
            }

            stk.push((entity,true));
            stk.extend(children_query.get(entity).map(|c|c.iter()).unwrap_or_default().map(|&x|(x,false)));
            continue;
        }

        //
        let span = span_query.get(entity).cloned().unwrap_or_default().span;// as usize;

        if let Ok(children)=children_query.get(entity) {
            //get enabled, not float child count
            let mut child_count=0;
            
            for &child_entity in children.iter() {
                let child_float = float_query.get(child_entity).cloned().unwrap_or_default().float;
                let mut child_computed = computed_query.get_mut(child_entity).unwrap();

                if span == 0 || child_count<span {
                    child_computed.col = child_count;
                } else {
                    child_computed.col = child_count%span;
                    child_computed.row = child_count/span;
                }

                if child_computed.enabled && !child_float {
                    child_count+=1;
                }
            }

            let cells_num = child_count;
            let cols_num = if span == 0 {cells_num} else {span.min(cells_num)};
            let rows_num = if cells_num==0 {0} else { (cells_num+cols_num-1)/cols_num };
        
            let mut computed = computed_query.get_mut(entity).unwrap();
            computed.rows=rows_num;
            computed.cols = cols_num;
        }
    }





    //
    let root_entities=root_query.iter().collect::<Vec<_>>();
    let top_computed=UiLayoutComputed{
            unlocked:true,
            visible:true,
            enabled:true,        
            size:Vec2::new(window_size.0, window_size.1),
        ..default()
    };

    // let top_size=UiSize {width:UiVal::Px(window_size.0),height:UiVal::Px(window_size.1)};


    //calc wh for none/negative sizes of non roots (todo: need to start with left leaves (does it?))
    //for non roots calc initial w,h (also edges,gaps, but not stored)
    //calc child w,h for congruent (aka perpendicular) sizes
    
    //
    //root_query.iter().collect::<Vec<_>>()

    // let mut stk = root_query.iter().map(|x|(x,false)).collect::<Vec<_>>();
    // stk.reverse(); //not needed?

    //
    let mut stk: Vec<(Option<Entity>, bool)> = vec![(None,false)]; 

    //    
    while let Some((entity,b)) = stk.pop()
    // for &entity in df_entities.iter().rev() 
    
    {

        if !b {
            let children=entity
                .map(|entity|children_query.get(entity).map(|children|children.iter().rev()).ok())
                .unwrap_or(Some(root_entities.iter().rev()));
            
            if entity.map(|entity|computed_query.get(entity).map(|computed|!computed.enabled).unwrap_or(true)).unwrap_or(false) 
            {
                continue;
            }

            stk.push((entity,true));
            stk.extend(children.clone().unwrap_or_default().map(|child_entity|(Some(*child_entity),false)));

            // println!("xx {entity:?}");
            continue;
        }

        //
        let children=entity
            .map(|entity|children_query.get(entity).map(|children|children.iter()).ok())
            .unwrap_or(Some(root_entities.iter()));

        //
        // let span = span_query.get(entity).cloned().unwrap_or_default().span as usize;
        // let gap = gap_query.get(entity).cloned().unwrap_or_default();

        //
        // let span=entity.and_then(|entity|span_query.get(entity).ok().cloned()).unwrap_or_default().span as usize;
        let gap=entity.and_then(|entity|gap_query.get(entity).ok().cloned()).unwrap_or_default();

        //
        let mut max_space_w : f32 = 0.0;
        let mut max_space_h : f32 = 0.0;

        if let Some(children)=&children //children_query.get(entity) 
        // if children.is_some()
        {
            //get non hidden/float child count
            // let child_count =  children.iter().fold(0 as usize, |acc,&c|{
            //     let child_computed = computed_query.get(c).cloned().unwrap_or_default();
            //     let child_float = float_query.get(c).cloned().unwrap_or_default().float;
            //     if child_computed.enabled && !child_float {acc+1} else {acc}
            // });

            // if child_count > 0 
   
            // let cells_num = child_count;
            // let cols_num = if span == 0 {cells_num} else {span.min(cells_num)};
            // let rows_num = if cells_num==0 {0} else { (cells_num+cols_num-1)/cols_num };

            
            // let computed = *computed_query.get(entity).unwrap();
            let computed = entity.map(|entity|*computed_query.get(entity).unwrap()).unwrap_or(top_computed);

            let cols_num = computed.cols as usize;
            let rows_num = computed.rows as usize;
            // let cells_num = cols_num*rows_num;

            let child_count=children.clone().filter(|&&child_entity|computed_query.get(child_entity).is_ok()).count();
            // println!("ccount {entity:?} {child_count}");

            {
                struct NodeScale {entity:Entity,col_vscale:f32,row_hscale:f32,w:f32,h:f32}
                let mut child_node_scales = Vec::<NodeScale>::new();
                
                // let mut row_hscales = Vec::<(Entity,f32,f32)>::new();
                // let mut col_vscales = Vec::<(Entity,f32,f32)>::new();

                for &child_entity in children.clone() { //children here!
                    // println!("child_entity {child_entity:?}");
                
                    //println!("Dfdsf8 {entity:?}");
                    let child_float = entity.is_none() || float_query.get(child_entity).cloned().unwrap_or_default().float;
                    // let child_size = size_query.get(child_entity).cloned().unwrap_or_default();
                    let child_computed=computed_query.get(child_entity).unwrap();

                    let child_congruent=congruent_query.get(child_entity).cloned().unwrap_or_default();

                    if !child_computed.enabled || child_float { //|| is_root_children
                        continue;
                    }

                    child_node_scales.push(NodeScale{
                        entity:child_entity,
                        row_hscale:child_congruent.row_width_scale,
                        col_vscale:child_congruent.col_height_scale,
                        w:child_computed.size.x,
                        h:child_computed.size.y,
                    });
                }

                //
                for col in 0..cols_num {
                    let col_vscale_sum=(0..rows_num)
                        // .map(|row|child_node_scales[col+row*cols_num].col_vscale)
                        .filter_map(|row|child_node_scales.get(col+row*cols_num))
                        .map(|x|x.col_vscale)
                        .sum::<f32>();
                    

                    let mut col_max_height = 0.0;

                    if col_vscale_sum == 0.0 {
                        continue;
                    }

                    for row in 0..rows_num {
                        let ind= col+row*cols_num;
                        let Some(node_scale)=child_node_scales.get_mut(ind) else {//&mut child_node_scales[ind];
                            continue;
                        };

                        if node_scale.col_vscale == 0.0 {
                            continue;
                        }

                        node_scale.col_vscale/=col_vscale_sum; //norm
                        col_max_height=(node_scale.h/node_scale.col_vscale).max(col_max_height); //calc max height
                    }
                    
                    for row in 0..rows_num {
                        let ind= col+row*cols_num;
                        // let node_scale=&mut child_node_scales[ind];
                        let Some(node_scale)=child_node_scales.get_mut(ind) else {
                            continue;
                        };

                        if node_scale.col_vscale == 0.0 {
                            continue;
                        }
                        
                        let mut child_computed=computed_query.get_mut(node_scale.entity).unwrap();
                        child_computed.size.y = node_scale.col_vscale*col_max_height;
                    }
                }

                //
                for row in 0..rows_num {
                    let row_hscale_sum=(0..cols_num)
                        // .map(|col|child_node_scales[row+col*rows_num].row_hscale)
                        .filter_map(|col|child_node_scales.get(row+col*rows_num))
                        .map(|x|x.row_hscale)
                        .sum::<f32>();
                    let mut row_max_width = 0.0;

                    if row_hscale_sum == 0.0 {
                        continue;
                    }

                    for col in 0..cols_num {
                        let ind= row+col*rows_num;
                        // let node_scale=&mut child_node_scales[ind];

                        let Some(node_scale)=child_node_scales.get_mut(ind) else {
                            continue;
                        };

                        if node_scale.row_hscale == 0.0 {
                            continue;
                        }

                        node_scale.row_hscale/=row_hscale_sum; //norm
                        row_max_width=(node_scale.w/node_scale.row_hscale).max(row_max_width); //calc max width
                    }
                    
                    for col in 0..cols_num {
                        let ind= row+col*rows_num;
                        // let node_scale=&mut child_node_scales[ind];

                        let Some(node_scale)=child_node_scales.get_mut(ind) else {
                            continue;
                        };

                        if node_scale.row_hscale == 0.0 {
                            continue;
                        }
                        
                        let mut child_computed=computed_query.get_mut(node_scale.entity).unwrap();
                        child_computed.size.x = node_scale.row_hscale*row_max_width;
                    }
                }
            }

            //get col_widths, row_heights
            let mut col_widths = vec![0.0 as f32;cols_num];
            let mut row_heights = vec![0.0 as f32;rows_num];

            { 
                // let mut child_ind = 0;

                for &child_entity in children.clone() //children.iter() 
                { //children here!
                    //println!("Dfdsf7 {entity:?}");
                    //child entities without ui components are treated as visible (ie have a row/col)
                    let child_edge = edge_query.get(child_entity).cloned().unwrap_or_default();
                    let child_float = entity.is_none() || float_query.get(child_entity).cloned().unwrap_or_default().float;
                    
                    let child_computed=computed_query.get(child_entity).unwrap();

                    if !child_computed.enabled {
                        continue;
                    }

                    //edge
                    // let child_hedge_scale= child_edge.l_scale.max(0.0)+child_edge.r_scale.max(0.0);
                    // let child_vedge_scale= child_edge.t_scale.max(0.0)+child_edge.b_scale.max(0.0);
                    // let child_hedge_px= child_edge.l_px.max(0.0)+child_edge.r_px.max(0.0);
                    // let child_vedge_px= child_edge.t_px.max(0.0)+child_edge.b_px.max(0.0);

                    // let child_hnedge_scale= child_edge.l_neg_scale.max(0.0)+child_edge.r_neg_scale.max(0.0);
                    // let child_vnedge_scale= child_edge.t_neg_scale.max(0.0)+child_edge.b_neg_scale.max(0.0);


                    let child_w = child_computed.size.x+
                        child_edge.h_px() + // child_hedge_px +
                        child_computed.size.x*child_edge.h_scale()+ //child_hedge_scale +
                        child_computed.size.y*child_edge.h_transverse_scale(); //child_hnedge_scale;

                    let child_h = child_computed.size.y+
                        child_edge.v_px() + //child_vedge_px +
                        child_computed.size.y* child_edge.v_scale()+ //child_vedge_scale +
                        child_computed.size.x*child_edge.v_transverse_scale(); //child_vnedge_scale;
                                                
                    //
                    if child_float { // || is_root_children
                        max_space_w = max_space_w.max(child_w);
                        max_space_h = max_space_h.max(child_h);
                    } else {
                        // let col = child_ind % cols_num;
                        // let row = child_ind / cols_num;
                        
                        let col=child_computed.col as usize;
                        let row=child_computed.row as usize;

                        col_widths[col]=col_widths[col].max(child_w);
                        row_heights[row]=row_heights[row].max(child_h);

                        // child_ind+=1;
                    }
                }
            }

            //
            let total_col_width = col_widths.iter().sum::<f32>();
            let total_row_height = row_heights.iter().sum::<f32>();

            //
            if child_count>0 { // !is_root_children || 
                //gaps

                //should probably recalc avg col/row width/height for gap percent size after
                // expands/fills in section further down like how margin/border/padding is 


                let hgap_space =  match gap.hgap {
                    // Val::Scale(p) => p.max(0.0)*(total_col_width/(cols_num as f32)),
                    UiVal::Scale(p) if p>=0.0 => p*(total_col_width/(cols_num as f32)),
                    UiVal::Scale(p) if p<0.0 => p.abs()*(total_row_height/(rows_num as f32)),
                    UiVal::Px(p) => p.max(0.0),
                    _ => 0.0
                };

                let vgap_space =  match gap.vgap {
                    // Val::Scale(p) => p.max(0.0)*(total_row_height/(rows_num as f32)),
                    UiVal::Scale(p) if p>=0.0 => p*(total_row_height/(rows_num as f32)),
                    UiVal::Scale(p) if p<0.0 => p.abs()*(total_col_width/(cols_num as f32)),
                    UiVal::Px(p) => p.max(0.0),
                    _ => 0.0
                };

                if cols_num!=0 && rows_num!=0 {
                    let hgap_spaces = hgap_space*((cols_num-1) as f32);
                    let vgap_spaces = vgap_space*((rows_num-1) as f32);
    
                    max_space_w=max_space_w.max(total_col_width + hgap_spaces);
                    max_space_h=max_space_h.max(total_row_height + vgap_spaces);
                }
            } else {
                max_space_w=max_space_w.max(total_col_width);
                max_space_h=max_space_h.max(total_row_height);
            }

            
            //
            //println!("e3b {entity:?} {:?}",computed.size);
        }

        // !is_root_children

        //
        // if let Ok(inner_size) = inner_size_query.get(entity)         
        if let Some(inner_size) = entity.and_then(|entity|inner_size_query.get(entity).ok()) 
        {
            max_space_w = max_space_w.max(inner_size.width);
            max_space_h = max_space_h.max(inner_size.height);
        }

        // if parent_query.get(entity).is_ok() //has parent
        if let Some(entity)=entity
        {
            let size = size_query.get(entity).cloned().unwrap_or_default();
            let mut computed = computed_query.get_mut(entity).unwrap();
    
            //
            //println!("e3a2 {entity:?} {:?}, {max_space_w} {max_space_h} : {size:?}",computed.size);

            match size.width {
                UiVal::Px(p) if p<0.0 => {
                    computed.size.x = max_space_w+p.abs();
                }
                UiVal::Scale(p) if p<0.0 => {
                    computed.size.x = max_space_w+max_space_w*p.abs();
                }
                UiVal::None => {
                    computed.size.x = max_space_w;
                }
                _ => {},
            }
    
            match size.height {
                UiVal::Px(p) if p<0.0 => {
                    computed.size.y = max_space_h+p.abs();
                }
                UiVal::Scale(p) if p<0.0 => {
                    computed.size.y = max_space_h+max_space_h*p.abs();
                }
                UiVal::None => {
                    computed.size.y = max_space_h;
                }
                _ => {},
            }

            
            //
            //println!("e3a1 {entity:?} {:?}",computed.size);
        }
    }









    // //
    // let mut stk: Vec<(Option<Entity>, bool)> = vec![(None,false)]; 

    // //    
    // while let Some((entity,b)) = stk.pop()



    //calc node gap and children_w/h, and child computed margins, wh, expands, fills, edges, spcs
    //
    // let mut stk = root_query.iter().collect::<Vec<_>>();

    // //    
    // while let Some(entity) = stk.pop()
    // // for &entity in df_entities.iter() 
    let mut stk: Vec<Option<Entity>> = vec![None]; 

    //
    while let Some(entity) = stk.pop()
    
    {

        {
            let children=entity
                .map(|entity|children_query.get(entity).map(|children|children.iter().rev()).ok())
                .unwrap_or(Some(root_entities.iter().rev()));

            if entity.map(|entity|computed_query.get(entity).map(|computed|!computed.enabled).unwrap_or(true)).unwrap_or(false) 

            {
                continue;
            }

            stk.extend(children.clone().unwrap_or_default().map(|child_entity|Some(*child_entity)));

            // stk.extend(children_query.get(entity).map(|c|c.iter().rev()).unwrap_or_default());
        }

        //
        let children=entity
            .map(|entity|children_query.get(entity).map(|children|children.iter()).ok())
            .unwrap_or(Some(root_entities.iter()));

        //

        // let computed = *computed_query.get(entity).unwrap();
        let computed = entity.map(|entity|computed_query.get(entity).unwrap().clone()).unwrap_or(top_computed);
        // let span = span_query.get(entity).cloned().unwrap_or_default().span as usize;
        // let span = entity.and_then(|entity|span_query.get(entity).ok()).cloned().unwrap_or_default().span as usize;
        // let gap = gap_query.get(entity).cloned().unwrap_or_default();
        let gap = entity.and_then(|entity|gap_query.get(entity).ok()).cloned().unwrap_or_default();

        //
        // let size = size_query.get(entity).cloned().unwrap_or_default();
        let size = entity.and_then(|entity|size_query.get(entity).ok()).cloned().unwrap_or_default();

        let neg_width= match size.width {
            UiVal::Px(p) if p<0.0 => p.abs(),
            UiVal::Scale(p) if p<0.0 => computed.size.x*(p.abs()/(p.abs()+1.0)), //computed.w*(1.0-1.0/(p.abs()+1.0)),
            _ => 0.0,
        };

        let neg_height= match size.height {
            UiVal::Px(p) if p<0.0 => p.abs(),
            UiVal::Scale(p) if p<0.0 => computed.size.y*(p.abs()/(p.abs()+1.0)), //computed.h*(1.0-1.0/(p.abs()+1.0)),
            _ => 0.0,
        };

        let pos_width=(computed.size.x-neg_width).max(0.0); //max probably not necessary
        let pos_height=(computed.size.y-neg_height).max(0.0);

        // if let Ok(children) = children_query.get(entity) 
        if let Some(children) = &children
        {
            //get non hidden/float child count

            // let child_count = children.iter().fold(0 as usize, |acc,&c|{
            //     // println!("={c:?}");
            //     let child_computed = computed_query.get(c).cloned().unwrap_or_default();
            //     let child_float = float_query.get(c).cloned().unwrap_or_default().float;
            //     if child_computed.enabled && !child_float {acc+1} else {acc}
            // });

            // if child_count>0 {
            // println!("=child_count:{child_count}");
            //
            // let cells_num = child_count;
            // let cols_num = if span == 0 {cells_num} else {span.min(cells_num)};
            // let rows_num = if cells_num==0 {0} else { (cells_num+cols_num-1)/cols_num };

            
            let cols_num = computed.cols as usize;
            let rows_num = computed.rows as usize;
            let cells_num = cols_num*rows_num;


            // println!("=cols_num:{cols_num}, rows_num:{rows_num}");

            //calc table row/col sizes (ignoring gaps/spaces) + max h/v expands
            let mut col_widths = vec![0.0 as f32;cols_num];
            let mut row_heights = vec![0.0 as f32;rows_num];
            
            //
            //if child_count > 0 //hmm
            {
                // let mut child_ind = 0;

                for &child_entity in children.clone() //.iter() 
                {
                    //println!("Dfdsf6 {entity:?}");
                    let child_computed = computed_query.get(child_entity).cloned().unwrap_or_default();
                    let child_float = entity.is_none() || float_query.get(child_entity).cloned().unwrap_or_default().float;
                    // let child_fill = fill_query.get(child_entity).cloned().unwrap_or_default();

                    let child_edge = edge_query.get(child_entity).cloned().unwrap_or_default();

                    //don't need to do float since min size of parent already calculated earlier
                    if child_computed.enabled && !child_float {
                        // let col = child_ind % cols_num;
                        // let row = child_ind / cols_num;

                        let col=child_computed.col as usize;
                        let row=child_computed.row as usize;

                        //edge
                        // let child_hedge_scale= child_edge.l_scale.max(0.0)+child_edge.r_scale.max(0.0);
                        // let child_vedge_scale= child_edge.t_scale.max(0.0)+child_edge.b_scale.max(0.0);
                        // let child_hedge_px= child_edge.l_px.max(0.0)+child_edge.r_px.max(0.0);
                        // let child_vedge_px= child_edge.t_px.max(0.0)+child_edge.b_px.max(0.0);


                        // let child_hnedge_scale= child_edge.l_neg_scale.max(0.0)+child_edge.r_neg_scale.max(0.0);
                        // let child_vnedge_scale= child_edge.t_neg_scale.max(0.0)+child_edge.b_neg_scale.max(0.0);


                        let child_w = child_computed.size.x+
                            child_edge.h_px()+//child_hedge_px +
                            child_computed.size.x*child_edge.h_scale()+ //child_hedge_scale +
                            child_computed.size.y*child_edge.h_transverse_scale(); //child_hnedge_scale;

                        let child_h = child_computed.size.y+
                            child_edge.v_px() + //child_vedge_px +
                            child_computed.size.y*child_edge.v_scale()+ //child_vedge_scale +
                            child_computed.size.x*child_edge.v_transverse_scale(); //child_vnedge_scale;
                        //
                        //child_computed.w*child_hedge_scale+child_computed.w+child_hedge_px
                        //child_computed.h*child_vedge_scale+child_computed.h+child_vedge_px

                        col_widths[col]=col_widths[col].max(child_w);
                        row_heights[row]=row_heights[row].max(child_h);

                        // println!("row_height2 {}",row_heights[row]);

                        // child_ind+=1;
                    }
                }
            }

            //
            let mut total_col_width = col_widths.iter().sum::<f32>();
            let mut total_row_height = row_heights.iter().sum::<f32>();

            //
            let mut hgap_space =  match gap.hgap {
                // Val::Scale(p) => p.max(0.0)*(total_col_width/(cols_num as f32)),
                UiVal::Scale(p) if p>=0.0 => p*(total_col_width/(cols_num as f32)),
                UiVal::Scale(p) if p<0.0 => p.abs()*(total_row_height/(rows_num as f32)),
                UiVal::Px(p) => p.max(0.0),
                _ => 0.0
            };

            let mut vgap_space =  match gap.vgap {
                // Val::Scale(p) => p.max(0.0)*(total_row_height/(rows_num as f32)),
                UiVal::Scale(p) if p>=0.0 => p*(total_row_height/(rows_num as f32)),
                UiVal::Scale(p) if p<0.0 => p.abs()*(total_col_width/(cols_num as f32)),
                UiVal::Px(p) => p.max(0.0),
                _ => 0.0
            };

            let mut hgap_spaces = if cells_num==0{0.0}else{hgap_space*((cols_num-1) as f32)};
            let mut vgap_spaces = if cells_num==0{0.0}else{vgap_space*((rows_num-1) as f32)};

            //distribute free space/gaps between rows/cols to the table row/col sizes
            {
                //
                let mut col_expands = vec![UiVal::None;cols_num];
                let mut row_expands = vec![UiVal::None;rows_num];

                //get col/row expands
                {
                    // let mut child_ind = 0;

                    for &child_entity in children.clone() //.iter() 
                    { //children here!
                        //println!("Dfdsf5 {entity:?}");
                        let child_computed = computed_query.get(child_entity).cloned().unwrap_or_default();
                        let child_float = entity.is_none() || float_query.get(child_entity).cloned().unwrap_or_default().float;
                        let child_expand = expand_query.get(child_entity).cloned().unwrap_or_default();
    
                        if child_computed.enabled && !child_float {
                            // let col = child_ind % cols_num;
                            // let row = child_ind / cols_num;    

                            let col=child_computed.col as usize;
                            let row=child_computed.row as usize;

                            if child_expand.hexpand != UiVal::None {
                                col_expands[col] = child_expand.hexpand;
                            }

                            if child_expand.vexpand != UiVal::None {
                                row_expands[row] = child_expand.vexpand;
                            }

                            // child_ind+=1;
                        }
                    }
                }

                //
                {
                    distrib_empty_space2(
                        pos_width,pos_height,
                        cols_num,rows_num,
                        total_col_width,total_row_height,
                        gap.hgap,gap.vgap,
                        hgap_spaces,vgap_spaces,
                        &col_expands,&mut col_widths
                    );

                    // println!("row_heights {:?}",row_heights);
                    // println!("pos_height {},pos_width {},
                    // rows_num {},cols_num {},
                    // total_row_height {},total_col_width {},
                    // gap.vgap {:?},gap.hgap {:?},
                    // vgap_spaces {},hgap_spaces {},
                    // &row_expands {:?}",pos_height,pos_width,
                    // rows_num,cols_num,
                    // total_row_height,total_col_width,
                    // gap.vgap,gap.hgap,
                    // vgap_spaces,hgap_spaces,
                    // &row_expands);
                    distrib_empty_space2(
                        pos_height,pos_width,
                        rows_num,cols_num,
                        total_row_height,total_col_width,
                        gap.vgap,gap.hgap,
                        vgap_spaces,hgap_spaces,
                        &row_expands,&mut row_heights
                    );

                    // println!("row_heights2 {:?}",row_heights);
                    // distrib_empty_space(computed.w,total_col_width,gap.hgap,hgap_spaces,&col_expands,&mut col_widths);
                    // distrib_empty_space(computed.h,total_row_height,gap.vgap,vgap_spaces,&row_expands,&mut row_heights);

                    //
                    total_col_width=col_widths.iter().sum::<f32>();
                    total_row_height=row_heights.iter().sum::<f32>();

                    //recalc gaps

                    hgap_space =  match gap.hgap {
                        // Val::Scale(p) => p.max(0.0)*(total_col_width/(cols_num as f32)),
                        UiVal::Scale(p) if p>=0.0 => p*(total_col_width/(cols_num as f32)),
                        UiVal::Scale(p) if p<0.0 => p.abs()*(total_row_height/(rows_num as f32)),
                        UiVal::Px(p) => p.max(0.0),
                        _ => 0.0
                    };
        
                    vgap_space =  match gap.vgap {
                        // Val::Scale(p) => p.max(0.0)*(total_row_height/(rows_num as f32)),
                        UiVal::Scale(p) if p>=0.0 => p*(total_row_height/(rows_num as f32)),
                        UiVal::Scale(p) if p<0.0 => p.abs()*(total_col_width/(cols_num as f32)),
                        UiVal::Px(p) => p.max(0.0),
                        _ => 0.0
                    };

                    hgap_spaces = if cells_num==0{0.0}else{hgap_space*((cols_num-1) as f32)};
                    vgap_spaces = if cells_num==0{0.0}else{vgap_space*((rows_num-1) as f32)};

                    if cols_num>1 && hgap_spaces+total_col_width>pos_width {
                        // let hgap_space_old=hgap_space;
                        hgap_spaces = (pos_width-total_col_width).max(0.0);
                        hgap_space = hgap_spaces/((cols_num-1) as f32);
                    }

                    if rows_num>1 && vgap_spaces+total_row_height>pos_height {
                        // let vgap_space_old=vgap_space;

                        vgap_spaces = (pos_height-total_row_height).max(0.0);
                        vgap_space = vgap_spaces/((rows_num-1) as f32);
                    }
                }
            }

            //

            //////////////////////////////

            //
            total_col_width = col_widths.iter().sum::<f32>();
            total_row_height = row_heights.iter().sum::<f32>();

            let children_w = total_col_width + hgap_spaces + neg_width;
            let children_h = total_row_height + vgap_spaces + neg_height;
            // computed.gap_w*((cols_num-1) as f32);
            // computed.gap_h*((rows_num-1) as f32);

            //
            if let Some(entity)=entity
            {
                let mut computed2=computed_query.get_mut(entity).unwrap();

                computed2.children_size.x=children_w;
                computed2.children_size.y=children_h;
                
                computed2.gap_size.x=hgap_space;
                computed2.gap_size.y=vgap_space;
            }
            // }

            //recalc child wh's for new expands, calc percent margins/paddings for fills
            {
                // let mut child_ind=0;

                for &child_entity in children.clone() //.iter() 
                {         
                    //println!("Dfdsf4 {entity:?}");           
                    if let Ok(mut child_computed) = computed_query.get_mut(child_entity) {
                        // let child_edge = edge_query.get(child_entity).cloned().unwrap_or_default();

                        if child_computed.enabled {
                            let child_float = entity.is_none() || float_query.get(child_entity).cloned().unwrap_or_default().float;
                            let child_fill = fill_query.get(child_entity).cloned().unwrap_or_default();

                            let child_edge = edge_query.get(child_entity).cloned().unwrap_or_default();

                            //edge
                            let child_edge_h_scale = child_edge.h_scale();//child_edge.l_scale.max(0.0)+child_edge.r_scale.max(0.0);
                            let child_edge_v_scale = child_edge.v_scale();//child_edge.t_scale.max(0.0)+child_edge.b_scale.max(0.0);

                            let child_edge_h_px = child_edge.h_px();//child_edge.l_px.max(0.0)+child_edge.r_px.max(0.0);
                            let child_edge_v_px = child_edge.v_px();//child_edge.t_px.max(0.0)+child_edge.b_px.max(0.0);

                            let child_edge_h_nscale = child_edge.h_transverse_scale();//child_edge.l_neg_scale.max(0.0)+child_edge.r_neg_scale.max(0.0);
                            let child_edge_v_nscale = child_edge.v_transverse_scale();//child_edge.t_neg_scale.max(0.0)+child_edge.b_neg_scale.max(0.0);

                            //
                            let cell_w=if child_float {
                                pos_width
                            } else {
                                // let col = child_ind % cols_num;
                                
                                let col=child_computed.col as usize;

                                col_widths[col]
                            };

                            let cell_h=if child_float {
                                pos_height
                            } else {
                                // let row = child_ind / cols_num;
                                
                                let row=child_computed.row as usize;

                                row_heights[row]
                            };
                            
                            //
                            let fill_w=match child_fill.hfill {
                                UiVal::Scale(p)=>{cell_w*p.max(0.0)},
                                UiVal::Px(p)=>{p.max(0.0)},
                                _=>{0.0}
                            };

                            let fill_h=match child_fill.vfill {
                                UiVal::Scale(p)=>{cell_h*p.max(0.0)},
                                UiVal::Px(p)=>{p.max(0.0)},
                                _=>{0.0}
                            };

                            let pre_w=cell_w.min(
                                child_computed.size.x+
                                child_edge_h_px+
                                child_computed.size.x*child_edge_h_scale+ 
                                child_computed.size.y*child_edge_h_nscale);
                            
                            let pre_h=cell_h.min(
                                child_computed.size.y+
                                child_edge_v_px+
                                child_computed.size.y*child_edge_v_scale+
                                child_computed.size.x*child_edge_v_nscale);

                            // let fill_w=(fill_w.clamp(pre_w,cell_w)-child_hedge_px).max(0.0);
                            // let fill_h=(fill_h.clamp(pre_h,cell_h)-child_vedge_px).max(0.0);
                            
                            // child_computed.w=fill_w/(child_hedge_scale+1.0); //move subtract px edge here ...
                            // child_computed.h=fill_h/(child_vedge_scale+1.0);


                            let fill_w=fill_w.clamp(pre_w,cell_w);
                            let fill_h=fill_h.clamp(pre_h,cell_h);
                            
                            // child_computed.w=(fill_w-child_hedge_px).max(0.0)/(child_hedge_scale+1.0);
                            // child_computed.h=(fill_h-child_vedge_px).max(0.0)/(child_vedge_scale+1.0);

                            {
                                let w = fill_w;
                                let h = fill_h;

                                let s = child_edge_h_scale;
                                let t = child_edge_v_scale;

                                let p = child_edge_h_px;
                                let q = child_edge_v_px;

                                let m = child_edge_h_nscale;
                                let n = child_edge_v_nscale;

                                let a = 1.0/(1.0+s);
                                let b = 1.0/(1.0+t);

                                let c=a*(w-p).max(0.0);
                                let d=b*(h-q).max(0.0);

                                let v=1.0-m*n*a*b;

                                let x=(c-m*a*d)/v;
                                let y=(d-n*b*c)/v;

                                // let x=w-m*h*b;
                                // let y=h*b;

                                // seems to occur when reloading or minimised
                                // if child_computed.size.x >fill_w || child_computed.size.y >fill_h {
                                //     println!("hmm child_computed={:?} fill={:?}, pre={:?}, cell={:?}",
                                //         child_computed.size,
                                //         (fill_w,fill_h),
                                //         (pre_w,pre_h),
                                //         (cell_w,cell_h),
                                //     );
                                // }

                                //quickfix for below
                                let fill_w=fill_w.max(child_computed.size.x);
                                let fill_h=fill_h.max(child_computed.size.y);
                                
                                //
                                child_computed.size.x=x.clamp(child_computed.size.x,fill_w);
                                child_computed.size.y=y.clamp(child_computed.size.y,fill_h);
                            }

                            //w*s+w=t => w*(s+1)=t => w=t/(s+1)

                            //edge

                            // child_computed.edge_l=child_edge.l_px.max(0.0)+child_edge.l_scale.max(0.0)*child_computed.w;
                            // child_computed.edge_r=child_edge.r_px.max(0.0)+child_edge.r_scale.max(0.0)*child_computed.w;
                            // child_computed.edge_t=child_edge.t_px.max(0.0)+child_edge.t_scale.max(0.0)*child_computed.h;
                            // child_computed.edge_b=child_edge.b_px.max(0.0)+child_edge.b_scale.max(0.0)*child_computed.h;
                                
                            // child_computed.nedge_l=child_edge.l_neg_scale.max(0.0)*child_computed.h;
                            // child_computed.nedge_r=child_edge.r_neg_scale.max(0.0)*child_computed.h;
                            // child_computed.nedge_t=child_edge.t_neg_scale.max(0.0)*child_computed.w;
                            // child_computed.nedge_b=child_edge.b_neg_scale.max(0.0)*child_computed.w;

                            child_computed.border_size=calc_edge(child_edge.border,child_computed.size.x,child_computed.size.y);
                            child_computed.padding_size=calc_edge(child_edge.padding,child_computed.size.x,child_computed.size.y);
                            child_computed.margin_size=calc_edge(child_edge.margin,child_computed.size.x,child_computed.size.y);
                
                            // let edge_w=child_computed.edge_l+child_computed.edge_r+child_computed.nedge_l+child_computed.nedge_r;
                            // let edge_h=child_computed.edge_t+child_computed.edge_b+child_computed.nedge_t+child_computed.nedge_b;

                            //what ?? doesn't prev calcs make sure borders+computed size stay inside cell size, if possible?
                            // if child_computed.w+edge_w > cell_w {
                            //     let dif=(cell_w-child_computed.w).max(0.0);

                            //     if dif>0.0 {
                            //         child_computed.edge_l=dif*(child_computed.edge_l/edge_w);
                            //         child_computed.edge_r=dif*(child_computed.edge_r/edge_w);
                            //         child_computed.nedge_l=dif*(child_computed.nedge_l/edge_w);
                            //         child_computed.nedge_r=dif*(child_computed.nedge_r/edge_w);
                            //     } else {
                            //         child_computed.edge_l=0.0;
                            //         child_computed.edge_r=0.0;
                            //         child_computed.nedge_l=0.0;
                            //         child_computed.nedge_r=0.0;
                            //     }
                            // }

                            // if child_computed.h+edge_h > cell_h {
                            //     let dif=(cell_h-child_computed.h).max(0.0);

                            //     if dif>0.0 {
                            //         child_computed.edge_t=dif*(child_computed.edge_t/edge_h);
                            //         child_computed.edge_b=dif*(child_computed.edge_b/edge_h);
                            //         child_computed.nedge_t=dif*(child_computed.nedge_t/edge_h);
                            //         child_computed.nedge_b=dif*(child_computed.nedge_b/edge_h);
                            //     } else {
                            //         child_computed.edge_t=0.0;
                            //         child_computed.edge_b=0.0;
                            //         child_computed.nedge_t=0.0;
                            //         child_computed.nedge_b=0.0;
                            //     }
                            // }
                            //                                                        
                            if !child_float {
                                // child_ind+=1;
                            }
                        }
                    } else {
                        // child_ind+=1;
                    }
                }
            }

            //calc spcs 
            {
                // let mut child_ind=0;

                for &child_entity in children.clone() //.iter() 
                {
                    //println!("Dfdsf3 {entity:?}");
                    let align = align_query.get(child_entity).cloned().unwrap_or_default();
                    
                    if let Ok(mut child_computed) = computed_query.get_mut(child_entity) {
                        if child_computed.enabled {
                            let child_float = entity.is_none() || float_query.get(child_entity).cloned().unwrap_or_default().float;
                
                            //edge
                            // let houter = child_computed.edge_l+child_computed.edge_r + child_computed.nedge_l+child_computed.nedge_r;
                            // let vouter = child_computed.edge_t+child_computed.edge_b + child_computed.nedge_t+child_computed.nedge_b;

                            let houter = 
                                child_computed.border_size.left+child_computed.border_size.right+
                                child_computed.padding_size.left+child_computed.padding_size.right+
                                child_computed.margin_size.left+child_computed.margin_size.right;

                            let vouter = 
                                child_computed.border_size.top+child_computed.border_size.bottom+
                                child_computed.padding_size.top+child_computed.padding_size.bottom+
                                child_computed.margin_size.top+child_computed.margin_size.bottom;

                            if child_float {
                                let hspace=(computed.size.x-child_computed.size.x - houter).max(0.0); 
                                let vspace=(computed.size.y-child_computed.size.y - vouter).max(0.0); 
                
                                let left_space=match align.halign {
                                    UiVal::Scale(p)=>{hspace*p.clamp(0.0,1.0)},
                                    UiVal::Px(p)=>{p.clamp(0.0,hspace)},
                                    _=>{hspace* 0.5},
                                };
                
                                // let vbottom_space=vspace-match align.valign { 
                                //     Val::Scale(p)=>{vspace*p.clamp(0.0,1.0)},
                                //     Val::Px(p)=>{p.clamp(0.0,vspace)},
                                //     _=>{vspace*0.5},
                                // }; //ydir

                                let top_space=match align.valign { 
                                    UiVal::Scale(p)=>{vspace*p.clamp(0.0,1.0)},
                                    UiVal::Px(p)=>{p.clamp(0.0,vspace)},
                                    _=>{vspace*0.5},
                                }; //ydir2

                                child_computed.cell_size.left=left_space;
                                child_computed.cell_size.right=(hspace-left_space).max(0.0); //incase of floating point errors?

                                // child_computed.spc_b=vbottom_space; //ydir
                                // child_computed.spc_t=(vspace-vbottom_space).max(0.0); //ydir

                                // child_computed.cell.top=top_space; //ydir2
                                // child_computed.cell.bottom=(vspace-top_space).max(0.0); //ydir2

                                child_computed.cell_size.top=top_space; //ydir2
                                child_computed.cell_size.bottom=(vspace-top_space).max(0.0); //ydir2
                            } else {
                                // let col = child_ind % cols_num;
                                // let row = child_ind / cols_num;

                                let col=child_computed.col as usize;
                                let row=child_computed.row as usize;

                                let col_width=col_widths[col];
                                let row_height=row_heights[row];

                                // println!("row_height {row_height}");

                                let col_width_ratio =col_width/total_col_width;
                                let row_height_ratio =row_height/total_row_height;
                                
                                let col_width=col_width + neg_width*col_width_ratio;
                                let row_height=row_height + neg_height*row_height_ratio;

                                let hspace=(col_width-child_computed.size.x-houter).max(0.0); //max, incase of floating pt errors
                                let vspace=(row_height-child_computed.size.y-vouter).max(0.0);

                                let left_space=match align.halign {
                                    UiVal::Scale(p)=>{hspace*p.clamp(0.0,1.0)},
                                    UiVal::Px(p)=>{p.clamp(0.0,hspace)},
                                    _=>{hspace* 0.5},
                                };
                
                                // let vbottom_space=vspace-match align.valign { 
                                //     Val::Scale(p)=>{vspace*p.clamp(0.0,1.0)},
                                //     Val::Px(p)=>{p.clamp(0.0,vspace)},
                                //     _=>{vspace*0.5},
                                // }; //ydir
                
                                let top_space=match align.valign { 
                                    UiVal::Scale(p)=>{vspace*p.clamp(0.0,1.0)},
                                    UiVal::Px(p)=>{p.clamp(0.0,vspace)},
                                    _=>{vspace*0.5},
                                }; //ydir2

                                child_computed.cell_size.left=left_space;
                                child_computed.cell_size.right=(hspace-left_space).max(0.0); //incase of floating point errors?

                                // child_computed.spc_b=vbottom_space; //ydir
                                // child_computed.spc_t=(vspace-vbottom_space).max(0.0); //ydir
                
                                // child_computed.cell.top=top_space; //ydir2
                                // child_computed.cell.bottom=(vspace-top_space).max(0.0); //ydir2

                                child_computed.cell_size.top=top_space;
                                child_computed.cell_size.bottom=(vspace-top_space).max(0.0); 

                                // child_ind+=1;
                            }
                        }
                    }
                }
            }
            // }

            //
            //println!("e2 {entity:?} {:?}",computed.size);
        }
    }








    //calc child xy's
    //
    // let mut stk = root_query.iter().collect::<Vec<_>>();

    // //    
    // while let Some(entity) = stk.pop()
    // // for &entity in df_entities.iter() 
    
    let mut stk: Vec<Option<Entity>> = vec![None]; 

    //
    while let Some(entity) = stk.pop()
    {
        {
            let children=entity
                .map(|entity|children_query.get(entity).map(|children|children.iter().rev()).ok())
                .unwrap_or(Some(root_entities.iter().rev()));

            if entity.map(|entity|computed_query.get(entity).map(|computed|!computed.enabled).unwrap_or(true)).unwrap_or(false) 

            {
                continue;
            }

            stk.extend(children.clone().unwrap_or_default().map(|child_entity|Some(*child_entity)));

            // stk.extend(children_query.get(entity).map(|c|c.iter().rev()).unwrap_or_default());
        }

        //
        let children=entity
            .map(|entity|children_query.get(entity).map(|children|children.iter()).ok())
            .unwrap_or(Some(root_entities.iter()));

        //

        // let computed = *computed_query.get(entity).unwrap();
        let computed = entity.and_then(|entity|computed_query.get(entity).ok()).cloned().unwrap_or(top_computed);
        // // let span = span_query.get(entity).cloned().unwrap_or_default().span as usize;
        // let span = entity.and_then(|entity|span_query.get(entity).ok()).cloned().unwrap_or_default().span as usize;

        // if let Ok(children) = children_query.get(entity) 
        if let Some(children) = &children
        {
            //get non hidden/float child count

            // let child_count = children.iter().fold(0 as usize, |acc,&c|{
            //     let child_computed = computed_query.get(c).cloned().unwrap_or_default();
            //     let child_float = float_query.get(c).cloned().unwrap_or_default().float;
            //     if child_computed.enabled && !child_float {acc+1} else {acc}
            // });

            // //
            // let cells_num = child_count;
            // let cols_num = if span == 0 {cells_num} else {span.min(cells_num)};
            // let rows_num = if cells_num==0 {0} else { (cells_num+cols_num-1)/cols_num };
  
            let cols_num = computed.cols as usize;
            let rows_num = computed.rows as usize;
            // let cells_num = cols_num*rows_num;

            //

            let mut col_widths = vec![0.0 as f32;cols_num];
            let mut row_heights = vec![0.0 as f32;rows_num];

            //get row/col sizes
            {
                // let mut child_ind = 0;

                for &child_entity in children.clone() //.iter() 
                {
                    //println!("Dfdsf2 {entity:?}");
                    let child_computed = computed_query.get(child_entity).cloned().unwrap_or_default();
                    let child_float = entity.is_none() || float_query.get(child_entity).cloned().unwrap_or_default().float;

                    //don't need to do float since min size of parent already calculated earlier
                    if child_computed.enabled && !child_float {
                        // let col = child_ind % cols_num;
                        // let row = child_ind / cols_num;

                        let col=child_computed.col as usize;
                        let row=child_computed.row as usize;
                        
                        // let w=child_computed.w+
                        //     child_computed.edge_l+child_computed.edge_r+
                        //     child_computed.nedge_l+child_computed.nedge_r+
                        //     child_computed.spc_l+child_computed.spc_r;

                        // let h=child_computed.h+
                        //     child_computed.edge_t+child_computed.edge_b+
                        //     child_computed.nedge_t+child_computed.nedge_b+
                        //     child_computed.spc_t+child_computed.spc_b;

                        let w=child_computed.size.x+
                            child_computed.border_size.left+child_computed.border_size.right+
                            child_computed.padding_size.left+child_computed.padding_size.right+
                            child_computed.margin_size.left+child_computed.margin_size.right+
                            child_computed.cell_size.left+child_computed.cell_size.right;

                        let h=child_computed.size.y+
                            child_computed.border_size.top+child_computed.border_size.bottom+
                            child_computed.padding_size.top+child_computed.padding_size.bottom+
                            child_computed.margin_size.top+child_computed.margin_size.bottom+
                            child_computed.cell_size.top+child_computed.cell_size.bottom;

                        col_widths[col]=col_widths[col].max(w);
                        row_heights[row]=row_heights[row].max(h);

                        // child_ind+=1;
                    }
                }
            }

            //calc xy's
            {
                //layout here!
                let mut x = computed.pos.x;

                // let mut y = computed.y+computed.h; //ydir
                let mut y = computed.pos.y; //ydir2
                // println!("ystart {}",computed.y);

                // let mut child_ind=0;

                for &child_entity in children.clone() //.iter() 
                {
                    //println!("Dfdsf1 {entity:?}");

                    if let Ok(mut child_computed) = computed_query.get_mut(child_entity) {
                        if child_computed.enabled {
                            let child_float = entity.is_none() || float_query.get(child_entity).cloned().unwrap_or_default().float;

                            if child_float {
                                // child_computed.x=computed.x+child_computed.spc_l+child_computed.edge_l+child_computed.nedge_l;

                                // // child_computed.y=computed.y+child_computed.spc_b+child_computed.edge_b+child_computed.nedge_b; //ydir
                                // child_computed.y=computed.y+child_computed.spc_t+child_computed.edge_t+child_computed.nedge_t; //ydir2


                                child_computed.pos.x=computed.pos.x+
                                    child_computed.cell_size.left+
                                    child_computed.border_size.left+
                                    child_computed.padding_size.left+
                                    child_computed.margin_size.left;

                                child_computed.pos.y=computed.pos.y+
                                    child_computed.cell_size.top+
                                    child_computed.border_size.top+
                                    child_computed.padding_size.top+
                                    child_computed.margin_size.top; //ydir2
                            } else {
                                // let col = child_ind % cols_num;
                                // let row = child_ind / cols_num;

                                let col=child_computed.col as usize;
                                let row=child_computed.row as usize;
                                
                                if col == 0 {
                                    x=computed.pos.x;
                                    // y-=row_heights[row]; //ydir
                                    // println!("y {}+{}",computed.y,row_heights[row]);

                                    if row>0 { //hmm
                                        // y-=computed.gap_h;//ydir
                                        y+=computed.gap_size.y; //ydir2
                                        
                                    // y+=row_heights[row]; //ydir2
                                    }
                                } else {
                                    x+=computed.gap_size.x;//hgap_space;
                                }

                                // child_computed.x=x+child_computed.spc_l+child_computed.edge_l+child_computed.nedge_l;
                                // // child_computed.y=y+child_computed.spc_b+child_computed.edge_b+child_computed.nedge_b; //ydir
                                // child_computed.y=y+child_computed.spc_t+child_computed.edge_t+child_computed.nedge_t; //ydir2


                                child_computed.pos.x=x+
                                    child_computed.cell_size.left+
                                    child_computed.border_size.left+
                                    child_computed.padding_size.left+
                                    child_computed.margin_size.left;
                                
                                child_computed.pos.y=y+
                                    child_computed.cell_size.top+
                                    child_computed.border_size.top+
                                    child_computed.padding_size.top+
                                    child_computed.margin_size.top; //ydir2

                                x+= col_widths[col];

                                if col==cols_num-1 { //ydir2
                                    y+=row_heights[row]; //ydir2
                                }

                                // child_ind+=1;
                            }
                        }
                    }
                    
                    // //todo remove? since no components without computed
                    // else if cols_num > 0 { //entities without computed component treated as visible                
                    //     let col = child_ind % cols_num;
                    //     let row = child_ind / cols_num;
                        
                    //     if col == 0 {
                    //         x=computed.x;
                    //         y-=row_heights[row];

                    //         if row>0 {
                    //             y-=computed.gap_h;//vgap_space;
                    //         }
                    //     } else {
                    //         x+=computed.gap_w;//hgap_space;
                    //     }
                        
                    //     x+= col_widths[col];

                    //     child_ind+=1;
                    // }
                }
            }

            //recalc child xy's for scrolling
            //todo handle neg px scale vals as being size.x-scroll.x
            if let Some(entity)=entity {
                if let Ok(children)=children_query.get(entity) {
                    if let Ok(scroll) = scroll_query.get(entity) 
                    {
                        let hscroll_space = (computed.children_size.x-computed.size.x).max(0.0);
                        let vscroll_space = (computed.children_size.y-computed.size.y).max(0.0);
        
                        let scroll_x=match scroll.hscroll {
                            UiVal::Scale(p)=>{hscroll_space*p.clamp(0.0,1.0)}
                            UiVal::Px(p) if p>=0.0 =>{p.clamp(0.0,hscroll_space)}
                            UiVal::Px(p)=>{hscroll_space-p.clamp(0.0,hscroll_space)}
                            _=>{0.0}
                        };
        
                        let scroll_y=match scroll.vscroll {
                            UiVal::Scale(p)=>{vscroll_space*p.clamp(0.0,1.0)}
                            UiVal::Px(p) if p>=0.0 =>{p.clamp(0.0,vscroll_space)}
                            UiVal::Px(p) =>{vscroll_space-p.clamp(0.0,vscroll_space)}
                            _=>{0.0}
                        };
                        
                        {
                            let mut computed2=computed_query.get_mut(entity).unwrap();
                            computed2.scroll_pos.x=scroll_x;
                            computed2.scroll_pos.y=scroll_y;
                            computed2.scroll_size.x=hscroll_space;
                            computed2.scroll_size.y=vscroll_space;
                        }
        
                        for &child_entity in children.iter() 
                        {
                            if let Ok(mut child_computed) = computed_query.get_mut(child_entity) {
                                child_computed.pos.x-=scroll_x;
                                // child_computed.y+=scroll_y; //ydir
                                child_computed.pos.y-=scroll_y; //ydir2
                            }
                        }
                    }
                }
            }

            //
            //println!("e1 {entity:?} {:?}",computed.size);

        }
    }









    //clamp child computeds xywh within their parents', and round xywh's

    //
    let mut stk = root_query.iter().collect::<Vec<_>>();

    //    
    while let Some(entity) = stk.pop()
    // for &entity in df_entities.iter() 
    {
        {
            let Ok(computed) = computed_query.get(entity) else {
                continue;
            };

            if !computed.enabled {
                continue;
            }
            // if entity.map(|entity|computed_query.get(entity).map(|computed|!computed.enabled).unwrap_or(true)).unwrap_or(false) 
            // {
            //     continue;
            // }

            stk.extend(children_query.get(entity).map(|c|c.iter().rev()).unwrap_or_default());
        }

        //
        let parent_computed = if let Ok(parent) = parent_query.get(entity) {
            *computed_query.get(parent.get()).unwrap()
        } else {
            UiLayoutComputed {clamped_rect:UiRect{
                left:0.0,
                top:0.0,
                right:window_size.0,
                bottom:window_size.1,
            }, //y is down
            ..Default::default()}
        };
        
        let mut computed = computed_query.get_mut(entity).unwrap();

        // println!("entity {entity:?}, vis {}",computed.visible);
        if computed.enabled {
            // let inner_rect = UiRect{
            //     left:computed.x,
            //     right:computed.x+computed.w,
            //     top:computed.y, //y is down
            //     bottom:computed.y+computed.h, //y is down
            // };

            let inner_rect = computed.inner_rect();
            let cell_rect = inner_rect.expand_by(computed.padding_size + computed.border_size + computed.margin_size + computed.cell_size);
            
            //
            computed.clamped_rect = inner_rect.clamp(parent_computed.clamped_rect);
            computed.clamped_cell_rect = cell_rect.clamp(parent_computed.clamped_rect);

            // println!("hmm {:?}",inner_rect);
        }
    }

    // println!("ui_calc_computeds Elapsed: {}", now.elapsed().as_secs_f64());
}

// pub fn validate_computeds(
//     // uinode_query: Query<(Entity, &UiComputed)>,
// ) {
//     // for (entity, computed) in uinode_query.iter() {
//     //     if computed.v { //don't care if hidden as they aren't computed
//     //         if computed.w < 0.0 || computed.h < 0.0 { //computed.x < 0.0 || computed.y < 0.0 || 
//     //             panic!("Not computed ui entity {:?}:\n\t{:?}",entity,computed); //\n\t{:?}\n\t{:?} ,size,parent
//     //         }
//     //     }
//     // }
// }
