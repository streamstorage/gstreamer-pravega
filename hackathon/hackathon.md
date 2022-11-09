# start pravega docker locally
HOST_IP=192.168.3.13 PRAVEGA_LTS_PATH=/home/luis/Documents/pravega_lts ./pravega-docker/up.sh
HOST_IP=192.168.3.13 PRAVEGA_LTS_PATH=/home/luis/Documents/pravega_lts ./pravega-docker/down.sh

# ingest test video
PRAVEGA_CONTROLLER=10.247.97.51:9090 ./scripts/videotestsrc-to-pravega-hls.sh

# ingest bilibili video & danmu
https://github.com/nilaoda/BBDown
./bbdown BV1DL4y1w7Gy -tv -dd

VIDEO_FILE="/home/luis/Downloads/bilibili/猫妈妈和小奶猫.mp4" PRAVEGA_STREAM=BV1xA411v7jA PRAVEGA_CONTROLLER=10.247.97.51:9090 ./scripts/file-to-pravega.sh
VIDEO_FILE=/home/luis/Downloads/bbdown/这小家伙能干翻长颈鹿？？？.mp4 PRAVEGA_STREAM=BV1hV4y157XN ./scripts/file-to-pravega.sh
pravega_client_auth_method=Bearer KEYCLOAK_SERVICE_ACCOUNT_FILE=/home/luis/projects/nautilus-gstreamer/gstreamer-pravega/hackathon/hackathon.keycloak.json pravega_client_tls_cert_path=/home/luis/projects/nautilus-gstreamer/gstreamer-pravega/hackathon/tba.crt ALLOW_CREATE_SCOPE=false PRAVEGA_SCOPE=hackathon PRAVEGA_STREAM=BV1xA411v7jA PRAVEGA_CONTROLLER=tls://pravega-controller.tba.nautilus-platform-dev.com:443 VIDEO_FILE="/home/luis/Downloads/bilibili/猫妈妈和小奶猫.mp4" ./scripts/file-to-pravega.sh

PRAVEGA_SCOPE=bilibili PRAVEGA_STREAM=BV1xA411v7jA ./scripts/pravega-table-updater.sh
pravega_client_auth_method=Bearer KEYCLOAK_SERVICE_ACCOUNT_FILE=/home/luis/projects/nautilus-gstreamer/gstreamer-pravega/hackathon/hackathon.keycloak.json pravega_client_tls_cert_path=/home/luis/projects/nautilus-gstreamer/gstreamer-pravega/hackathon/tba.crt PRAVEGA_SCOPE=hackathon PRAVEGA_STREAM=BV1xA411v7jA PRAVEGA_CONTROLLER_URI=tls://pravega-controller.tba.nautilus-platform-dev.com:443 ./scripts/pravega-table-updater.sh

python danmu-loader.py --danmu-file --video-id 1
python ./hackathon/danmu-loader.py --danmu-file "/home/luis/Downloads/bilibili/猫 妈 妈 和 小 奶 猫.xml" --video-id 2

# run video server
PRAVEGA_CONTROLLER_URI=10.247.97.51:9090 POSTGRES_URI=postgres://admin:password@10.247.97.51:5432/hackathon ./scripts/pravega-video-server.sh

cd pravega-video-server && cargo build --release && tar -zcvf pravega-video-server.tar.gz ../target/release/pravega-video-server resources && scp pravega-video-server.tar.gz luis@node2:/home/luis/video-server
cd video-server && rm nohup.out resources/ target/ -rf && tar -zxvf pravega-video-server.tar.gz && sudo systemctl restart pravega-video-server.service

./hackathon/pravega-video-server-image-builder.sh
helm upgrade --install --recreate-pods video-server ./hackathon/charts/video-server -n hackathon

## env
postgresql: 10.247.97.51:5432 user/pass: admin/password
postgresql admin portal: http://10.247.97.51:8080/
pravega uri: tcp://10.247.97.51:9090

http://127.0.0.1:3030/player?scope=examples&stream=hls1&begin=2022-10-26T02:56:08Z&end=2022-10-26T02:56:12Z
http://10.247.97.51:3030/player?scope=examples&stream=hls1&begin=2022-10-26T07:14:19Z&end=2022-10-26T07:14:29Z
http://10.247.97.51:3030/player?scope=bilibili&stream=BV1hV4y157XN&begin=2022-10-26T10:34:15Z&end=2022-10-26T10:39:05Z

https://www.youtube.com/embed/A-UV7Z13uAQ


USE_NEW_NVSTREAMMUX=yes  gst-launch-1.0 uridecodebin3 uri=$input1 name=demux1 ! queue ! nvvideoconvert ! "video/x-raw(memory:NVMM)" ! mux1.sink_0 nvstreammux batch-size=1 sync-inputs=1 max-latency=250000000 name=mux1 ! queue ! nvmultistreamtiler ! nvvideoconvert ! "video/x-raw(memory:NVMM)" ! nvv4l2h264enc ! h264parse ! queue ! flvmux name=mux streamable=true ! filesink location=out.flv  async=0 qos=0 sync=1 demux1. ! queue ! audioconvert ! audiomux.sink_0 nvstreammux name=audiomux batch-size=1 max-latency=250000000 sync-inputs=1 ! nvstreamdemux name=audiodemux audiodemux.src_0  ! audioconvert ! mixer.sink_0 audiomixer latency=250000000 name=mixer ! queue ! avenc_aac ! aacparse ! queue ! mux. fakesrc num-buffers=0 is-live=1 ! mixer. -e

http://127.0.0.1:3030/player?scope=bilibili&stream=BV1hV4y157XN&begin=2022-10-30T05:55:47Z&end=2022-10-30T06:00:37Z
http://127.0.0.1:3030/player?scope=bilibili&stream=BV1hV4y157XN&begin=2022-10-30T05:55:57Z&end=2022-10-30T05:56:07Z
