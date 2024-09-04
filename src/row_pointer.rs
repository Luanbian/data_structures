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