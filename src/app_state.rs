use crate::config::HLSConfig;
use crate::object_database::ObjectDatabase;

#[derive(Debug, Clone)]
pub struct AppState {
    pub env: HLSConfig,
    pub s3_bucket: ObjectDatabase
}