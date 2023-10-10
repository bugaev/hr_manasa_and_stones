/*
 * Complete the 'stones' function below.
 *
 * The function is expected to return an INTEGER_ARRAY.
 * The function accepts following parameters:
 *  1. INTEGER n
 *  2. INTEGER a
 *  3. INTEGER b
 */

fn stones(n: i32, a: i32, b: i32) -> Vec<i32> {
    if n == 1 {
        return vec![0];
    }
    let mut res = Vec::new();
    let N = n - 1;
    for n_a in 0..=N {
        let n_b = N - n_a;
        res.push(n_b * b + n_a * a);
    }
    res.sort();
    res.dedup();
    res
}

fn main() {
    println!("Answer: {:?}", stones(4, 10, 100));
}