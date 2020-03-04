fn five() -> i32 {
    let ret = {
        let x = 4;
        let y = 3;
        x+y
    };
    ret
}

fn main() {
    let x = five();
    
    let g = 5;
    let g = 4;

    println!("The value of x is: {}", x);
}
