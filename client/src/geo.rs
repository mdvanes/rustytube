use egui::Context;
use walkers::{
    extras::{GroupedPlaces, LabeledSymbol, LabeledSymbolStyle},
    lon_lat,
    sources::OpenStreetMap,
    HttpTiles, Map, MapMemory, Plugin,
};

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
        let mut map = Map::new(
            Some(&mut self.tiles),
            &mut self.memory,
            lon_lat(5.114037, 52.0562824),
        );

        pub fn markers() -> impl Plugin {
            GroupedPlaces::new(vec![LabeledSymbol {
                // use walkers::{lon_lat, Position};
                // pub fn wroclaw_glowny() -> Position {
                //     lon_lat(17.03664, 51.09916)
                // }
                // position: places::wroclaw_glowny(),
                position: lon_lat(5.1115, 52.0578),
                label: "HQ".to_owned(),
                symbol: '🏢',
                style: LabeledSymbolStyle::default(),
            }])
        }

        map = map.with_plugin(markers());
        ui.add(map);
    }
}
