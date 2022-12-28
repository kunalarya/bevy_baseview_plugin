use bevy::app::{PluginGroup, PluginGroupBuilder};
pub struct DefaultBaseviewPlugins;

impl PluginGroup for DefaultBaseviewPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        // Disable log plugin as it sets global state and will panic if you re-open the app.
        group.add(bevy::core::CorePlugin::default());
        group.add(bevy::time::TimePlugin::default());
        group.add(bevy::transform::TransformPlugin::default());
        group.add(bevy::hierarchy::HierarchyPlugin::default());
        group.add(bevy::diagnostic::DiagnosticsPlugin::default());
        group.add(bevy::input::InputPlugin::default());
        group.add(bevy::window::WindowPlugin::default());
        group.add(bevy::asset::AssetPlugin::default());
        //group.add(bevy::asset::debug_asset_server::DebugAssetServerPlugin::default());
        group.add(bevy::scene::ScenePlugin::default());
        group.add(crate::BaseviewPlugin::default());
        group.add(bevy::render::RenderPlugin::default());
        group.add(bevy::core_pipeline::CorePipelinePlugin::default());
        group.add(bevy::sprite::SpritePlugin::default());
        group.add(bevy::text::TextPlugin::default());
        group.add(bevy::ui::UiPlugin::default());
        group.add(bevy::pbr::PbrPlugin::default());

        // NOTE: Load this after renderer initialization so that it knows about the supported
        // compressed texture formats
        group.add(bevy::gltf::GltfPlugin::default());
        group.add(bevy::animation::AnimationPlugin::default());
    }
}
