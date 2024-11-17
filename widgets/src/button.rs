use super::*;

#[derive(Debug)]
pub struct Style {
    pub font: Option<Font>,
    pub padding: LogicalRect<f32>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Message {
    Clicked,
}

#[derive(Debug)]
pub struct Button {
    id: Id,
    widget_state: WidgetState,
    pub text: String,
    pub style: Style,
}

impl Button {
    #[inline]
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            id: Id::new(),
            widget_state: WidgetState::None,
            text: text.into(),
            style: Style {
                font: None,
                padding: LogicalRect::new(7.0, 3.0, 7.0, 3.0),
            },
        }
    }
}

impl HasId for Button {
    fn id(&self) -> Id {
        self.id
    }
}

impl Widget for Button {
    fn input(&mut self, ctx: &Context, input: &Input, events: &mut Events) -> ControlFlow {
        let Some(layout) = ctx.find_layout(self).nth(0) else {
            return ControlFlow::Continue;
        };
        match input {
            Input::MouseInput(m) => {
                let state = WidgetState::current(layout.rect(), &m.mouse_state);
                let clicked = m.button == MouseButton::Left
                    && m.button_state == ButtonState::Released
                    && state == WidgetState::Hover;
                if clicked {
                    events.push_message(self, Message::Clicked);
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
            Input::CursorLeft(m) => {
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
        let font = self
            .style
            .font
            .as_ref()
            .unwrap_or_else(|| ctx.ctx.default_font.as_ref().unwrap());
        let rect = bounding_box_with_str(ctx.ctx, font, &self.text);
        LogicalSize::new(
            rect.right + self.style.padding.left + self.style.padding.right,
            rect.bottom + self.style.padding.top + self.style.padding.bottom,
        )
    }

    fn layout(&self, lc: LayoutContext, result: &mut LayoutConstructor) {
        let size = self.size(&lc);
        let rect = LogicalRect::from_position_size(lc.rect.left_top(), size);
        result.push(
            &lc,
            LayoutElement::area(
                self,
                self.widget_state,
                rect,
                &lc.ancestors,
                lc.layer,
                false,
            ),
        );
        let rect = LogicalRect::new(
            rect.left + self.style.padding.left,
            rect.top + self.style.padding.top,
            rect.right - self.style.padding.right,
            rect.bottom - self.style.padding.bottom,
        );
        result.push(
            &lc,
            LayoutElement::text(
                self,
                self.widget_state,
                rect,
                &lc.ancestors,
                self.text.clone(),
                lc.layer,
                false,
            ),
        );
    }
}

impl WidgetMessage for Button {
    type Message = Message;
}
