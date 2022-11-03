import asyncio
import websockets
import json

async def hello():
    async with websockets.connect("ws://127.0.0.1:8080/chat") as websocket:
        await websocket.send('{"GetUsersIds": {"start": 0, "count": 5}}')
        print(await websocket.recv())


if __name__ == "__main__":
    asyncio.run(hello())