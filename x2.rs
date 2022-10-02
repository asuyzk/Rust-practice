fn main(){
    let mut num = 1;

    loop {
        println!("{}",num);

        num = x2(num);
    }
}

fn x2(num: u64) -> u64 {
    num * 2
}

