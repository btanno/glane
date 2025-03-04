use super::*;

#[derive(Default, Debug)]
pub struct Style {
    font: Option<Font>,
}

#[derive(Debug)]
pub struct Text {
    id: Id,
    style: Style,
    pub text: String,
}

impl Text {
    #[inline]
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            id: Id::new(),
            style: Default::default(),
            text: text.into(),
        }
    }
}

impl HasId for Text {
    fn id(&self) -> Id {
        self.id
    }
}

impl Widget for Text {
    fn input(&mut self, _ctx: &Context, _input: &Input, _events: &mut Events) -> ControlFlow {
        ControlFlow::Continue
    }

    fn apply(&mut self, funcs: &mut ApplyFuncs) {
        funcs.apply(self);
    }

    fn size(&self, lc: &LayoutContext) -> LogicalSize<f32> {
        let font = self
            .style
            .font
            .as_ref()
            .unwrap_or_else(|| lc.ctx.default_font.as_ref().unwrap());
        let shape = bounding_box_with_str(lc.ctx, font, &self.text);
        LogicalSize::new(shape.right - shape.left, shape.bottom - shape.top)
    }

    fn size_types(&self) -> SizeTypes {
        SizeTypes::fix()
    }

    fn layout(&self, lc: LayoutContext, result: &mut LayoutConstructor) {
        let size = self.size(&lc);
        result.push(
            &lc,
            LayoutElement::text(
                self,
                WidgetState::None,
                LogicalRect::from_position_size(lc.rect.left_top(), size),
                &lc.ancestors,
                self.style
                    .font
                    .as_ref()
                    .or(lc.ctx.default_font.as_ref())
                    .cloned(),
                self.text.clone(),
                lc.layer,
                false,
            ),
        );
    }
}
