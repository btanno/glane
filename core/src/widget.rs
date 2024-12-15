use super::*;
use std::any::{Any, TypeId};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ControlFlow {
    Break,
    Continue,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SizeType {
    Fix,
    Flexible,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct SizeTypes {
    pub width: SizeType,
    pub height: SizeType,
}

impl SizeTypes {
    #[inline]
    pub const fn new(width: SizeType, height: SizeType) -> Self {
        Self { width, height }
    }

    #[inline]
    pub const fn fix() -> Self {
        Self {
            width: SizeType::Fix,
            height: SizeType::Fix,
        }
    }

    #[inline]
    pub const fn flexible() -> Self {
        Self {
            width: SizeType::Flexible,
            height: SizeType::Flexible,
        }
    }
}

pub trait Widget: Any + HasId {
    fn input(&mut self, ctx: &Context, input: &Input, events: &mut Events) -> ControlFlow;
    fn apply(&mut self, funcs: &mut ApplyFuncs);
    fn size(&self, ctx: &LayoutContext) -> LogicalSize<f32>;
    fn size_types(&self) -> SizeTypes;
    fn layout(&self, lc: LayoutContext, result: &mut LayoutConstructor);
}

pub trait WidgetMessage: Widget {
    type Message;
}

#[derive(Copy, PartialEq, Eq, Debug)]
pub struct Handle<T>
where
    T: Widget,
{
    id: Id,
    _t: std::marker::PhantomData<T>,
}

impl<T> Handle<T>
where
    T: Widget,
{
    #[inline]
    pub fn new(widget: &T) -> Self {
        Self {
            id: widget.id(),
            _t: std::marker::PhantomData,
        }
    }

    #[inline]
    pub fn id(&self) -> Id {
        self.id
    }
}

impl<T> Clone for Handle<T>
where
    T: Widget,
{
    #[inline]
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            _t: std::marker::PhantomData,
        }
    }
}

impl<T> HasId for Handle<T>
where
    T: Widget,
{
    #[inline]
    fn id(&self) -> Id {
        self.id
    }
}

impl<T> From<&T> for Handle<T>
where
    T: Widget,
{
    fn from(value: &T) -> Self {
        Self::new(value)
    }
}

impl<T> From<&Handle<T>> for Handle<T>
where
    T: Widget,
{
    fn from(value: &Handle<T>) -> Self {
        value.clone()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct AnyHandle {
    id: Id,
    t: TypeId,
}

impl AnyHandle {
    #[inline]
    pub fn new<T>(widget: &T) -> Self
    where
        T: Widget,
    {
        Self {
            id: widget.id(),
            t: TypeId::of::<T>(),
        }
    }

    #[inline]
    pub fn id(&self) -> Id {
        self.id
    }

    #[inline]
    pub fn type_id(&self) -> TypeId {
        self.t
    }

    #[inline]
    pub fn is<T: Widget>(&self, other: &T) -> bool {
        self.id == other.id()
    }

    #[inline]
    pub fn downcast<T>(&self) -> Option<Handle<T>>
    where
        T: Widget,
    {
        (self.t == TypeId::of::<T>()).then_some(Handle {
            id: self.id,
            _t: std::marker::PhantomData,
        })
    }
}

impl HasId for AnyHandle {
    #[inline]
    fn id(&self) -> Id {
        self.id
    }
}

impl<T> PartialEq<Handle<T>> for AnyHandle
where
    T: Widget,
{
    #[inline]
    fn eq(&self, other: &Handle<T>) -> bool {
        self.id == other.id
    }
}

impl<T> PartialEq<AnyHandle> for Handle<T>
where
    T: Widget,
{
    #[inline]
    fn eq(&self, other: &AnyHandle) -> bool {
        self.id == other.id
    }
}

impl<T> PartialEq<T> for AnyHandle
where
    T: Widget,
{
    #[inline]
    fn eq(&self, other: &T) -> bool {
        self.id == other.id()
    }
}

impl<T> From<Handle<T>> for AnyHandle
where
    T: Widget,
{
    #[inline]
    fn from(value: Handle<T>) -> Self {
        AnyHandle {
            id: value.id,
            t: TypeId::of::<T>(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum WidgetState {
    None,
    Hover,
    Pressed,
}

impl WidgetState {
    #[inline]
    pub fn current(rect: &LogicalRect<f32>, mouse_state: &MouseState) -> Self {
        if rect.contains(&mouse_state.position) {
            if mouse_state.buttons.contains(MouseButton::Left) {
                Self::Pressed
            } else {
                Self::Hover
            }
        } else {
            Self::None
        }
    }
}

pub trait HasChildren {
    fn len(&self) -> usize;
    fn push(&mut self, child: impl Widget);
    fn erase(&mut self, child: &impl HasId);

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl std::fmt::Debug for Box<dyn Widget> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Widget({:?})", self.id())
    }
}
