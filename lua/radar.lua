i = input
gn = i.getNumber
gb = i.getBool

function sendRadarData(target_index, radar_azimuth, radar_elevation, radar_distance, radar_rotation,
    radar_gps_location_x, radar_gps_location_y, radar_gps_location_z)

    local url = string.format(
        "/add_radar_raw_data?target_index=%d&distance=%.2f&azimuth=%.2f&elevation=%.2f&radar_rotation=%.2f&radar_unit_gps_location_x=%.2f&radar_unit_gps_location_y=%.2f&radar_unit_gps_location_z=%.2f",
        target_index, radar_distance, radar_azimuth, radar_elevation, radar_rotation, radar_gps_location_x,
        radar_gps_location_y, radar_gps_location_z)
    -- send request
    async.httpGet(420, url)
end

-- Tick function that will be executed every logic tick
function onTick()
    local CHANNEL = {
        dist = gn(1),
        azi = gn(2),
        ele = gn(3),
        radar_rotation = gn(4),
        radar_x = gn(5),
        radar_y = gn(6),
        radar_z = gn(7),
        radar_lock = gb(1)
    }
    -- Check if the radar lock is true
    if CHANNEL.radar_lock then
        sendRadarData(1, CHANNEL.azi, CHANNEL.ele, CHANNEL.dist, CHANNEL.radar_rotation, CHANNEL.radar_x,
            CHANNEL.radar_y, CHANNEL.radar_z)
    end
end
