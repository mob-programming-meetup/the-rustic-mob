fn sonar_sweep(measurements: &[u32]) -> usize {
    measurements
        .windows(2)
        .fold(0, |acc, win| if win[0] < win[1] { acc + 1 } else { acc })
}

#[cfg(test)]
mod tests {
    use super::sonar_sweep;

    #[test]
    fn sonar_sweep_returns_2_increases() {
        let measurements = vec![199, 200, 208];
        assert_eq!(sonar_sweep(&measurements), 2);
    }

    #[test]
    fn sonar_sweep_returns_3_increases() {
        let measurements = vec![199, 200, 208, 210];
        assert_eq!(sonar_sweep(&measurements), 3);
    }

    #[test]
    fn sonar_sweep_returns_7_increases() {
        let measurements = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(sonar_sweep(&measurements), 7);
    }

    // #[test]
    // fn sonar_sweep_returns_amount_of_increases() {
    //     let s = include_str!("../input/day1.txt"); // "198\n201\n208\n..."
    //     s.split();
    //     let measurements = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    //     assert_eq!(sonar_sweep(&measurements), 7);
    // }
}
