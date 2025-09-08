use std::io;
fn main() {
    println!("Hello, world!");
    const LOWEST_RANK: u8 = 25;
    let mut winstreak = 0;
    let mut stars = 0;
    let mut temp: u8 = 0;

    let mut total_stars_rank: Vec<u8> = Vec::new();

    for i in (0..=25).rev() {
        if  i == 25{
            
        } else if i >= 20 {
            temp += 2
        } else if i >= 16 {
            temp += 3
        } else if i >= 10 {
            temp += 4
        } else {
            temp += 5
        }
        total_stars_rank.push(temp as u8);
    }

    print!("{:?}", total_stars_rank);

    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let match_history: Vec<char> = input.trim().chars().collect::<Vec<char>>();

    print!("{:?}", match_history);
    for game in &match_history {
        if game.eq_ignore_ascii_case(&'W') {
            stars += 1;
            winstreak += 1;
            println!("{}", stars);
            if winstreak >= 3 {
                stars += 1;
            }
        } else if game.eq_ignore_ascii_case(&'L') {
            if stars > total_stars_rank[25 -20] {
                 stars -= 1;
            }
           
            winstreak = 0;
        }
    }
      for i in (0..=25).rev() {
        if stars >  total_stars_rank[i] {
            
           println!("you have {} stars, so you are rank{}",stars,  25- i);
           println!("{}", total_stars_rank[25 -20]);
           return; 
        }
    }
}
