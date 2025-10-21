import os
from typing import Type

import requests
from crewai.tools import BaseTool
from pydantic import BaseModel, Field

class PushNotificationInput(BaseModel):
    """Input schema for PushTool."""
    message: str = Field(description="The message to be send the user")

class PushNotificationTool(BaseTool):
    name: str = "Send a push notification"
    description: str = "This tool is used to send a push notification to the user."
    args_schema: Type[BaseModel] = PushNotificationInput

    def _run(self, message: str) -> str:
        user = os.getenv("PUSHOVER_USER")
        token = os.getenv("PUSHOVER_TOKEN")
        url = os.getenv("PUSHOVER_URL")
        device = os.getenv("PUSHOVER_DEVICE")

        print(f"Push: {message}")
        payload = {"user": user, "token": token, "device": device, "message": message}
        requests.post(url, payload)
        return '{"success": "ok"}'
