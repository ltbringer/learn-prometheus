from fastapi import FastAPI, Request
from prometheus_fastapi_instrumentator import Instrumentator
import psutil
import os
import logging

app = FastAPI()

# Set up logging
logging.basicConfig(level=logging.INFO)

# Initialize and configure Prometheus metrics
Instrumentator().instrument(app).expose(app, endpoint="/metrics")

PROCESS = psutil.Process(os.getpid())


@app.get("/")
async def read_root():
    return {"message": "Hello from FastAPI!"}


# Webhook endpoint for Alertmanager
@app.post("/webhook")
async def receive_alert(request: Request):
    payload = await request.json()
    logging.info(f"Received alert: {payload}")
    return {"status": "received"}
