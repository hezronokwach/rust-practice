pub fn score(input: &str) -> u64 {
    input
        .chars()
        .map(|c| match c.to_ascii_uppercase() {
            // 1 point letters
            'A' | 'E' | 'I' | 'O' | 'U' | 'L' | 'N' | 'R' | 'S' | 'T' => 1,
            // 2 point letters
            'D' | 'G' => 2,
            // 3 point letters
            'B' | 'C' | 'M' | 'P' => 3,
            // 4 point letters
            'F' | 'H' | 'V' | 'W' | 'Y' => 4,
            // 5 point letters
            'K' => 5,
            // 8 point letters
            'J' | 'X' => 8,
            // 10 point letters
            'Q' | 'Z' => 10,
            // Any other character
            _ => 0,
        })
        .sum()
}