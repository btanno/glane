use super::*;
use std::collections::VecDeque;

#[derive(Clone, Debug)]
pub struct LayoutElement {
    pub handle: AnyHandle,
    pub rect: LogicalRect<f32>,
    pub z: f32,
    pub string: Option<String>,
    pub state: WidgetState,
}

impl LayoutElement {
    #[inline]
    pub fn new(
        handle: AnyHandle,
        rect: LogicalRect<f32>,
        z: f32,
        string: Option<String>,
        state: WidgetState,
    ) -> Self {
        Self {
            handle,
            rect,
            z,
            string,
            state,
        }
    }
}

#[derive(Debug)]
pub struct LayoutContext<'a> {
    pub ctx: &'a Context,
    pub rect: LogicalRect<f32>,
    pub z: f32,
}

impl<'a> LayoutContext<'a> {
    pub(crate) fn new(ctx: &'a Context) -> Self {
        Self {
            ctx,
            rect: LogicalRect::from_position_size(LogicalPosition::new(0.0, 0.0), ctx.viewport),
            z: 0.0,
        }
    }

    #[inline]
    pub fn next(&self, rect: LogicalRect<f32>, z: f32) -> Self {
        Self {
            ctx: self.ctx,
            rect,
            z,
        }
    }
}

#[derive(Debug)]
pub struct LayoutConstructor {
    v: VecDeque<LayoutElement>,
}

impl LayoutConstructor {
    pub(crate) fn new() -> Self {
        Self { v: VecDeque::new() }
    }

    #[inline]
    pub fn push_front(
        &mut self,
        widget: &impl Widget,
        rect: LogicalRect<f32>,
        z: f32,
        string: Option<String>,
        state: WidgetState,
    ) {
        self.v
            .push_front(LayoutElement::new(AnyHandle::new(widget), rect, z, string, state));
    }

    #[inline]
    pub fn push_back(
        &mut self,
        widget: &impl Widget,
        rect: LogicalRect<f32>,
        z: f32,
        string: Option<String>,
        state: WidgetState,
    ) {
        self.v
            .push_back(LayoutElement::new(AnyHandle::new(widget), rect, z, string, state));
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

    pub(crate) fn new(_ctx: &Context, mut c: LayoutConstructor) -> Self {
        let v = c.v.make_contiguous();
        v.sort_by(|a, b| a.z.partial_cmp(&b.z).unwrap_or(std::cmp::Ordering::Less));
        Self { v: c.v }
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &LayoutElement> {
        self.v.iter()
    }
}
