# HLS Transcoding Service - for SaaS Backends

A high-performance, scalable transcoding solution specifically designed to transcode MP4 to HLS. Tokio and Axum provides an asynchronous, stable, and efficient solution.


## Key Points of HLS Transcoding Service:

-   **Scalability**: Built with the asynchronous Tokio runtime, ISM efficiently handles hundreds of simultaneous connections.
-   **Easy Integration**: Designed for seamless integration with existing SaaS architectures.
-   **S3 Support**: Functionality for uploading content to S3 Buckets.


### Configure container environment

To configure the container, you need to mount a configuration file named `production.config.toml` to the `/app` directory within the container.

These are the available configuration settings:

```toml
hls_url = "localhost"
hls_port= 5555
log_level = "info"

[object_db_config]
db_user = "minioadmin"
db_url = "http://localhost:9000"
db_password = "minioadmin"
bucket_name = "meventure"


```
An example Docker Compose:

```yaml
services:
  hls:
    image: ghcr.io/jrtimha/hsl_service:latest
    container_name: hls-container
    ports:
      - "5555:5555"
    environment:
      HLS_MODE: production
    volumes:
      - ./production.config.toml:/app/production.config.toml
```

Now go to `http://localhost:5555` in your browser, if everything works you will see: 

```
Hello, world! I'm your new HLS_SERVICE. 🤗
```

## HLS-Transcoder Endpoints:

Here's a summary of the available API endpoints:

*   **`POST /api/transcode`**
    *   Handler: `transcode_video`
    *   Description: Transcodes a video file to HLS, save it to S3.


