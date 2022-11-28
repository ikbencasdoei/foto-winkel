use bevy::prelude::*;

use bevy_egui::{egui, EguiContext};

use self::{
    pencils::{simple::SimplePencil, PencilBoxPlugin},
    plugin::{Tool, ToolIndex},
};

mod pencils;
mod plugin;

#[derive(SystemLabel)]
struct ToolManagerLabel;
pub struct ToolBoxPlugin;
impl Plugin for ToolBoxPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ToolCollection::default())
            .insert_resource(CurrentTool::default())
            .add_event::<ToolEvent>()
            .add_plugin(PencilBoxPlugin)
            .add_system(ui)
            .add_system(sort)
            .add_system(events.label(ToolManagerLabel));
    }
}

#[derive(Resource, Default)]
pub struct ToolCollection {
    tools: Vec<ToolIndex>,
}

pub enum ToolEvent {
    Switched {
        from: Option<ToolIndex>,
        to: Option<ToolIndex>,
    },
}

#[derive(Resource, Deref, DerefMut)]
pub struct CurrentTool(Option<ToolIndex>);

impl Default for CurrentTool {
    fn default() -> Self {
        Self(Some(SimplePencil::get_index()))
    }
}

fn sort(mut tool_collection: ResMut<ToolCollection>) {
    if tool_collection.is_changed() {
        tool_collection
            .tools
            .sort_by(|a, b| a.description.name.partial_cmp(&b.description.name).unwrap());
    }
}

fn events(
    mut current_tool: ResMut<CurrentTool>,
    tool_collection: Res<ToolCollection>,
    mut event_reader: EventReader<ToolEvent>,
) {
    for event in event_reader.iter() {
        match event {
            ToolEvent::Switched { from: _, to } => {
                if let Some(to) = to {
                    for index in &tool_collection.tools {
                        if to.type_id == index.type_id {
                            **current_tool = Some(to.to_owned());
                        }
                    }
                }
            }
        }
    }
}

fn ui(
    current_tool: Res<CurrentTool>,
    mut egui_context: ResMut<EguiContext>,
    tool_collection: Res<ToolCollection>,
    mut event_writer: EventWriter<ToolEvent>,
) {
    egui::Window::new("Tools").show(egui_context.ctx_mut(), |ui| {
        for tool in tool_collection.tools.iter() {
            if ui
                .radio(
                    **current_tool == Some(tool.to_owned()),
                    tool.description.name.to_string(),
                )
                .clicked()
            {
                event_writer.send(ToolEvent::Switched {
                    from: current_tool.to_owned(),
                    to: Some(tool.to_owned()),
                });
            };
        }
    });
}
