# 1
VIDEO_FILE="/home/luis/Downloads/bilibili/猫妈妈和小奶猫.mp4" PRAVEGA_STREAM=BV1xA411v7jA PRAVEGA_CONTROLLER=10.247.97.51:9090 ./scripts/file-to-pravega.sh
PRAVEGA_CONTROLLER_URI=10.247.97.51:9090 PRAVEGA_SCOPE=bilibili PRAVEGA_STREAM=BV1xA411v7jA ./scripts/pravega-table-updater.sh

INSERT INTO videos(scope, stream, start_time, end_time, likes, title, category)
VALUES('bilibili', 'BV1xA411v7jA', '2022-11-02 05:20:36', '2022-11-02 05:30:55', 25111, '猫妈妈和小奶猫', '萌宠');

python ./hackathon/danmu-loader.py --danmu-file "/home/luis/Downloads/bilibili/猫 妈 妈 和 小 奶 猫.xml" --video-id 2


# 2
VIDEO_FILE="/home/luis/Downloads/bilibili/猫：哈哈哈，又是手欠的一天！.mp4" PRAVEGA_STREAM=BV1DL4y1w7Gy PRAVEGA_CONTROLLER=10.247.97.51:9090 ./scripts/file-to-pravega.sh
PRAVEGA_CONTROLLER_URI=10.247.97.51:9090 PRAVEGA_SCOPE=bilibili PRAVEGA_STREAM=BV1DL4y1w7Gy ./scripts/pravega-table-updater.sh

INSERT INTO videos(scope, stream, start_time, end_time, likes, title, category)
VALUES('bilibili', 'BV1DL4y1w7Gy', '2022-11-02 05:46:09', '2022-11-02 05:49:36', 1216, '猫：哈哈哈，又是手欠的一天！', '萌宠');

python ./hackathon/danmu-loader.py --danmu-file "/home/luis/Downloads/bilibili/猫：哈哈哈，又是手欠的一天！.xml" --video-id 3


# 3
VIDEO_FILE="/home/luis/Downloads/bilibili/所以这个视频就叫“辣椒的一生”？.mp4" PRAVEGA_STREAM=BV12E41167Hg PRAVEGA_CONTROLLER=10.247.97.51:9090 ./scripts/file-to-pravega.sh
PRAVEGA_CONTROLLER_URI=10.247.97.51:9090 PRAVEGA_SCOPE=bilibili PRAVEGA_STREAM=BV12E41167Hg ./scripts/pravega-table-updater.sh

INSERT INTO videos(scope, stream, start_time, end_time, likes, title, category)
VALUES('bilibili', 'BV12E41167Hg', '2022-11-02 05:59:00', '2022-11-02 06:10:32', 51111, '所以这个视频就叫“辣椒的一生”？', '人文');

python ./hackathon/danmu-loader.py --danmu-file "/home/luis/Downloads/bilibili/所以这个视频就叫“辣椒的一生”？.xml" --video-id 4


# 4
VIDEO_FILE="/home/luis/Downloads/bilibili/【桃花节】骑马踏青，取花为食，才不辜负春日好时节——上巳节.mp4" PRAVEGA_STREAM=BV1c54y1d7Tv PRAVEGA_CONTROLLER=10.247.97.51:9090 ./scripts/file-to-pravega.sh
PRAVEGA_CONTROLLER_URI=10.247.97.51:9090 PRAVEGA_SCOPE=bilibili PRAVEGA_STREAM=BV1c54y1d7Tv ./scripts/pravega-table-updater.sh

INSERT INTO videos(scope, stream, start_time, end_time, likes, title, category)
VALUES('bilibili', 'BV1c54y1d7Tv', '2022-11-02 06:11:54', '2022-11-02 06:21:58', 1321111, '【桃花节】骑马踏青，取花为食，才不辜负春日好时节——上巳节', '人文');

python ./hackathon/danmu-loader.py --danmu-file "/home/luis/Downloads/bilibili/【桃花节】骑马踏青，取花为食，才不辜负春日好时节——上巳节.xml" --video-id 6


# 5
VIDEO_FILE="/home/luis/Downloads/bilibili/出来混，总是要胖的【4】.mp4" PRAVEGA_STREAM=BV1wt4y1u7VZ PRAVEGA_CONTROLLER=10.247.97.51:9090 ./scripts/file-to-pravega.sh
PRAVEGA_CONTROLLER_URI=10.247.97.51:9090 PRAVEGA_SCOPE=bilibili PRAVEGA_STREAM=BV1wt4y1u7VZ ./scripts/pravega-table-updater.sh

INSERT INTO videos(scope, stream, start_time, end_time, likes, title, category)
VALUES('bilibili', 'BV1wt4y1u7VZ', '2022-11-02 06:17:03', '2022-11-02 06:20:00', 43111, '出来混，总是要胖的【4】', '美食');

python ./hackathon/danmu-loader.py --danmu-file "/home/luis/Downloads/bilibili/出来混，总是要胖的【4】.xml" --video-id 5



# 6
VIDEO_FILE="/home/luis/Downloads/bilibili/出来混，总是要胖的\!【2】.mp4" PRAVEGA_STREAM=BV1v34y1b7es PRAVEGA_CONTROLLER=10.247.97.51:9090 ./scripts/file-to-pravega.sh
PRAVEGA_CONTROLLER_URI=10.247.97.51:9090 PRAVEGA_SCOPE=bilibili PRAVEGA_STREAM=BV1v34y1b7es ./scripts/pravega-table-updater.sh

INSERT INTO videos(scope, stream, start_time, end_time, likes, title, category)
VALUES('bilibili', 'BV1v34y1b7es', '2022-11-02 06:52:18', '2022-11-02 06:55:25', 35111, '出来混，总是要胖的!【2】', '美食');

python ./hackathon/danmu-loader.py --danmu-file "/home/luis/Downloads/bilibili/出来混，总是要胖的【2】.xml" --video-id 7


# 7
VIDEO_FILE="/home/luis/Downloads/bilibili/《冰雪奇缘》艾莎两次华丽变身名场面完整版剪辑.mp4" PRAVEGA_STREAM=BV1R7411E7PL PRAVEGA_CONTROLLER=10.247.97.51:9090 ./scripts/file-to-pravega.sh
PRAVEGA_CONTROLLER_URI=10.247.97.51:9090 PRAVEGA_SCOPE=bilibili PRAVEGA_STREAM=BV1R7411E7PL ./scripts/pravega-table-updater.sh

INSERT INTO videos(scope, stream, start_time, end_time, likes, title, category)
VALUES('bilibili', 'BV1R7411E7PL', '2022-11-02 07:52:46', '2022-11-02 08:00:31', 4004, '《冰雪奇缘》艾莎两次华丽变身名场面完整版剪辑', '动画');

python ./hackathon/danmu-loader.py --danmu-file "/home/luis/Downloads/bilibili/《冰雪奇缘》艾莎两次华丽变身名场面完整版剪辑.xml" --video-id 8


# 8
VIDEO_FILE="/home/luis/Downloads/bilibili/大鱼海棠\ 《大鱼》周深MV\ 完整版.mp4" PRAVEGA_STREAM=BV1Xs411s7HX PRAVEGA_CONTROLLER=10.247.97.51:9090 ./scripts/file-to-pravega.sh
PRAVEGA_CONTROLLER_URI=10.247.97.51:9090 PRAVEGA_SCOPE=bilibili PRAVEGA_STREAM=BV1Xs411s7HX ./scripts/pravega-table-updater.sh

INSERT INTO videos(scope, stream, start_time, end_time, likes, title, category)
VALUES('bilibili', 'BV1Xs411s7HX', '2022-11-02 08:04:02', '2022-11-02 08:09:20', 22111, '大鱼海棠 《大鱼》周深MV 完整版', '动画');

python ./hackathon/danmu-loader.py --danmu-file "/home/luis/Downloads/bilibili/大鱼海棠 《大鱼》周深MV 完整版.xml" --video-id 9


# 9
VIDEO_FILE="/home/luis/Downloads/bilibili/亲爱的旅人啊【千与千寻】.mp4" PRAVEGA_STREAM=BV1A4411N7Kb PRAVEGA_CONTROLLER=10.247.97.51:9090 ./scripts/file-to-pravega.sh
PRAVEGA_CONTROLLER_URI=10.247.97.51:9090 PRAVEGA_SCOPE=bilibili PRAVEGA_STREAM=BV1A4411N7Kb ./scripts/pravega-table-updater.sh

INSERT INTO videos(scope, stream, start_time, end_time, likes, title, category)
VALUES('bilibili', 'BV1A4411N7Kb', '2022-11-02 08:12:04', '2022-11-02 08:16:18', 357000, '亲爱的旅人啊【千与千寻】', '动画');

python ./hackathon/danmu-loader.py --danmu-file "/home/luis/Downloads/bilibili/亲爱的旅人啊【千与千寻】.xml" --video-id 10


# 10
VIDEO_FILE="/home/luis/Downloads/bilibili/应观众要求要看给蛇喂仓鼠是啥样的。。。.mp4" PRAVEGA_STREAM=BV13s411m7v5 PRAVEGA_CONTROLLER=10.247.97.51:9090 ./scripts/file-to-pravega.sh
PRAVEGA_CONTROLLER_URI=10.247.97.51:9090 PRAVEGA_SCOPE=bilibili PRAVEGA_STREAM=BV13s411m7v5 ./scripts/pravega-table-updater.sh

INSERT INTO videos(scope, stream, start_time, end_time, likes, title, category)
VALUES('bilibili', 'BV13s411m7v5', '2022-11-02 08:17:41', '2022-11-02 08:37:28', 1834, '应观众要求要看给蛇喂仓鼠是啥样的。。。', '恶心不适');

python ./hackathon/danmu-loader.py --danmu-file "/home/luis/Downloads/bilibili/应观众要求要看给蛇喂仓鼠是啥样的。。。.xml" --video-id 11


# 11
VIDEO_FILE="/home/luis/Downloads/bilibili/【密恐慎入】暗黑系水宝宝.mp4" PRAVEGA_STREAM=BV1B54y117gy PRAVEGA_CONTROLLER=10.247.97.51:9090 ./scripts/file-to-pravega.sh
PRAVEGA_CONTROLLER_URI=10.247.97.51:9090 PRAVEGA_SCOPE=bilibili PRAVEGA_STREAM=BV1B54y117gy ./scripts/pravega-table-updater.sh

INSERT INTO videos(scope, stream, start_time, end_time, likes, title, category)
VALUES('bilibili', 'BV1B54y117gy', '2022-11-02 08:23:47', '2022-11-02 08:26:02', 225, '【密恐慎入】暗黑系水宝宝', '恶心不适');

python ./hackathon/danmu-loader.py --danmu-file "/home/luis/Downloads/bilibili/【密恐慎入】暗黑系水宝宝.xml" --video-id 12


# 12
VIDEO_FILE="/home/luis/Downloads/bilibili/世界上最令人不适的视频#124.mp4" PRAVEGA_STREAM=BV11x411g7SE PRAVEGA_CONTROLLER=10.247.97.51:9090 ./scripts/file-to-pravega.sh
PRAVEGA_CONTROLLER_URI=10.247.97.51:9090 PRAVEGA_SCOPE=bilibili PRAVEGA_STREAM=BV11x411g7SE ./scripts/pravega-table-updater.sh

INSERT INTO videos(scope, stream, start_time, end_time, likes, title, category)
VALUES('bilibili', 'BV11x411g7SE', '2022-11-02 08:39:11', '2022-11-02 08:49:22', 2037, '世界上最令人不适的视频#124', '恶心不适');

python ./hackathon/danmu-loader.py --danmu-file "/home/luis/Downloads/bilibili/世界上最令人不适的视频#124.xml" --video-id 13
