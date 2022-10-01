fn main(){
    let mut num = 1;

    loop {
        println!("{}",num);

        num = x2(num);
    }
}

fn x2(num: i32) -> i32 {
    num * 2
}

