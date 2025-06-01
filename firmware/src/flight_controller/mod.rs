//! Flight Controller
//==============================================================================

/// Flight Controller modes
enum Mode {
    /// default state - will introspect current position to determine next mode
    Unknown,
    /// vertical takeoff - ignores maneuver updates
    TakeOff,
    /// controls toward current manuever
    Flying,
    /// vertical landing - ignores manuever updates
    Landing,
}

/// Attempt maneuver to a waypoint by a deadline(time duration)
pub struct Maneuver {
    waypoint: crate::Position,
    deadline: crate::Duration,
}

struct ManeuverOperation {
    manuever: Maneuver,
    /// when the maneuver started
    start: crate::time::Instant,
}

struct FlightController {
    mode: Mode,
    manuever_operation: Option<ManeuverOperation>,
}

// impl FlightController {
//     fn new(unsafe_altitude_range: UnsafeAltitudeRange) -> FlightController {
//         FlightController {
//             unsafe_altitude_range: unsafe_altitude_range,
//             mode: Mode::Unknown,
//             manuever_operation: None,
//         }
//     }
// }
