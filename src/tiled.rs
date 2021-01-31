
use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
    utils::BoxedFuture,
};
use roxmltree::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TiledError {
    #[error("No width provided for map")]
    NoMapWidth,
    #[error("No height provided for map")]
    NoMapHeight,
    #[error("No width provided for layer")]
    NoLayerWidth,
    #[error("No height provided for layer")]
    NoLayerHeight,
    #[error("No data provided for layer")]
    NoLayerData,
    #[error("No source provided for tileset image")]
    NoImageSource,
}

#[derive(Debug, TypeUuid,Clone,PartialEq, Eq, PartialOrd, Ord)]
#[uuid = "e6a01dcf-5e85-4d29-9d51-44763edcc642"]
pub struct Map {
    pub width: usize,
    pub height: usize,
    pub layers: Vec<Layer>,
}


impl Default for Map {
    fn default() -> Self {
        Self {width:0,height:0,layers:vec![]}
    }
}

impl Map {
    fn load<'a>(data: &'a [u8]) -> Result<Map, anyhow::Error> {
        let doc=Document::parse(std::str::from_utf8(data)?)?;
        let mut map = Map::default();
        let e=doc.root().first_element_child().unwrap();
        map.width=e.attribute("width").ok_or(TiledError::NoMapWidth)?.parse()?;
        map.height=e.attribute("height").ok_or(TiledError::NoMapHeight)?.parse()?;
        for d in e.children() {
            if d.tag_name().name() == "layer"{
                let mut layer=Layer::default();
                layer.width=d.attribute("width").ok_or(TiledError::NoLayerWidth)?.parse()?;
                layer.height=d.attribute("height").ok_or(TiledError::NoLayerHeight)?.parse()?;
                let data=d.first_element_child().ok_or(TiledError::NoLayerData)?.text().ok_or(TiledError::NoLayerData)?.trim();
                for l in data.lines(){
                    for t in l.split(",") {
                        if t.len()>0{
                            layer.tiles.push(t.parse()?);
                        }
                    }
                }
                map.layers.push(layer);
            }
        }
        Ok(map)
    }
}

#[derive(Default)]
pub struct MapAssetLoader;

impl AssetLoader for MapAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let map_asset = Map::load(bytes)?;
            load_context.set_default_asset(LoadedAsset::new(map_asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["tmx"]
    }
}


#[derive(Debug,Clone,PartialEq, Eq, PartialOrd, Ord)]
pub struct Layer {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<usize>,
}

impl Default for Layer {
    fn default() -> Self {
        Self {width:0,height:0,tiles:vec![]}
    }
}



#[derive(Debug, TypeUuid,Clone,PartialEq, Eq, PartialOrd, Ord)]
#[uuid = "9c5da9ce-05d1-424d-931e-acf6c56019a7"]
pub struct TileSet {
    pub tiles: Vec<String>,
}

impl Default for TileSet {
    fn default() -> Self {
        Self {tiles:vec![]}
    }
}

impl TileSet {
    fn load<'a>(data: &'a [u8]) -> Result<TileSet, anyhow::Error> {
        let doc=Document::parse(std::str::from_utf8(data)?)?;
        let mut ts = TileSet::default();
        for d in doc.descendants() {
            if d.tag_name().name() == "image"{
                ts.tiles.push(d.attribute("source").ok_or(TiledError::NoImageSource)?.to_owned());
            }
        }
        Ok(ts)
    }
}


#[derive(Default)]
pub struct TileSetAssetLoader;

impl AssetLoader for TileSetAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let ts_asset = TileSet::load(bytes)?;
            load_context.set_default_asset(LoadedAsset::new(ts_asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["tsx"]
    }
}

pub fn is_tile_passable(path:&str) -> bool{
    let img=path.split("/").last().unwrap();
    if img.contains("wall"){
        return false;
    }
    if img.contains("brick"){
        return false;
    }
    if img.contains("open"){
        return true;
    }
    if img.contains("gate"){
        return false;
    }
    if img.contains("column"){
        return false;
    }
    if img.contains("fountain"){
        return false;
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tileset() -> Result<(),anyhow::Error> {
        let data = std::fs::read("assets/anthea_tileset.tsx")?;
        let ts=TileSet::load(&data)?;
        assert_eq!(47,ts.tiles.len());
        assert_eq!("sprites/tiles/brick_gray0.png",&ts.tiles[0]);
        assert_eq!("sprites/tiles/gate_runed_right.png",&ts.tiles[46]);
        Ok(())
    }

    #[test]
    fn test_map1() -> Result<(),anyhow::Error> {
        let data = std::fs::read("assets/castle1.tmx")?;
        let map=Map::load(&data)?;
        assert_eq!(38,map.width);
        assert_eq!(31,map.height);
        assert_eq!(2,map.layers.len());
        let l1=&map.layers[0];
        assert_eq!(38,l1.width);
        assert_eq!(31,l1.height);
        assert_eq!(38*31,l1.tiles.len());
        assert_eq!(0,l1.tiles[39]);
        assert_eq!(1,l1.tiles[40]);
        let l2=&map.layers[1];
        assert_eq!(38,l2.width);
        assert_eq!(31,l2.height);
        assert_eq!(38*31,l2.tiles.len());
        Ok(())
    }
}