gn = input.getNumber
gb = input.getBool
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
    local CHANNEL_ASSIGNMENT = {
        distance = gn(1),
        azimuth = gn(2),
        elevation = gn(3),
        radar_rotation = gn(4),
        radar_unit_gps_location_x = gn(5),
        radar_unit_gps_location_y = gn(6),
        radar_unit_gps_location_z = gn(7),
        radar_lock = gb(1)
    }

    -- Check if the radar lock is true
    if (CHANNEL_ASSIGNMENT.radar_lock) then
        sendRadarData(1, CHANNEL_ASSIGNMENT.azimuth, CHANNEL_ASSIGNMENT.elevation, CHANNEL_ASSIGNMENT.distance,
            CHANNEL_ASSIGNMENT.radar_rotation, CHANNEL_ASSIGNMENT.radar_unit_gps_location_x,
            CHANNEL_ASSIGNMENT.radar_unit_gps_location_y, CHANNEL_ASSIGNMENT.radar_unit_gps_location_z)

    end

end
