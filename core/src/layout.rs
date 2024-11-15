use super::*;

#[derive(Clone, Debug)]
pub struct Area {
    pub handle: AnyHandle,
    pub widget_state: WidgetState,
    pub rect: LogicalRect<f32>,
    pub selected: bool,
}

#[derive(Clone, Debug)]
pub struct Collision {
    pub handle: AnyHandle,
    pub widget_state: WidgetState,
    pub rect: LogicalRect<f32>,
}

#[derive(Clone, Debug)]
pub struct Text {
    pub handle: AnyHandle,
    pub widget_state: WidgetState,
    pub rect: LogicalRect<f32>,
    pub string: String,
    pub selected: bool,
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
    pub c: Option<char>,
}

#[derive(Clone, Debug)]
pub struct Clipping {
    pub handle: AnyHandle,
    pub rect: LogicalRect<f32>,
}

#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum LayoutElement {
    Area(Area),
    Collision(Collision),
    Text(Text),
    CompositionText(CompositionText),
    Cursor(Cursor),
    StartClipping(Clipping),
    EndClipping(Clipping),
}

impl LayoutElement {
    #[inline]
    pub fn area(
        widget: &impl Widget,
        widget_state: WidgetState,
        rect: LogicalRect<f32>,
        selected: bool,
    ) -> Self {
        Self::Area(Area {
            handle: AnyHandle::new(widget),
            widget_state,
            rect,
            selected,
        })
    }

    #[inline]
    pub fn collision(
        widget: &impl Widget,
        widget_state: WidgetState,
        rect: LogicalRect<f32>,
    ) -> Self {
        Self::Collision(Collision {
            handle: AnyHandle::new(widget),
            widget_state,
            rect,
        })
    }

    #[inline]
    pub fn text(
        widget: &impl Widget,
        widget_state: WidgetState,
        rect: LogicalRect<f32>,
        string: String,
        selected: bool,
    ) -> Self {
        Self::Text(Text {
            handle: AnyHandle::new(widget),
            widget_state,
            rect,
            string,
            selected,
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
    pub fn cursor(
        widget: &impl Widget,
        widget_state: WidgetState,
        rect: LogicalRect<f32>,
        c: Option<char>,
    ) -> Self {
        Self::Cursor(Cursor {
            handle: AnyHandle::new(widget),
            widget_state,
            rect,
            c,
        })
    }

    #[inline]
    pub fn start_clipping(widget: &impl Widget, rect: LogicalRect<f32>) -> Self {
        Self::StartClipping(Clipping {
            handle: AnyHandle::new(widget),
            rect,
        })
    }

    #[inline]
    pub fn end_clipping(widget: &impl Widget, rect: LogicalRect<f32>) -> Self {
        Self::EndClipping(Clipping {
            handle: AnyHandle::new(widget),
            rect,
        })
    }

    #[inline]
    pub fn handle(&self) -> AnyHandle {
        match self {
            Self::Area(a) => a.handle,
            Self::Collision(c) => c.handle,
            Self::Text(t) => t.handle,
            Self::CompositionText(t) => t.handle,
            Self::Cursor(c) => c.handle,
            Self::StartClipping(c) => c.handle,
            Self::EndClipping(c) => c.handle,
        }
    }

    #[inline]
    pub fn rect(&self) -> &LogicalRect<f32> {
        match self {
            Self::Area(a) => &a.rect,
            Self::Collision(c) => &c.rect,
            Self::Text(t) => &t.rect,
            Self::CompositionText(t) => &t.rect,
            Self::Cursor(c) => &c.rect,
            Self::StartClipping(c) => &c.rect,
            Self::EndClipping(c) => &c.rect,
        }
    }

    #[inline]
    pub fn as_area(&self) -> Option<&Area> {
        match self {
            Self::Area(v) => Some(v),
            _ => None,
        }
    }

    #[inline]
    pub fn as_collision(&self) -> Option<&Collision> {
        match self {
            Self::Collision(v) => Some(v),
            _ => None,
        }
    }
    
    #[inline]
    pub fn as_text(&self) -> Option<&Text> {
        match self {
            Self::Text(v) => Some(v),
            _ => None,
        }
    }

    #[inline]
    pub fn as_composition_text(&self) -> Option<&CompositionText> {
        match self {
            Self::CompositionText(v) => Some(v),
            _ => None,
        }
    }

    #[inline]
    pub fn as_cursor(&self) -> Option<&Cursor> {
        match self {
            Self::Cursor(v) => Some(v),
            _ => None,
        }
    }
    
    #[inline]
    pub fn as_start_clipping(&self) -> Option<&Clipping> {
        match self {
            Self::StartClipping(v) => Some(v),
            _ => None,
        }
    }

    #[inline]
    pub fn as_end_clipping(&self) -> Option<&Clipping> {
        match self {
            Self::EndClipping(v) => Some(v),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct LayoutContext<'a> {
    pub ctx: &'a Context,
    pub rect: LogicalRect<f32>,
    pub layer: u32,
    pub selected: bool,
}

impl<'a> LayoutContext<'a> {
    pub(crate) fn new(ctx: &'a Context) -> Self {
        Self {
            ctx,
            rect: LogicalRect::from_position_size(LogicalPosition::new(0.0, 0.0), ctx.viewport),
            layer: 0,
            selected: false,
        }
    }

    #[inline]
    pub fn next(&self, rect: LogicalRect<f32>, layer: u32, selected: bool) -> Self {
        Self {
            ctx: self.ctx,
            rect,
            layer,
            selected,
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Element {
    element: LayoutElement,
    layer: u32,
}

#[derive(Debug, Default)]
pub struct LayoutConstructor {
    v: Vec<Element>,
}

impl LayoutConstructor {
    #[inline]
    pub fn new() -> Self {
        Self { v: vec![] }
    }

    #[inline]
    pub fn push(&mut self, ctx: &LayoutContext, element: LayoutElement) {
        self.v.push(Element {
            element,
            layer: ctx.layer,
        });
    }

    #[inline]
    pub fn append(&mut self, mut other: Self) {
        self.v.append(&mut other.v);
    }

    #[inline]
    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&LayoutElement) -> bool,
    {
        self.v.retain(|a| f(&a.element));
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &LayoutElement> {
        self.v.iter().map(|elem| &elem.element)
    }

    #[inline]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut LayoutElement> {
        self.v.iter_mut().map(|elem| &mut elem.element)
    }

    #[inline]
    pub fn clipping(&self) -> Option<LogicalRect<f32>> {
        let mut result = None;
        let mut end_count = 0;
        for elem in self.v.iter().rev() {
            match &elem.element {
                LayoutElement::StartClipping(ev) => {
                    if end_count == 0 {
                        result = Some(ev.rect);
                        break;
                    } else {
                        end_count -= 1;
                        if end_count < 0 {
                            return None;
                        }
                    }
                }
                LayoutElement::EndClipping(_) => {
                    end_count += 1;
                }
                _ => {}
            }
        }
        result
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
    pub fn iter(&self) -> impl Iterator<Item = &LayoutElement> {
        self.v.iter().map(|elem| &elem.element)
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
