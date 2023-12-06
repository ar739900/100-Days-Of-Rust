
fn progressDay(arr:&[i16]) -> i8{
    let mut count:i8=0;
    for i in 0..arr.len()-1{
        if arr[i+1] > arr[i] {
            println!("Progress day: {} -> {}", arr[i], arr[i+1]);
            count+=1;
        }
    }
    return count
}

fn main() {
    println!("There are {} progress days",progressDay(&[10, 11, 12, 9, 10]));
}