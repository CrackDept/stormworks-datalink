from fastapi import FastAPI, Query, HTTPException
from pydantic import BaseModel, Field
import socketio
import asyncio
import logging

app = FastAPI()
logger = logging.getLogger("radar_app")
sio = socketio.AsyncClient()


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


@app.on_event("startup")
async def startup_event():
    asyncio.create_task(connect_and_handle_reconnect())


async def connect_and_handle_reconnect():
    while True:
        if not sio.connected:
            try:
                await sio.connect("http://172.28.158.49:5000", namespaces=["/"])
                logger.info("Connected to Socket.IO server")
            except socketio.exceptions.ConnectionError as e:
                logger.error(f"Failed to connect to Socket.IO server: {e}")
            except Exception as e:
                logger.error(f"Unexpected error: {e}")
        await asyncio.sleep(5)


async def safe_emit(event, data, namespace="/"):
    if sio.connected:
        try:
            await sio.emit(event, data, namespace=namespace)
            return True
        except Exception as e:
            logger.error(f"Failed to send data to Socket.IO server: {e}")
            return False
    else:
        logger.warning("Not connected to Socket.IO server.")
        return False


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
    success = await safe_emit("new_radar_data", new_radar_data.dict())
    if not success:
        raise HTTPException(
            status_code=503, detail="Failed to send data to Socket.IO server."
        )

    RADAR_DATA_STORE.append(new_radar_data)
    return new_radar_data


@app.get("/get_radar_all_data")
async def get_all_radar_data():
    return list(RADAR_DATA_STORE)


if __name__ == "__main__":
    import uvicorn

    uvicorn.run(app, host="0.0.0.0", port=420)
