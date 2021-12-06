fn sonar_sweep(measurements: &[u32]) -> usize {
    measurements
        .windows(2)
        .fold(0, |acc, win| if win[0] < win[1] { acc + 1 } else { acc })
}

fn sonar_sweep_filtered(measurements: &[u32]) -> usize {
    measurements
        .windows(4)
        .filter(|win| win[3] > win[0])
        .count()
}

fn buff_to_vec(input: &str) -> Vec<u32> {
    let mut res = vec![];

    for line in input.lines() {
        res.push(line.parse::<u32>().unwrap());
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn sonar_sweep_returns_0_increases_on_emtpy_array() {
        let measurements = vec![];
        assert_eq!(sonar_sweep(&measurements), 0);
    }

    #[test]
    fn sonar_sweep_returns_amount_of_increases() {
        let s = include_str!("../input/day1.txt"); // "198\n201\n208\n..."
        let measurements = buff_to_vec(&s);
        assert_eq!(sonar_sweep(&measurements), 1266);
    }

    #[test]
    fn filter_sonar_sweep_returns_5_increases() {
        let measurements = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(sonar_sweep_filtered(&measurements), 5);
    }

    #[test]
    fn sonar_sweep_filter_returns_amount_of_increases() {
        let s = include_str!("../input/day1.txt"); // "198\n201\n208\n..."
        let measurements = buff_to_vec(&s);
        assert_eq!(sonar_sweep_filtered(&measurements), 1217);
    }

    #[test]
    fn sonar_sweep_filtered_returns_0_increases() {
        let measurements = vec![199, 200, 208];
        assert_eq!(sonar_sweep_filtered(&measurements), 0);
    }
}

// --- Part Two ---

// Considering every single measurement isn't as useful as you expected: there's just too much noise in the data.

// Instead, consider sums of a three-measurement sliding window. Again considering the above example:

// 199  A
// 200  A B
// 208  A B C
// 210    B C D
// 200  E   C D
// 207  E F   D
// 240  E F G
// 269    F G H
// 260      G H
// 263        H

// Start by comparing the first and second three-measurement windows. The measurements in the first window
// are marked A (199, 200, 208);
// their sum is 199 + 200 + 208 = 607. The second window is marked B (200, 208, 210);
// its sum is 618. The sum of measurements in the second
// window is larger than the sum of the first, so this first comparison increased.

// Your goal now is to count the number of times the sum of measurements in this sliding window
// increases from the previous sum.
// So, compare A with B, then compare B with C, then C with D, and so on.
// Stop when there aren't enough measurements left to create a new
// three-measurement sum.

// In the above example, the sum of each three-measurement window is as follows:

// A: 607 (N/A - no previous sum)
// B: 618 (increased)
// C: 618 (no change)
// D: 617 (decreased)
// E: 647 (increased)
// F: 716 (increased)
// G: 769 (increased)
// H: 792 (increased)

// In this example, there are 5 sums that are larger than the previous sum.

// Consider sums of a three-measurement sliding window. How many sums are larger than the previous sum?
