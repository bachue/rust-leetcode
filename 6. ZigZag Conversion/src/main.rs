pub fn convert(s: String, num_rows: i32) -> String {
    match num_rows {
        0 => "".to_string(),
        1 => s.to_string(),
        _ => {
            let mut rows: Vec<String> = Vec::new();
            let mut cur_rows: i32 = 0;
            let mut going_down: bool = false;
            rows.resize(s.len().min(num_rows as usize), String::new());

            for &byte in s.as_bytes() {
                rows[cur_rows as usize].push(byte.into());
                if cur_rows == 0 || cur_rows == num_rows - 1 {
                    going_down = !going_down;
                }
                cur_rows += if going_down { 1 } else { -1 };
            }

            let mut results = String::with_capacity(s.len());
            for row in rows {
                results.push_str(&row);
            }
            results
        }
    }
}

fn main() {
    assert_eq!(convert("a".to_string(), 2), "a".to_string());
    assert_eq!(convert("abcd".to_string(), 2), "acbd".to_string());
    assert_eq!(
        convert("PAYPALISHIRING".to_string(), 3),
        "PAHNAPLSIIGYIR".to_string()
    );
    assert_eq!(
        convert("PAYPALISHIRING".to_string(), 4),
        "PINALSIGYAHRPI".to_string()
    );
}
