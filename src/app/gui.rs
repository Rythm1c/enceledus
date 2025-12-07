/// GUI management for egui integration
/// Handles UI for scene manipulation, model loading, physics parameters, etc

use crate::src::core::scene::{Scene, EntityId};
use crate::src::loader::AssetManager;

/// State for the GUI
pub struct GuiState {
    pub show_demo_window: bool,
    pub show_scene_inspector: bool,
    pub show_asset_browser: bool,
    pub show_physics_settings: bool,
    
    // Scene inspector state
    pub selected_entity: Option<EntityId>,
    pub entity_name_buffer: String,
    pub entity_transform_buffer: [f32; 3], // position
    
    // Asset browser state
    pub selected_model: String,
    pub model_name_input: String,
    
    // Physics settings state
    pub gravity: [f32; 3],
    pub physics_enabled: bool,
}

impl Default for GuiState {
    fn default() -> Self {
        Self {
            show_demo_window: false,
            show_scene_inspector: true,
            show_asset_browser: true,
            show_physics_settings: false,
            selected_entity: None,
            entity_name_buffer: String::new(),
            entity_transform_buffer: [0.0, 0.0, 0.0],
            selected_model: String::new(),
            model_name_input: String::new(),
            gravity: [0.0, -9.81, 0.0],
            physics_enabled: false,
        }
    }
}

impl GuiState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Handle model creation from GUI input
    pub fn create_model_from_gui(&self, scene: &mut Scene, asset_manager: &AssetManager, gpu_model_id: usize) {
        if !self.model_name_input.is_empty() && asset_manager.has_model(&self.selected_model) {
            let entity_id = scene.create_entity(self.model_name_input.clone(), gpu_model_id);
            println!("Created entity: {} ({})", self.model_name_input, entity_id.0);
        }
    }

    /// Update selected entity's transform
    pub fn update_selected_entity_transform(&self, scene: &mut Scene) {
        if let Some(id) = self.selected_entity {
            if let Some(entity) = scene.get_entity_mut(id) {
                entity.transform.position = self.entity_transform_buffer;
            }
        }
    }
}

/// UI system that renders with egui
/// This would be called each frame in the main loop
pub struct GuiSystem {
    pub state: GuiState,
}

impl GuiSystem {
    pub fn new() -> Self {
        Self {
            state: GuiState::new(),
        }
    }

    /// Render the GUI (called each frame)
    /// This is a placeholder - actual egui rendering happens in main_loop.rs
    pub fn render(&mut self, scene: &Scene, asset_manager: &AssetManager) {
        // This would be called with egui context in the actual rendering loop
        // Example:
        // egui::Window::new("Scene Inspector").show(ctx, |ui| {
        //     self.draw_scene_inspector(ui, scene);
        // });
    }

    /// Draw scene inspector window
    pub fn draw_scene_inspector(&mut self, scene: &Scene) {
        let renderable_entities = scene.get_renderable_entities();
        println!("Scene: {} | Entities: {}", scene.name, renderable_entities.len());
        
        for entity in renderable_entities {
            println!("  - {} (ID: {})", entity.name, entity.id.0);
            if self.state.selected_entity == Some(entity.id) {
                self.state.entity_name_buffer = entity.name.clone();
                self.state.entity_transform_buffer = entity.transform.position;
            }
        }
    }

    /// Draw asset browser window
    pub fn draw_asset_browser(&mut self, asset_manager: &AssetManager) {
        let models = asset_manager.list_models();
        println!("Available Models: {}", models.len());
        for model_name in models {
            println!("  - {}", model_name);
        }
    }

    /// Draw physics settings window
    pub fn draw_physics_settings(&mut self) {
        println!("Physics Settings:");
        println!("  Enabled: {}", self.state.physics_enabled);
        println!("  Gravity: {:?}", self.state.gravity);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gui_state_defaults() {
        let gui = GuiState::new();
        assert!(gui.show_scene_inspector);
        assert!(!gui.physics_enabled);
    }
}
