use super::*;

pub enum Message {}

pub struct DropdownBox {
    id: Id,
    list: ListBox,
    list_visiblity: bool,
    widget_state: WidgetState,
}

impl DropdownBox {
    #[inline]
    pub fn new() -> Self {
        Self {
            id: Id::new(),
            list: ListBox::new(),
            list_visiblity: false,
            widget_state: WidgetState::None,
        }
    }

    #[inline]
    pub fn clear(&mut self) {
        self.list_visiblity = false;
        self.list.clear();
    }
}

impl HasId for DropdownBox {
    fn id(&self) -> Id {
        self.id
    }
}

impl Widget for DropdownBox {
    fn input(&mut self, ctx: &Context, input: &Input, events: &mut Events) -> ControlFlow {
        let list_visiblity = self.list_visiblity;
        if let Some(layout) = ctx.find_layout(self).next() {
            let rect = layout.rect();
            match input {
                Input::CursorMoved(m) => {
                    if rect.is_crossing(&m.mouse_state.position) {
                        self.widget_state =
                            events.push_state_changed(self, WidgetState::Hover, self.widget_state);
                    } else {
                        self.widget_state =
                            events.push_state_changed(self, WidgetState::None, self.widget_state);
                    }
                }
                Input::MouseInput(m) => {
                    if !rect.is_crossing(&m.mouse_state.position) {
                        if let Some(layout) = ctx.find_layout(&self.list).next() {
                            if !layout.rect().is_crossing(&m.mouse_state.position) {
                                self.list_visiblity = false;
                            }
                        }
                    } else {
                        if m.button == MouseButton::Left {
                            match m.button_state {
                                ButtonState::Pressed => {
                                    self.widget_state = events.push_state_changed(
                                        self,
                                        WidgetState::Pressed,
                                        self.widget_state,
                                    );
                                }
                                ButtonState::Released => {
                                    self.widget_state = events.push_state_changed(
                                        self,
                                        WidgetState::Hover,
                                        self.widget_state,
                                    );
                                    self.list_visiblity = !self.list_visiblity;
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        if list_visiblity {
            self.list.input(ctx, input, events);
            let selected = events
                .iter()
                .rev()
                .find(|event| {
                    if let Some(msg) = event.message(&Handle::new(&self.list)) {
                        matches!(msg, list_box::Message::Selected(_))
                    } else {
                        false
                    }
                })
                .is_some();
            if selected {
                self.list_visiblity = false;
            }
            ControlFlow::Break
        } else {
            ControlFlow::Continue
        }
    }

    fn apply(&mut self, funcs: &mut ApplyFuncs) {
        funcs.apply(self);
    }

    fn size(&self, ctx: &LayoutContext) -> LogicalSize<f32> {
        if let Some(selected) = self.list.selected_child() {
            let size = selected.size(&ctx);
            (ctx.rect.size().width, size.height).into()
        } else {
            let Some(font) = ctx.ctx.default_font.as_ref() else {
                return (0.0, 0.0).into();
            };
            let size = font.global_bounding_size();
            (ctx.rect.size().width, size.height).into()
        }
    }

    fn layout(&self, lc: LayoutContext, result: &mut LayoutConstructor) {
        let size = self.size(&lc);
        let rect = LogicalRect::from_position_size(lc.rect.left_top(), size);
        result.push(&lc, LayoutElement::area(self, self.widget_state, rect));
        if let Some(child) = self.list.selected_child() {
            child.layout(lc.next(rect, lc.layer), result);
        }
        if self.list_visiblity {
            let size = LogicalSize::new(size.width, 100.0);
            let rect = LogicalRect::from_position_size(lc.rect.left_bottom(), size);
            self.list.layout(lc.next(rect, lc.layer + 1), result);
        }
    }
}

impl WidgetMessage for DropdownBox {
    type Message = Message;
}

impl HasChildren for DropdownBox {
    #[inline]
    fn len(&self) -> usize {
        self.list.len()
    }

    fn push(&mut self, child: impl Widget) {
        let is_empty = self.list.is_empty();
        self.list.push(child);
        if is_empty {
            self.list.select(Some(0));
        }
    }

    fn erase(&mut self, child: impl HasId) {
        self.list.erase(child);
    }
}
