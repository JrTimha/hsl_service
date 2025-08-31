use std::sync::Arc;
use hlskit::models::hls_video::HlsVideo;
use log::{debug, info};
use minio::s3::{Client, ClientBuilder};
use minio::s3::builders::{ObjectContent, ObjectToDelete};
use minio::s3::creds::StaticProvider;
use minio::s3::http::BaseUrl;
use minio::s3::segmented_bytes::SegmentedBytes;
use minio::s3::types::S3Api;
use uuid::Uuid;
use crate::config::ObjectDbConfig;


#[derive(Debug, Clone)]
pub struct ObjectDatabase {
    session: Arc<Client>,
    config: ObjectDbConfig,
}

impl ObjectDatabase {

    pub async fn new(config: &ObjectDbConfig) -> Self {
        let static_provider = Box::new(StaticProvider::new(
            &config.db_user,
            &config.db_password,
            None,
        ));
        let url = match config.db_url.parse::<BaseUrl>() {
            Ok(url) => url,
            Err(error) => panic!("Unable to parse db url: {:?}", error)
        };
        let client: Client = match ClientBuilder::new(url).provider(Some(static_provider)).build() {
            Ok(client) => client,
            Err(error) => panic!("Unable to initialize client: {:?}", error)
        };
        match client.bucket_exists(&config.bucket_name).send().await {
            Ok(buckets) => {
                info!("Established connection to the s3 storage.");
                if buckets.exists == false {
                    panic!("The configured bucket does not exist: {:?}", &config.bucket_name);
                }
            },
            Err(error) => {
                panic!("Unable to check if bucket exists: {:?}", error)
            }
        };
        ObjectDatabase { session: Arc::new(client), config: config.clone() }
    }

    pub async fn get_object(&self, object_id: &String) -> Result<SegmentedBytes,  Box<dyn std::error::Error + Send + Sync>> {
        let session = self.session.clone();
        let response = session.get_object(&self.config.bucket_name, object_id).send().await?;
        let object = response.content.to_segmented_bytes().await?;
        Ok(object)
    }

    pub async fn delete_object(&self, object_id: &String) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let session = self.session.clone();
        let response = session.delete_object(&self.config.bucket_name, ObjectToDelete::from(object_id)).send().await?;
        debug!("Deleted object, marker: {:?}", response.version_id);
        Ok(())
    }

    pub async fn insert_object(&self, object_id: Uuid, content: HlsVideo) -> Result<String, Box<dyn std::error::Error+Send+Sync>> {
        let session = self.session.clone();
        let hls_master = ObjectContent::from(content.master_m3u8_data);
        let response = session.put_object_content(&self.config.bucket_name, format!("videos/{}/master.m3u8", &object_id), hls_master).content_type("application/vnd.apple.mpegurl".to_string()).send().await?;
        for resolution in content.resolutions {
            session.put_object_content(&self.config.bucket_name, format!("videos/{}/{}", &object_id, resolution.playlist_name), ObjectContent::from(resolution.playlist_data)).content_type("application/vnd.apple.mpegurl".to_string()).send().await?;
            for segment in resolution.segments {
                session.put_object_content(&self.config.bucket_name, format!("videos/{}/{}", &object_id, segment.segment_name), ObjectContent::from(segment.segment_data)).content_type("application/vnd.apple.mpegurl".to_string()).send().await?;
            }
        }
        debug!("Saved object with name: {:?}", response.object);
        Ok(format!("videos/{}", &object_id))
    }

}