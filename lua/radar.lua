i = input
gn = i.getNumber
gb = i.getBool

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
    local target = {
        distance = gn(1),
        azimuth = gn(2),
        elevation = gn(3),
        radar_rotation = gn(4),
        radar_unit_gps_location_x = gn(5),
        radar_unit_gps_location_y = gn(6),
        radar_unit_gps_location_z = gn(7),
        radar_lock = gb(1)
    }

    -- Check if all the variables have been assigned
    if target.distance == nil or target.azimuth == nil or target.elevation == nil or
        target.radar_rotation == nil or target.radar_unit_gps_location_x == nil or
        target.radar_unit_gps_location_y == nil or target.radar_unit_gps_location_z == nil or
        target.radar_lock == nil then
        error("Not all variables have been assigned", 2)
    end

    -- Check if the radar lock is true
    if target.radar_lock then
        sendRadarData(1, target.azimuth, target.elevation, target.distance,
            target.radar_rotation, target.radar_unit_gps_location_x,
            target.radar_unit_gps_location_y, target.radar_unit_gps_location_z)
    end
end
