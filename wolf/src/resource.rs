use sdl2::render::Texture;

use crate::io::{app_root_dir, asset_file};
use crate::wolf_asset::{
    read_atlas, read_level, read_map, read_texture, WolfLevel, WolfMapAtlas, WolfVSWAP, read_vswap
};
use std::cell::{Ref, RefCell, RefMut};
use std::fs::File;

use crate::texture::{From2DData, Texture2D};
use std::collections::HashMap;

pub struct Map {
    pub data: Vec<u16>,
    pub width: i32,
    pub height: i32,
}

pub struct MapCache {
    atlas: WolfMapAtlas,                 //  the index of the map resouces
    map_head: std::fs::File,             // map file handle
    wolf_level: WolfLevel,               // map file level info, needed by the file reading  routine
    cur_level_index: Option<(u32, u32)>, // (episode, level)
}

pub struct TexturePool {
    texture_pool: HashMap<usize, Texture2D<u8>>,
    vswap_head: WolfVSWAP,
    vswap_file: std::fs::File
}

impl TexturePool {
    pub fn open() -> Self {
        let mut vswap_file = asset_file("resources/original/VSWAP.WL6").unwrap();
        let vswap_header = read_vswap(&mut vswap_file);
        TexturePool {
            texture_pool: HashMap::new(),
            vswap_head:vswap_header,
            vswap_file:vswap_file
        }
    }
    pub fn get_texture(
        &mut self,
        texture_id: usize,
    ) -> &Texture2D<u8> {
        let mut file = &mut self.vswap_file;
        let head =&self.vswap_head;
        self.texture_pool
            .entry(texture_id)
            .or_insert_with_key(|&key| {
                read_texture(&mut file, &head, key).map_or_else(
                    |err| {
                        println!("{}", err);
                        From2DData::from_data(vec![0u8; 64 * 64], 64, 64, 3)
                    },
                    |tex| From2DData::from_data(tex, 64, 64, 3),
                )
            })
    }
}

impl MapCache {
    pub fn load() -> MapCache {
        let mut atlas_dir = app_root_dir().unwrap();
        atlas_dir.push("resources/original/MAPHEAD.WL6");
        let atlas = read_atlas(atlas_dir);
        let map_head = asset_file("resources/original/GAMEMAPS.WL6").unwrap();
        MapCache {
            atlas: atlas,
            map_head: map_head,
            wolf_level: Default::default(),
            cur_level_index: None,
        }
    }

    pub fn read_map(&mut self, episode: u32, level: u32, map: i32) -> Map {
        let new_level_index = (episode, level);
        match self.cur_level_index {
            Some(index) => {
                if index != new_level_index {
                    self.wolf_level = read_level(
                        &self.atlas,
                        &mut self.map_head,
                        episode as i32,
                        level as i32,
                    );
                }
            }
            None => {
                self.wolf_level = read_level(
                    &self.atlas,
                    &mut self.map_head,
                    episode as i32,
                    level as i32,
                );
            }
        }

        self.cur_level_index = Some(new_level_index);
        let map_data = read_map(
            &self.atlas,
            &self.wolf_level,
            &mut self.map_head,
            map as usize,
        );
        Map {
            data: map_data,
            width: 64,
            height: 64,
        }
    }
}


#[cfg(test)]
mod resource_test{
    use super::TexturePool;

    #[test]
    fn texture_pool_test(){
        let mut tp = TexturePool::open();
        for tid in 0..1000{
            let t = tp.get_texture(tid);
            // println!("{:?}", t.buffer());
        }
    }
}