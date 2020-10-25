use crate::style_spec::Scheme;
use radix_fmt::radix;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Eq, Clone)]
pub(crate) struct CanonicalTileId {
    pub z: u32,
    pub x: u32,
    pub y: u32,
    key: String,
}

impl CanonicalTileId {
    pub fn new(z: u32, x: u32, y: u32) -> Self {
        assert!(z <= 25);
        assert!(x < 2u32.pow(z));
        assert!(y < 2u32.pow(z));
        Self {
            z,
            x,
            y,
            key: calculate_key(0, z, z, x, y),
        }
    }

    pub fn url(&self, urls: &[String], scheme: Option<Scheme>) -> String {
        let index = (self.x + self.y) as usize % urls.len();
        let y = match scheme {
            Some(Scheme::TMS) => 2u32.pow(self.z) - self.y - 1,
            _ => self.y,
        };
        urls[index]
            .replace(
                "{prefix}",
                &format!("{}{}", radix(self.x % 16, 16), radix(self.y % 16, 16)),
            )
            .replace("{z}", &self.z.to_string())
            .replace("{x}", &self.x.to_string())
            .replace("{y}", &y.to_string())
    }
}

impl PartialEq for CanonicalTileId {
    fn eq(&self, other: &Self) -> bool {
        self.z == other.z && self.x == other.x && self.y == other.y
    }
}

impl Display for CanonicalTileId {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}/{}/{}", self.z, self.x, self.y)
    }
}

fn calculate_key(mut wrap: i32, overscaled_z: u32, z: u32, x: u32, y: u32) -> String {
    wrap *= 2;
    let wrap = if wrap < 0 { -wrap - 1 } else { wrap } as u32;
    let dim = 1u32 << z;

    return format!(
        "{}{}{}",
        radix(dim * dim * wrap + dim * y + x, 36),
        radix(z, 36),
        radix(overscaled_z, 36)
    );
}

pub(crate) struct UnwrappedTileId {
    wrap: i32,
    canonical: CanonicalTileId,
    key: String,
}

impl UnwrappedTileId {
    pub fn new(wrap: i32, canonical: &CanonicalTileId) -> Self {
        Self {
            wrap,
            canonical: canonical.clone(),
            key: calculate_key(wrap, canonical.z, canonical.z, canonical.x, canonical.y),
        }
    }
}

#[derive(Debug, Eq, Clone)]
pub(crate) struct OverscaledTileId {
    overscaled_z: u32,
    wrap: i32,
    canonical: CanonicalTileId,
    key: String,
}

impl OverscaledTileId {
    pub fn new(overscaled_z: u32, wrap: i32, z: u32, x: u32, y: u32) -> Self {
        assert!(overscaled_z >= z);
        Self {
            overscaled_z,
            wrap,
            canonical: CanonicalTileId::new(z, x, y),
            key: calculate_key(wrap, overscaled_z, z, x, y),
        }
    }

    pub fn scaled_to(&self, target_z: u32) -> OverscaledTileId {
        assert!(target_z <= self.overscaled_z);
        let z_diff = self.canonical.z - target_z;
        if target_z > self.canonical.z {
            OverscaledTileId::new(
                target_z,
                self.wrap,
                self.canonical.z,
                self.canonical.x,
                self.canonical.y,
            )
        } else {
            OverscaledTileId::new(
                target_z,
                self.wrap,
                target_z,
                self.canonical.x >> z_diff,
                self.canonical.y >> z_diff,
            )
        }
    }

    pub fn children(&self, source_max_zoom: u32) -> Vec<OverscaledTileId> {
        if self.overscaled_z >= source_max_zoom {
            return vec![OverscaledTileId::new(
                self.overscaled_z + 1,
                self.wrap,
                self.canonical.z,
                self.canonical.x,
                self.canonical.y,
            )];
        }

        let z = self.canonical.z + 1;
        let x = self.canonical.x * 2;
        let y = self.canonical.y * 2;

        vec![
            OverscaledTileId::new(z, self.wrap, z, x, y),
            OverscaledTileId::new(z, self.wrap, z, x + 1, y),
            OverscaledTileId::new(z, self.wrap, z, x, y + 1),
            OverscaledTileId::new(z, self.wrap, z, x + 1, y + 1),
        ]
    }

    pub fn wrapped(&self) -> OverscaledTileId {
        OverscaledTileId::new(
            self.overscaled_z,
            0,
            self.canonical.z,
            self.canonical.x,
            self.canonical.y,
        )
    }

    pub fn key(&self) -> String {
        self.key.clone()
    }
}

impl PartialEq for OverscaledTileId {
    fn eq(&self, other: &Self) -> bool {
        self.overscaled_z == other.overscaled_z
            && self.wrap == other.wrap
            && self.canonical == other.canonical
    }
}

impl Display for OverscaledTileId {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}/{}/{}",
            self.overscaled_z, self.canonical.x, self.canonical.y
        )
    }
}
