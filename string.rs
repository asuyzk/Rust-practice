fn main(){
    let mut str = String::from("This is ");
    str.push_str("String."); 

    let pushed_str = str;

    println!("{}",pushed_str);
    // println!("{}",str);
}
