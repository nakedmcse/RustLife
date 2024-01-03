// Game of Life Rust Implementation
use rand::Rng;

// Cell class
trait CellGenerations {
    fn live_neighbors(&self, others:&Vec<Cell>, max_x:i64, max_y:i64) -> i32;
    fn spawn_next(&self, others:&Vec<Cell>, max_x:i64, max_y:i64) -> bool;
}

#[derive(Debug, Copy, Clone)]
struct Cell {
    x: i64,
    y: i64,
    live: bool
}

impl Cell {
    fn new(x:i64, y:i64, live:bool) -> Self {
        Self{x, y, live}
    }
}

impl CellGenerations for Cell {
    fn live_neighbors(&self, others:&Vec<Cell>, max_x:i64, max_y:i64) -> i32 {
        let ystart = if self.y-1 < 0 { self.y } else { self.y-1 };
        let yend = if self.y+1 >= max_y { max_y } else { self.y+1 };
        let xstart = if self.x-1 < 0 { self.x } else { self.x-1 };
        let xend = if self.x+1 >= max_x { max_x } else { self.x+1 };
        let mut lives: i32 = if self.live { -1 } else { 0 };

        let lives_count =  others.iter().filter(|&c| c.live 
            && c.x >= xstart && c.x <= xend 
            && c.y >= ystart && c.y <= yend).count();
        lives += lives_count as i32;

        return lives;
    }

    fn spawn_next(&self, others:&Vec<Cell>, max_x:i64, max_y:i64) -> bool {
        let live_neighbour_count = self.live_neighbors(others, max_x, max_y);

        if !self.live && live_neighbour_count == 3 {
            //Dead cell + 3 live neighbors -> resurrected
            return true;
        }

        return if self.live {
            match live_neighbour_count {
                1 => {
                    // 1 neighbor only - cell dies
                    false
                },
                2|3 => {
                    // 2 or 3 neighbors - cell lives
                    true
                },
                _ => {
                    // more than 3 neighbors - cell dies
                    false
                }
            }
        } else {
            false
        }
    }
}

fn render_colony(colony:&Vec<Cell>) {
    let mut last_y:i64 = 0;
    for c in colony {
        if c.y != last_y {
            println!();
            last_y = c.y;
        }
        let outchar = if c.live {"*"} else {"-"};
        print!("{}",outchar);
    }
    println!();
}

fn colony_dead(colony:&Vec<Cell>) -> bool {
    return !colony.iter().any(|&c| c.live);
}

fn main() {
    // Base variables
    let mut rng = rand::thread_rng();
    let mut generation_number:i64 = 1;
    let mut colony = Vec::<Cell>::new();
    let max_x:i64 = 20;
    let max_y:i64 = 20;

    // Create initial generation
    for y in 0..20 {
        for x in 0..20 {
            colony.push(Cell::new(x,y,rng.gen_range(0..9) > 4));
        }
    }

    // Loop life
    loop {
        // Render current generation
        render_colony(&colony);

        // Generate next generation
        for idx in 0..colony.len() {
            colony[idx].live = colony[idx].spawn_next(&colony, max_x, max_y);
        }
        generation_number += 1;
        println!("\nGeneration Number: {}",generation_number);

        // Colony dies then exit loop
        if colony_dead(&colony) {
            println!("COLONY DEAD");
            break;
        }
    }
}