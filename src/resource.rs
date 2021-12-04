pub use crate::{wolf_asset::WolfAssetCache};

pub struct ResoucesSystem {
    asset: WolfAssetCache,
}

impl ResoucesSystem {
    pub fn new() -> ResoucesSystem {
        ResoucesSystem {
            asset: WolfAssetCache::open(),
        }
    }

    pub fn get_or_read_map(&mut self, map_index: (i32, i32, i32)) -> &Vec<u16> {
        self.asset
            .get_or_read_map(map_index.0, map_index.1, map_index.2)
    }
}
