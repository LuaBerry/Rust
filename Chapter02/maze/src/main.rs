use rand::Rng;

const MAP_N: usize = 25;

fn main() {
    //Init rand instance
    let mut rng = rand::thread_rng();
    //Init maze 2D array, represented as maze[y][x] 0,0 is top left
    let mut maze = [[0; MAP_N * 2]; MAP_N];
    //Surround wall
    for i in 0..MAP_N {
        maze[i][0] = 1; //Upper
        maze[i][(MAP_N*2) - 1] = 1; //Bottom
        maze[0][i*2+1] = 1; //Left
        maze[0][i*2] = 1; //Left
        maze[MAP_N - 1][i*2] = 1; //Right
        maze[MAP_N - 1][i*2+1] = 1; //Right
    }

    for y in 2..MAP_N-2 {
        for x in 2..(MAP_N*2)-2 {
            if x % 2 == 1 || y % 2 == 1 { continue; } //if x or y is even, skip
            let r = rng.gen_range(0..=3);
            let color = rng.gen_range(2..=8);
            maze[y][x] = color;
            match r {
                0 => maze[y-1][x] = color, //Up
                1 => maze[y+1][x] = color, //down
                2 => maze[y][x-1] = color, //left
                3 => maze[y][x+1] = color, //right
                _ => {},
            }
        }
    }
    let tiles = ["\u{2B1C}", "\u{2B1B}", "\u{1F7E5}", "\u{1F7E6}", "\u{1F7E7}", "\u{1F7E8}", "\u{1F7E9}", "\u{1F7EA}","\u{1F7EB}"];
    for y in 0..MAP_N {
        for x in 0..MAP_N*2 {
            print!("{}", tiles[maze[y][x]]);
        }
        println!("");
    }
}
