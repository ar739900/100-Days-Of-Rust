fn pair_of_socks(socks: &str) -> i32 {
    let mut pair = 0;
    let mut socks = socks.chars().collect::<Vec<char>>();
    while let Some(sock) = socks.pop() {
        for sock2 in &socks {
            if sock == *sock2 {
                pair += 1;
                socks.remove(socks.iter().position(|&x| x == *sock2).unwrap());//took help from stackoverflow
                break;
            }
        }
    }
    if pair > 0 {
        return pair
    }
    else{
        return 0
    }
}

fn main() {
    println!("{}",pair_of_socks("CABBACCC"));
}
