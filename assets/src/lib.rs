#![cfg_attr(not(feature = "std"), no_std)]
/// Assets and Package implementation for Move VM compiled scripts/modules/packages.
use once_cell::sync::OnceCell;
use std::path::Path;

/// Asset.
#[derive(Clone)]
pub struct Asset {
    name: &'static str,
    path: &'static str,
    bytes: OnceCell<Vec<u8>>,
}

impl Asset {
    /// Create new asset instance.
    pub const fn new(name: &'static str, path: &'static str) -> Self {
        Self {
            name,
            path,
            bytes: OnceCell::new(),
        }
    }

    /// Get asset name.
    pub fn name(&self) -> &'static str {
        self.name
    }

    /// Get asset bytes.
    pub fn bytes(&self) -> &[u8] {
        self.bytes
            .get_or_init(|| {
                let dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
                let path = Path::new(dir.as_str()).join(self.path);
                std::fs::read(&path)
                    .unwrap_or_else(|_| panic!("Failed to load test asset: {:?}", path.display()))
            })
            .as_slice()
    }
}

/// Move VM package.
pub struct Package {
    modules: &'static [&'static str],
    package: Asset,
}

impl Package {
    /// Create new Package instance.
    pub const fn new(modules: &'static [&'static str], package: Asset) -> Self {
        Self { modules, package }
    }

    /// Get modules inside package.
    pub fn modules(&self) -> &'static [&'static str] {
        self.modules
    }

    /// Get package bytes.
    pub fn bytes(&self) -> &[u8] {
        self.package.bytes()
    }
}
