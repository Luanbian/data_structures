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

pub fn walk() {
    let vector = [10 , 20, 30, 40];

    let first_adress: *const i32 = &vector[0];
    let second_adress = unsafe {
        first_adress.add(1)
    };
    let second_value = unsafe {
        *second_adress
    };
    
    println!("{:p}", first_adress);
    println!("{:p}", second_adress);
    println!("{}", vector[0]);
    println!("{}", second_value);
}