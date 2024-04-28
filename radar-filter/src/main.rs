use axum::handler::Handler;
use socketioxide::{
    extract::{AckSender, Data, SocketRef, State},
    SocketIo,
};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{debug, info, Level};
use tracing_subscriber::FmtSubscriber;

use serde::{Deserialize, Serialize};
use std::f64::consts::PI;
use std::ops::{Add, Sub};

const PI2: f64 = PI * 2.0;

// Angle represented as a number between 0.5 and -0.5
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, PartialOrd)]
struct Turns(f64);

// Angle represented as a number between pi and -pi
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, PartialOrd)]
struct Radians(f64);

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, PartialOrd)]
#[allow(dead_code)]
struct RawRadarReturn {
    #[serde(rename = "target_index")]
    pub index: u8,

    #[serde(rename = "distance")]
    pub dst: f64,

    #[serde(rename = "azimuth")]
    pub azm: Turns,

    #[serde(rename = "elevation")]
    pub elv: Turns,

    #[serde(rename = "radar_rotation")]
    pub r: Turns,

    #[serde(rename = "radar_unit_gps_location_x")]
    pub x: f64,

    #[serde(rename = "radar_unit_gps_location_y")]
    pub y: f64,

    #[serde(rename = "radar_unit_gps_location_z")]
    pub z: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, PartialOrd)]
struct RadarReturn {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, PartialOrd)]
struct Target {
    pub friendly: bool,
    pub id: u64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set log level
    tracing::subscriber::set_global_default(
        FmtSubscriber::builder()
            .with_max_level(Level::INFO)
            .finish(),
    )?;

    // init socketio
    let targets: Vec<Target> = vec![Target {
        friendly: false,
        id: 1337,
        x: 1.0,
        y: 1.0,
        z: 1.0,
    }];

    let (layer, io) = SocketIo::builder().with_state(targets).build_layer();

    // listen on root route
    io.ns("/", |s: SocketRef| {
        info!("New connection: {}", s.id);

        // Log new radar returns
        s.on(
            "new_radar_data",
            |s: SocketRef,
             ack: AckSender,
             Data::<RawRadarReturn>(target),
             targets: State<Vec<Target>>| {
                let target = RadarReturn::from(target);
                info!("{:?}", target);
                ack.send(target).ok();

                s.broadcast().emit("global_targets", targets.0).ok();
            },
        );
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
    fn from(raw: RawRadarReturn) -> Self {
        // Convert to radians
        let azm: Radians = raw.azm.into();
        let elv: Radians = raw.elv.into();
        let rot: Radians = raw.r.into();

        let azm = azm + rot;
        let dst = raw.dst;

        let x = azm.cos() * dst;
        let y = azm.sin() * dst;
        let z = elv.sin() * dst;

        RadarReturn {
            x: raw.x + x,
            y: raw.y + y,
            z: raw.z + z,
        }
    }
}

impl From<Turns> for Radians {
    fn from(t: Turns) -> Self {
        Self(t.0 * PI2)
    }
}

impl From<Radians> for Turns {
    fn from(r: Radians) -> Self {
        Self(r.0 / PI2)
    }
}

impl Add for Radians {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Radians(rhs.0 + self.0)
    }
}

impl Sub for Radians {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Radians(rhs.0 - self.0)
    }
}

impl Radians {
    pub fn cos(&self) -> f64 {
        self.0.cos()
    }

    pub fn sin(&self) -> f64 {
        self.0.sin()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_return() {
        let raw = RawRadarReturn {
            index: 1,
            dst: 1.0,
            azm: Turns(0.25),
            elv: Turns(0.0),
            r: Turns(0.0),
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };

        let expected = RadarReturn {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };

        let actual = RadarReturn::from(raw);

        assert!(actual.x - expected.x <= 6.3e-17);
        assert!(actual.y - expected.y <= 0.0);
        assert!(actual.z - expected.z <= 0.0);
    }
}
