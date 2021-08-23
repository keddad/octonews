use log::{debug, error};
use redis::streams::StreamReadReply;
use redis::{cmd, from_redis_value, Client};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct News {
    title: String,
    text: String,
    uri: String,
    posted: String,
    links: Vec<String>,
}

fn main() {
    env_logger::init();
    let client = Client::open("redis://redis/").expect("Failed to connect to Redis");
    let mut connection = client
        .get_connection()
        .expect("Failed to get Redis connection");

    loop {
        let from_stream: StreamReadReply = cmd("XREAD")
            .arg(&["BLOCK", "0", "STREAMS", "deduplicator", "$"])
            .query(&mut connection)
            .expect("Idk, something wrong with XREAD");

        debug!("Got {:#?} from dedup queue", from_stream);

        for returned_stream in from_stream.keys.into_iter() {
            for redis_obj in returned_stream.ids.into_iter() {
                let _ = cmd("XDEL").arg(&["deduplicator", redis_obj.id.as_ref()]);
                if redis_obj.map.contains_key("news") {
                    let raw_news: String =
                        from_redis_value(redis_obj.map.get("news").expect("That can't happen."))
                            .expect("Failed to cast RedisValue to String");
                    let n_s: News =
                        serde_json::from_str(&raw_news).expect("Failed to serialise News");

                    let exists: i32 = cmd("EXISTS").arg(&n_s.uri).query(&mut connection).unwrap();

                    if exists == 1 {
                        debug!("{} already exists, dumping it", n_s.uri);
                    } else {
                        debug!("{} doesn't exist, processing it", n_s.uri);
                        let _: () = cmd("SET")
                            .arg(&[&n_s.uri, "Xs", "EX", "604800"])
                            .query(&mut connection)
                            .unwrap();

                        let _: () = cmd("XADD")
                            .arg(&[
                                "nametract",
                                "*",
                                "news",
                                &serde_json::to_string(&n_s).unwrap(),
                            ])
                            .query(&mut connection)
                            .expect("Failed to put object to stream");
                    }
                } else {
                    error!("Got object wihout news key from stream, {:#?}", redis_obj)
                }
            }
        }
    }
}
