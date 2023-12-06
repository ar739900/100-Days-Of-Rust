
fn skewer(arr:&[&str]) -> [usize;2] {
    let mut veg:usize=0;
    let mut non_veg:usize=0;
    for i in 0..arr.len(){
        if arr[i].contains('x') {
            non_veg+=1;
        } else {
        veg+=1;
        }
    }
    return [veg,non_veg]
}

fn main() {
    println!("{:?}",skewer(&["--xo--x--ox--","--xx--x--xx--","--oo--o--oo--","--xx--x--ox--","--xx--x--ox--","--oooo-ooo--","--xx--x--xx--","--o---o--oo--","--xx--x--ox--","--xx--x--ox--"]));
    
    //Replace the array of string with your desired string array to check the code.
}
