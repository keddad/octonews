from abc import ABC, abstractmethod
from datetime import datetime
from time import mktime

import aiohttp
import feedparser
from bs4 import BeautifulSoup

from rss.utils import News


class MediaParser(ABC):

    @abstractmethod
    async def work(self):
        pass


class TassParser(MediaParser):
    rss_link = "http://tass.ru/rss/v2.xml"

    async def work(self):
        async with aiohttp.ClientSession() as session:
            async with session.get(self.rss_link) as resp:
                rss = await resp.text()

            rss = feedparser.parse(rss)

            news = []
            for rss_e in rss["entries"]:
                async with session.get(rss_e["link"]) as resp:
                    news_text = await resp.text()
                    news_soup = BeautifulSoup(news_text, 'html.parser')
                    block = news_soup.find(class_="text-block")
                    text = block.text
                    links = [link.get('href') for link in block.find_all('a')]
                    new = News(title=rss_e['title'], text=text, uri=rss_e['link'], links=links,
                               posted=datetime.fromtimestamp(mktime(rss_e["published_parsed"])).isoformat())

                    news.append(new)
            return news
