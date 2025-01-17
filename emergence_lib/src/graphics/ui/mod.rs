//! Creates the UI from all modules.
//!
use bevy::prelude::*;

use self::hover_panel::HoverPanelPlugin;

mod hover_panel;
mod intent;

/// The different stages of the UI setup.
///
/// Stages are needed here to flush the commands in-between.
#[derive(Debug, Clone, PartialEq, Eq, Hash, StageLabel)]
pub enum UiStage {
    /// Create the initial layout structure.
    LayoutInitialization,

    /// Populate the layout structure with more content.
    LayoutPopulation,
}

/// The font handles for the `FiraSans` font family.
///
/// This is cached in a resource to improve performance.
#[derive(Debug, Resource)]
struct FiraSansFontFamily {
    /// The font to use for regular text.
    regular: Handle<Font>,

    /// The font to use for bold text.
    bold: Handle<Font>,
}

/// Struct to build the UI plugin
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        info!("Building UI plugin...");

        let asset_server = app.world.get_resource::<AssetServer>().unwrap();

        app.insert_resource(FiraSansFontFamily {
            regular: asset_server.load("fonts/FiraSans-Medium.ttf"),
            bold: asset_server.load("fonts/FiraSans-Bold.ttf"),
        })
        .add_startup_stage_before(
            StartupStage::Startup,
            UiStage::LayoutInitialization,
            SystemStage::parallel(),
        )
        .add_startup_stage_after(
            UiStage::LayoutInitialization,
            UiStage::LayoutPopulation,
            SystemStage::parallel(),
        )
        .add_startup_system_to_stage(UiStage::LayoutInitialization, setup_ui)
        .add_plugin(HoverPanelPlugin);
    }
}

/// The UI panel on the left side.
#[derive(Debug, Component)]
pub struct LeftPanel;

/// The UI panel on the right side.
#[derive(Debug, Component)]
pub struct RightPanel;

/// Create the basic UI layout.
fn setup_ui(mut commands: Commands) {
    // UI layout
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // UI panel on the left side
            parent.spawn((
                NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        size: Size::new(Val::Px(200.), Val::Percent(100.)),
                        ..default()
                    },
                    ..default()
                },
                LeftPanel,
            ));

            // UI panel on the right side
            parent.spawn((
                NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        size: Size::new(Val::Px(400.), Val::Percent(100.)),
                        ..default()
                    },
                    ..default()
                },
                RightPanel,
            ));
        });
}
