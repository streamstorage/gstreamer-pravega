CREATE TABLE danmuku (
    ID SERIAL,
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
    content text
);
CREATE INDEX danmuku_idx1 ON danmuku (video_id, stime);
