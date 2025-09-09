use std::io;

const LOWEST_RANK: usize = 25;
const LEGEND_RANK: usize = 0;
const THREE_STAR_THRESHOLD: usize = 20;
const FOUR_STAR_THRESHOLD: usize = 16;
const FIVE_STAR_THRESHOLD: usize = 10;
fn main() {
  //  println!("Hello, world!");

    let mut winstreak = 0;
    let mut stars: u8 = 0;

    let mut stars_for_rank: Vec<u8> = Vec::new();

    for rank in (LEGEND_RANK..=LOWEST_RANK).rev() {
        let temp = if rank >= THREE_STAR_THRESHOLD {
            2
        } else if rank >= FOUR_STAR_THRESHOLD {
            3
        } else if rank >= FIVE_STAR_THRESHOLD {
            4
        } else if rank >= LEGEND_RANK {
            5
        } else {
            0
        };
        stars_for_rank.push(temp);
    }

   // print!("{:?}", stars_for_rank);

    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let match_history: Vec<char> = input.trim().chars().collect::<Vec<char>>();

    //print!("{:?}", match_history);

    for game in &match_history {
        if game.eq_ignore_ascii_case(&'W') {
            stars += 1;
            winstreak += 1;
            if winstreak >= 3 {
                stars += 1;
            }
        } else if game.eq_ignore_ascii_case(&'L') {
            if get_rank(stars, &stars_for_rank) < 20 {
                stars -= 1;
            }

            winstreak = 0;
        }
    }

   /*  println!(
        "you have {} stars, so you are rank{}",
        stars,
        get_rank(stars, &stars_for_rank)
    );*/
    println!("{}", get_rank(stars, &stars_for_rank));
}
fn get_rank(stars: u8, stars_for_rank: &Vec<u8>) -> usize {
    let mut stars_needed_for_rank = 0;

    for (i, &stars_for_this_rank) in stars_for_rank.iter().enumerate() {
        stars_needed_for_rank += stars_for_this_rank as u8;
        if stars <= stars_needed_for_rank {
            return LOWEST_RANK - i; //current rank
        }
    }
    return 25;
}
