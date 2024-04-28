i = input
gn = i.getNumber
gb = i.getBool

TRACKED_TARGETS = 8

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

    if target.radar_lock then
        sendRadarData(1, target.azimuth, target.elevation, target.distance,
            target.radar_rotation, target.radar_unit_gps_location_x,
            target.radar_unit_gps_location_y, target.radar_unit_gps_location_z)
    end
end
