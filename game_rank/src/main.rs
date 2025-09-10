use std::io;

const LOWEST_RANK: usize = 25;
const LEGEND_RANK: usize = 0;
const THREE_STAR_THRESHOLD: usize = 20;
const FOUR_STAR_THRESHOLD: usize = 16;
const FIVE_STAR_THRESHOLD: usize = 10;
fn main() {
    //  println!("Hello, world!");

    let mut winstreak = 0;
    let mut losestreak = 0;
    let mut curent_stars: u32 = 0;
    let mut curent_rank: u32 = LOWEST_RANK.try_into().unwrap();

    let mut stars_for_rank: Vec<u32> = Vec::new();

    for rank in (LEGEND_RANK..=LOWEST_RANK).rev() {
        let temp = if rank > THREE_STAR_THRESHOLD {
            2
        } else if rank > FOUR_STAR_THRESHOLD {
            3
        } else if rank > FIVE_STAR_THRESHOLD {
            4
        } else if rank > LEGEND_RANK {
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
            losestreak =0;

            if winstreak >= 3 {
                curent_stars += 2;
            } else {
                curent_stars += 1;
                
            }
            curent_rank = get_rank(curent_stars, &stars_for_rank);
            
        } else if game.eq_ignore_ascii_case(&'L') {
            losestreak +=1;
            winstreak = 0;
            if curent_stars != 0 && get_rank(curent_stars-1, &stars_for_rank) <= 20 && curent_rank as usize != LEGEND_RANK{
               
                curent_stars -= 1;
                if losestreak >=2 {
                    curent_rank = get_rank(curent_stars, &stars_for_rank);
                }
                
            }

            
        }
    }

   // println!("stars: {}", curent_stars);
    println!("{}", curent_rank);
}
fn get_rank(curent_stars: u32, stars_for_rank: &Vec<u32>) -> u32 {
    let mut stars_needed_for_rank = 0;

    for (i, &stars_for_this_rank) in stars_for_rank.iter().enumerate() {
        stars_needed_for_rank += stars_for_this_rank as u32;
        if curent_stars <= stars_needed_for_rank  {
            return (LOWEST_RANK - i).try_into().unwrap(); //current rank
        }
    }
    return 25;
}
