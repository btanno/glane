use super::*;

#[derive(Debug)]
pub struct Style {
    pub spacing: f32,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Message {
    Clicked(bool),
}

#[derive(Debug)]
pub struct Check {
    id: Id,
}

impl Check {
    #[inline]
    pub fn new() -> Self {
        Self { id: Id::new() }
    }
}

impl HasId for Check {
    fn id(&self) -> Id {
        self.id
    }
}

impl Widget for Check {
    fn input(&mut self, _ctx: &Context, _input: &Input, _events: &mut Events) -> ControlFlow {
        ControlFlow::Continue
    }

    fn apply(&mut self, funcs: &mut ApplyFuncs) {
        funcs.apply(self);
    }

    fn size(&self, _ctx: &LayoutContext) -> LogicalSize<f32> {
        LogicalSize::new(0.0, 0.0)
    }

    fn layout(&self, _lc: LayoutContext, _result: &mut LayoutConstructor) {}
}

#[derive(Debug)]
pub struct CheckBox {
    id: Id,
    widget_state: WidgetState,
    label: Label,
    checked: bool,
    check: Check,
    pub style: Style,
}

impl CheckBox {
    #[inline]
    pub fn new(text: impl Into<String>, checked: bool) -> Self {
        Self {
            id: Id::new(),
            widget_state: WidgetState::None,
            label: Label::new(text),
            checked,
            style: Style { spacing: 10.0 },
            check: Check::new(),
        }
    }

    #[inline]
    pub fn is_checked(&self) -> bool {
        self.checked
    }

    #[inline]
    pub fn set_check(&mut self, checked: bool) {
        self.checked = checked;
    }

    #[inline]
    pub fn set_text(&mut self, text: impl Into<String>) {
        self.label.text = text.into();
    }
}

impl HasId for CheckBox {
    fn id(&self) -> Id {
        self.id
    }
}

impl Widget for CheckBox {
    fn input(&mut self, ctx: &Context, input: &Input, events: &mut Events) -> ControlFlow {
        let Some(layout) = ctx.find_layout(self).nth(0) else {
            return ControlFlow::Continue;
        };
        let Some(text_layout) = ctx.find_layout(&self.label).nth(0) else {
            return ControlFlow::Continue;
        };
        let rect = layout.rect();
        let size = layout.rect().size();
        let text_rect = text_layout.rect();
        let text_size = text_rect.size();
        let rect = LogicalRect::from_position_size(
            rect.left_top(),
            (
                size.width + self.style.spacing + text_size.width,
                size.height,
            ),
        );
        match input {
            Input::MouseInput(m) => {
                let state = WidgetState::current(&rect, &m.mouse_state);
                let clicked = m.button == MouseButton::Left
                    && m.button_state == ButtonState::Released
                    && state == WidgetState::Hover;
                if clicked {
                    self.checked = !self.checked;
                    events.push_message(self, Message::Clicked(self.checked));
                }
                if state != self.widget_state {
                    self.widget_state = events.push_state_changed(self, state, self.widget_state);
                }
            }
            Input::CursorMoved(m) => {
                let state = WidgetState::current(layout.rect(), &m.mouse_state);
                if state != self.widget_state {
                    self.widget_state = events.push_state_changed(self, state, self.widget_state);
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
        let text_size = self.label.size(ctx);
        let height = text_size.height;
        LogicalSize::new(
            height + text_size.width + self.style.spacing,
            text_size.height,
        )
    }

    fn layout(&self, lc: LayoutContext, result: &mut LayoutConstructor) {
        let text_size = self.label.size(&lc);
        let rect = LogicalRect::from_position_size(
            lc.rect.left_top(),
            (text_size.height, text_size.height),
        );
        result.push(
            &lc,
            LayoutElement::area(self, self.widget_state, rect, false),
        );
        if self.checked {
            result.push(
                &lc,
                LayoutElement::area(&self.check, self.widget_state, rect, false),
            );
        }
        let rect = LogicalRect::new(
            rect.left + text_size.height + self.style.spacing,
            rect.top,
            rect.right + self.style.spacing + text_size.width,
            rect.bottom,
        );
        self.label.layout(lc.next(rect, lc.layer, false), result);
    }
}

impl WidgetMessage for CheckBox {
    type Message = Message;
}
