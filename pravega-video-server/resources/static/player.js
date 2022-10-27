
function load_video() {
    var scope = document.getElementById("scope").className;
    var stream = document.getElementById("stream").className;

    var query = ""
    var begin = document.getElementById("begin").className;
    if (begin != "") {
        query = query + ((query == "") ? "?" : "&") + "begin=" + new Date(begin).toISOString();
    }
    var end = document.getElementById("end").className;
    if (end != "") {
        query = query + ((query == "") ? "?" : "&") + "end=" + new Date(end).toISOString();
    }

    var manifestUri = "/scopes/" + scope + "/streams/" + stream + "/m3u8" + query;
    console.log(manifestUri);
    
    var art = new Artplayer({
        container: '.artplayer-app',
        url: manifestUri,
        title: scope + "/" + stream,
        autoplay: true,
        muted: true,
        loop: true,
        fullscreen: true,
        fullscreenWeb: true,
        type: 'm3u8',
        customType: {
            m3u8: async function (video, url) {
                if (Hls.isSupported()) {
                    const hls = new Hls();
                    hls.loadSource(url);
                    hls.attachMedia(video);
                } else if (video.canPlayType('application/vnd.apple.mpegurl')) {
                    video.src = url;
                } else {
                    art.notice.show = 'unsupported: m3u8';
                }
            },
        },
        plugins: [
            artplayerPluginDanmuku({
                danmuku: '/static/BV1hV4y157XN.xml',
                //mount: document.querySelector('.noshow')
            }),
        ],
    });

}

load_video();
