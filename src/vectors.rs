pub fn main() {
    let vector = [0, 1, 2, 3, 4];

    for (index, value) in vector.iter().enumerate() {
        println!("&value[{}] = {:p}, vector[{}] = {}", index, &value, index, value);
    }
    println!("----------");
    for (index, value) in vector.iter().enumerate() {
        println!("&vector[{}] = {:p}, vector[{}] = {}", index, &vector[index], index, value);
    }
    println!("----------");
    for (index, value) in vector.iter().enumerate() {
        println!("&value[{}] = {:p}, &vector[{}] = {:p}", index, &value, index, &vector[index]);
    }
}