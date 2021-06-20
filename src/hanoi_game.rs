pub mod hanoi_game {

    use std::fmt;

    #[derive(Debug)]
    pub struct Hanoi {
        state: Vec<Vec<usize>>,
        towers: usize,
        tower_height: usize,
        tower_position_on_last_win: usize,
    }

    impl Hanoi {
        pub fn new(tower_height: usize, towers: usize) -> Self {
            Self {
                state: {
                    let mut vec = vec![vec![]; towers - 1];
                    vec.push((1..=tower_height).rev().collect::<Vec<usize>>());
                    vec
                },
                towers,
                tower_height,
                tower_position_on_last_win: towers - 1,
            }
        }
        pub fn is_won(&self) -> bool {
            //loop over all towers
            for j in 0..self.state.len() {
                // skip the tower where the player started
                if j == self.tower_position_on_last_win {
                    continue;
                }

                //check if the height of the tower is correct - if the tower is not tall enough, there is now way the game is won
                if self.state[j].len() == self.tower_height {
                    //then loop over all the floors
                    for i in 0..self.state[j].len() {
                        //it will actually start at the top of the tower
                        //if any of the floors has the wrong width, move onto the next tower
                        if !(self.state[j][i] == (self.tower_height - i)) {
                            break;
                        }

                        //if all floors had the correct width, then it is a complete tower
                        return true;
                    }
                }
            }
            //if none of the towers were correct, the game is not won
            false
        }

        pub fn do_command(&mut self, from: usize, to: usize) -> bool {
            if !self.state[from].is_empty() {
                if self.state[to].is_empty() || self.state[from].last() < self.state[to].last() {
                    let temp = self.state[from].pop().unwrap();
                    self.state[to].push(temp);
                    true
                } else {
                    false
                }
            } else {
                false
            }
        }

        pub fn ask_turn(&self) -> String {
            String::from("Input your move in the format of `from .. to ..`")
        }
        pub fn get_num_towers(&self) -> usize {
            self.towers
        }
        pub fn get_tower(&self, index: usize) -> &Vec<usize> {
            &self.state[index]
        }
        pub fn get_tower_height(&self) -> usize {
            self.tower_height
        }
    }

    impl fmt::Display for Hanoi {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const SPACE_BETWEEN_TOWERS: usize = 4;
            let mut s = String::with_capacity(
                self.towers * self.tower_height + self.tower_height * SPACE_BETWEEN_TOWERS,
            );
            for i in (0..self.tower_height).rev() {
                for j in 0..self.towers {
                    let whitespace;
                    let mut tower = String::with_capacity(self.tower_height);
                    let number;
                    if self.state[j].len() > i {
                        whitespace = " ".repeat(self.tower_height - self.state[j][i]);
                        tower = "|".repeat(self.state[j][i]);
                        number = self.state[j][i].to_string();
                    } else {
                        whitespace = " ".repeat(self.tower_height);
                        number = String::from("|");
                    }
                    s += whitespace.as_str();
                    s += tower.as_str();
                    s += number.as_str();
                    s += tower.as_str();
                    s += whitespace.as_str();
                    s += &" ".repeat(SPACE_BETWEEN_TOWERS);
                }
                s += "\n";
            }

            write!(
                f,
                "{}{}",
                s,
                "-".repeat(
                    self.towers * 2 * self.tower_height
                        + (SPACE_BETWEEN_TOWERS * (self.towers - 1))
                        + self.towers
                )
            )
        }
    }
}
