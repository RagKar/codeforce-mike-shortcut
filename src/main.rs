use std::collections::VecDeque;

// i = intersection
// cost from i to a[i] == 1
// cost from i to i+1 == 1

fn path(n: usize, a: Vec<i32>) -> Vec<i32> {
    let mut dist = (0..n)
        .map(|i| if i == 0 { 0 } else { -1 })
        .collect::<Vec<_>>();

    let mut queue = VecDeque::new();
    queue.push_back(1);

    while let Some(i) = queue.pop_back() {
        let prev = i - 1;
        let next = i + 1;
        let ai = a[i - 1] as usize;
        let cost = 1 + dist[i - 1];

        if prev > 1 && (dist[prev - 1] < 0 || dist[prev - 1] > cost) {
            dist[prev - 1] = cost;
            queue.push_back(prev);
        }

        if next <= n && (dist[next - 1] < 0 || dist[next - 1] > cost) {
            dist[next - 1] = cost;
            queue.push_back(next);
        }

        if dist[ai - 1] < 0 || dist[ai - 1] > cost {
            dist[ai - 1] = cost;
            queue.push_back(ai);
        }
    }

    return dist;
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

    path(n, a).into_iter().for_each(|c| print!("{} ", c));
}
