from fastapi import FastAPI, Query, WebSocket
from pydantic import BaseModel, Field
from typing import List, Optional
import asyncio

app = FastAPI()

new_radar_data_event = asyncio.Event()
WEBSOCKET_QUEUE = asyncio.Queue()


# Set up CORS policy allowing all origins
@app.middleware("http")
async def add_cors_header(request, call_next):
    response = await call_next(request)
    response.headers["Access-Control-Allow-Origin"] = "*"
    return response


class RawRadarData(BaseModel):
    index: int = Field(..., description="The index of the radar data.")
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


@app.get("/add_radar_raw_data", response_model=RawRadarData)
async def add_radar_raw_data(
    target_index: int = Query(..., description="The index of the radar data."),
    distance: float = Query(..., description="The distance from the radar"),
    azimuth: float = Query(..., description="The azimuth angle of the radar"),
    elevation: float = Query(..., description="The elevation angle of the radar"),
    radar_rotation: float = Query(
        ..., description="Rotation speed or orientation of the radar unit."
    ),
    radar_unit_gps_location_x: float = Query(
        ..., description="The x-coordinate of the radar unit's GPS location"
    ),
    radar_unit_gps_location_y: float = Query(
        ..., description="The y-coordinate of the radar unit's GPS location"
    ),
    radar_unit_gps_location_z: float = Query(
        ..., description="The z-coordinate of the radar unit's GPS location"
    ),
) -> RawRadarData:
    new_radar_data = RawRadarData(
        index=index,
        distance=distance,
        azimuth=azimuth,
        elevation=elevation,
        radar_rotation=radar_rotation,
        radar_unit_gps_location_x=radar_unit_gps_location_x,
        radar_unit_gps_location_y=radar_unit_gps_location_y,
        radar_unit_gps_location_z=radar_unit_gps_location_z,
    )
    RADAR_DATA_STORE.append(new_radar_data)
    await WEBSOCKET_QUEUE.put(new_radar_data)
    new_radar_data_event.set()
    return new_radar_data


@app.get("/get_radar_all_data")
async def get_all_radar_data():
    return list(RADAR_DATA_STORE)


@app.websocket("/ws/new_radar_data")
async def websocket_endpoint(websocket: WebSocket):
    await websocket.accept()
    while True:
        await new_radar_data_event.wait()
        while not WEBSOCKET_QUEUE.empty():
            radar_data = await WEBSOCKET_QUEUE.get()
            await websocket.send_json(radar_data.dict())
            new_radar_data_event.clear()


if __name__ == "__main__":
    import uvicorn

    uvicorn.run(app, host="0.0.0.0", port=420)
