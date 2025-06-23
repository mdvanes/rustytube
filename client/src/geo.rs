use egui::Context;
use walkers::{sources::OpenStreetMap, HttpTiles, Map, MapMemory};

pub struct RustyMapState {
    tiles: HttpTiles,
    memory: MapMemory,
}

impl RustyMapState {
    pub fn new(ctx: Context) -> Self {
        Self {
            tiles: walkers::HttpTiles::new(OpenStreetMap, ctx.clone()),
            memory: walkers::MapMemory::default(),
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.add(Map::new(
            Some(&mut self.tiles),
            &mut self.memory,
            walkers::lon_lat(5.114037, 52.0562824),
        ));
    }
}
