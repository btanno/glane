use super::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Message {
    Selected(usize),
    OpenedList,
    ClosedList,
}

#[derive(Debug)]
pub struct DropdownBox {
    id: Id,
    widget_state: WidgetState,
    list: ListBox,
    list_visiblity: bool,
    pub list_size: LogicalSize<Option<f32>>,
    pub padding: LogicalRect<f32>,
}

impl DropdownBox {
    #[inline]
    pub fn new() -> Self {
        Self {
            id: Id::new(),
            list: ListBox::new(),
            list_visiblity: false,
            widget_state: WidgetState::None,
            list_size: LogicalSize::new(None, None),
            padding: LogicalRect::new(5.0, 3.0, 5.0, 3.0),
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
                    } else if m.button == MouseButton::Left {
                        match m.button_state {
                            ButtonState::Pressed => {
                                self.widget_state = events.push_state_changed(
                                    self,
                                    WidgetState::Pressed,
                                    self.widget_state,
                                );
                                self.list_visiblity = !self.list_visiblity;
                            }
                            ButtonState::Released => {
                                self.widget_state = events.push_state_changed(
                                    self,
                                    WidgetState::Hover,
                                    self.widget_state,
                                );
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        if self.list_visiblity != list_visiblity {
            if self.list_visiblity {
                events.push_message(self, Message::OpenedList);
            } else {
                events.push_message(self, Message::ClosedList);
            }
        }
        if list_visiblity {
            self.list.input(ctx, input, events);
            let ret = events.iter().enumerate().find_map(|(i, event)| {
                if let Some(msg) = event.message(&self.list) {
                    let list_box::Message::Selected(selected) = msg;
                    Some((i, *selected))
                } else {
                    None
                }
            });
            if let Some((i, selected)) = ret {
                self.list_visiblity = false;
                events.remove(i);
                events.push_message(self, Message::Selected(selected));
                events.push_message(self, Message::ClosedList);
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
            let size = selected.size(ctx);
            (ctx.rect.size().width, size.height).into()
        } else {
            let Some(font) = ctx.ctx.default_font.as_ref() else {
                return (0.0, 0.0).into();
            };
            let size = font.global_bounding_size();
            LogicalSize::new(ctx.rect.size().width, size.height)
        }
    }

    fn layout(&self, lc: LayoutContext, result: &mut LayoutConstructor) {
        let size = self.size(&lc);
        let mut rect = LogicalRect::from_position_size(lc.rect.left_top(), size);
        result.push(
            &lc,
            LayoutElement::area(self, self.widget_state, rect, &lc.ancestors, lc.layer, false),
        );
        rect.left += self.padding.left;
        rect.top += self.padding.top;
        rect.right -= self.padding.right;
        rect.bottom -= self.padding.bottom;
        if let Some(child) = self.list.selected_child() {
            child.layout(lc.next(self, rect, lc.layer, lc.selected), result);
        }
        if self.list_visiblity {
            let size = LogicalSize::new(
                self.list_size.width.unwrap_or(size.width),
                self.list_size.height.unwrap_or(100.0),
            );
            let rect = LogicalRect::from_position_size(lc.rect.left_bottom(), size);
            self.list
                .layout(lc.next(self, rect, lc.layer + 1, lc.selected), result);
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

    #[inline]
    fn push(&mut self, child: impl Widget) {
        let is_empty = self.list.is_empty();
        self.list.push(child);
        if is_empty {
            self.list.select(Some(0));
        }
    }

    #[inline]
    fn erase(&mut self, child: &impl HasId) {
        self.list.erase(child);
    }
}

impl Default for DropdownBox {
    fn default() -> Self {
        Self::new()
    }
}
