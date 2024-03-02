use super::*;
use std::any::{Any, TypeId};

pub trait Widget: Any + HasId {
    fn input(&mut self, ctx: &Context, input: &Input, events: &mut Vec<Event>);
    fn apply(&mut self, funcs: &mut ApplyFuncs);
    fn size(&self, ctx: &LayoutContext) -> LogicalSize<f32>;
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
        (self.t == TypeId::of::<T>()).then(|| Handle {
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
    fn push(&mut self, child: impl Widget);
    fn erase(&mut self, child: impl HasId);
}

#[inline]
pub fn push_child<T, U>(scene: &mut Scene, parent: &Handle<T>, child: U) -> Handle<U>
where
    T: Widget + HasChildren,
    U: Widget,
{
    let handle = Handle::new(&child);
    scene.apply(parent, move |r| r.push(child));
    handle
}

#[inline]
pub fn erase_child<T, U>(scene: &mut Scene, parent: &Handle<T>, child: Handle<U>)
where
    T: Widget + HasChildren,
    U: Widget,
{
    scene.apply(parent, move |r| r.erase(child));
}