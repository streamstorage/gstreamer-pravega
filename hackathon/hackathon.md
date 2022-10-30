# start pravega docker locally
HOST_IP=192.168.31.250 PRAVEGA_LTS_PATH=/home/luis/Documents/pravega_lts ./pravega-docker/up.sh
./pravega-docker/down.sh

# ingest test video
PRAVEGA_CONTROLLER=10.247.97.51:9090 ./scripts/videotestsrc-to-pravega-hls.sh

# ingest bilibili video & danmu
VIDEO_FILE=/home/luis/projects/nautilus-gstreamer/gstreamer-pravega/bilibili/这小家伙能干翻长颈鹿？？？.mp4 PRAVEGA_STREAM=BV1hV4y157XN PRAVEGA_CONTROLLER=10.247.97.51:9090 ./scripts/file-to-pravega.sh

python danmu-loader.py --danmu-file --video-id 1

# run video server
cd pravega-video-server && cargo build --release && tar -zcvf pravega-video-server.tar.gz ../target/release/pravega-video-server resources && scp pravega-video-server.tar.gz luis@node2:/home/luis/video-server
cd video-server && nohup ./target/release/pravega-video-server && rm nohup.out resources/ target/ -rf && tar -zxvf pravega-video-server.tar.gz && sudo systemctrl restart pravega-video-server.service

PRAVEGA_CONTROLLER_URI=10.247.97.51:9090 POSTGRES_URI=postgres://admin:password@10.247.97.51:5432/hackathon ../target/debug/pravega-video-server

## env
postgresql: 10.247.97.51:5432 user/pass: admin/password
postgresql admin portal: http://10.247.97.51:8080/
pravega uri: tcp://10.247.97.51:9090

http://127.0.0.1:3030/player?scope=examples&stream=hls1&begin=2022-10-26T02:56:08Z&end=2022-10-26T02:56:12Z
http://10.247.97.51:3030/player?scope=examples&stream=hls1&begin=2022-10-26T07:14:19Z&end=2022-10-26T07:14:29Z
http://10.247.97.51:3030/player?scope=bilibili&stream=BV1hV4y157XN&begin=2022-10-26T10:34:15Z&end=2022-10-26T10:39:05Z

https://www.youtube.com/embed/A-UV7Z13uAQ


USE_NEW_NVSTREAMMUX=yes  gst-launch-1.0 uridecodebin3 uri=$input1 name=demux1 ! queue ! nvvideoconvert ! "video/x-raw(memory:NVMM)" ! mux1.sink_0 nvstreammux batch-size=1 sync-inputs=1 max-latency=250000000 name=mux1 ! queue ! nvmultistreamtiler ! nvvideoconvert ! "video/x-raw(memory:NVMM)" ! nvv4l2h264enc ! h264parse ! queue ! flvmux name=mux streamable=true ! filesink location=out.flv  async=0 qos=0 sync=1 demux1. ! queue ! audioconvert ! audiomux.sink_0 nvstreammux name=audiomux batch-size=1 max-latency=250000000 sync-inputs=1 ! nvstreamdemux name=audiodemux audiodemux.src_0  ! audioconvert ! mixer.sink_0 audiomixer latency=250000000 name=mixer ! queue ! avenc_aac ! aacparse ! queue ! mux. fakesrc num-buffers=0 is-live=1 ! mixer. -e

http://127.0.0.1:3030/player?scope=bilibili&stream=2&begin=2022-10-26T10:09:07Z&end=2022-10-26T10:13:57Z
