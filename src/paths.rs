pub mod navigation {
    pub fn position() = {
        "navigation.position".to_string()
    }
    pub fn lights() = {
        "navigation.lights".to_string()
    }
    pub fn course_over_ground_magnetic() = {
        "navigation.courseOverGroundMagnetic".to_string()
    }
    pub fn course_over_ground_true() = {
        "navigation.courseOverGroundTrue".to_string()
    }
    pub mod course_rhumbline {
        pub fn cross_track_error() = {
            "navigation.courseRhumbline.crossTrackError".to_string()
        }
        pub fn bearing_track_true() = {
            "navigation.courseRhumbline.bearingTrackTrue".to_string()
        }
        pub mod active_route {
            pub fn href() = {
                "navigation.courseRhumbline.activeRoute.href".to_string()
            }
            pub fn estimated_time_of_arrival() = {
                "navigation.courseRhumbline.activeRoute.estimatedTimeOfArrival".to_string()
            }
            pub fn start_time() = {
                "navigation.courseRhumbline.activeRoute.startTime".to_string()
            }
            pub fn next_point() = {
                "navigation.courseRhumbline.activeRoute.nextPoint".to_string()
            }
            pub fn previous_point() = {
                "navigation.courseRhumbline.activeRoute.previousPoint".to_string()
            }
            pub mod previous_point {
                pub fn distance() = {
                    "navigation.courseRhumbline.activeRoute.previousPoint.distance".to_string()
                }
                pub fn position() = {
                    "navigation.courseRhumbline.activeRoute.previousPoint.position".to_string()
                }
            }

        }
    }
}
