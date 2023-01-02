#![allow(dead_code)]
use std::collections::VecDeque;

struct PacketTiming {
    arrival_time_ms: f64,
    smoothed_delay_ms: f64,
    raw_delay: f64,
}

fn linear_fit_slope(packets: &VecDeque<PacketTiming>) -> Option<f64> {
    if packets.len() < 2 {
        return None;
    }

    let mut sum_x: f64 = 0.0;
    let mut sum_y: f64 = 0.0;

    for packet in packets {
        sum_x += packet.arrival_time_ms;
        sum_y += packet.smoothed_delay_ms;
    }

    let x_avg = sum_x / packets.len() as f64;
    let y_avg = sum_y / packets.len() as f64;

    // Compute the slope k = \sum (x_i-x_avg)(y_i-y_avg) / \sum (x_i-x_avg)^2
    let mut numerator: f64 = 0.0;
    let mut denominator: f64 = 0.0;

    for packet in packets {
        let x = packet.arrival_time_ms;
        let y = packet.smoothed_delay_ms;
        numerator += (x - x_avg) * (y - y_avg);
        denominator += (x - x_avg) * (x - x_avg);
    }

    if denominator == 0.0 {
        return None;
    }

    Some(numerator / denominator)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_linear_fit_slope_pos() {
        let mut packets = VecDeque::new();
        packets.push_back(PacketTiming {
            arrival_time_ms: 1.0,
            smoothed_delay_ms: 1.0,
            raw_delay: 1.0,
        });
        packets.push_back(PacketTiming {
            arrival_time_ms: 2.0,
            smoothed_delay_ms: 2.0,
            raw_delay: 2.0,
        });
        packets.push_back(PacketTiming {
            arrival_time_ms: 3.0,
            smoothed_delay_ms: 3.0,
            raw_delay: 3.0,
        });

        let slope = linear_fit_slope(&packets);
        assert!(slope.is_some());
        assert_eq!(slope.unwrap(), 1.0);
    }

    #[test]
    fn test_linear_fit_slope_neg() {
        let mut packets = VecDeque::new();
        packets.push_back(PacketTiming {
            arrival_time_ms: 1.0,
            smoothed_delay_ms: 3.0,
            raw_delay: 3.0,
        });
        packets.push_back(PacketTiming {
            arrival_time_ms: 2.0,
            smoothed_delay_ms: 2.0,
            raw_delay: 2.0,
        });
        packets.push_back(PacketTiming {
            arrival_time_ms: 3.0,
            smoothed_delay_ms: 1.0,
            raw_delay: 1.0,
        });

        let slope = linear_fit_slope(&packets);
        assert!(slope.is_some());
        assert_eq!(slope.unwrap(), -1.0);
    }

    #[test]
    fn test_linear_fit_slope_none() {
        let mut packets = VecDeque::new();
        packets.push_back(PacketTiming {
            arrival_time_ms: 1.0,
            smoothed_delay_ms: 3.0,
            raw_delay: 3.0,
        });
        let slope = linear_fit_slope(&packets);
        assert!(slope.is_none());
    }
}
