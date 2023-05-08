

use std::rc::Rc;

use dui_util::Rf;
use vello::{kurbo::Rect, peniko::{Brush}, SceneBuilder};

use crate::simple_text::FontManager;



#[derive(Clone)]
pub struct DrawingContext<'a> {
    pub builder: Rf<SceneBuilder<'a>>,
    pub path: Rf<Vec<u32>>,

    pub font_manager: Rf<FontManager>,

    pub background_brush: Brush,
    pub fill_brush: Brush,
    pub foreground_color: Brush,

    pub bounding: Rect,
    pub first: bool,

    pub scale_factor: f64,
}

#[derive(Debug)]
pub struct LayoutContext<'a> {
    pub font_manager: Rf<FontManager>,
    // pub path: Rc<Vec<u32>>,
    pub path: &'a mut Vec<u32>,
    pub scale_factor: f64,
}

// pub fn layout(mut dctx: DrawingContext<'_>, view: impl Element, index: u32) {

//     Rc::make_mut(&mut dctx.path).push(index);
//     if view.is_leaf() { return }
// }

// pub fn layout_iter(mut dctx: DrawingContext<'_>, view: impl ElementIterator, index: u32) {
//     for i in 0..view.len() {
//         if !view.is_leaf_at(i) {
//             view.l
//         }
//     }

//     Rc::make_mut(&mut dctx.path).push(index);
//     if view.is_leaf() { return }
// }

// pub fn draw(mut dctx: DrawingContext<'_>, view: impl Element, index: u32) {
//     if view.is_leaf() { return }

//     Rc::make_mut(&mut dctx.path).push(index);

//     get_id_manger_mut().insert(Vec::clone(&dctx.path));

//     let layout = view.view().layout(dctx.bounding);
//     // get_id_manger_mut().set_layout_full_rect(id, layout);

//     dctx.bounding = layout;

//     view.view().draw(&mut dctx);

//     draw(dctx.clone(), view.body(), 0);

//     Rc::make_mut(&mut dctx.path).pop();
// }

// pub fn draw_iter(mut dctx: DrawingContext<'_>, view: impl ElementIterator, index: u32) {
//     for i in 0..view.len() {
//         get_id_manger().get_layout(id)
//     }
//     if view.is_leaf() { return }

//     Rc::make_mut(&mut dctx.path).push(index);

//     let layout = view.view().layout(dctx.bounding);
//     // get_id_manger_mut().set_layout_full_rect(id, layout);

//     dctx.bounding = layout;

//     view.view().draw(&mut dctx);

//     draw(dctx.clone(), view.body(), 0);

//     Rc::make_mut(&mut dctx.path).pop();
// }

