#[derive(Debug, Copy, Clone)]
struct Point {
    x: usize,
    y: usize,
}

fn walk(
    maze: &Vec<Vec<char>>,
    wall: char,
    curr: Point,
    end: Point,
    seen: &mut Vec<Vec<bool>>,
    path: &mut Vec<Point>,
) -> bool {
    let width = maze[0].len();
    let height = maze.len();
    let cx = curr.x;
    let cy = curr.y;

    if cx == end.x && cy == end.y {
        path.push(end);
        return true;
    }

    if cx >= width || cy >= height {
        return false;
    }

    if maze[cy][cx] == wall {
        return false;
    }

    if seen[cy][cx] {
        return false;
    }

    seen[cy][cx] = true;
    path.push(curr);
    let dirs: [[i32; 2]; 4] = [[-1, 0], [1, 0], [0, 1], [0, -1]];
    for i in dirs {
        let [x, y] = i;
        if (x < 0 && cx == 0) || (y < 0 && cy == 0) {
            continue;
        }
        let nx = match (cx, x) {
            (cx, x) if x < 0 => cx - 1, //when x i -ive
            (cx, x) => cx + x as usize, //otherwise
        };
        let ny = match (cy, y) {
            (cy, y) if y < 0 => cy - 1, //when x i -ive
            (cy, y) => cy + y as usize, //otherwise
        };
        let curr = Point { x: nx, y: ny };
        if walk(maze, wall, curr, end, seen, path) {
            return true;
        }
    }
    path.pop();
    false
}

fn solve(maze: &Vec<Vec<char>>, wall: char, start: Point, end: Point) -> Vec<Point> {
    let mut path = Vec::new();

    let num_rows = maze.len();
    let num_cols = maze[0].len();
    let mut seen: Vec<Vec<bool>> = vec![vec![false; num_cols]; num_rows];
    display(maze, &Vec::new());
    walk(maze, wall, start, end, &mut seen, &mut path);
    display(maze, &path);
    path
}

enum Dir {
    Up,
    Down,
    Left,
    Right,
}

fn display(maze: &Vec<Vec<char>>, path: &Vec<Point>) {
    let mut display = maze.to_owned();
    let path = path.to_owned();
    for pair in path.windows(2) {
        let (p1, p2) = (pair[0], pair[1]);
        let (x1, x2, y1, y2) = (p1.x, p2.x, p1.y, p2.y);
        let dirs = if y1 > y2 {
            Dir::Up
        } else if y1 < y2 {
            Dir::Down
        } else if x1 > x2 {
            Dir::Left
        } else {
            Dir::Right
        };
        match dirs {
            Dir::Down => display[y1][x1] = 'v',
            Dir::Left => display[y1][x1] = '<',
            Dir::Right => display[y1][x1] = '>',
            Dir::Up => display[y1][x1] = '^',
        }
    }
    println!();
    for i in display {
        // println!("{:?}", i);
        for j in i {
            print!("{} ", j);
        }
        println!();
    }
}

fn main() {
    let maze = vec![
        vec!['#', '#', '#', '#', '#', '#', '#', '#', '#', '#', 'S', '#'],
        vec!['#', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '#', ' ', '#'],
        vec!['#', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '#', ' ', '#'],
        vec!['#', ' ', '#', '#', '#', '#', '#', '#', '#', '#', ' ', '#'],
        vec!['#', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '#'],
        vec!['#', 'E', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#'],
    ];
    let wall = '#';
    let start = Point { x: 10, y: 0 };
    let end = Point { x: 1, y: 5 };
    let _path = solve(&maze, wall, start, end);
}
