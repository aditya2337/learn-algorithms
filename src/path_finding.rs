#[derive(Clone, Debug, PartialEq)]
pub struct Point {
    x: isize,
    y: isize,
}

impl Point {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

type Maze = Vec<String>;

pub fn find_path(maze: Maze, wall: char, start: Point, end: Point) -> Vec<Point> {
    let mut seen: Vec<Vec<bool>> = vec![];

    let mut i = 0;

    loop {
        if (i >= maze.len()) {
            break;
        }

        let mut a = vec![false; maze[i].len()];
        a.fill(false);
        seen.push(a);
        i += 1;
    }

    let mut path: Vec<_> = vec![];

    if walk(&maze, wall, start, &end, &mut seen, &mut path) {
        return path;
    }
    vec![Point::new(0, 0)]
}

pub fn walk(
    maze: &Maze,
    wall: char,
    curr: Point,
    end: &Point,
    seen: &mut Vec<Vec<bool>>,
    path: &mut Vec<Point>,
) -> bool {
    let dir = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
    // out of map
    if curr.x < 0 || curr.x as usize > maze[0].len() || curr.y < 0 || curr.y as usize > maze.len() {
        return false;
    }
    // on a wall
    let my_str_vec: Vec<char> = maze[curr.y as usize].chars().collect();
    if my_str_vec[curr.x as usize] == wall {
        return false;
    }
    // been there before?
    if seen[curr.y as usize][curr.x as usize] {
        return false;
    }
    // at the end
    if curr.x == end.x && curr.y == end.y {
        path.push(end.clone());
        return true;
    }

    // recursion
    // pre
    path.push(curr.clone());
    seen[curr.y as usize][curr.x as usize] = true;
    // recurse
    for i in dir.iter() {
        if (walk(
            maze,
            wall,
            Point::new(curr.x + i.0, curr.y + i.1),
            end,
            seen,
            path,
        )) {
            return true;
        }
    }
    // post
    path.pop();
    false
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_path_finder() {
        let maze: Maze = vec![
            String::from("xxxxxxxxxx x"),
            String::from("x        x x"),
            String::from("x        x x"),
            String::from("x xxxxxxxx x"),
            String::from("x          x"),
            String::from("x xxxxxxxxxx"),
        ];

        let path = find_path(maze, 'x', Point::new(10, 0), Point::new(1, 5));
        assert_eq!(
            &path,
            &vec![
                Point { x: 10, y: 0 },
                Point { x: 10, y: 1 },
                Point { x: 10, y: 2 },
                Point { x: 10, y: 3 },
                Point { x: 10, y: 4 },
                Point { x: 9, y: 4 },
                Point { x: 8, y: 4 },
                Point { x: 7, y: 4 },
                Point { x: 6, y: 4 },
                Point { x: 5, y: 4 },
                Point { x: 4, y: 4 },
                Point { x: 3, y: 4 },
                Point { x: 2, y: 4 },
                Point { x: 1, y: 4 },
                Point { x: 1, y: 5 }
            ]
        );
    }
}
