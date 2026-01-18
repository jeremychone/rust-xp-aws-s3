# Rustfs quick video demo

Simple code examples using the Rust AWS S3 library and rustfs as the S3 mock

(video on https://youtube.com/jeremychone coming soon)

## Rustfs container (for mock s3 dev)

Modified from: https://rustfs.com/download/?platform=docker

```
# add -d for detached
podman run --rm \
  --name rustfs \
  -p 9000:9000 \
  -p 9001:9001 \
  -it \
  rustfs/rustfs:latest
```

http://127.0.0.1:9001 with rustfsadmin/rustfsadmin



<br />

[This Repo](https://github.com/jeremychone-channel/rust-xp-aws-s3)