
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

    var mode = document.getElementById("mode").className;
    var danmuUri = "/scopes/" + scope + "/streams/" + stream + "/danmu" + query;
    if (mode != "") {
        danmuUri = danmuUri + "&mode=" + mode.toString();
    }

    console.log(manifestUri);
    console.log(danmuUri);
    
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
        layers: [
            {
                name: 'title',
                html: '<span>' + scope + "/" + stream + '<br />' + begin + '<br />~ ' + end + '</span>',
                style: {
                    position: 'absolute',
                    top: '15px',
                    right: '15px',
                    display: 'none',
                    'text-align': 'right',
                },
            },
        ],
        plugins: [
            artplayerPluginDanmuku({
                danmuku: danmuUri,
                fontSize: '2%',
                //mount: document.querySelector('.noshow')
            }),
        ],
    });
    art.on('pause', () => {
        art.layers.title.style.display = 'block';
    });
    
    art.on('play', () => {
        art.layers.title.style.display = 'none';
    });
}

load_video();
