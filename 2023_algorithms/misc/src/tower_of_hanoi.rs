// Tower of Hanoi
fn tower_of_hanoi(n: usize) -> usize {
    if n == 0 {
        return 0;
    }

    // initial status: A: (1..n), B: -, C: -
    // target status: A: -, B (1..n), C: -

    // status_1; A: -, B: n, C: (1..n-1)
    let steps1 = 1 + tower_of_hanoi(n - 1); // move (1..n-1) from A to C, and move n from A to B;

    let steps2 = tower_of_hanoi(n - 1); // steps from status_1 to target status, move (1..n-1) from C to B

    return steps1 + steps2;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_toh() {
        assert_eq!(tower_of_hanoi(7), 127);
        assert_eq!(tower_of_hanoi(10), 1023);
    }
}
