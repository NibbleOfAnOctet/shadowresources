use serde::Serialize;

#[derive(Serialize)]
struct Layer {
    #[serde(rename(serialize = "type"), default = "layer")]
    layer_type: String,
}

#[derive(Serialize)]
pub struct Tileset {
    #[serde(rename(serialize = "type"), default = "tileset")]
    layer_type: String,
}

#[derive(Serialize)]
pub struct TiledMap {
    tiledversion: String,
    #[serde(rename(serialize = "type"))]
    file_type: String,
    data: Vec<u32>,
    width: u32,
    height: u32,
    tileheight: u32,
    tilewidth: u32,

    orientation: String,
    nextobjectid: u32,

    layers: Vec<u32>,
    properties: Vec<u32>,
    tilesets: Vec<u32>,
}

impl TiledMap {
    pub fn new(width: u32, height: u32, tileheight: u32, tilewidth: u32) -> Self {
        Self {
            tiledversion: "1.11".to_string(),
            file_type: "map".to_string(),
            data: Vec::new(),
            width: 32,
            height: 32,
            tileheight,
            tilewidth,
            orientation: "orthogonal".to_string(),
            nextobjectid: 0,
            layers: Vec::new(),
            properties: Vec::new(),
            tilesets: Vec::new(),
        }
    }
}
