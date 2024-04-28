from fastapi import FastAPI, Query
from fastapi.concurrency import asynccontextmanager
from pydantic import BaseModel, Field
from typing import List, Optional
import socketio
import asyncio

import socketio.exceptions

app = FastAPI()


sio = socketio.AsyncClient()
message_queue = asyncio.Queue()


@asynccontextmanager
async def lifespan(app: FastAPI):
    asyncio.create_task(connect_and_handle_reconnect())


class RawRadarData(BaseModel):
    target_index: int = Field(..., description="The target index of the radar data.")
    distance: float = Field(..., description="The distance from the radar.")
    azimuth: float = Field(..., description="The azimuth angle of the radar.")
    elevation: float = Field(..., description="The elevation angle of the radar.")
    radar_rotation: float = Field(
        ..., description="Rotation speed or orientation of the radar unit."
    )
    radar_unit_gps_location_x: float = Field(
        ..., description="The x-coordinate of the radar unit's GPS location."
    )
    radar_unit_gps_location_y: float = Field(
        ..., description="The y-coordinate of the radar unit's GPS location."
    )
    radar_unit_gps_location_z: float = Field(
        ..., description="The z-coordinate of the radar unit's GPS location."
    )


RADAR_DATA_STORE: list[RawRadarData] = []


async def connect_and_handle_reconnect():
    while True:
        if not sio.connected:
            try:
                await sio.connect("http://172.28.158.49:5000")
                print("Connected to Socket.IO server")
                await handle_queued_messages()
            except socketio.exceptions.ConnectionError as e:
                print(f"Failed to connect to Socket.IO server: {e}")
                await asyncio.sleep(5)  # Wait for 5 seconds before trying to reconnect
            except Exception as e:
                print(f"Unexpected error: {e}")
                await asyncio.sleep(5)  # General error handling
        else:
            await asyncio.sleep(1)  # Sleep if already connected to avoid tight loop


async def handle_queued_messages():
    while not message_queue.empty():
        data = await message_queue.get()
        try:
            await sio.emit("new_radar_data", data.dict())
        except Exception as e:
            message_queue.put_nowait(
                data
            )  # Put the data back at the front of the queue
            break  # Exit the loop and wait for reconnection to retry


@app.get("/add_radar_raw_data")
async def add_radar_raw_data(
    target_index: int = Query(...),
    distance: float = Query(...),
    azimuth: float = Query(...),
    elevation: float = Query(...),
    radar_rotation: float = Query(...),
    radar_unit_gps_location_x: float = Query(...),
    radar_unit_gps_location_y: float = Query(...),
    radar_unit_gps_location_z: float = Query(...),
):
    new_radar_data = RawRadarData(
        target_index=target_index,
        distance=distance,
        azimuth=azimuth,
        elevation=elevation,
        radar_rotation=radar_rotation,
        radar_unit_gps_location_x=radar_unit_gps_location_x,
        radar_unit_gps_location_y=radar_unit_gps_location_y,
        radar_unit_gps_location_z=radar_unit_gps_location_z,
    )
    RADAR_DATA_STORE.append(new_radar_data)
    try:
        await sio.emit("new_radar_data", new_radar_data.dict(), callback=ack)
        if message_queue.qsize() > 0:
            print("Sending queued messages")
            # If there are messages in the queue, send them all
            await handle_queued_messages()
    except Exception as e:
        message_queue.put_nowait(new_radar_data)  # Add to queue if emit fails
        return {
            "notice": "Socket.IO server not connected but the data was stored and will be sent later when connection is restored.",
            "queue_length": message_queue.qsize(),
            "data": new_radar_data.dict(),
        }

    return new_radar_data


async def ack(data):
    print("acknowledged", data)


@app.get("/queue_length")
async def get_queue_length():
    return {"queue_length": message_queue.qsize()}


@app.get("/get_radar_all_data")
async def get_all_radar_data():
    return list(RADAR_DATA_STORE)


if __name__ == "__main__":
    import uvicorn

    uvicorn.run(app, host="0.0.0.0", port=420)
