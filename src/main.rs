mod row_pointer;
fn main() {
    let a = 10;
    let b;
    let c;
    b = 20;
    c = a + b;
    println!("{}", c); // mostrando valor
    println!("{:p}", &c); // mostrando endereço na memória
    row_pointer::main();
    row_pointer::change_value();
}
