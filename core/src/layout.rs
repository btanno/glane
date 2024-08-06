use super::*;
use std::cell::{Ref, RefCell, RefMut};

#[derive(Clone, Debug)]
pub struct Area {
    pub handle: AnyHandle,
    pub widget_state: WidgetState,
    pub rect: LogicalRect<f32>,
}

#[derive(Clone, Debug)]
pub struct SelectedArea {
    pub handle: AnyHandle,
    pub widget_state: WidgetState,
    pub rect: LogicalRect<f32>,
}

#[derive(Clone, Debug)]
pub struct ClippedArea {
    pub handle: AnyHandle,
    pub widget_state: WidgetState,
    pub rect: LogicalRect<f32>,
    pub layout: Layout,
}

#[derive(Clone, Debug)]
pub struct Text {
    pub handle: AnyHandle,
    pub widget_state: WidgetState,
    pub rect: LogicalRect<f32>,
    pub string: String,
}

#[derive(Clone, Debug)]
pub struct CompositionText {
    pub handle: AnyHandle,
    pub widget_state: WidgetState,
    pub rect: LogicalRect<f32>,
    pub string: String,
    pub targeted: bool,
}

#[derive(Clone, Debug)]
pub struct Cursor {
    pub handle: AnyHandle,
    pub widget_state: WidgetState,
    pub rect: LogicalRect<f32>,
}

#[derive(Clone, Debug)]
pub enum LayoutElement {
    Area(Area),
    SelectedArea(SelectedArea),
    ClippedArea(ClippedArea),
    Text(Text),
    CompositionText(CompositionText),
    Cursor(Cursor),
}

impl LayoutElement {
    #[inline]
    pub fn area(widget: &impl Widget, widget_state: WidgetState, rect: LogicalRect<f32>) -> Self {
        Self::Area(Area {
            handle: AnyHandle::new(widget),
            widget_state,
            rect,
        })
    }

    #[inline]
    pub fn selected_area(
        widget: &impl Widget,
        widget_state: WidgetState,
        rect: LogicalRect<f32>,
    ) -> Self {
        Self::SelectedArea(SelectedArea {
            handle: AnyHandle::new(widget),
            widget_state,
            rect,
        })
    }

    #[inline]
    pub fn clipped_area(
        widget: &impl Widget,
        widget_state: WidgetState,
        rect: LogicalRect<f32>,
        ctx: &Context,
        layout: LayoutConstructor,
    ) -> Self {
        Self::ClippedArea(ClippedArea {
            handle: AnyHandle::new(widget),
            widget_state,
            rect,
            layout: Layout::new(ctx, layout),
        })
    }

    #[inline]
    pub fn text(
        widget: &impl Widget,
        widget_state: WidgetState,
        rect: LogicalRect<f32>,
        string: String,
    ) -> Self {
        Self::Text(Text {
            handle: AnyHandle::new(widget),
            widget_state,
            rect,
            string,
        })
    }

    #[inline]
    pub fn composition_text(
        widget: &impl Widget,
        widget_state: WidgetState,
        rect: LogicalRect<f32>,
        string: String,
        targeted: bool,
    ) -> Self {
        Self::CompositionText(CompositionText {
            handle: AnyHandle::new(widget),
            widget_state,
            rect,
            string,
            targeted,
        })
    }

    #[inline]
    pub fn cursor(widget: &impl Widget, widget_state: WidgetState, rect: LogicalRect<f32>) -> Self {
        Self::Cursor(Cursor {
            handle: AnyHandle::new(widget),
            widget_state,
            rect,
        })
    }

    #[inline]
    pub fn handle(&self) -> AnyHandle {
        match self {
            Self::Area(a) => a.handle,
            Self::SelectedArea(a) => a.handle,
            Self::ClippedArea(a) => a.handle,
            Self::Text(t) => t.handle,
            Self::CompositionText(t) => t.handle,
            Self::Cursor(c) => c.handle,
        }
    }

    #[inline]
    pub fn widget_state(&self) -> WidgetState {
        match self {
            Self::Area(a) => a.widget_state,
            Self::SelectedArea(a) => a.widget_state,
            Self::ClippedArea(a) => a.widget_state,
            Self::Text(t) => t.widget_state,
            Self::CompositionText(t) => t.widget_state,
            Self::Cursor(c) => c.widget_state,
        }
    }

    #[inline]
    pub fn rect(&self) -> &LogicalRect<f32> {
        match self {
            Self::Area(a) => &a.rect,
            Self::SelectedArea(a) => &a.rect,
            Self::ClippedArea(a) => &a.rect,
            Self::Text(t) => &t.rect,
            Self::CompositionText(t) => &t.rect,
            Self::Cursor(c) => &c.rect,
        }
    }
}

#[derive(Debug)]
pub struct LayoutContext<'a> {
    pub ctx: &'a Context,
    pub rect: LogicalRect<f32>,
    pub layer: u32,
}

impl<'a> LayoutContext<'a> {
    pub(crate) fn new(ctx: &'a Context) -> Self {
        Self {
            ctx,
            rect: LogicalRect::from_position_size(LogicalPosition::new(0.0, 0.0), ctx.viewport),
            layer: 0,
        }
    }

    #[inline]
    pub fn next(&self, rect: LogicalRect<f32>, layer: u32) -> Self {
        Self {
            ctx: self.ctx,
            rect,
            layer,
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Element {
    element: RefCell<LayoutElement>,
    layer: u32,
}

#[derive(Debug)]
pub struct LayoutConstructor {
    v: Vec<Element>,
}

impl LayoutConstructor {
    pub fn new() -> Self {
        Self {
            v: vec![],
        }
    }

    #[inline]
    pub fn push(&mut self, ctx: &LayoutContext, element: LayoutElement) {
        self.v.push(Element {
            element: RefCell::new(element),
            layer: ctx.layer,
        });
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = Ref<LayoutElement>> {
        self.v.iter().map(|elem| elem.element.borrow())
    }

    #[inline]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = RefMut<LayoutElement>> {
        self.v.iter().map(|elem| elem.element.borrow_mut())
    }
}

#[derive(Clone, Debug)]
pub struct Layout {
    v: Vec<Element>,
}

impl Layout {
    pub(crate) fn empty() -> Self {
        Self { v: vec![] }
    }

    pub(crate) fn new(_ctx: &Context, mut c: LayoutConstructor) -> Self {
        c.v.sort_by(|a, b| a.layer.cmp(&b.layer));
        Self { v: c.v }
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = Ref<LayoutElement>> {
        self.v.iter().map(|elem| elem.element.borrow())
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.v.is_empty()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.v.len()
    }
}
