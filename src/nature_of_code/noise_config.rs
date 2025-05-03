use nannou::noise::{BasicMulti, MultiFractal};
use nannou_egui::egui;

pub struct NoiseConfig {
    pub octaves: usize,
    pub frequency: f64,
    pub lacunarity: f64,
    pub persistence: f64,
}

impl Default for NoiseConfig {
    fn default() -> Self {
        Self {
            octaves: BasicMulti::DEFAULT_OCTAVES,
            frequency: BasicMulti::DEFAULT_FREQUENCY,
            lacunarity: BasicMulti::DEFAULT_LACUNARITY,
            persistence: BasicMulti::DEFAULT_PERSISTENCE,
        }
    }
}

impl NoiseConfig {
    pub fn create_noise(&self) -> BasicMulti {
        BasicMulti::new()
            .set_octaves(self.octaves)
            .set_frequency(self.frequency)
            .set_lacunarity(self.lacunarity)
            .set_persistence(self.persistence)
    }
    
    pub fn ui(&mut self, ui: &mut egui::Ui) -> bool {
        let mut changed = false;
        
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.add(egui::Slider::new(&mut self.octaves, 1..=10).text("Octaves"))
                    .drag_released()
                    .then(|| changed = true);
                
                ui.add(egui::Slider::new(&mut self.frequency, 0.1..=4.0).text("Frequency"))
                    .drag_released()
                    .then(|| changed = true);
            });
            
            ui.vertical(|ui| {
                ui.add(egui::Slider::new(&mut self.lacunarity, 0.1..=4.0).text("Lacunarity"))
                    .drag_released()
                    .then(|| changed = true);
                
                ui.add(egui::Slider::new(&mut self.persistence, 0.1..=4.0).text("Persistence"))
                    .drag_released()
                    .then(|| changed = true);
            });
        });
        
        changed
    }
}
