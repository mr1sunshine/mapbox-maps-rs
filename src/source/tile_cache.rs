use super::tile::Tile;
use super::tile_id::OverscaledTileId;
use std::collections::{HashMap, VecDeque};

pub(crate) struct TileCache {
    max: u32,
    data: HashMap<String, Tile>,
    order: VecDeque<String>,
}

impl TileCache {
    pub fn new(max: u32) -> Self {
        Self {
            max,
            data: HashMap::new(),
            order: VecDeque::new(),
        }
    }

    pub fn reset(&mut self) {
        self.data.clear();
        self.order.clear();
    }

    pub fn add(&mut self, tile_id: &OverscaledTileId, data: Tile) {
        let key = tile_id.wrapped().key();

        self.data.insert(key.clone(), data);
        self.order.push_back(key);

        if self.order.len() > self.max as usize {
            if let Some(front) = self.order.pop_front() {
                self.get_and_remove_by_key(&front);
            }
        }
    }

    fn get_and_remove_by_key(&mut self, key: &str) {
        self.data.remove(key);
        if let Some(pos) = self.order.iter().position(|x| *x == key) {
            self.order.remove(pos);
        }
    }

    pub fn has(&self, tile_id: &OverscaledTileId) -> bool {
        let key = tile_id.wrapped().key();
        self.data.contains_key(&key)
    }

    pub fn remove(&mut self, tile_id: &OverscaledTileId) {
        if self.has(tile_id) {
            self.get_and_remove_by_key(&tile_id.wrapped().key())
        }
    }

    pub fn get_by_key(&self, key: &str) -> Option<&Tile> {
        self.data.get(key)
    }

    pub fn get(&self, tile_id: &OverscaledTileId) -> Option<&Tile> {
        let key = tile_id.wrapped().key();
        self.data.get(&key)
    }
}
