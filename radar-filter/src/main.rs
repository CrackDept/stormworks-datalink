use socketioxide::{
    extract::{AckSender, Data, SocketRef},
    SocketIo,
};
use tracing::{debug, info, Level};
use tracing_subscriber::FmtSubscriber;

use serde::{Deserialize, Serialize};

// Angle represented as a number between 0.5 and -0.5
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
struct Turns(f64);

// Angle represented as a number between pi and -pi
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
struct Radians(f64);

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[allow(dead_code)]
struct RawRadarReturn {
    #[serde(rename = "target_index")]
    pub index: u8,

    #[serde(rename = "distance")]
    pub dst: f64,

    #[serde(rename = "azimuth")]
    pub azm: f64,

    #[serde(rename = "elevation")]
    pub elv: f64,

    #[serde(rename = "radar_rotation")]
    pub r: f64,

    #[serde(rename = "radar_unit_gps_location_x")]
    pub x: f64,

    #[serde(rename = "radar_unit_gps_location_y")]
    pub y: f64,

    #[serde(rename = "radar_unit_gps_location_z")]
    pub z: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
struct RadarReturn {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set log level
    tracing::subscriber::set_global_default(
        FmtSubscriber::builder()
            .with_max_level(Level::TRACE)
            .finish(),
    )?;

    // init socketio
    let (layer, io) = SocketIo::new_layer();

    // listen on root route
    io.ns("/", |s: SocketRef| {
        info!("New connection: {}", s.id);

        // Log new radar returns
        s.on(
            "new_radar_data",
            |_s: SocketRef, ack: AckSender, Data::<RawRadarReturn>(target)| {
                ack.send(target).ok();
                debug!("New radar return, {:#?}", target);
            },
        )
    });

    // make adress
    let host = "0.0.0.0";
    let port = "5000";
    let adress = format!("{host}:{port}");

    debug!("Starting server {adress}");

    let app = axum::Router::new().layer(layer);
    let listener = tokio::net::TcpListener::bind(adress).await.unwrap();

    info!("Server started");

    axum::serve(listener, app).await.unwrap();

    Ok(())
}

impl From<RawRadarReturn> for RadarReturn {
    fn from(_raw: RawRadarReturn) -> Self {

        todo!();
    }
}

impl From<Turns> for Radians {
    fn from(_t: Turns) -> Self {
        todo!();
    }
}
