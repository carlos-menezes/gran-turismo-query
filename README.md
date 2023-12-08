# gran-turismo-query

![ci workflow](https://github.com/carlos-menezes/openmultiplayer-query/actions/workflows/ci.yml/badge.svg)
![Crates.io](https://img.shields.io/crates/v/gran-turismo-query)

Implements the packet parser for Gran Turismo 7 telemetry data, allowing a developer to retrieve data from a running game.

## Example

1. Find the address of your PlayStation 5;
2. Run the following code:

```rs
 // Create a UDP socket bound to any available address
let socket = UdpSocket::bind("0.0.0.0:33740").expect("Failed to bind socket");

// IP address and port to send/receive packets
let ip_address = "192.168.1.84"; // This will be the address of your PS5
let port = 33739; // Do NOT change this port, as the telemetry server listens to incoming packets on this port
let destination: SocketAddr = format!("{}:{}", ip_address, port)
    .parse()
    .expect("Invalid IP address or port");

// Send heartbeat packet to the telemetry server
socket
    .send_to(PACKET_HEARTBEAT_DATA, destination)
    .expect("Failed to send packet");

// Send a packet
loop {
    socket
        .send_to(PACKET_HEARTBEAT_DATA, destination)
        .expect("Failed to send packet");
    let mut buf = [0u8; PACKET_SIZE];
    socket
        .recv_from(&mut buf)
        .expect("Failed to receive packet");
    let packet = Packet::try_from(&buf).expect("Failed to parse packet");
    println!("{:#?}", packet);
    // Send hearbeat packet every 100 packets, otherwise the telemetry server will assume this client is dead
    if packet.packet_id % 100 == 0 {
        socket
            .send_to(PACKET_HEARTBEAT_DATA, destination)
            .expect("Failed to send packet");
    }   
}
```

You should see an output similar to this in your terminal:

```sh
Packet {
    position: [
        996.11865,
        95.22594,
        41.165268,
    ],
    velocity: [
        -25.783506,
        2.6718328,
        12.235032,
    ],
    rotation: [
        -0.012828048,
        0.83853436,
        -0.061140057,
    ],
    relative_orientation_to_north: 0.5412555,
    angular_velocity: [
        0.1495306,
        -0.5295577,
        0.19355063,
    ],
    body_height: 0.047546387,
    engine_rpm: 6040.0,
    gas_level: 100.0,
    gas_capacity: 100.0,
    meters_per_second: 28.656612,
    turbo_boost: 0.18931998,
    oil_pressure: 6.5625277,
    water_temperature: 85.0,
    oil_temperature: 110.0,
    tire_fl_surface_temperature: 90.79641,
    tire_fr_surface_temperature: 60.37589,
    tire_rl_surface_temperature: 70.37558,
    tire_rr_surface_temperature: 61.503597,
    packet_id: 143467,
    lap_count: 1,
    laps_in_race: 2,
    best_lap_time: -1,
    last_lap_time: -1,
    time_of_day_progression: 65438716,
    qualifying_position: -1,
    num_cars_pre_race: -1,
    alert_rpm_min: 7200,
    alert_rpm_max: 8000,
    calculated_max_speed: 279,
    flags: Some(
        PacketFlags(
            CarOnTrack | Paused | InGear | HasTurbo | ASMActive,
        ),
    ),
    current_gear: 2,
    suggested_gear: 15,
    throttle: 0,
    brake: 0,
    road_plane: [
        0.039712485,
        0.9897655,
        -0.13706571,
    ],
    road_plane_distance: -128.1198,
    wheel_fl_rps: -81.54625,
    wheel_fr_rps: -79.17614,
    wheel_rl_rps: -81.414276,
    wheel_rr_rps: -79.96095,
    tire_fl_radius: 0.35500002,
    tire_fr_radius: 0.35500002,
    tire_rl_radius: 0.35500002,
    tire_rr_radius: 0.35500002,
    tire_fl_suspension_height: 0.3052817,
    tire_fr_suspension_height: 0.28858984,
    tire_rl_suspension_height: 0.2946418,
    tire_rr_suspension_height: 0.2775881,
    clutch_pedal: 0.0,
    clutch_engagement: 1.0,
    rpm_from_clutch_to_gearbox: 6040.0,
    transmission_top_speed: 2.421,
    gear_ratios: [
        3.0000002,
        2.2350001,
        1.6320001,
        1.32,
        1.12,
        0.96000004,
        0.0,
    ],
    car_code: 3474,
}
```