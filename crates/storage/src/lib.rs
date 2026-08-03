pub enum StorageTier {
    Files,
    Artifacts,
    Exports,
}

pub struct EnterpriseStorageClient {
    pub bucket: String,
}

impl EnterpriseStorageClient {
    pub fn new(bucket: String) -> Self {
        Self { bucket }
    }

    pub fn get_storage_path(&self, tier: StorageTier, filename: &str) -> String {
        let prefix = match tier {
            StorageTier::Files => "files",
            StorageTier::Artifacts => "artifacts",
            StorageTier::Exports => "exports",
        };
        format!(
            "http://localhost:9000/{}/{}/{}",
            self.bucket, prefix, filename
        )
    }
}
