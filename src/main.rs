extern crate game_of_life;

fn main() {
    let initial_state = vec![(1,0), (1,1), (1,2), (5,3), (5, 4), (5, 5), (3,3)];
    game_of_life::run(20, initial_state)
}
