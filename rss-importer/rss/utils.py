from dataclasses import dataclass
from datetime import datetime
from typing import List

from dataclasses_json import dataclass_json


@dataclass_json()
@dataclass()
class News:
    title: str
    text: str
    url: str
    links: List[str]
    posted: str
