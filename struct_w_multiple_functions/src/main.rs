use rand::Rng;

fn main() {
    let rec1 = Rectangle { l: rand::thread_rng().gen_range(1, 11), w:rand::thread_rng().gen_range(1, 11) };
    let rec2 = Rectangle { l: rand::thread_rng().gen_range(1, 11), w:rand::thread_rng().gen_range(1, 11) };


    println!("{:?} {:?}", rec1,rec2);
    println!("Can rec1 fit in rec2 currently: {}", rec1.fits_in(&rec2));
    println!("Can rec1 fit in rec2 in any orientation?: {}", rec1.fits_in_either_way(&rec2));
}

#[derive(Debug)]
struct Rectangle {
    w: i32,
    l: i32,
}

impl Rectangle {
    // does one rectangle fit in another
    fn fits_in(&self, other_rec: &Rectangle) -> bool {
        return self.w < other_rec.w && self.l < other_rec.l
    }

    // does one rectangle fit in another if you flip it
    fn fits_in_either_way(&self, other_rec: &Rectangle) -> bool {
        return (self.w < other_rec.w && self.l < other_rec.l) || (self.w < other_rec.l && self.l < other_rec.w)
    }

}
