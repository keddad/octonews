import asyncio
import logging
from asyncio import run, as_completed
from os import environ

from rss.parsers import *
from rss.utils import News

logging.basicConfig(level=logging.WARNING)

parsers = [TassParser()]
QUEUER_URI = environ.get("QUEUER_URI")
DELAY = int(environ.get("DELAY", "60"))


async def send(item: News):
    async with aiohttp.ClientSession() as s:
        async with s.post(QUEUER_URI, data=item.to_json()) as resp:
            if resp.ok:
                logging.info(f"Sent {item.uri} to queuer")
            else:
                logging.warning(
                    f"Failed to send {item.uri} to queuer, {resp.status}")


async def main():
    while True:
        for parser in as_completed([x.work() for x in parsers]):
            news_list = await parser

            if len(news_list) == 0:
                logging.warning("Some parser returned list of 0 news")

            await asyncio.gather(*[send(x) for x in news_list])

        await asyncio.sleep(DELAY)


if __name__ == "__main__":
    run(main())
