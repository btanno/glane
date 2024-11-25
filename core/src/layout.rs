use super::*;

#[derive(Clone, Debug)]
pub struct Area {
    pub handle: AnyHandle,
    pub widget_state: WidgetState,
    pub rect: LogicalRect<f32>,
    pub ancestors: Vec<AnyHandle>,
    pub layer: u32,
    pub selected: bool,
}

#[derive(Clone, Debug)]
pub struct Collision {
    pub handle: AnyHandle,
    pub widget_state: WidgetState,
    pub rect: LogicalRect<f32>,
    pub ancestors: Vec<AnyHandle>,
    pub layer: u32,
}

#[derive(Clone, Debug)]
pub struct Text {
    pub handle: AnyHandle,
    pub widget_state: WidgetState,
    pub rect: LogicalRect<f32>,
    pub ancestors: Vec<AnyHandle>,
    pub font: Option<Font>,
    pub string: String,
    pub layer: u32,
    pub selected: bool,
}

#[derive(Clone, Debug)]
pub struct CompositionText {
    pub handle: AnyHandle,
    pub widget_state: WidgetState,
    pub rect: LogicalRect<f32>,
    pub ancestors: Vec<AnyHandle>,
    pub font: Option<Font>,
    pub string: String,
    pub targeted: bool,
    pub layer: u32,
}

#[derive(Clone, Debug)]
pub struct Cursor {
    pub handle: AnyHandle,
    pub widget_state: WidgetState,
    pub rect: LogicalRect<f32>,
    pub ancestors: Vec<AnyHandle>,
    pub c: Option<char>,
    pub layer: u32,
}

#[derive(Clone, Debug)]
pub struct Clipping {
    pub handle: AnyHandle,
    pub rect: LogicalRect<f32>,
    pub ancestors: Vec<AnyHandle>,
    pub layer: u32,
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
        ancestors: &[AnyHandle],
        layer: u32,
        selected: bool,
    ) -> Self {
        Self::Area(Area {
            handle: AnyHandle::new(widget),
            widget_state,
            rect,
            ancestors: ancestors.to_vec(),
            layer,
            selected,
        })
    }

    #[inline]
    pub fn collision(
        widget: &impl Widget,
        widget_state: WidgetState,
        rect: LogicalRect<f32>,
        ancestors: &[AnyHandle],
        layer: u32,
    ) -> Self {
        Self::Collision(Collision {
            handle: AnyHandle::new(widget),
            widget_state,
            rect,
            ancestors: ancestors.to_vec(),
            layer,
        })
    }

    #[inline]
    pub fn text(
        widget: &impl Widget,
        widget_state: WidgetState,
        rect: LogicalRect<f32>,
        ancestors: &[AnyHandle],
        font: Option<Font>,
        string: String,
        layer: u32,
        selected: bool,
    ) -> Self {
        Self::Text(Text {
            handle: AnyHandle::new(widget),
            widget_state,
            rect,
            ancestors: ancestors.to_vec(),
            font,
            string,
            layer,
            selected,
        })
    }

    #[inline]
    pub fn composition_text(
        widget: &impl Widget,
        widget_state: WidgetState,
        rect: LogicalRect<f32>,
        ancestors: &[AnyHandle],
        font: Option<Font>,
        string: String,
        targeted: bool,
        layer: u32,
    ) -> Self {
        Self::CompositionText(CompositionText {
            handle: AnyHandle::new(widget),
            widget_state,
            rect,
            ancestors: ancestors.to_vec(),
            font,
            string,
            targeted,
            layer,
        })
    }

    #[inline]
    pub fn cursor(
        widget: &impl Widget,
        widget_state: WidgetState,
        rect: LogicalRect<f32>,
        ancestors: &[AnyHandle],
        c: Option<char>,
        layer: u32,
    ) -> Self {
        Self::Cursor(Cursor {
            handle: AnyHandle::new(widget),
            widget_state,
            rect,
            ancestors: ancestors.to_vec(),
            c,
            layer,
        })
    }

    #[inline]
    pub fn start_clipping(
        widget: &impl Widget,
        rect: LogicalRect<f32>,
        ancestors: &[AnyHandle],
        layer: u32,
    ) -> Self {
        Self::StartClipping(Clipping {
            handle: AnyHandle::new(widget),
            rect,
            ancestors: ancestors.to_vec(),
            layer,
        })
    }

    #[inline]
    pub fn end_clipping(
        widget: &impl Widget,
        rect: LogicalRect<f32>,
        ancestors: &[AnyHandle],
        layer: u32,
    ) -> Self {
        Self::EndClipping(Clipping {
            handle: AnyHandle::new(widget),
            rect,
            ancestors: ancestors.to_vec(),
            layer,
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
    pub fn ancestors(&self) -> &[AnyHandle] {
        match self {
            Self::Area(a) => &a.ancestors,
            Self::Collision(c) => &c.ancestors,
            Self::Text(t) => &t.ancestors,
            Self::CompositionText(t) => &t.ancestors,
            Self::Cursor(c) => &c.ancestors,
            Self::StartClipping(c) => &c.ancestors,
            Self::EndClipping(c) => &c.ancestors,
        }
    }

    #[inline]
    pub fn layer(&self) -> u32 {
        match self {
            Self::Area(a) => a.layer,
            Self::Collision(c) => c.layer,
            Self::Text(t) => t.layer,
            Self::CompositionText(t) => t.layer,
            Self::Cursor(c) => c.layer,
            Self::StartClipping(c) => c.layer,
            Self::EndClipping(c) => c.layer,
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
    pub ancestors: Vec<AnyHandle>,
    pub layer: u32,
    pub selected: bool,
}

impl<'a> LayoutContext<'a> {
    pub(crate) fn new(ctx: &'a Context) -> Self {
        Self {
            ctx,
            rect: LogicalRect::from_position_size(LogicalPosition::new(0.0, 0.0), ctx.viewport),
            ancestors: vec![],
            layer: 0,
            selected: false,
        }
    }

    #[inline]
    pub fn next(
        &self,
        widget: &impl Widget,
        rect: LogicalRect<f32>,
        layer: u32,
        selected: bool,
    ) -> Self {
        let mut ancestors = self.ancestors.clone();
        ancestors.push(AnyHandle::new(widget));
        Self {
            ctx: self.ctx,
            rect,
            ancestors,
            layer,
            selected,
        }
    }
}

#[derive(Debug, Default)]
pub struct LayoutConstructor {
    v: Vec<LayoutElement>,
}

impl LayoutConstructor {
    #[inline]
    pub fn new() -> Self {
        Self { v: vec![] }
    }

    #[inline]
    pub fn push(&mut self, _ctx: &LayoutContext, element: LayoutElement) {
        self.v.push(element);
    }

    #[inline]
    pub fn append(&mut self, mut other: Self) {
        self.v.append(&mut other.v);
    }

    #[inline]
    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&LayoutElement) -> bool,
    {
        self.v.retain(f);
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &LayoutElement> {
        self.v.iter()
    }

    #[inline]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut LayoutElement> {
        self.v.iter_mut()
    }

    #[inline]
    pub fn clipping(&self) -> Option<LogicalRect<f32>> {
        let mut result = None;
        let mut end_count = 0;
        for elem in self.v.iter().rev() {
            match &elem {
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
    v: Vec<LayoutElement>,
}

impl Layout {
    pub(crate) fn empty() -> Self {
        Self { v: vec![] }
    }

    pub(crate) fn new(_ctx: &Context, mut c: LayoutConstructor) -> Self {
        c.v.sort_by_key(|a| a.layer());
        Self { v: c.v }
    }

    #[inline]
    pub fn iter(&self) -> impl ExactSizeIterator<Item = &LayoutElement> {
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
