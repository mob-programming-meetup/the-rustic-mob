fn sonar_sweep(measurements: &[u32]) -> usize {
    let sub_measurements = &measurements[1..];
    let mut counter = 0;
    for (index, measurement) in sub_measurements.iter().enumerate() {
        if measurement > &measurements[index] {
            counter += 1;
        }
    }

    counter
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
}
