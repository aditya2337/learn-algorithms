use crate::queue::Queue;

type WeightedAdjacencyMatrix = Vec<Vec<i32>>;

pub fn bfs(graph: WeightedAdjacencyMatrix, source: i32, needle: i32) -> Option<Vec<i32>> {
    let mut seen = vec![false; graph.len()];
    let mut prev = vec![-1; graph.len()];

    seen[source as usize] = true;
    prev[0] = 0;

    let mut q = Queue::<i32>::new();
    q.enqueue(source);

    loop {
        let curr = q.dequeue();

        if let Some(curr) = curr {
            if curr == needle {
                break;
            }

            for i in 0..graph[curr as usize].len() {
                if graph[curr as usize][i] == 0 || seen[i] {
                    continue;
                }

                seen[i] = true;
                prev[i] = curr;
                q.enqueue(i as i32);
            }
        }

        if q.length <= 0 {
            break;
        }
    }

    // build backwards
    let mut curr = needle;
    let mut out: Vec<i32> = vec![source];

    while prev[curr as usize] >= 0 {
        out.push(prev[curr as usize]);
        curr = prev[curr as usize];
    }

    out.reverse();

    Some(out)
}
