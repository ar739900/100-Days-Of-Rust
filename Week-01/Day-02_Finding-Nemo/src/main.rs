fn finding_Nemo(s:&str) ->i32{
    let mut count = 1;
    for word in s.split_whitespace() {
        if word == "Nemo" {
            
        }
        count += 1;
        break;
    }
    count
}




fn main() {
    let nemo = finding_Nemo("Nemooo Nemo Nemo Nemo Nemo");
    println!(" Found Nemo at index : {}", nemo);
}
