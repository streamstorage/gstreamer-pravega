# start pravega docker locally
HOST_IP=192.168.31.250 PRAVEGA_LTS_PATH=/home/luis/Documents/pravega_lts ./pravega-docker/up.sh
./pravega-docker/down.sh

# ingest test video
PRAVEGA_CONTROLLER=10.247.97.51:9090 ./scripts/videotestsrc-to-pravega-hls.sh

# ingest bilibili video
VIDEO_FILE=/home/luis/projects/nautilus-gstreamer/gstreamer-pravega/bilibili/这小家伙能干翻长颈鹿？？？.mp4 PRAVEGA_STREAM=BV1hV4y157XN PRAVEGA_CONTROLLER=10.247.97.51:9090 ./scripts/file-to-pravega.sh

# run video server
cd pravega-video-server && cargo build --release && tar -zcvf pravega-video-server.tar.gz ../target/release/pravega-video-server resources && scp pravega-video-server.tar.gz luis@node2:/home/luis/video-server
nohup ./target/release/pravega-video-server&

## env
postgresql: 10.247.97.51:5432 user/pass: admin/password
postgresql admin portal: http://10.247.97.51:8080/
pravega uri: tcp://10.247.97.51:9090

http://127.0.0.1:3030/player?scope=examples&stream=hls1&begin=2022-10-26T02:56:08Z&end=2022-10-26T02:56:12Z
http://10.247.97.51:3030/player?scope=examples&stream=hls1&begin=2022-10-26T07:14:19Z&end=2022-10-26T07:14:29Z
http://10.247.97.51:3030/player?scope=bilibili&stream=BV1hV4y157XN&begin=2022-10-26T10:34:15Z&end=2022-10-26T10:39:05Z

https://www.youtube.com/embed/A-UV7Z13uAQ
