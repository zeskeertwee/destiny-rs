use crate::models::manifest::ManifestKey;

pub trait ManifestTableKey {
    const TABLE_KEY: ManifestKey;
}