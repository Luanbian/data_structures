pub fn main() {
    let a = 10;
    let b = 20;

    // Criando uma referência segura a 'a'
    let ref_a: &i32 = &a;

    // Criando um ponteiro bruto para 'a'
    let ptr_a: *const i32 = &a;

    // Mostrando endereços de memória
    println!("Endereço de a (referência): {:p}", ref_a);
    println!("Endereço de a (ponteiro bruto): {:p}", ptr_a);
    println!("Endereço de b (referência): {:p}", &b);

    // Desreferenciando o ponteiro bruto (unsafe)
    unsafe {
        println!("Valor de a através do ponteiro bruto: {}", *ptr_a);
    }
}

pub fn change_value() {
    let mut a: i32 = 10;
    let p1: *mut i32 = &mut a;
    let p2: *mut i32 = p1;

    unsafe {
        *p2 = 5;
    }

    println!("O valor de a é: {}", a);
}

pub fn exercise() {
    let mut a = 4;
    let mut b = 3;
    let mut p1: *mut i32 = &mut a;
    let p2: *mut i32 = p1;

    unsafe {
        *p2 = *p1 + 3;
        b = b * (*p1);
        *p2 += 1;
    }

    p1 = &mut b;

    unsafe {
        println!("{:?}, {:?}", p1, p2);
        println!("{}, {}", *p1, *p2);
        println!("{}, {}", a, b);
    }
}