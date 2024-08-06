use super::*;
use std::any::Any;
use std::cell::Ref;
use std::sync::Arc;

#[derive(Debug)]
pub struct Context {
    pub viewport: LogicalSize<f32>,
    pub layout: Arc<Layout>,
    pub default_font: Option<Font>,
    pub prev_input: Option<Input>,
    focus: Option<AnyHandle>,
}

impl Context {
    #[inline]
    pub fn find_layout<'a>(
        &'a self,
        widget: &dyn Widget,
    ) -> impl Iterator<Item = Ref<'a, LayoutElement>> {
        let id = widget.id();
        self.layout.iter().filter(move |l| l.handle().id() == id)
    }

    #[inline]
    pub fn has_focus<T: Widget>(&self, widget: &T) -> bool {
        self.focus
            .map_or(false, |focus| focus == Handle::new(widget))
    }
}

struct ApplyElement {
    handle: AnyHandle,
    f: Option<Box<dyn FnOnce(&mut dyn Any)>>,
}

pub struct ApplyFuncs(Vec<ApplyElement>);

impl ApplyFuncs {
    pub fn new() -> Self {
        Self(vec![])
    }

    #[inline]
    pub fn push<T, F>(&mut self, handle: &Handle<T>, f: F)
    where
        T: Widget,
        F: FnOnce(&mut T) + 'static,
    {
        let handle = handle.clone();
        self.0.push(ApplyElement {
            handle: handle.into(),
            f: Some(Box::new(|widget| f(widget.downcast_mut::<T>().unwrap()))),
        });
    }

    pub fn apply<T>(&mut self, widget: &mut T)
    where
        T: Widget,
    {
        let id = widget.id();
        self.0
            .iter_mut()
            .filter(move |elem| elem.handle.id() == id)
            .for_each(|elem| {
                let f = elem.f.take().unwrap();
                f(widget);
            });
    }
}

pub struct Scene {
    ctx: Context,
    root: Box<dyn Widget>,
    prev_input: Option<Input>,
    apply_funcs: ApplyFuncs,
}

impl Scene {
    #[inline]
    pub fn new<T: Widget>(root: T) -> (Self, Handle<T>) {
        let root = Box::new(root);
        let handle = Handle::new(root.as_ref());
        (
            Self {
                ctx: Context {
                    viewport: LogicalSize::new(1024.0, 768.0),
                    focus: None,
                    layout: Arc::new(Layout::empty()),
                    default_font: FontFace::from_os_default()
                        .ok()
                        .map(|face| Font::new(&face, 14.0)),
                    prev_input: None,
                },
                root,
                prev_input: None,
                apply_funcs: ApplyFuncs::new(),
            },
            handle,
        )
    }

    #[inline]
    pub fn set_viewport(&mut self, size: LogicalSize<f32>) {
        self.ctx.viewport = size;
    }

    #[inline]
    pub fn default_font(&self) -> Option<&Font> {
        self.ctx.default_font.as_ref()
    }

    pub fn input(&mut self, input: Input) -> Events {
        if !self.apply_funcs.0.is_empty() {
            self.root.apply(&mut self.apply_funcs);
            self.apply_funcs.0.clear();
        }
        self.ctx.prev_input = self.prev_input.take();
        let mut events = Events::new();
        self.root.input(&self.ctx, &input, &mut events);
        if let Input::MouseInput(m) = &input {
            if m.button_state == ButtonState::Pressed {
                self.ctx.focus = events
                    .iter()
                    .find(|event| event.is_set_focus())
                    .map(|event| event.handle());
            }
        }
        self.prev_input = Some(input);
        events
    }

    #[inline]
    pub fn apply<T, F>(&mut self, handle: &Handle<T>, f: F)
    where
        T: Widget,
        F: FnOnce(&mut T) + 'static,
    {
        self.apply_funcs.push(handle, f);
    }

    pub fn layout(&mut self) -> Arc<Layout> {
        if !self.apply_funcs.0.is_empty() {
            self.root.apply(&mut self.apply_funcs);
            self.apply_funcs.0.clear();
        }
        let mut layout = LayoutConstructor::new();
        self.root.layout(LayoutContext::new(&self.ctx), &mut layout);
        self.ctx.layout = Arc::new(Layout::new(&self.ctx, layout));
        self.ctx.layout.clone()
    }
}
