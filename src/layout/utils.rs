use bevy::math::{Rect, Vec2};

use super::values::{UiVal, UiRectVal, };

pub fn calc_edge(edge:UiRectVal,w:f32,h:f32) -> Rect {
    let left = match edge.left {
        UiVal::Px(v)=>v.max(0.0),
        UiVal::Scale(v) if v<0.0=>v.abs()*h,
        UiVal::Scale(v)=>v*w,
        UiVal::None => 0.0
    };
    let right = match edge.right {
        UiVal::Px(v)=>v.max(0.0),
        UiVal::Scale(v) if v<0.0=>v.abs()*h,
        UiVal::Scale(v)=>v*w,
        UiVal::None => 0.0
    };
    let top = match edge.top {
        UiVal::Px(v)=>v.max(0.0),
        UiVal::Scale(v) if v<0.0=>v.abs()*w,
        UiVal::Scale(v)=>v*h,
        UiVal::None => 0.0
    };
    let bottom = match edge.bottom {
        UiVal::Px(v)=>v.max(0.0),
        UiVal::Scale(v) if v<0.0=>v.abs()*w,
        UiVal::Scale(v)=>v*h,
        UiVal::None => 0.0
    };
    Rect {
        min:Vec2::new(left,top),
        max:Vec2::new(right,bottom),
    }
}

pub fn distrib_empty_space2(
    computed_w:f32,computed_h:f32,
    cols_num:usize,rows_num:usize,
    total_col_width:f32,_total_row_height:f32,
    gap_hgap:UiVal,gap_vgap:UiVal,
    hgap_spaces:f32,vgap_spaces:f32,
    col_expands:&Vec<UiVal>,
    col_widths:&mut Vec<f32>,
) {
    let the_width_avail = match gap_hgap {
        UiVal::Scale(p) if p>=0.0 => computed_w/(1.0+p*(1.0-1.0/(cols_num as f32))),
        UiVal::Scale(p) if p<0.0 => match gap_vgap {
            UiVal::Scale(q) if q>=0.0 => {
                let y=computed_h/(1.0+q*(1.0-1.0/(rows_num as f32)));
                let x=computed_w - (y/(rows_num as f32))*p.abs()*((cols_num-1) as f32);
                x
            },
            UiVal::Scale(q) if q<0.0 => {
                let w=computed_w;
                let h=computed_h;
                let c = cols_num as f32;
                let r = rows_num as f32;
                let p = p.abs();
                let q= q.abs();
                let a = p*((c-1.0)/r);
                let b = q*((r-1.0)/c);
                let x = (w-h*a)/(1.0-a*b);
                x
            },
            UiVal::Px(_r) => {
                let y=(computed_h-vgap_spaces).max(0.0);
                let x=computed_w - (y/(rows_num as f32))*p.abs()*((cols_num-1) as f32);
                x
            },
            _ => {
                let y=computed_h;
                let x=computed_w - (y/(rows_num as f32))*p.abs()*((cols_num-1) as f32);
                x
            }
        },
        UiVal::Px(_p) => (computed_w - hgap_spaces).max(0.0),
        _ => computed_w
    };

    let mut empty_space_width = (the_width_avail-total_col_width).max(0.0);

    let col_expand_px_count = col_expands.iter().map(|e|match e{UiVal::Px(_)=>1,_=>0}).sum::<usize>();
    let col_expand_scale_count = col_expands.iter().map(|e|match e{UiVal::Scale(_)=>1,_=>0}).sum::<usize>();
    let col_expand_none_count = col_expands.len() - col_expand_px_count - col_expand_scale_count;

    let col_expand_px_sum = col_expands.iter().map(|&e|match e{UiVal::Px(p)=>p.max(0.0),_=>0.0}).sum::<f32>();
    let col_expand_scale_sum = col_expands.iter().map(|&e|match e{UiVal::Scale(s)=>s.max(0.0),_=>0.0}).sum::<f32>();
    let col_non_expand_width_sum = (0..cols_num).map(|col|match col_expands[col]{UiVal::None=>col_widths[col],_=>0.0}).sum::<f32>();

    //distrib space to px expands
    if col_expand_px_sum > 0.0 {
        let spc=empty_space_width;

        for col in 0..cols_num {
            if let UiVal::Px(p) = col_expands[col] {
                let x = (if col_expand_px_sum>spc{(p/col_expand_px_sum)*spc}else{p}).max(0.0);
                col_widths[col]+=x;
                empty_space_width=(empty_space_width-x).max(0.0);
            }
        }
    }

    //distrib space to percent expands
    if col_expand_scale_sum > 0.0 {
        let spc=empty_space_width;

        for col in 0..cols_num {
            if let UiVal::Scale(s) = col_expands[col] {
                let x=spc*(if col_expand_scale_sum>1.0||col_expand_none_count==0{s/col_expand_scale_sum}else{s}).max(0.0);
                col_widths[col]+=x;
                empty_space_width=(empty_space_width-x).max(0.0);
            }
        }
    }

    //distrib remaining space
    if empty_space_width > 0.0 {
        if col_expand_none_count==0 || col_non_expand_width_sum == 0.0 {
            //distrib evenly
            let n = if col_expand_none_count==0{cols_num}else{col_expand_none_count};
            let avg = empty_space_width/(n as f32);

            for col in 0..cols_num {
                if col_expand_none_count==0 || UiVal::None == col_expands[col] {
                    col_widths[col]+=avg;
                }
            }
        } else {
            //distrib by their size ratios
            for col in 0..cols_num {
                if UiVal::None == col_expands[col] {
                    let x = empty_space_width*(col_widths[col]/col_non_expand_width_sum);
                    col_widths[col]+=x;
                }
            }
        }
    }
}




// pub fn distrib_empty_space(computed_w:f32,total_col_width:f32,gap_hgap:UiVal,hgap_spaces:f32,col_expands:&Vec<UiVal>,col_widths:&mut Vec<f32>) {
//     let cols_num=col_widths.len();

//     let the_width_avail = match gap_hgap {
//         UiVal::Scale(p) => computed_w/(1.0+p*(1.0-1.0/(cols_num as f32))),
//         UiVal::Px(p) => (computed_w - hgap_spaces).max(0.0),
//         UiVal::None => computed_w
//     };

//     let mut empty_space_width = (the_width_avail-total_col_width).max(0.0);

//     let col_expand_px_count = col_expands.iter().map(|e|match e{UiVal::Px(_)=>1,_=>0}).sum::<usize>();
//     let col_expand_scale_count = col_expands.iter().map(|e|match e{UiVal::Scale(_)=>1,_=>0}).sum::<usize>();
//     let col_expand_none_count = col_expands.len() - col_expand_px_count - col_expand_scale_count;

//     let col_expand_px_sum = col_expands.iter().map(|&e|match e{UiVal::Px(p)=>p.max(0.0),_=>0.0}).sum::<f32>();
//     let col_expand_scale_sum = col_expands.iter().map(|&e|match e{UiVal::Scale(s)=>s.max(0.0),_=>0.0}).sum::<f32>();
//     let col_non_expand_width_sum = (0..cols_num).map(|col|match col_expands[col]{UiVal::None=>col_widths[col],_=>0.0}).sum::<f32>();


//     //distrib space to px expands
//     if col_expand_px_sum > 0.0 {
//         let spc=empty_space_width;

//         for col in 0..cols_num {
//             if let UiVal::Px(p) = col_expands[col] {
//                 let x = (if col_expand_px_sum>spc{(p/col_expand_px_sum)*spc}else{p}).max(0.0);
//                 col_widths[col]+=x;
//                 empty_space_width=(empty_space_width-x).max(0.0);
//             }
//         }
//     }

//     //distrib space to percent expands
//     if col_expand_scale_sum > 0.0 {
//         let spc=empty_space_width;

//         for col in 0..cols_num {
//             if let UiVal::Scale(s) = col_expands[col] {
//                 let x=spc*(if col_expand_scale_sum>1.0||col_expand_none_count==0{s/col_expand_scale_sum}else{s}).max(0.0);
//                 col_widths[col]+=x;
//                 empty_space_width=(empty_space_width-x).max(0.0);
//             }
//         }
//     }

//     //distrib remaining space
//     if empty_space_width > 0.0 {
//         if col_expand_none_count==0 || col_non_expand_width_sum == 0.0 {
//             //distrib evenly
//             let n = if col_expand_none_count==0{cols_num}else{col_expand_none_count};
//             let avg = empty_space_width/(n as f32);

//             for col in 0..cols_num {
//                 if col_expand_none_count==0 || UiVal::None == col_expands[col] {
//                     col_widths[col]+=avg;
//                 }
//             }
//         } else {
//             //distrib by their size ratios
//             for col in 0..cols_num {
//                 if UiVal::None == col_expands[col] {
//                     let x = empty_space_width*(col_widths[col]/col_non_expand_width_sum);
//                     col_widths[col]+=x;
//                 }
//             }
//         }
//     }
// }















