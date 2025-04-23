pub fn to_roman(num: u32) -> String {
    if num < 1 || num > 3999 {
        return String::new();
    }

    let roman_map = [
        (1000, "M"),
        (900, "CM"),
        (500, "D"),
        (400, "CD"),
        (100, "C"),
        (90, "XC"),
        (50, "L"),
        (40, "XL"),
        (10, "X"),
        (9, "IX"),
        (5, "V"),
        (4, "IV"),
        (1, "I"),
    ];

    let mut result = String::new();
    let mut remaining = num;

    for &(value, symbol) in roman_map.iter() {
        while remaining >= value {
            result.push_str(symbol);
            remaining -= value;
        }
    }

    result
}