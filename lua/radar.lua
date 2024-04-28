function sendRadarData(target_index, radar_azimuth, radar_elevation, radar_distance, radar_rotation,
    radar_gps_location_x, radar_gps_location_y, radar_gps_location_z)

    -- send request
    async.httpGet(420,
        "/add_radar_raw_data?target_index=" .. target_index .. "&distance=" .. radar_distance .. "&azimuth=" ..
            radar_azimuth .. "&elevation=" .. radar_elevation .. "&radar_rotation=" .. radar_rotation ..
            "&radar_unit_gps_location_x=" .. radar_gps_location_x .. "&radar_unit_gps_location_y=" ..
            radar_gps_location_y .. "&radar_unit_gps_location_z=" .. radar_gps_location_z)
end
-- Tick function that will be executed every logic tick
function onTick()
    local CHANNEL_ASSIGNMENT = {
        distance = input.getNumber(1),
        azimuth = input.getNumber(2),
        elevation = input.getNumber(3),
        radar_rotation = input.getNumber(4),
        radar_unit_gps_location_x = input.getNumber(5),
        radar_unit_gps_location_y = input.getNumber(6),
        radar_unit_gps_location_z = input.getNumber(7),
        radar_lock = input.getBool(1)
    }
    -- Check if all the variables have been assigned
    if CHANNEL_ASSIGNMENT.distance == nil or CHANNEL_ASSIGNMENT.azimuth == nil or CHANNEL_ASSIGNMENT.elevation == nil or
        CHANNEL_ASSIGNMENT.radar_rotation == nil or CHANNEL_ASSIGNMENT.radar_unit_gps_location_x == nil or
        CHANNEL_ASSIGNMENT.radar_unit_gps_location_y == nil or CHANNEL_ASSIGNMENT.radar_unit_gps_location_z == nil or
        CHANNEL_ASSIGNMENT.radar_lock == nil then
        error("Not all variables have been assigned", 2)
    end
    -- Check if the radar lock is true
    if (CHANNEL_ASSIGNMENT.radar_lock) then
        sendRadarData(1, CHANNEL_ASSIGNMENT.azimuth, CHANNEL_ASSIGNMENT.elevation, CHANNEL_ASSIGNMENT.distance,
            CHANNEL_ASSIGNMENT.radar_rotation, CHANNEL_ASSIGNMENT.radar_unit_gps_location_x,
            CHANNEL_ASSIGNMENT.radar_unit_gps_location_y, CHANNEL_ASSIGNMENT.radar_unit_gps_location_z)

    end

end
