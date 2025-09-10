use std::io;

const LOWEST_RANK: usize = 25;
const LEGEND_RANK: usize = 0;
const THREE_STAR_THRESHOLD: usize = 20;
const FOUR_STAR_THRESHOLD: usize = 16;
const FIVE_STAR_THRESHOLD: usize = 10;
fn main() {
    //  println!("Hello, world!");

    let mut winstreak = 0;
    let mut curent_stars: u8 = 0;
    let mut curent_rank: u8 = LOWEST_RANK.try_into().unwrap();

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
            winstreak += 1;

            if winstreak >= 3 {
                curent_stars += 2;
            } else {
                curent_stars += 1;
            }
            if curent_rank > get_rank(curent_stars, &stars_for_rank) {
                curent_rank = get_rank(curent_stars, &stars_for_rank);
               // println!("rank updated to {}", curent_rank);
            }
        } else if game.eq_ignore_ascii_case(&'L') {
            if get_rank(curent_stars, &stars_for_rank) <= 20 {
                curent_stars -= 1;
                if curent_rank > get_rank(curent_stars, &stars_for_rank) {
                    curent_rank = get_rank(curent_stars, &stars_for_rank);
                }
            }

            winstreak = 0;
        }
    }

    println!("stars: {}", curent_stars);
    println!("{}", curent_rank);
}
fn get_rank(curent_stars: u8, stars_for_rank: &Vec<u8>) -> u8 {
    let mut stars_needed_for_rank = 0;

    for (i, &stars_for_this_rank) in stars_for_rank.iter().enumerate() {
        stars_needed_for_rank += stars_for_this_rank as u8;
        if curent_stars < stars_needed_for_rank  {
            return (LOWEST_RANK - i).try_into().unwrap(); //current rank
        }
    }
    return 25;
}
