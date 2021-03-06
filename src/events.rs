use crate::{Hover, PickableMesh, RayCastPluginState, Selection};
use bevy::prelude::*;

/// An event that triggers when the hover state of a [Selection] enabled [PickableMesh] changes.
#[derive(Debug)]
pub enum SelectionEvent {
    JustSelected(Entity),
    JustDeselected(Entity),
}

/// An event that triggers when the hover state of a [Hover] enabled [PickableMesh] changes.
#[derive(Debug)]
pub enum HoverEvent {
    JustEntered(Entity),
    JustLeft(Entity),
}

/// An event that wraps selection and hover events
#[derive(Debug)]
pub enum PickingEvent {
    Selection(SelectionEvent),
    Hover(HoverEvent),
}

/// Looks for changes in selection or hover state, and sends the appropriate events
#[allow(clippy::type_complexity)]
pub fn mesh_events_system(
    state: Res<RayCastPluginState>,
    mut picking_events: EventWriter<PickingEvent>,
    hover_query: Query<
        (Entity, &Hover, ChangeTrackers<Hover>),
        (Changed<Hover>, With<PickableMesh>),
    >,
    selection_query: Query<
        (Entity, &Selection, ChangeTrackers<Selection>),
        (Changed<Selection>, With<PickableMesh>),
    >,
) {
    if !state.enabled {
        return;
    }
    for (entity, hover, hover_change) in hover_query.iter() {
        if hover_change.is_added() {
            continue; // Avoid a false change detection when a component is added.
        }
        if hover.hovered() {
            picking_events.send(PickingEvent::Hover(HoverEvent::JustEntered(entity)));
        } else {
            picking_events.send(PickingEvent::Hover(HoverEvent::JustLeft(entity)));
        }
    }
    for (entity, selection, selection_change) in selection_query.iter() {
        if selection_change.is_added() {
            continue; // Avoid a false change detection when a component is added.
        }
        if selection.selected() {
            picking_events.send(PickingEvent::Selection(SelectionEvent::JustSelected(
                entity,
            )));
        } else {
            picking_events.send(PickingEvent::Selection(SelectionEvent::JustDeselected(
                entity,
            )));
        }
    }
}

/// Listens for [HoverEvent] and [SelectionEvent] events and prints them
pub fn event_debug_system(state: Res<RayCastPluginState>, mut events: EventReader<PickingEvent>) {
    if !state.enabled {
        return;
    }
    for event in events.iter() {
        info!("{:?}", event);
    }
}
