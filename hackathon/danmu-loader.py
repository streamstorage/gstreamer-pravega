import xml.etree.ElementTree as ET
import psycopg2
import configargparse as argparse

def parse_p(str):
    splits = str.split(",")
    return splits

def parse_xml(xmlfile, video_id):
    # create element tree object
    tree = ET.parse(xmlfile)
    # get root element
    root = tree.getroot()

    danmu_list = []

    # iterate news items
    for item in root.findall('d'):
        p = parse_p(item.attrib['p'])
        assert len(p) == 9, f"{p} length not equals to 9, got: {len(p)}"

     
        # video_id integer NOT NULL,
        # stime real NOT NULL,
        # mode integer NOT NULL,
        # size integer NOT NULL,
        # color integer NOT NULL,
        # timestamp integer,
        # pool integer,
        # user_id varchar(50),
        # dbid bigint,
        # mask integer,
        # content text
        danmu_item = (video_id, float(p[0]), int(p[1]), int(p[2]), int(p[3]), int(p[4]), int(p[5]), p[6], int(p[7]), int(p[8]), item.text)

        print(danmu_item)

        danmu_list.append(danmu_item)
      
    # return news items list
    return danmu_list

def insert_danmu_list(danmu_list, host, port, db, login, password):
    conn = None

    sql = "INSERT INTO danmuku(video_id, stime, mode, size, color, timestamp, pool, user_id, dbid, mask, content) VALUES(%s, %s, %s, %s, %s, %s, %s, %s, %s, %s, %s)"

    try:
        conn = psycopg2.connect(
            database=db, user=login, password=password, host=host, port=port
        )

        cur = conn.cursor()
        # execute the INSERT statement
        cur.executemany(sql, danmu_list)
        # commit the changes to the database
        conn.commit()
        # close communication with the database
        cur.close()
    except (Exception, psycopg2.DatabaseError) as error:
        print(f"Failed to insert danmu list: {error}")
    finally:
        if conn is not None:
            conn.close()
      
def main():
    parser = argparse.ArgumentParser(
        description="Parse bilibili danmu xml file into postgres db",
        auto_env_var_prefix="")
    parser.add_argument("--host", default="10.247.97.51")
    parser.add_argument("--port", default="5432")
    parser.add_argument("--db", default="hackathon")
    parser.add_argument("--login", default="admin")
    parser.add_argument("--password", default="password")
    parser.add_argument("--video-id", type=int, default=1)
    parser.add_argument("--danmu-file", default="/home/luis/projects/nautilus-gstreamer/gstreamer-pravega/pravega-video-server/resources/static/BV1hV4y157XN.xml")
    args = parser.parse_args()

    # parse xml file
    danmu_list = parse_xml(args.danmu_file, args.video_id)
    insert_danmu_list(danmu_list, args.host, args.port, args.db, args.login, args.password)
      
if __name__ == "__main__":
    # calling main function
    main()
