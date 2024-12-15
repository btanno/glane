use super::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Message {
    Changed(f32),
}

#[derive(Debug)]
pub struct Knob {
    id: Id,
}

impl Knob {
    fn new() -> Self {
        Self { id: Id::new() }
    }
}

impl HasId for Knob {
    fn id(&self) -> Id {
        self.id
    }
}

impl Widget for Knob {
    fn input(&mut self, _ctx: &Context, _input: &Input, _events: &mut Events) -> ControlFlow {
        ControlFlow::Continue
    }

    fn apply(&mut self, _funcs: &mut ApplyFuncs) {}

    fn size(&self, ctx: &LayoutContext) -> LogicalSize<f32> {
        ctx.rect.size()
    }

    fn size_types(&self) -> SizeTypes {
        SizeTypes::fix()
    }

    fn layout(&self, _lc: LayoutContext, _result: &mut LayoutConstructor) {}
}

#[derive(Debug)]
pub struct Slider {
    id: Id,
    widget_state: WidgetState,
    knob: Knob,
    pub height: f32,
    current: f32,
    d: f32,
}

impl Slider {
    #[inline]
    pub fn new() -> Self {
        Self {
            id: Id::new(),
            widget_state: WidgetState::None,
            knob: Knob::new(),
            height: 13.0,
            current: 0.0,
            d: 0.0,
        }
    }

    #[inline]
    pub fn set_current(&mut self, value: f32) {
        assert!((0.0..=1.0).contains(&value));
        self.current = value;
    }
}

impl Default for Slider {
    fn default() -> Self {
        Self::new()
    }
}

impl HasId for Slider {
    fn id(&self) -> Id {
        self.id
    }
}

impl Widget for Slider {
    fn input(&mut self, ctx: &Context, input: &Input, events: &mut Events) -> ControlFlow {
        let Some(layout) = ctx.find_layout(self).next() else {
            return ControlFlow::Continue;
        };
        let Some(knob) = ctx.find_layout(&self.knob).next() else {
            return ControlFlow::Continue;
        };
        let size = layout.rect().size();
        match input {
            Input::MouseInput(m) => {
                if knob.rect().contains(&m.mouse_state.position) {
                    if m.button == MouseButton::Left && m.button_state == ButtonState::Pressed {
                        self.d = m.mouse_state.position.x - knob.rect().left;
                        self.widget_state = WidgetState::Pressed;
                    } else {
                        self.widget_state = WidgetState::Hover;
                    }
                } else {
                    self.widget_state = WidgetState::None;
                }
            }
            Input::CursorMoved(m) => {
                if self.widget_state == WidgetState::Pressed {
                    let p = m.mouse_state.position.x - layout.rect().left - self.d;
                    self.current = (p / size.width).clamp(0.0, 1.0);
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
        let size = ctx.rect.size();
        LogicalSize::new(size.width, self.height)
    }

    fn size_types(&self) -> SizeTypes {
        SizeTypes::new(SizeType::Flexible, SizeType::Fix)
    }

    fn layout(&self, lc: LayoutContext, result: &mut LayoutConstructor) {
        let size = self.size(&lc);
        let rect = LogicalRect::from_position_size(lc.rect.left_top(), size);
        let knob_rect = LogicalRect::from_position_size(
            (lc.rect.left + size.width * self.current, lc.rect.top),
            (self.height, self.height),
        );
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
                &self.knob,
                self.widget_state,
                knob_rect,
                &lc.ancestors,
                lc.layer,
                false,
            ),
        );
    }
}

impl WidgetMessage for Slider {
    type Message = Message;
}
