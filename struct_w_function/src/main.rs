fn main() {
    let test = FirstStruct { name: String::from("test"), number: 14, boolean: true};
    test.print_struct();
}

impl FirstStruct {
    fn print_struct(&self) {
        println!("{} {} {}", self.name, self.number, self.boolean);
    }
}

#[derive(Debug)]
struct FirstStruct {
    name: String,
    number: i32,
    boolean: bool
}
