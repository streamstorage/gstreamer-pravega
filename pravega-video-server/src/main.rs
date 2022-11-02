//
// Copyright (c) Dell Inc., or its subsidiaries. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//

use clap::Clap;
use pravega_client::client_factory::ClientFactoryAsync;
use pravega_video::utils;
use std::path::Path;
use tokio::runtime::Runtime;
use tracing_subscriber::fmt::format::FmtSpan;
#[allow(unused_imports)]
use tracing::{error, info, info_span, warn, trace, event, Level, span};
use warp::Filter;

/// Serve HTTP Live Streaming (HLS) from a Pravega Video Stream.
/// Point your browser to: http://localhost:3030/player?scope=examples&stream=hlsav4
#[derive(Clap, Debug)]
struct Opts {
    /// Pravega controller in format "tcp://127.0.0.1:9090"
    #[clap(long, env = "PRAVEGA_CONTROLLER_URI", default_value = "tcp://127.0.0.1:9090")]
    pravega_controller_uri: String,
    /// The filename containing the Keycloak credentials JSON. If missing or empty, authentication will be disabled.
    #[clap(long, env = "KEYCLOAK_SERVICE_ACCOUNT_FILE", default_value = "", setting(clap::ArgSettings::AllowEmptyValues))]
    keycloak_service_account_file: String,
    /// Directory containing static files and templates.
    #[clap(long, env = "PRAVEGA_VIDEO_SERVER_RESOURCE_DIR", default_value = "./resources")]
    resource_dir: String,
    /// Postgres DB url in format "username:password@localhost/diesel_demo"
    #[clap(long, env = "POSTGRES_URI", default_value = "username:password@localhost/diesel_demo")]
    postgres_url: String,
}

fn main() {
    let opts: Opts = Opts::parse();

    let filter = std::env::var("RUST_LOG")
        .unwrap_or_else(|_| "pravega_video_server=debug,warp=debug,debug".to_owned());
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_span_events(FmtSpan::CLOSE)
        .init();
    info!("main: BEGIN");
    info!("opts={:?}", opts);

    let static_dir_name = format!("{}/static", opts.resource_dir);
    ensure_extra_files(opts.resource_dir.clone());

    // Use the Tokio runtime. It will also be used by Warp.
    let runtime  = Runtime::new().unwrap();
    let config = utils::create_client_config(opts.pravega_controller_uri, Some(opts.keycloak_service_account_file)).expect("creating config");
    let client_factory = ClientFactoryAsync::new(config, runtime.handle().to_owned());
    let database_url = opts.postgres_url.clone();

    runtime.block_on(async {
        let db = models::new(client_factory);
        let postgres = models::new_postgres(database_url);
        let api = filters::get_all_filters(db, postgres.clone());
        let ui = ui::get_all_filters(postgres);
        let static_dir = warp::path("static").and(warp::fs::dir(static_dir_name));
        // let redirect = warp::path::end().map(|| {
        //     warp::redirect::temporary(Uri::from_static("/static/hls-js.html"))
        // });

        let routes = api
            .or(ui)
            .or(static_dir)
            .with(warp::trace::request());
        warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
    })
}

fn ensure_extra_files(resource_dir: String) {
    for file in ["static/gap-5s.mp4", "static/hls-js.js", "static/player.css", "templates/player.html"] {
        let path = format!("{}/{}", resource_dir, file);
        if ! Path::new(&path).exists() {
            error!("Missing file to run pravega-video-server: {}", file);
            std::process::exit(1);
        }
    }
}

mod filters {
    use super::handlers;
    use super::models::{Postgres, Db, GetMediaSegmentOptions, GetM3u8PlaylistOptions};
    use warp::Filter;

    pub fn get_all_filters(
        db: Db,
        postgres: Postgres,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        get_media_segment(db.clone())
            .or(get_m3u8_playlist(db.clone()))
            .or(list_video_streams(db.clone()))
            .or(list_scopes(db.clone()))
            .or(get_danmu_file(postgres.clone()))
    }

    /// GET /scopes/my_scope/streams/my_stream/media?begin=0&end=204
    /// Returns a media segment consisting of fragmented MP4 or MPEG TS.
    pub fn get_media_segment(
        db: Db,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("scopes" / String / "streams" / String / "media" )
            .and(warp::get())
            .and(warp::query::<GetMediaSegmentOptions>())
            .and(with_db(db))
            .and_then(handlers::get_media_segment)
    }

    /// GET /scopes/my_scope/streams/my_stream/m3u8?begin=2021-04-19T00:00:00Z&end=2021-04-20T00:00:00Z
    pub fn get_m3u8_playlist(
        db: Db,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("scopes" / String / "streams" / String / "m3u8" )
            .and(warp::get())
            .and(warp::query::<GetM3u8PlaylistOptions>())
            .and(with_db(db))
            .and_then(handlers::get_m3u8_playlist)
            .with(warp::compression::gzip())
    }

    /// List scopes this player has access to
    /// GET /scopes
    pub fn list_scopes(
        db: Db,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("scopes")
            .and(warp::get())
            .and(with_db(db))
            .and_then(handlers::list_scopes)
    }

    /// List streams within the given scope
    /// GET /scopes/my_scope/streams
    pub fn list_video_streams(
        db: Db,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("scopes" / String / "streams")
            .and(warp::get())
            .and(with_db(db))
            .and_then(handlers::list_video_streams)
    }

    fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = std::convert::Infallible> + Clone {
        warp::any().map(move || db.clone())
    }

    /// GET /scopes/my_scope/streams/my_stream/danmu?begin=0&end=204
    /// Returns the danmu in bilibili xml format for the video clip.
    pub fn get_danmu_file(
        postgres: Postgres,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("scopes" / String / "streams" / String / "danmu" )
            .and(warp::get())
            .and(warp::query::<GetM3u8PlaylistOptions>())
            .and(with_postgres(postgres))
            .and_then(handlers::get_danmu_file)
    }
    fn with_postgres(postgres: Postgres) -> impl Filter<Extract = (Postgres,), Error = std::convert::Infallible> + Clone {
        warp::any().map(move || postgres.clone())
    }
}

mod ui {
    use chrono::{DateTime, Utc};
    use handlebars::Handlebars;
    use serde_derive::{Deserialize, Serialize};
    use super::*;
    use warp::Filter;
    use askama::Template;
    use super::models::Postgres;
    use diesel::pg::PgConnection;
    use diesel::prelude::*;
    use chrono::NaiveDateTime;

    #[derive(Debug, Deserialize, Serialize)]
    pub struct GetPlayerHtmlOptions {
        #[serde(rename = "scope")]
        pub scope_name: Option<String>,
        #[serde(rename = "stream")]
        pub stream_name: Option<String>,
        pub begin: Option<DateTime<Utc>>,
        pub end: Option<DateTime<Utc>>,
    }

    pub fn get_all_filters(
        postgres: Postgres,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        get_player_html()
        .or(get_library_html(postgres.clone()))
    }

    pub fn get_player_html(
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("player")
            .and(warp::get())
            .and(warp::query::<GetPlayerHtmlOptions>())
            .map(|opts: GetPlayerHtmlOptions| {
                let app_opts: Opts = Opts::parse();
                let template_file_name = format!("{}/templates/player.html", app_opts.resource_dir);
                let mut hb = Handlebars::new();
                let template_name = "player.html";
                hb.register_template_file(template_name, template_file_name).unwrap();
                let html = hb.render(template_name, &opts).unwrap();
                Ok(warp::reply::html(html))
            })
    }

    #[derive(Deserialize, Serialize, Queryable, Debug)]
    pub struct Video {
        pub id: i32,
        pub scope: String,
        pub stream: String,
        pub start_time: NaiveDateTime,
        pub end_time: NaiveDateTime,
        pub likes: i32,
    }

    diesel::table! {
        videos (id) {
            id -> Int4,
            scope -> Varchar,
            stream -> Varchar,
            start_time -> Timestamp,
            end_time -> Timestamp,
            likes -> Int4,
        }
    }

    #[derive(Template)]
    #[template(path = "library.html")]
    struct VideosTemplate<'a> {
        videos: &'a Vec<Video>,
    }

    mod filters {
        use chrono::NaiveDateTime;
        pub fn myfilter(s: &NaiveDateTime) -> ::askama::Result<String> {
            Ok(format!("{:?}", s))
        }
    }

    pub fn get_library_html(
        postgres: Postgres,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("videos")
            .and(warp::get())
            .and(with_postgres(postgres))
            .map(|postgres: Postgres| {
                let mut conn = PgConnection::establish(&postgres.database_url).unwrap_or_else(|_| panic!("Error connecting to {}", postgres.database_url));
                let videos = self::videos::dsl::videos
                    .load::<Video>(&mut conn)
                    .expect("Error loading video list");
                    info!("{:?}", videos);
                let template = VideosTemplate {
                        videos: &videos,
                    };
                let res = template
                    .render().unwrap();
                Ok(warp::reply::html(res))
            })
    }

    fn with_postgres(postgres: Postgres) -> impl Filter<Extract = (Postgres,), Error = std::convert::Infallible> + Clone {
        warp::any().map(move || postgres.clone())
    }
}

mod handlers {
    use std::convert::Infallible;
    use super::models::{Postgres, Db, GetMediaSegmentOptions, GetM3u8PlaylistOptions};
    use super::*;

    pub async fn get_media_segment(
        scope_name: String,
        stream_name: String,
        opts: GetMediaSegmentOptions,
        db: Db,
    ) -> Result<impl warp::Reply, Infallible> {
        db.get_media_segment(scope_name, stream_name, opts).await
    }

    pub async fn get_m3u8_playlist(
        scope_name: String,
        stream_name: String,
        opts: GetM3u8PlaylistOptions,
        db: Db,
    ) -> Result<impl warp::Reply, Infallible> {
        let playlist = db.get_m3u8_playlist(scope_name, stream_name, opts).await.unwrap();
        Ok(warp::reply::with_header(playlist, "content-type", "application/x-mpegURL"))
    }

    pub async fn list_scopes(
        db: Db,
    ) -> Result<impl warp::Reply, Infallible> {
        info!("list_scopes");
        let streams = db.list_scopes().await.unwrap();
        Ok(warp::reply::json(&streams))
    }

    pub async fn list_video_streams(
        scope_name: String,
        db: Db,
    ) -> Result<impl warp::Reply, Infallible> {
        info!("list_video_streams: scope_name={}", scope_name);
        let streams = db.list_video_streams(scope_name).await.unwrap();
        Ok(warp::reply::json(&streams))
    }

    pub async fn get_danmu_file(
        scope_name: String,
        stream_name: String,
        opts: GetM3u8PlaylistOptions,
        postgres: Postgres,
    ) -> Result<impl warp::Reply, Infallible> {
        postgres.get_danmu_file(scope_name, stream_name, opts).await
    }
}

mod models {
    use anyhow;
    use chrono::{DateTime, NaiveDateTime, Utc};
    use diesel::pg::PgConnection;
    use diesel::prelude::*;
    use futures::{StreamExt, future};
    use hyper::body::{Body, Bytes};
    use pravega_client::client_factory::ClientFactoryAsync;
    use pravega_client_shared::{Scope, ScopedStream, Stream};
    use pravega_controller_client::paginator::{list_streams_for_tag, list_scopes};
    use pravega_video::{event_serde::{EventReader}, index::IndexSearcher};
    use pravega_video::index::{IndexRecord, IndexRecordReader, SearchMethod, get_index_stream_name};
    use pravega_video::timestamp::PravegaTimestamp;
    use pravega_video::utils::SyncByteReader;
    use serde_derive::{Deserialize, Serialize};
    use std::convert::Infallible;
    use std::io::{ErrorKind, Read, Seek, SeekFrom};
    use super::*;

    #[derive(Deserialize, Serialize, Queryable)]
    pub struct Video {
        pub id: i32,
        pub scope: String,
        pub stream: String,
        pub start_time: NaiveDateTime,
        pub end_time: NaiveDateTime,
        pub likes: i32,
    }

    diesel::table! {
        videos (id) {
            id -> Int4,
            scope -> Varchar,
            stream -> Varchar,
            start_time -> Timestamp,
            end_time -> Timestamp,
            likes -> Int4,
        }
    }

    #[derive(Queryable)]
    pub struct Danmu {
        pub id: i32,
        pub video_id: i32,
        pub stime: f32,
        pub mode: i32,
        pub size: i32,
        pub color: i32,
        pub timestamp: i32,
        pub pool: i32,
        pub user_id: String,
        pub dbid: i64,
        pub mask: i32,
        pub content: String,
        pub score_color: Option<i32>,
    }

    diesel::table! {
        danmuku (id) {
            id -> Int4,
            video_id -> Int4,
            stime -> Float4,
            mode -> Int4,
            size -> Int4,
            color -> Int4,
            timestamp -> Int4,
            pool -> Int4,
            user_id -> Varchar,
            dbid -> Int8,
            mask -> Int4,
            content -> VarChar,
            score_color -> Nullable<Int4>,
        }
    }

    #[derive(Clone)]
    pub struct Postgres {
        pub database_url: String,
    }

    impl Postgres {
        pub async fn get_danmu_file(
            self,
            scope_name: String,
            stream_name: String,
            opts: GetM3u8PlaylistOptions,
        ) -> Result<impl warp::Reply, Infallible> {
            info!("get_danmu_file: scope_name={}, stream_name={}, begin={:?}, end={:?}", scope_name, stream_name, opts.begin, opts.end);
            assert!(opts.begin <= opts.end);

            let mut conn = PgConnection::establish(&self.database_url).unwrap_or_else(|_| panic!("Error connecting to {}", self.database_url));
            
            let videos = self::videos::dsl::videos
                .filter(self::videos::dsl::scope.eq(scope_name))
                .filter(self::videos::dsl::stream.eq(stream_name))
                .limit(1)
                .load::<Video>(&mut conn)
                .expect("Error loading video list");
            
            assert!(videos.len() == 1);

            let video_id = videos[0].id;
            let start_time = videos[0].start_time.clone();
            let datetime = DateTime::<Utc>::from_utc(start_time, Utc);
            let start_timestamp = datetime.with_timezone(&Utc);
            let begin = match opts.begin {
                Some(t) => {
                    if t < start_timestamp {
                        0 as f32
                    } else {
                        (t - start_timestamp).num_milliseconds() as f32 / 1000.0
                    }
                },
                None => 0 as f32,
            };
            let end = match opts.end {
                Some(t) => {
                    if t < start_timestamp {
                        0 as f32
                    } else {
                        (t - start_timestamp).num_milliseconds() as f32 / 1000.0
                    }
                },
                None => f32::MAX,
            };
            info!("get_danmu_file: video id={}, begin={}s, end={}s", video_id, begin, end);
            let danmus = self::danmuku::dsl::danmuku
                .filter(self::danmuku::dsl::video_id.eq(video_id))
                .filter(self::danmuku::dsl::stime.gt(begin))
                .filter(self::danmuku::dsl::stime.lt(end))
                .load::<Danmu>(&mut conn)
                .expect("Error loading danmu list");
            info!("get_danmu_file: danmu list length={}", danmus.len());
            let mut danmu_list = format!("");
            for item in danmus {
                let color = item.score_color.unwrap_or(item.color);
                danmu_list = format!("{}<d p=\"{},{},{},{},{},{},{},{},{}\">{}</d>", danmu_list, item.stime - begin, item.mode, item.size, color, item.timestamp, item.pool, item.user_id, item.dbid, item.mask, item.content);
            }

            //let danmu_list = r#"<d p="1,1,25,16777215,1666774369,0,18a4dd3d,1171747350759326976,11">好家伙，这个更可爱</d><d p="3.43600,1,25,16777215,1666773410,0,2d034e11,1171739307107660288,11">好可爱来姐姐亲亲</d>"#;
            let content = format!("<i><chatserver>chat.bilibili.com</chatserver><chatid>869480093</chatid><mission>0</mission><maxlimit>1000</maxlimit><state>0</state><real_name>0</real_name><source>k-v</source>{}</i>", danmu_list);
            let body = Body::from(content);
            
            Ok(warp::reply::with_header(warp::reply::Response::new(body), "content-type", "text/xml"))
        }
    }

    pub fn new_postgres(database_url: String) -> Postgres {
        Postgres { database_url }
    }

    #[derive(Clone)]
    pub struct Db {
        pub client_factory: ClientFactoryAsync,
    }

    pub fn new(client_factory: ClientFactoryAsync) -> Db {
        Db { client_factory }
    }

    // The query parameters for get_media_segment.
    #[derive(Debug, Deserialize)]
    pub struct GetMediaSegmentOptions {
        /// Begin byte offset
        pub begin: u64,
        /// End byte offset (exclusive)
        pub end: u64,
    }

    // The query parameters for get_m3u8_playlist.
    #[derive(Debug, Deserialize)]
    pub struct GetM3u8PlaylistOptions {
        pub begin: Option<DateTime<Utc>>,
        pub end: Option<DateTime<Utc>>,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct ListScopesResult {
        pub scopes: Vec<ListScopesRecord>,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct ListStreamsResult {
        pub streams: Vec<ListStreamsRecord>,
    }


    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct ListScopesRecord {
        #[serde(rename = "scopeName")]
        pub scope_name: String
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct ListStreamsRecord {
        #[serde(rename = "scopeName")]
        pub scope_name: String,
        #[serde(rename = "streamName")]
        pub stream_name: String,
    }

    impl Db {
        pub async fn get_media_segment(
            self,
            scope_name: String,
            stream_name: String,
            opts: GetMediaSegmentOptions,
        ) -> Result<impl warp::Reply, Infallible> {
            info!("get_media_segment: scope_name={}, stream_name={}, begin={}, end={}", scope_name, stream_name, opts.begin, opts.end);
            assert!(opts.begin <= opts.end);

            // TODO: Provide chunks to the HTTP client as a stream instead of buffering the entire response.

            // Use spawn_blocking to allow Pravega non-async methods to block this thread.
            // See https://stackoverflow.com/a/65452213/5890553.
            let client_factory = self.client_factory.clone();
            let (chunks, length) = client_factory.runtime_handle().spawn_blocking(move || {
                let span = span!(Level::INFO, "get_media_segment: SPAWNED THREAD");
                span.in_scope(|| {
                    info!("BEGIN");
                    let scoped_stream = ScopedStream {
                        scope: Scope::from(scope_name),
                        stream: Stream::from(stream_name),
                    };
                    let reader = client_factory.runtime_handle().block_on(client_factory.create_byte_reader(scoped_stream));
                    let mut reader = SyncByteReader::new(reader, client_factory.runtime_handle());
                    info!("Opened Pravega reader");

                    reader.seek(SeekFrom::Start(opts.begin)).unwrap();
                    let limit = opts.end - opts.begin;
                    let mut reader = reader.take(limit);

                    let mut chunks: Vec<Result<Bytes, std::io::Error>> = Vec::new();
                    let mut length = 0;
                    loop {
                        let mut event_reader = EventReader::new();
                        let required_buffer_length =
                            match event_reader.read_required_buffer_length(&mut reader) {
                                Ok(n) => n,
                                Err(e) if e.kind() == ErrorKind::UnexpectedEof && reader.limit() == 0 => {
                                    trace!("Reached requested end");
                                    break;
                                },
                                Err(e) => return Err(e),
                        };
                        let mut read_buffer: Vec<u8> = vec![0; required_buffer_length];
                        let event = match event_reader.read_event(&mut reader, &mut read_buffer[..]) {
                            Ok(n) => n,
                            Err(e) if e.kind() == ErrorKind::UnexpectedEof && reader.limit() == 0 => {
                                trace!("Reached requested end");
                                break;
                            },
                            Err(e) => return Err(e),
                        };
                        trace!("event={:?}", event);
                        length += event.payload.len();
                        chunks.push(Ok(Bytes::copy_from_slice(&event.payload)));
                    }
                    info!("Created {} chunks", chunks.len());
                    assert!(reader.limit() == 0);
                    info!("END");
                    Ok((chunks, length))
                })
            })
            .await
            .unwrap()
            .unwrap();

            trace!("get_media_segment: spawn_blocking done");
            let stream = futures_util::stream::iter(chunks);
            let body = Body::wrap_stream(stream);
            // TODO: Get content type from Pravega stream tag. For now "video/mp4" appears to work for MP4 and MPEG TS.
            // let content_type = "video/MP2T";
            let content_type = "video/mp4";
            //Ok(warp::reply::with_header(warp::reply::Response::new(body), "content-type", content_type))
            let builder = warp::http::response::Builder::new();
            Ok(builder
            .header("content-type", content_type)
            .header("Content-Length", length)
            .status(200)
            .body(body)
            .unwrap())
        }

        pub async fn get_m3u8_playlist(
            self,
            scope_name: String,
            stream_name: String,
            opts: GetM3u8PlaylistOptions,
        ) -> anyhow::Result<String> {

            info!("get_m3u8_playlist: BEGIN: scope_name={}, stream_name={}, begin={:?}, end={:?}", scope_name, stream_name, opts.begin, opts.end);

            let index_stream_name = get_index_stream_name(&stream_name);
            let begin_timestamp = PravegaTimestamp::from(opts.begin).or(PravegaTimestamp::MIN);
            let end_timestamp = PravegaTimestamp::from(opts.end).or(PravegaTimestamp::MAX);
            info!("get_m3u8_playlist: begin_timestamp={}, end_timestamp={}", begin_timestamp, end_timestamp);
            assert!(begin_timestamp <= end_timestamp);

            // Use spawn_blocking to allow Pravega non-async methods to block this thread.
            // See https://stackoverflow.com/a/65452213/5890553.

            let playlist = tokio::task::spawn_blocking(move || {
                let span = span!(Level::INFO, "get_m3u8_playlist: SPAWNED THREAD");
                span.in_scope(|| {
                    info!("BEGIN");
                    let client_factory = self.client_factory;
                    let scoped_stream = ScopedStream {
                        scope: Scope::from(scope_name),
                        stream: Stream::from(index_stream_name),
                    };
                    let index_reader = client_factory.runtime_handle().block_on(client_factory.create_byte_reader(scoped_stream));
                    let index_reader = SyncByteReader::new(index_reader, client_factory.runtime_handle());
                    info!("Opened Pravega reader");

                    let mut index_searcher = IndexSearcher::new(index_reader);
                    let begin_index_record = index_searcher.search_timestamp_and_return_index_offset(
                        begin_timestamp, SearchMethod::After)?;
                    let end_index_record = index_searcher.search_timestamp_and_return_index_offset(
                        end_timestamp, SearchMethod::After)?;
                    // Determine whether we can possibly get more data in the future.
                    // If the caller specified an end time and we already have an index record beyond this, then
                    // future appends will not affect our result.
                    // TODO: We can also guarantee this if the stream has been sealed.
                    let have_all_data = end_index_record.0.timestamp >= end_timestamp;
                    info!("begin_index_record={:?}, end_index_record={:?}, have_all_data={}",
                            begin_index_record, end_index_record, have_all_data);
                    let mut index_reader = index_searcher.into_inner();

                    // skip the record with false rand 
                    let index_begin_offset = if !(begin_index_record.0.random_access) {
                        let mut index_record_reader = IndexRecordReader::new();
                        let mut offset = begin_index_record.1;
                        loop {
                            offset += IndexRecord::RECORD_SIZE as u64;
                            index_reader.seek(SeekFrom::Start(offset)).unwrap();
                            let index_record = index_record_reader.read(&mut index_reader).unwrap();
                            if index_record.random_access {
                                break offset;
                            }
                        }
                    } else {
                        begin_index_record.1   
                    };

                    // Determine begin and end offsets of the index.
                    // let index_begin_offset = begin_index_record.1;
                    let index_end_offset = end_index_record.1 + IndexRecord::RECORD_SIZE as u64;
                    let index_size = index_end_offset - index_begin_offset;
                    info!("index_begin_offset={}, index_end_offset={}, index_size={}", index_begin_offset, index_end_offset, index_size);

                    // Position index reader at current beginning of the index.
                    index_reader.seek(SeekFrom::Start(index_begin_offset)).unwrap();

                    // Ensure EOF instead of waiting (potentially forever) for appends when we get to the current end.
                    let mut index_reader = index_reader.take(index_size);

                    // Media Sequence Number will always equal the index record number, even after truncation.
                    let initial_media_sequence_number: u64 = index_begin_offset / IndexRecord::RECORD_SIZE as u64;
                    info!("initial_media_sequence_number={}", initial_media_sequence_number);

                    // Initial value for target duration. This will be updated with an exponential moving average, then rounded.
                    let mut target_duration_seconds = 10.0;

                    let mut playlist_body = String::new();
                    let mut prev_index_record: Option<IndexRecord> = None;
                    let mut next_segment_discont = false;

                    loop {
                        let mut index_record_reader = IndexRecordReader::new();
                        let index_record = match index_record_reader.read(&mut index_reader) {
                            Ok(n) => n,
                            Err(e) if e.kind() == ErrorKind::UnexpectedEof && index_reader.limit() == 0 => {
                                trace!("Reached requested end");
                                break;
                            },
                            Err(e) => return Err(e),
                        };
                        trace!("index_record={:?}", index_record);
                        if let Some(prev_index_record) = prev_index_record {
                            // If index_record indicates a discontinuity, then assume there is a gap in the data
                            // between the previous record and this one.
                            // Any recorded content that falls in this gap may be corrupt so we will not display it.
                            // Instead, we'll play a short media segment containing blue video and silent audio.
                            // The length of this replacement content will be fixed, regardless of the timestamps.
                            // The EXT-X-GAP tag should be used for this but it doesn't appear to be supported by hls.js.
                            // It is possible that the duration of the gap in the index is very short or even 0.
                            // However, we still need to count the gap so that the Media Sequence Numbers
                            // correspond to the index offset.

                            let mut discont = index_record.discontinuity;
                            if discont {
                                warn!("Detected discontinuity; discontinuity flag set in {:?}", index_record);
                            } else {
                                if let Some(timestamp_nanos) = index_record.timestamp.nanoseconds() {
                                    let prev_timestamp_nanos = prev_index_record.timestamp.nanoseconds().unwrap();
                                    if timestamp_nanos < prev_timestamp_nanos {
                                        let rewind_seconds = (prev_timestamp_nanos - timestamp_nanos) as f64 * 1e-9;
                                        warn!("Detected discontinuity; rewind of {:.3} seconds from {} to {}",
                                        rewind_seconds, prev_index_record.timestamp, index_record.timestamp);
                                        discont = true;
                                    } else {
                                        let duration_seconds = (timestamp_nanos - prev_timestamp_nanos) as f64 * 1e-9;
                                        // If the timestamp increased by much more than the target duration,
                                        // then assume we have a discontinuity.
                                        if duration_seconds > target_duration_seconds + 1.0 {
                                            warn!("Detected discontinuity; {:.3} second gap from {} to {}, target_duration_seconds={:.3}",
                                                duration_seconds, prev_index_record.timestamp, index_record.timestamp, target_duration_seconds);
                                            discont = true;
                                        } else {
                                            if next_segment_discont {
                                                playlist_body.push_str("#EXT-X-DISCONTINUITY\n");
                                                next_segment_discont = false;
                                            }
                                            let ema_alpha = 0.1;
                                            target_duration_seconds = ema_alpha * duration_seconds + (1.0 - ema_alpha) * target_duration_seconds;
                                            let begin_offset = prev_index_record.offset;
                                            let end_offset = index_record.offset;
                                            // "#EXTINF:10," where 10 is the duration of the segment in seconds
                                            playlist_body.push_str(&format!("#EXTINF:{},\n", duration_seconds));
                                            // "#EXT-X-PROGRAM-DATE-TIME:2010-02-19T14:54:23.123456789Z"
                                            playlist_body.push_str(&format!("#EXT-X-PROGRAM-DATE-TIME:{}\n", prev_index_record.timestamp.to_iso_8601().unwrap()));
                                            // "media?begin=0&end=204" where 0 and 204 are the begin and end byte offsets
                                            playlist_body.push_str(&format!("media?begin={}&end={}\n", begin_offset, end_offset));
                                        }
                                    }
                                } else {
                                    warn!("Detected discontinuity; missing timestamp in index at offset {}",
                                        index_record.offset);
                                    discont = true;
                                }
                            }
                            if discont {
                                // warn!("Detected discontinuity; index_record={:?}", index_record);
                                let gap_content_duration_seconds = 5;
                                playlist_body.push_str("#EXT-X-DISCONTINUITY\n");
                                playlist_body.push_str(&format!("#EXTINF:{},\n", gap_content_duration_seconds));
                                playlist_body.push_str(&format!("/static/gap-{}s.mp4\n", gap_content_duration_seconds));
                                next_segment_discont = true;
                            }
                        }
                        prev_index_record = Some(index_record);
                    }

                    let mut playlist = String::new();
                    let target_duration_seconds = target_duration_seconds.round();
                    info!("target_duration_seconds={}", target_duration_seconds);
                    playlist.push_str("#EXTM3U\n#EXT-X-VERSION:3\n#EXT-X-ALLOW-CACHE:NO\n");
                    playlist.push_str(&format!("#EXT-X-MEDIA-SEQUENCE:{}\n", initial_media_sequence_number));
                    playlist.push_str(&format!("#EXT-X-TARGETDURATION:{}\n", target_duration_seconds));
                    playlist.push_str(&playlist_body);

                    // Write ENDLIST if we have all data up to the requested end time.
                    // This will prevent the browser from polling for updated playlists.
                    if have_all_data {
                        playlist.push_str("#EXT-X-ENDLIST\n");
                    }
                    info!("END");
                    Ok(playlist)
                })
            })
            .await??;
            trace!("get_m3u8_playlist: spawn_blocking done");
            trace!("get_m3u8_playlist: playlist={}", playlist);
            info!("get_m3u8_playlist: END");
            Ok(playlist)
        }

        pub async fn list_scopes(
            self
        ) -> anyhow::Result<ListScopesResult> {

            info!("list_scopes");
            let controller_client = self.client_factory.controller_client();
            let mut scopes = Vec::new();
            let mut had_error = false;

            list_scopes(controller_client).for_each(|scope| {
                if scope.is_ok() {
                    scopes.push(scope.unwrap())
                } else {
                    had_error = true;
                }

                future::ready(())
            }).await;

            if had_error {
                anyhow::bail!("Error listing scopes");
            }

            let scopes: Vec<_> = scopes.into_iter().map(|scope| ListScopesRecord {
                scope_name: scope.name.clone()
            }).collect();
            Ok(ListScopesResult { scopes })
        }

        pub async fn list_video_streams(
            self,
            scope_name: String,
        ) -> anyhow::Result<ListStreamsResult> {

            info!("list_video_streams: scope_name={}", scope_name.clone());
            let controller_client = self.client_factory.controller_client();
            let scope = Scope { name : scope_name.clone() };
            let mut streams = Vec::new();
            let mut had_error = false;
            list_streams_for_tag(scope, utils::get_video_tag_query(), controller_client).for_each(|stream| {
                if stream.is_ok() {
                    streams.push(stream.unwrap());
                } else {
                    had_error = true;
                }
                future::ready(())
            }).await;

            if had_error {
                anyhow::bail!("Error listing streams for scope={}", scope_name.clone());
            }
            let streams: Vec<_> = streams.into_iter().map(|scoped_stream| ListStreamsRecord {
                scope_name: scope_name.clone(),
                stream_name: scoped_stream.stream.name
            }).collect();
            Ok(ListStreamsResult { streams })
        }
    }
}
