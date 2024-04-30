use chrono::{DateTime, Utc};
use socketioxide::{
    extract::{AckSender, Data, SocketRef, State},
    SocketIo,
};
use tracing::{debug, info, Level};
use tracing_subscriber::FmtSubscriber;

use serde::{Deserialize, Serialize};
use std::f64::consts::PI;
use std::ops::{Add, Sub};
use std::sync::Arc;

use tokio::sync::Mutex;

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
    pub pos: Vec3D,
}

struct TargetInfo {
    pub detected: DateTime<Utc>,
    pub positions: Vec<(Vec3D, DateTime<Utc>)>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, PartialOrd)]
struct Vec3D {
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
    let targets: Arc<Mutex<Vec<TargetInfo>>> = Arc::new(Mutex::new(vec![]));

    let (layer, io) = SocketIo::builder().with_state(targets).build_layer();

    // listen on root route
    io.ns("/", |s: SocketRef| {
        info!("New connection: {}", s.id);

        // Log new radar returns
        s.on(
            "new_radar_data",
            |s: SocketRef,
             ack: AckSender,
             Data::<RawRadarReturn>(raw),
             targets: State<Arc<Mutex<Vec<TargetInfo>>>>| async move {
                // Conversions
                let mut targets = targets.0.lock().await;
                let target: RadarReturn = (&raw).into();

                // try to find potential previous readings that might be the same reading from
                // earlier
                let found = targets.iter_mut().any(|t| t.check_or_add(&target, raw.dst));

                // Create new Target
                if !found {
                    targets.push((&target).into());
                }

                let targets: Vec<Target> = targets.iter().map(|z| z.into()).collect();

                // Broadcast and ack
                s.broadcast().emit("global_targets", targets).ok();
                ack.send(target).ok();
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

impl TargetInfo {
    pub fn check_or_add(&mut self, t: &RadarReturn, d: f64) -> bool {
        let o: Vec3D = t.into();
        let c: Vec3D = self.avg();
        let dst = (o - c).dst();

        if dst > d.log(4.0) {
            return false;
        }

        self.positions.push((o, Utc::now()));

        true
    }

    fn avg(&self) -> Vec3D {
        let s: usize = self.positions.len().saturating_sub(10);
        let items = &self.positions[s..];
        let count = items.len();
        let sum = items.iter().fold(
            Vec3D {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            |x, y| x + y.0,
        );

        Vec3D {
            x: sum.x / count as f64,
            y: sum.y / count as f64,
            z: sum.z / count as f64,
        }
    }
}

impl From<&TargetInfo> for Target {
    fn from(raw: &TargetInfo) -> Self {
        Self {
            friendly: false,
            id: 0,
            pos: raw.positions.first().unwrap().0,
        }
    }
}

impl From<&RadarReturn> for TargetInfo {
    fn from(raw: &RadarReturn) -> Self {
        TargetInfo {
            detected: Utc::now(),
            positions: vec![(raw.into(), Utc::now())],
        }
    }
}

impl From<&RawRadarReturn> for RadarReturn {
    fn from(raw: &RawRadarReturn) -> Self {
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

impl Add for Vec3D {
    type Output = Vec3D;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: rhs.x + self.x,
            y: rhs.y + self.y,
            z: rhs.z + self.z,
        }
    }
}

impl Sub for Vec3D {
    type Output = Vec3D;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: rhs.x - self.x,
            y: rhs.y - self.y,
            z: rhs.z - self.z,
        }
    }
}

impl From<&RadarReturn> for Vec3D {
    fn from(r: &RadarReturn) -> Self {
        Self {
            x: r.x,
            y: r.y,
            z: r.z,
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

impl Vec3D {
    pub fn dst(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
}
