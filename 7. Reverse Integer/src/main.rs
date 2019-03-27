pub fn reverse(x: i32) -> i32 {
    let mut big_x: i64 = x as i64;
    let mut neg = 1i64;
    let mut result = 0i64;
    if big_x < 0 {
        big_x = -big_x;
        neg = -1;
    }
    while big_x > 0 {
        let low_int = (big_x - (big_x / 10) * 10) as i64;
        big_x /= 10;
        result = result * 10 + low_int;
    }
    result = neg * result;
    if result < i32::min_value() as i64 || result > i32::max_value() as i64 {
        0i32
    } else {
        result as i32
    }
}

fn main() {
    assert_eq!(reverse(123), 321);
    assert_eq!(reverse(-123), -321);
    assert_eq!(reverse(2147483647), 0);
    assert_eq!(reverse(-2147483648), 0);
    assert_eq!(reverse(0), 0);
    assert_eq!(reverse(1), 1);
    assert_eq!(reverse(-1), -1);
}
