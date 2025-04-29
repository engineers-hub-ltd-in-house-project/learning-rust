fn main() {
    let num = 123;
    if num % 5 == 0 {
        println!("{}は、5で割れます。", num);
    } else if num % 4 == 0 {
        println!("{}は、4で割れます。", num);
    } else if num % 3 == 0 {
        println!("{}は、３で割れます。", num);
    } else if num % 2 == 0 {
        println!("{}は、２で割れます。", num);
    } else {
        println!("{}は、うまく割れませんでした。", num);
    }
}
