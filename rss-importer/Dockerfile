FROM python:3.9-slim

RUN pip install pipenv
COPY Pipfile* ./
RUN pipenv install --system

COPY rss rss/
CMD python -m rss