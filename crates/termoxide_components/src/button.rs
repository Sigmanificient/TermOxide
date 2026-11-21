use ratatui::layout::Rect;
use termoxide_event::event::{Event, MouseEventKind};
use termoxide_rendering::{
    builder::{CommonFields, NodeBuilder},
    view_node::ViewNode,
};

use crate::RsxComponent;

#[derive(RsxComponent)]
#[rsx(component = Button, builder = ButtonBuilder, constructor = button)]
pub struct ButtonProps {
    pub label: String,
    pub msg: Msg,
}

/// Message associated with a button activation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Msg {
    Run,
    CustomEvent,
}

/// Builder for a text button component.
pub struct ButtonBuilder {
    common: CommonFields,
    label: String,
    msg: Msg,
    area: Rect,
    on_click: Option<Box<dyn Fn() + Send + Sync>>,
}

impl ButtonBuilder {
    fn new(label: impl Into<String>, msg: Msg) -> Self {
        Self {
            common: CommonFields::new(),
            label: label.into(),
            msg,
            area: Rect::default(),
            on_click: None,
        }
    }

    pub fn area(mut self, area: Rect) -> Self {
        self.area = area;
        self
    }

    /// Return the activation message carried by this component.
    pub fn message(&self) -> &Msg { &self.msg }

    pub fn on_click<F: Fn() + Send + Sync + 'static>(mut self, callback: F) -> Self {
        self.on_click = Some(Box::new(callback));
        self
    }
}

impl NodeBuilder for ButtonBuilder {
    fn common(&mut self) -> &mut CommonFields { &mut self.common }

    fn build(self) -> ViewNode {
        let label = termoxide_rendering::builder::text(self.label)
            .area(self.area)
            .style(self.common.style)
            .build();
        let mut node = ViewNode::container(self.area, vec![label]);
        node.id = self.common.id;
        if let Some(callback) = self.on_click {
            node = node.with_event_handler(Box::new(move |event: &Event| {
                if matches!(event, Event::Mouse(mouse) if matches!(mouse.kind, MouseEventKind::Down(_))) {
                    callback();
                }
            }));
        } else if let Some(handler) = self.common.event_handler {
            node = node.with_event_handler(handler);
        }
        node
    }
}

/// Construct a button component.
pub fn button(label: impl Into<String>, msg: Msg) -> ButtonBuilder { ButtonBuilder::new(label, msg) }
