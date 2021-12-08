use std::cell::Ref;

pub use crate::{wolf_asset::WolfAssetCache};

pub struct ResoucesSystem{
    asset: WolfAssetCache,
}

impl<'a> ResoucesSystem {
    pub fn new() -> ResoucesSystem {
        ResoucesSystem {
            asset: WolfAssetCache::open(),
        }
    }
}
