struct ZigZag {
    values: Vec<Option<u8>>,
    num_columns: usize,
    num_rows: usize,
}

impl ZigZag {
    fn new(num_rows: usize, num_columns: usize) -> ZigZag {
        let mut values = Vec::with_capacity(num_rows * num_columns);
        values.resize(num_rows * num_columns, None);
        ZigZag {
            values: values,
            num_columns: num_columns,
            num_rows: num_rows,
        }
    }

    fn get(&self, x: usize, y: usize) -> &Option<u8> {
        self.values
            .get(y * self.num_columns + x)
            .expect("cannot get value from values, which should be impossible")
    }

    fn get_mut(&mut self, x: usize, y: usize) -> &mut Option<u8> {
        self.values
            .get_mut(y * self.num_columns + x)
            .expect("cannot get value from values, which should be impossible")
    }

    fn len(&self) -> usize {
        self.values.len()
    }

    fn num_columns(&self) -> usize {
        self.num_columns
    }

    fn num_rows(&self) -> usize {
        self.num_rows
    }
}

pub fn convert(s: String, num_rows: i32) -> String {
    match num_rows {
        0 => "".to_string(),
        1 => s.to_string(),
        _ => convert_zigzag(s, num_rows as usize),
    }
}

fn convert_zigzag(s: String, num_rows: usize) -> String {
    let mut zig_zag = ZigZag::new(num_rows, calc_zigzag_num_columns(s.len(), num_rows));
    let mut x: usize = 0;
    let mut iter = s.as_bytes().iter();
    'out: loop {
        for y in 0..num_rows {
            if let Some(&byte) = iter.next() {
                *zig_zag.get_mut(x, y) = Some(byte);
            } else {
                break 'out;
            }
        }
        x += 1;
        for y in (1..(num_rows - 1)).rev() {
            if let Some(&byte) = iter.next() {
                *zig_zag.get_mut(x, y) = Some(byte);
                x += 1;
            } else {
                break 'out;
            }
        }
    }
    let mut result = String::with_capacity(zig_zag.len());
    for y in 0..zig_zag.num_rows() {
        for x in 0..zig_zag.num_columns() {
            if let &Some(byte) = zig_zag.get(x, y) {
                result.push(byte.into());
            }
        }
    }
    result
}

fn calc_zigzag_num_columns(len: usize, num_rows: usize) -> usize {
    let mut slen = len;
    let mut num_columns = 0;
    'out: loop {
        if slen > 0 {
            num_columns += 1;
            if let Some(new_slen) = slen.checked_sub(num_rows) {
                slen = new_slen;
            } else {
                break 'out;
            }
        } else {
            break 'out;
        }
        for _ in 0..(num_rows - 2) {
            if slen > 0 {
                slen -= 1;
                num_columns += 1;
            } else {
                break 'out;
            }
        }
    }
    num_columns
}

fn main() {
    assert_eq!(convert("a".to_string(), 2), "a".to_string());
    // assert_eq!(convert("abcd".to_string(), 2), "acbd".to_string());
    // assert_eq!(
    //     convert("PAYPALISHIRING".to_string(), 3),
    //     "PAHNAPLSIIGYIR".to_string()
    // );
    // assert_eq!(
    //     convert("PAYPALISHIRING".to_string(), 4),
    //     "PINALSIGYAHRPI".to_string()
    // );
}
