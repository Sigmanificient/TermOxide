//! Common components built on top of TermOxide's rendering primitives.
//!
//! This crate intentionally depends on `termoxide_rendering`; the rendering
//! crate contains only the runtime tree and primitive builders.

extern crate self as termoxide_components;

mod button;
mod progress_bar;

pub use button::{Button, ButtonBuilder, ButtonProps, Msg, button};
pub use progress_bar::{ProgressBar, ProgressBarBuilder, ProgressBarProps, progress_bar};
pub use termoxide_component_macros::RsxComponent;
pub use termoxide_reactive::{Effect, Signal};
use termoxide_rendering::{
    builder::{IntoViewNode, NodeBuilder},
    view_node::{ViewContent, ViewNode},
};

/// Connects a component marker type to its props and builder.
pub trait Component {
    type Props;
    type Builder: NodeBuilder;

    fn render(props: Self::Props) -> Self::Builder;
}

/// Create a reactive side effect from component code.
///
/// Components should create effects from their constructor or mount/setup
/// code, not from an RSX expression. The effect belongs to the active
/// application owner and is disposed with it.
pub fn use_effect<F, T>(f: F) -> Effect
where
    F: Fn(Option<T>) -> T + 'static,
    T: 'static,
{
    Effect::new(f)
}

/// Convert a component or primitive into a view node.
pub fn into_view_node(value: impl IntoViewNode) -> ViewNode { value.into_view_node() }

/// Ensure this crate's component output is a renderable node with children.
pub fn is_component_node(node: &ViewNode) -> bool {
    matches!(node.content, ViewContent::Container { .. }) && !node.children.is_empty()
}
