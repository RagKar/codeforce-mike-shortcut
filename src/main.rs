use std::collections::VecDeque;

fn costs_from_origin(n: usize, a: Vec<i32>) -> Vec<i32> {
    let mut costs = (0..n)
        .map(|i| if i == 0 { 0 } else { -1 })
        .collect::<Vec<_>>();
    let mut intersections = VecDeque::from([1]);

    while let Some(i) = intersections.pop_back() {
        // Intersection at cost 1
        let prev = i - 1;
        let next = i + 1;
        let ai = a[i - 1] as usize;

        // Cost from intersection 1
        let cost = 1 + costs[i - 1];

        // Update cost to next interesction if its cheaper or uninitialized
        if prev > 1 && (costs[prev - 1] < 0 || costs[prev - 1] > cost) {
            costs[prev - 1] = cost;
            intersections.push_back(prev);
        }

        if next <= n && (costs[next - 1] < 0 || costs[next - 1] > cost) {
            costs[next - 1] = cost;
            intersections.push_back(next);
        }

        if costs[ai - 1] < 0 || costs[ai - 1] > cost {
            costs[ai - 1] = cost;
            intersections.push_back(ai);
        }
    }

    return costs;
}

fn main() {
    let mut s = String::new();
    std::io::stdin()
        .read_line(&mut s)
        .expect("fail to read_line");
    let n: usize = s.trim().parse().expect("invalid n");

    s.clear();
    std::io::stdin()
        .read_line(&mut s)
        .expect("fail to read_line");
    let a = s
        .trim()
        .split(" ")
        .map(|a| a.parse().expect("invalid ai"))
        .collect::<Vec<i32>>();

    assert!(a.len() == n);

    costs_from_origin(n, a)
        .into_iter()
        .for_each(|c| print!("{} ", c));
}
