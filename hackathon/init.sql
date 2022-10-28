DROP TABLE IF EXISTS videos;
CREATE TABLE videos (
    id INT GENERATED ALWAYS AS IDENTITY,
    scope varchar(100) NOT NULL,
    stream varchar(100) NOT NULL,
    start_time timestamp NOT NULL,
    end_time timestamp NOT NULL,
    likes integer NOT NULL,
    PRIMARY KEY(id)
);

INSERT INTO videos(scope, stream, start_time, end_time, likes)
VALUES('bilibili', 'BV1hV4y157XN', '2022-10-26 10:34:15', '2022-10-26 10:39:05', 234);

DROP TABLE IF EXISTS danmuku;
CREATE TABLE danmuku (
    id INT GENERATED ALWAYS AS IDENTITY,
    video_id integer NOT NULL,
    stime real NOT NULL,
    mode integer NOT NULL,
    size integer NOT NULL,
    color integer NOT NULL,
    timestamp integer,
    pool integer,
    user_id varchar(50),
    dbid bigint,
    mask integer,
    content text,
    location varchar(50),
    CONSTRAINT fk_videos
      FOREIGN KEY(videos_id) 
	  REFERENCES videos(id)
	  ON DELETE CASCADE
);
CREATE INDEX danmuku_idx1 ON danmuku (video_id, stime);
