mod builtin;
mod bus;
mod component;
mod compositor;
mod container;
mod context;
mod disposable;
mod element;
mod hooks;
mod instance;
mod key;
mod link;
mod manager;
mod memory;
mod platform;
mod reference;
mod render;
mod state;

pub use builtin::Builtin;
pub use bus::Bus;
pub use component::Component;
pub use compositor::{CommandBuffer, Compositor};
pub use container::Container;
pub use context::{Context, ContextProvider, ContextTree};
pub use disposable::Disposable;
pub use element::Element;
pub use hooks::{UseAsync, UseContext, UseEffect, UseReference, UseState};
pub use instance::Instance;
pub use key::Key;
pub use link::{Link, Trigger};
pub use manager::Manager;
pub use memory::Memory;
pub use platform::Platform;
pub use reference::Reference;
pub use render::{render, Renderer};
pub use state::State;

#[macro_export]
macro_rules! with {
    (($($name:ident),*), $($tt:tt)*) => {{
        $(
            let $name = $name.clone();
        )*
        move $($tt)*
    }};
}
