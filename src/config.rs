/// Pattern to match for cleanup
#[derive(Debug)]
pub struct CleanupPattern {
    pub target_dir: String,
    pub indicator_file: String,
}
