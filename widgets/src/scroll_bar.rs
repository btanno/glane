use super::*;

#[derive(Debug)]
pub struct Style {
    pub width: f32,
    pub min_thumb_size: f32,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            width: 13.0,
            min_thumb_size: 7.0,
        }
    }
}

pub trait Direction: 'static {}

mod direction {
    use super::*;

    #[derive(Debug)]
    pub struct Vertical;

    impl Direction for Vertical {}

    #[derive(Debug)]
    pub struct Horizontal;

    impl Direction for Horizontal {}
}

#[derive(Clone, Copy, Debug)]
pub enum Message {
    Changed(usize),
}

#[derive(Debug)]
pub struct Thumb {
    id: Id,
    pub len: usize,
    widget_state: WidgetState,
}

impl Thumb {
    fn new(len: usize) -> Self {
        Self {
            id: Id::new(),
            len,
            widget_state: WidgetState::None,
        }
    }
}

impl HasId for Thumb {
    fn id(&self) -> Id {
        self.id
    }
}

impl Widget for Thumb {
    fn input(&mut self, _ctx: &Context, _input: &Input, _events: &mut Events) -> ControlFlow {
        ControlFlow::Continue
    }

    fn apply(&mut self, _funcs: &mut ApplyFuncs) {}

    fn size(&self, _ctx: &LayoutContext) -> LogicalSize<f32> {
        (0.0, 0.0).into()
    }

    fn size_types(&self) -> SizeTypes {
        SizeTypes::fix()
    }

    fn layout(&self, _lc: LayoutContext, _result: &mut LayoutConstructor) {}
}

#[derive(Debug)]
pub struct ScrollBar<T: Direction> {
    id: Id,
    style: Style,
    pub len: usize,
    pub thumb: Thumb,
    current: usize,
    d: f32,
    min_collision: f32,
    _direction: std::marker::PhantomData<T>,
}

impl<T: Direction> ScrollBar<T> {
    #[inline]
    pub fn new(len: usize, thumb_len: usize) -> Self {
        Self {
            id: Id::new(),
            style: Style::default(),
            len,
            current: 0,
            thumb: Thumb::new(thumb_len),
            d: 0.0,
            min_collision: 15.0,
            _direction: std::marker::PhantomData,
        }
    }

    #[inline]
    pub fn current(&self) -> usize {
        self.current
    }

    #[inline]
    pub fn advance(&mut self, d: isize) {
        use std::cmp::Ordering;
        match d.cmp(&0) {
            Ordering::Less => {
                let d = -d as usize;
                if self.current <= d {
                    self.current = 0;
                } else {
                    self.current -= d;
                }
            }
            Ordering::Greater => {
                let max = self.len - self.thumb.len;
                let d = d as usize;
                if self.current + d >= max {
                    self.current = max;
                } else {
                    self.current += d;
                }
            }
            Ordering::Equal => {}
        }
    }
}

impl<T: Direction> HasId for ScrollBar<T> {
    fn id(&self) -> Id {
        self.id
    }
}

impl Widget for ScrollBar<direction::Vertical> {
    fn input(&mut self, ctx: &Context, input: &Input, events: &mut Events) -> ControlFlow {
        let Some(layout) = ctx.find_layout(self).next() else {
            return ControlFlow::Continue;
        };
        let Some(thumb_layout) = ctx
            .find_layout(&self.thumb)
            .find(|l| matches!(l, LayoutElement::Collision(_)))
        else {
            return ControlFlow::Continue;
        };
        let size = layout.rect().size();
        let thumb_size = thumb_layout.rect().size();
        match input {
            Input::MouseInput(m) => {
                if thumb_layout.rect().contains(&m.mouse_state.position) {
                    if m.button == MouseButton::Left && m.button_state == ButtonState::Pressed {
                        self.d = m.mouse_state.position.y - thumb_layout.rect().top;
                        self.thumb.widget_state = WidgetState::Pressed;
                    } else {
                        self.thumb.widget_state = WidgetState::Hover;
                    }
                } else {
                    self.thumb.widget_state = WidgetState::None;
                }
            }
            Input::CursorMoved(m) => {
                if self.thumb.widget_state == WidgetState::Pressed {
                    let height = size.height - thumb_size.height;
                    let p = m.mouse_state.position.y - layout.rect().top - self.d;
                    let current = (self.len - self.thumb.len) as f32 * p / height;
                    self.current = (current.floor() as usize).min(self.len - self.thumb.len - 1);
                    events.push_message(self, Message::Changed(self.current))
                }
            }
            _ => {}
        }
        ControlFlow::Continue
    }

    fn apply(&mut self, funcs: &mut ApplyFuncs) {
        funcs.apply(self);
    }

    fn size(&self, ctx: &LayoutContext) -> LogicalSize<f32> {
        LogicalSize::new(self.style.width, ctx.rect.bottom - ctx.rect.top)
    }

    fn size_types(&self) -> SizeTypes {
        SizeTypes::new(SizeType::Fix, SizeType::Flexible)
    }

    fn layout(&self, lc: LayoutContext, result: &mut LayoutConstructor) {
        let size = self.size(&lc);
        let thumb_size = LogicalSize::new(
            self.style.width,
            (self.thumb.len as f32 / self.len as f32) * size.height,
        );
        let thumb_pt = LogicalPosition::new(
            lc.rect.left,
            lc.rect.top + (self.current as f32 / self.len as f32) * size.height,
        );
        let rect = LogicalRect::from_position_size(lc.rect.left_top(), size);
        let thumb_rect = LogicalRect::from_position_size(thumb_pt, thumb_size);
        let collision_rect = if thumb_rect.size().height >= self.min_collision {
            thumb_rect
        } else {
            LogicalRect::new(
                thumb_rect.left,
                thumb_rect.top - self.min_collision / 2.0,
                thumb_rect.right,
                thumb_rect.bottom + self.min_collision / 2.0,
            )
        };
        result.push(
            &lc,
            LayoutElement::area(
                self,
                WidgetState::None,
                rect,
                &lc.ancestors,
                lc.layer,
                false,
            ),
        );
        result.push(
            &lc,
            LayoutElement::area(
                &self.thumb,
                self.thumb.widget_state,
                thumb_rect,
                &lc.ancestors,
                lc.layer,
                false,
            ),
        );
        result.push(
            &lc,
            LayoutElement::collision(
                &self.thumb,
                self.thumb.widget_state,
                collision_rect,
                &lc.ancestors,
                lc.layer,
            ),
        );
    }
}

impl Widget for ScrollBar<direction::Horizontal> {
    fn input(&mut self, ctx: &Context, input: &Input, events: &mut Events) -> ControlFlow {
        let Some(layout) = ctx.find_layout(self).next() else {
            return ControlFlow::Continue;
        };
        let Some(thumb_layout) = ctx
            .find_layout(&self.thumb)
            .find(|l| matches!(l, LayoutElement::Collision(_)))
        else {
            return ControlFlow::Continue;
        };
        let size = layout.rect().size();
        let thumb_size = thumb_layout.rect().size();
        match input {
            Input::MouseInput(m) => {
                if thumb_layout.rect().contains(&m.mouse_state.position) {
                    if m.button == MouseButton::Left && m.button_state == ButtonState::Pressed {
                        self.d = m.mouse_state.position.x - thumb_layout.rect().left;
                        self.thumb.widget_state = WidgetState::Pressed;
                    } else {
                        self.thumb.widget_state = WidgetState::Hover;
                    }
                } else {
                    self.thumb.widget_state = WidgetState::None;
                }
            }
            Input::CursorMoved(m) => {
                if self.thumb.widget_state == WidgetState::Pressed {
                    let width = size.width - thumb_size.width;
                    let p = m.mouse_state.position.x - layout.rect().left - self.d;
                    let current = (self.len - self.thumb.len) as f32 * p / width;
                    self.current = (current.floor() as usize).min(self.len - self.thumb.len - 1);
                    events.push_message(self, Message::Changed(self.current));
                }
            }
            _ => {}
        }
        ControlFlow::Continue
    }
    fn apply(&mut self, funcs: &mut ApplyFuncs) {
        funcs.apply(self);
    }

    fn size(&self, ctx: &LayoutContext) -> LogicalSize<f32> {
        LogicalSize::new(ctx.rect.right - ctx.rect.left, self.style.width)
    }

    fn size_types(&self) -> SizeTypes {
        SizeTypes::new(SizeType::Flexible, SizeType::Fix)
    }

    fn layout(&self, lc: LayoutContext, result: &mut LayoutConstructor) {
        let size = self.size(&lc);
        let thumb_size = LogicalSize::new(
            (self.thumb.len as f32 / self.len as f32) * size.width,
            self.style.width,
        );
        let thumb_pt = LogicalPosition::new(
            lc.rect.left + (self.current as f32 / self.len as f32) * size.width,
            lc.rect.top,
        );
        let rect = LogicalRect::from_position_size(lc.rect.left_top(), size);
        let thumb_rect = LogicalRect::from_position_size(thumb_pt, thumb_size);
        let collision_rect = if thumb_rect.size().height >= self.min_collision {
            thumb_rect
        } else {
            LogicalRect::new(
                thumb_rect.left - self.min_collision / 2.0,
                thumb_rect.top,
                thumb_rect.right + self.min_collision / 2.0,
                thumb_rect.bottom,
            )
        };
        result.push(
            &lc,
            LayoutElement::area(
                self,
                WidgetState::None,
                rect,
                &lc.ancestors,
                lc.layer,
                false,
            ),
        );
        result.push(
            &lc,
            LayoutElement::area(
                &self.thumb,
                self.thumb.widget_state,
                thumb_rect,
                &lc.ancestors,
                lc.layer,
                false,
            ),
        );
        result.push(
            &lc,
            LayoutElement::collision(
                &self.thumb,
                self.thumb.widget_state,
                collision_rect,
                &lc.ancestors,
                lc.layer,
            ),
        );
    }
}

impl WidgetMessage for ScrollBar<direction::Vertical> {
    type Message = Message;
}

impl WidgetMessage for ScrollBar<direction::Horizontal> {
    type Message = Message;
}

pub type VScrollBar = ScrollBar<direction::Vertical>;
pub type HScrollBar = ScrollBar<direction::Horizontal>;
