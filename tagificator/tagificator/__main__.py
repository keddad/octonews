import json

import redis
from loguru import logger
from json import loads, dumps

redis = redis.Redis(host="redis")

def main():
    logger.info("Namextractor up and running")
    while True:
        r_obj = redis.xread({"tagification": 0}, 1, 0)
        r_obj_id = r_obj[0][1][0][0]  # That is bad.

        redis.xdel("tagification", r_obj_id)

        data = loads(r_obj[0][1][0][1][b"news"])  # That is worse.
        data["tags"] = []
        redis.xadd("insertor", {"news": json.dumps(data, ensure_ascii=False)})

        logger.debug(f"Processed {data}")


main()