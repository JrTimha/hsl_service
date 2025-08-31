# HSL Transcoding Service - for SaaS Backends

A high-performance, scalable transcoding solution specifically designed to transcode mp4 to hsl. Tokio and Axum provides an asynchronous, stable, and efficient solution.


## Key Points of HSL:

-   **Scalability**: Built with the asynchronous Tokio runtime, ISM efficiently handles thousands of simultaneous connections.
-   **Easy Integration**: Designed for seamless integration with existing SaaS architectures.
-   **S3 Support**: Functionality for uploading content to S3 Buckets is currently in progress.


### Configure container environment

To configure the container, you need to mount a configuration file named `production.config.toml` to the `/app` directory within the container.

These are the available configuration settings:

```toml
hsl_url = "localhost"
hsl_port= 5555
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
  ism:
    image: ghcr.io/jrtimha/hsl_service:latest
    container_name: hsl-container
    ports:
      - "5555:5555"
    environment:
      ISM_MODE: production
    volumes:
      - ./production.config.toml:/app/production.config.toml
```

Now go to `http://localhost:5555` in your browser, if everything works you will see: 

```
Hello, world! I'm your new HSL_SERVICE. 🤗
```

## ISM Endpoints:

Here's a summary of the available API endpoints:

*   **`POST /api/transcode`**
    *   Handler: `transcode_video`
    *   Description: Transcodes a video file to HSL, save it to S3.


