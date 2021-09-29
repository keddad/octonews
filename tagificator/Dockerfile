FROM python:3.9

ENV PYTHONDONTWRITEBYTECODE 1
ENV PYTHONUNBUFFERED 1

RUN pip install -U pip \
  && pip install poetry

WORKDIR /app
COPY . .

RUN poetry config virtualenvs.create false \
  && poetry install --no-interaction --no-ansi --no-dev

CMD python -m tagificator