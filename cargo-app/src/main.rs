fn main() {

    let animals = vec!["Rabbit","cat","Dog"];
    for a in animals.iter().enumerate(){
        println!("data is {}",a);
    }
}
