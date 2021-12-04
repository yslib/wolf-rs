use crate::io::{app_root_dir, asset_file};
use std::convert::AsRef;
use std::fs;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::path::{Path};

const LEVELS_PER_EPS: usize = 10;
const MAP_PLANE: usize = 3;
const NEAR: u8 = 0xA7;
const FAR: u8 = 0xA8;
const ROOT: u8 = 254;

pub struct WolfMapAtlas {
    pub rlew_flag: u16,
    pub map_offset: Vec<i32>,
}

pub struct WolfLevel {
    pub map_offset: [i32; MAP_PLANE],
    pub cc_length: [i16; MAP_PLANE],
    pub width: i16,
    pub height: i16,
    pub name: [u8; 16],
}

impl Default for WolfLevel {
    fn default() -> WolfLevel {
        WolfLevel {
            map_offset: [0; MAP_PLANE],
            cc_length: [0; MAP_PLANE],
            width: 0,
            height: 0,
            name: [0u8; 16],
        }
    }
}

pub struct WolfVSWAP {
    pub chunck_num: i16,
    pub sprite_start: i16,
    pub sound_start: i16,
    pub chunk: Vec<(u32, i16)>,
}

fn read_vswap(vswap_file: &mut std::fs::File) -> WolfVSWAP {
    vswap_file.seek(SeekFrom::Start(0)).unwrap();
    let mut u16_buf = [0u8, 2];
    vswap_file.read(&mut u16_buf).unwrap();
    let num_of_chunk = i16::from_ne_bytes(u16_buf);

    vswap_file.read(&mut u16_buf).unwrap();
    let sprite_start = i16::from_ne_bytes(u16_buf);

    vswap_file.read(&mut u16_buf).unwrap();
    let sound_start = i16::from_ne_bytes(u16_buf);

    let offset = (0..num_of_chunk)
        .map(|_| {
            let mut u32_buf = [0u8; 4];
            vswap_file.read(&mut u32_buf).unwrap();
            u32::from_ne_bytes(u32_buf)
        })
        .collect::<Vec<u32>>();

    let length = (0..num_of_chunk)
        .map(|_| {
            let mut u16_buf = [0u8; 2];
            vswap_file.read(&mut u16_buf).unwrap();
            i16::from_ne_bytes(u16_buf)
        })
        .collect::<Vec<i16>>();

    WolfVSWAP {
        chunck_num: num_of_chunk,
        sprite_start: sprite_start,
        sound_start: sound_start,
        chunk: offset
            .iter()
            .zip(length.iter())
            .map(|(&x, &y)| (x, y))
            .collect(),
    }
}

// returns a 64x64 Vec<u8>
fn read_texture(
    vswap_file: &mut std::fs::File,
    vswap_header: &WolfVSWAP,
    texture_index: usize,
) -> Result<Vec<u8>, ()> {
    if texture_index >= vswap_header.sprite_start as usize {
        Err(())
    } else {
        let texture_offset = vswap_header.chunk[texture_index].0 as u64;
        let texture_length = vswap_header.chunk[texture_index].1 as usize;
        vswap_file.seek(SeekFrom::Start(texture_offset)).unwrap();
        let mut tex = vec![0u8; texture_length];
        vswap_file.read_exact(&mut tex[0..]).unwrap();
        Ok(tex)
    }
}

///
/// MAPHEAD.WL6
fn read_altas<T: AsRef<Path>>(path: T) -> WolfMapAtlas {
    let atlas = fs::read(path).unwrap();

    // MAPEHAD format
    // the first word size is the rlew_flag
    // the rest of it is an array with length of 100 with i32 element
    // 402 bytes in total
    let rlew_flag = u16::from_ne_bytes([atlas[0], atlas[1]]);
    let map_atlas: Vec<i32> = (2..atlas.len())
        .step_by(4)
        .map(|ind| {
            i32::from_ne_bytes([
                atlas[ind + 0],
                atlas[ind + 1],
                atlas[ind + 3],
                atlas[ind + 3],
            ])
        })
        .collect();

    WolfMapAtlas {
        rlew_flag: rlew_flag,
        map_offset: map_atlas,
    }
}
fn read_level(
    map_atlas: &WolfMapAtlas,
    map_head: &mut fs::File,
    episode: i32,
    level: i32,
) -> WolfLevel {
    let map_index = (episode - 1) as usize * LEVELS_PER_EPS + level as usize - 1;

    let map_meta_offset = map_atlas.map_offset[map_index] as u64;

    map_head.seek(SeekFrom::Start(map_meta_offset)).unwrap();

    let mut map_offset = [0; 3];
    for ind in 0..MAP_PLANE {
        let mut i32_buf = [0u8; 4];
        map_head.read(&mut i32_buf).unwrap();
        map_offset[ind] = i32::from_ne_bytes(i32_buf);
    }

    let mut cc_length = [0i16; 3];
    for ind in 0..MAP_PLANE {
        let mut u16_buf = [0u8; 2];
        map_head.read(&mut u16_buf).unwrap();
        cc_length[ind] = i16::from_ne_bytes(u16_buf);
    }
    let mut u16_buf = [0u8; 2];
    map_head.read(&mut u16_buf).unwrap();
    let width = i16::from_ne_bytes(u16_buf);

    map_head.read(&mut u16_buf).unwrap();
    let height = i16::from_ne_bytes(u16_buf);

    let mut name_buf = [0u8; 16];
    map_head.read(&mut name_buf).unwrap();

    WolfLevel {
        map_offset: map_offset,
        cc_length: cc_length,
        width: width,
        height: height,
        name: name_buf,
    }
}

fn read_map(
    atlas: &WolfMapAtlas,
    level_head: &WolfLevel,
    map_file: &mut fs::File,
    map: usize,
) -> Vec<u16> {
    let cc_length = level_head.cc_length[map] as usize;

    // read by word
    map_file
        .seek(SeekFrom::Start(level_head.map_offset[map] as u64))
        .unwrap();

    let carmark_buffer: Vec<u16> = (0..cc_length / 2)
        .map(|_| {
            let mut word_buf = [0u8; 2];
            map_file.read(&mut word_buf).unwrap();
            u16::from_ne_bytes(word_buf)
        })
        .collect();

    let rlew_length_words = carmark_buffer[0] / 2;
    let mut rlew_buffer: Vec<u16> = vec![0u16; rlew_length_words as usize];

    let map_unit_size = (level_head.width * level_head.height) as usize;
    // let map_bytes = (map_unit_size) as usize * std::mem::size_of::<u16>();
    let mut map = vec![0u16; map_unit_size];

    // 受不了了，直接搞unsafe
    unsafe {
        // carmarck expand: expand carmarck_buffer to rlew_buffer
        let mut read: *const u8 = carmark_buffer.as_ptr().add(1) as *const u8;
        let dest = rlew_buffer.as_mut_ptr() as *mut u16;
        let mut write: *mut u8 = dest as *mut u8;
        let mut copy: *const u8;
        let mut offset: u16;

        let mut length: i32 = rlew_length_words as i32;
        while length > 0 {
            let count = *(read);
            read = read.add(1);
            let flag = *(read);
            read = read.add(1);
            if flag == NEAR && count != 0 {
                offset = (*read) as u16;
                read = read.add(1);
                copy = write.sub(2 * offset as usize);
                for _ in 0..count {
                    *write = *copy;
                    write = write.add(1);
                    copy = copy.add(1);
                    *write = *copy;
                    write = write.add(1);
                    copy = copy.add(1);
                    length -= 1;
                }
            } else if flag == FAR && count != 0 {
                offset = *(read as *const u16);
                read = read.add(2);
                copy = dest.offset(offset as isize) as *mut u8;
                for _ in 0..count {
                    *write = *copy;
                    write = write.add(1);
                    copy = copy.add(1);
                    *write = *copy;
                    write = write.add(1);
                    copy = copy.add(1);
                    length -= 1;
                }
            } else if (flag == NEAR || flag == FAR) && count == 0 {
                *write = *read;
                write = write.add(1);
                read = read.add(1);
                *write = flag;
                write = write.add(1);
                length -= 1;
            } else {
                *(write) = count;
                write = write.add(1);
                *(write) = flag;
                write = write.add(1);
                length -= 1;
            }
        }
    }

    // rlew_expand expand rlew_buffer to final map buffer
    let (mut read, mut write) = (1, 0);
    while write < map_unit_size {
        let current_word = rlew_buffer[read];
        read += 1;
        if current_word == atlas.rlew_flag {
            let count = rlew_buffer[read];
            read += 1;
            for _ in 0..count {
                map[write] = rlew_buffer[read];
                write += 1;
            }
            read += 1;
        } else {
            map[write] = current_word;
            write += 1;
        }
    }

    map
}


pub struct WolfAssetCache {
    cur_map: Vec<u16>,
    pub width:i32,
    pub height:i32,
    cur_map_index: (i32, i32, i32),
    texture_pool: Vec<Vec<u8>>,
    map_head: std::fs::File,
    altas: WolfMapAtlas,
    wolf_level: WolfLevel,
}

impl WolfAssetCache {
    pub fn open() -> WolfAssetCache {
        let mut atlas_dir = app_root_dir().unwrap();
        atlas_dir.push("resources/original/MAPHEAD.WL6");
        let atlas = read_altas(atlas_dir);
        let mut map_head = asset_file("resources/original/GAMEMAPS.WL6").unwrap();
        WolfAssetCache {
            cur_map: vec![],
            width:64,
            height:64,
            cur_map_index: (-1, -1, -1),
            texture_pool: vec![],
            altas: atlas,
            map_head: map_head,
            wolf_level: Default::default(),
        }
    }

    fn _check_map_index(&self, episode: i32, level: i32, map: i32) -> bool {
        episode >= 0
            && episode <= 6
            && level >= 0
            && level <= 10
            && map >= 0
            && map < MAP_PLANE as i32
    }

    pub fn get_cur_map_index(&self)->(i32,i32,i32){
        self.cur_map_index
    }

    pub fn get_or_read_map(&mut self, episode: i32, level: i32, map: i32) -> &Vec<u16> {
        if self._check_map_index(episode, level, map) == false {
            panic!("Invalid map index: {} {} {}", episode, level, map);
        }
        let new_map = (episode, level, map);
        if new_map != self.cur_map_index {
            let cur_level_index = (self.cur_map_index.0, self.cur_map_index.1);
            let new_level_index = (new_map.0, new_map.1);
            if cur_level_index != new_level_index {
                self.wolf_level = read_level(&self.altas, &mut self.map_head, episode, level);
            }
            self.cur_map = read_map(
                &self.altas,
                &self.wolf_level,
                &mut self.map_head,
                map as usize,
            );
            self.cur_map_index = new_map;
        }
        &self.cur_map
    }
}

fn read_vswap_header(vswap_file: &mut std::fs::File) {
    vswap_file.seek(SeekFrom::Start(0));
}

#[cfg(test)]
mod tests {
    use super::{read_altas, read_level, read_map, read_texture, read_vswap};
    use crate::io::{app_root_dir, asset_file};

    #[test]
    fn read_map_test() {
        let mut atlas_dir = app_root_dir().unwrap();
        atlas_dir.push("resources/orignal/MAPHEAD.WL6");
        let atlas = read_altas(atlas_dir);

        let mut map_head = asset_file("resources/orignal/GAMEMAPS.WL6").unwrap();

        let level_data = read_level(&atlas, &mut map_head, 1, 1);
        let map_data = read_map(&atlas, &level_data, &mut map_head, 0);
    }
    #[test]
    fn read_vswap_test() {
        let mut vswap_file = asset_file("resources/original/VSWAP.WL6").unwrap();

        let vswap_header = read_vswap(&mut vswap_file);
        let tex_buf = read_texture(&mut vswap_file, &vswap_header, 1).unwrap();
        //println!("{} {} {} ",vswap_header.chunck_num, vswap_header.sprite_start,vswap_header.sound_start);
        println!("{:?}", tex_buf);
    }
}
