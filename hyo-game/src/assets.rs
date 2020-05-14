use std::{
    borrow::Borrow,
    collections::HashSet,
    hash::Hash,
    path::{Path, PathBuf},
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Asset {
    pub name: String,
    pub path: PathBuf,
}

impl Asset {
    fn new(name: String, path: PathBuf) -> Self {
        Self { name, path }
    }
}

#[derive(Debug, Eq)]
struct AssetByName(Asset);

impl AssetByName {
    fn inner(&self) -> &Asset {
        &self.0
    }

    fn name(&self) -> &str {
        &self.inner().name
    }
}

impl Borrow<str> for AssetByName {
    fn borrow(&self) -> &str {
        self.name()
    }
}

impl Hash for AssetByName {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name().hash(state)
    }
}

impl PartialEq for AssetByName {
    fn eq(&self, other: &AssetByName) -> bool {
        self.name() == other.name()
    }
}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct Assets(HashSet<AssetByName>);

impl Assets {
    pub fn load(assets_dir: &Path) -> Result<Self, anyhow::Error> {
        let mut inst = Self::default();

        let mut dir_queue = vec![assets_dir.canonicalize()?];
        while let Some(dir) = dir_queue.pop() {
            for entry in dir.read_dir()? {
                let entry_path = entry?.path();

                if entry_path.is_dir() {
                    dir_queue.push(entry_path);
                    continue;
                }

                let name = entry_path
                    .strip_prefix(&assets_dir)?
                    .with_extension("")
                    .to_str()
                    .ok_or_else(|| {
                        anyhow::anyhow!(
                            "asset path contains invalid unicode characters: {:?}",
                            entry_path
                        )
                    })?
                    .to_string();

                inst.add(Asset::new(name, entry_path));
            }
        }

        Ok(inst)
    }

    pub fn add(&mut self, asset: Asset) -> bool {
        self.0.insert(AssetByName(asset))
    }

    pub fn get(&self, name: &str) -> Option<&Asset> {
        self.0.get(name).map(AssetByName::inner)
    }
}
