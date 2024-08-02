use super::*;
use std::collections::VecDeque;

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
}

impl<'a> LayoutContext<'a> {
    pub(crate) fn new(ctx: &'a Context) -> Self {
        Self {
            ctx,
            rect: LogicalRect::from_position_size(LogicalPosition::new(0.0, 0.0), ctx.viewport),
        }
    }

    #[inline]
    pub fn next(&self, rect: LogicalRect<f32>) -> Self {
        Self {
            ctx: self.ctx,
            rect,
        }
    }
}

#[derive(Debug)]
pub struct LayoutConstructor {
    v: VecDeque<LayoutElement>,
}

impl LayoutConstructor {
    pub fn new() -> Self {
        Self { v: VecDeque::new() }
    }

    #[inline]
    pub fn push_front(&mut self, element: LayoutElement) {
        self.v.push_front(element);
    }

    #[inline]
    pub fn push_back(&mut self, element: LayoutElement) {
        self.v.push_back(element);
    }

    #[inline]
    pub fn front(&self) -> Option<&LayoutElement> {
        self.v.front()
    }

    #[inline]
    pub fn back(&self) -> Option<&LayoutElement> {
        self.v.back()
    }

    #[inline]
    pub fn front_mut(&mut self) -> Option<&mut LayoutElement> {
        self.v.front_mut()
    }

    #[inline]
    pub fn back_mut(&mut self) -> Option<&mut LayoutElement> {
        self.v.back_mut()
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &LayoutElement> {
        self.v.iter()
    }

    #[inline]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut LayoutElement> {
        self.v.iter_mut()
    }
}

#[derive(Clone, Debug)]
pub struct Layout {
    v: VecDeque<LayoutElement>,
}

impl Layout {
    pub(crate) fn empty() -> Self {
        Self { v: VecDeque::new() }
    }

    pub(crate) fn new(_ctx: &Context, c: LayoutConstructor) -> Self {
        Self { v: c.v }
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &LayoutElement> {
        self.v.iter()
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
