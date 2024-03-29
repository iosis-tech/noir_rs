use super::Srs;
use std::fs;

pub struct LocalSrs(pub Srs);

const SRS_DEFAULT_PATH: &str = "srs.local";

impl LocalSrs {
    pub fn new(num_points: u32, path: Option<&str>) -> Self {
        let file = fs::read(path.unwrap_or(SRS_DEFAULT_PATH)).unwrap();
        let srs: Srs = bincode::deserialize(&file).unwrap();
        LocalSrs(srs.get(num_points))
    }

    pub fn save(&self, path: Option<&str>) {
        fs::write(
            path.unwrap_or(SRS_DEFAULT_PATH),
            bincode::serialize(&self.0).unwrap(),
        )
        .unwrap();
    }
}
