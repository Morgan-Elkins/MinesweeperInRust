mod random;

use std::collections::HashSet;

use random::random_range;

pub type Position = (usize,usize);

#[derive(Debug)]
pub struct Minesweeper {
    width: usize,
    height: usize,
    open_fields: HashSet<Position>,
    mines: HashSet<Position>, //Should use list or vector as more performant, but more convenient 
    flagged_fields: HashSet<Position>
}

impl Minesweeper {
    pub fn new(width:usize, height : usize, mine_count: usize) -> Minesweeper {
        Minesweeper { 
            width, 
            height, 
            open_fields: HashSet::new(), 
            mines: {
                let mut mines =  HashSet::new();
                while mines.len() < mine_count {
                    mines.insert((random_range(0, width),random_range(0, height)));
                };

                mines
            }, 
            flagged_fields: HashSet::new() 
        }
    }
}

#[cfg(test)]
mod tests{
    use crate::Minesweeper;

    #[test]
    fn test(){
        let ms = Minesweeper::new(10,10,5);

        println!("{:?}",ms);
    }
}