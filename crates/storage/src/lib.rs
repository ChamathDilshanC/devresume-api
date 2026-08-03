pub struct StorageClient {
    pub bucket: String,
}

impl StorageClient {
    pub fn new(bucket: String) -> Self {
        Self { bucket }
    }

    pub fn get_file_url(&self, filename: &str) -> String {
        format!("http://localhost:9000/{}/{}", self.bucket, filename)
    }
}
