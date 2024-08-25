FROM python:3.11 AS builder

WORKDIR /app

ENV PYTHONUNBUFFERED=1 \
    PYTHONDONTWRITEBYTECODE=1

RUN pip install poetry \
    && poetry config virtualenvs.in-project true

COPY poetry.lock pyproject.toml ./
RUN poetry install --no-root

FROM python:3.11-slim-bookworm AS main

WORKDIR /app

RUN apt update && apt install -y curl \ 
    && apt clean && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/.venv /app/.venv
COPY entrypoint.sh /app/entrypoint.sh
