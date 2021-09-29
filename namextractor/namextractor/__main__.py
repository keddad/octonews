import json

import redis
from loguru import logger
from nametract import extract
from namextractor.deduplicator import deduplicate
from json import loads, dumps

redis = redis.Redis(host="redis")


def main():
    logger.info("Namextractor up and running")
    while True:
        r_obj = redis.xread({"nametract": 0}, 1, 0)
        r_obj_id = r_obj[0][1][0][0]  # That is bad.

        redis.xdel("nametract", r_obj_id)

        data = loads(r_obj[0][1][0][1][b"news"])  # That is worse.
        data["names"] = deduplicate(extract(data["text"], minimal_name_size=3))
        redis.xadd("tagification", {"news": json.dumps(data, ensure_ascii=False)})

        logger.debug(f"Processed {data}")


main()
