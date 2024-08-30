use bspc_rust_lib::bspc::{node::{command::NodeCommand, modifier::NodeModifier, selector::NodeSelector, state::NodeState}, query::QueryCommand};

fn main() {
    if is_focused_fullscreen() {
        NodeCommand::State(NodeState::Tiled).get_response();
    } else {
        NodeCommand::State(NodeState::Fullscreen).get_response();
    }
}

fn is_focused_fullscreen() -> bool {
    let focused_and_fullscreen_nodes = QueryCommand::Nodes(
        Some(NodeSelector::new().add_modifier(NodeModifier::Focused).add_modifier(NodeModifier::Fullscreen)),
        None,
        None).get_response();
    match focused_and_fullscreen_nodes {
        Some(_ids) => true,
        None => false,
    }
}
