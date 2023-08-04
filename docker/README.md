<!--
Copyright (c) Dell Inc., or its subsidiaries. All Rights Reserved.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0
-->
# Docker Containers for GStreamer Pravega

## Overview

This builds ubuntu-based container images with latest GStreamer and gstreamer-pravega plugin.

GStreamer build is from [https://github.com/streamstorage/docker-gstreamer](https://github.com/streamstorage/docker-gstreamer/tree/22.04-1.22.5).

## Image Types

There are 2 kinds of images that are built.

- streamstorage/pravega-gstreamer:latest-dev - with streamstorage/gstreamer:22.04-1.22.5-dev and gstreamer-pravega source code, libraries, and applications
- streamstorage/pravega-gstreamer:latest-prod - with streamstorage/gstreamer:22.04-1.22.5-prod and gstreamer-pravega libraries, and applications

## Build Procedure

```bash
BUILD_PROD=1 ./build-release.sh
```
