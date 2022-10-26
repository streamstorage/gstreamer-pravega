# start pravega docker
HOST_IP=192.168.31.250 PRAVEGA_LTS_PATH=/home/luis/Documents/pravega_lts ./pravega-docker/up.sh
./pravega-docker/down.sh

# ingest test video
./scripts/videotestsrc-to-pravega-hls.sh

# run video server
cd pravega-video-server && cargo run

http://127.0.0.1:3030/player?scope=examples&stream=hls1&begin=2022-10-26T02:56:08Z&end=2022-10-26T02:56:12Z


https://www.youtube.com/embed/A-UV7Z13uAQ
