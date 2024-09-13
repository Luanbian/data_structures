struct Aluno {
    nome: String,
    idade: u32,
}

impl Aluno {
    fn constructor(nome: String, idade: u32) -> Aluno {
        Aluno {nome, idade}
    }

    fn greeting(&self) {
        println!("Saudação aluno {} que tem {} anos", self.nome, self.idade);
    }
}

pub fn main() {
    let aluno = Aluno::constructor(String::from("Luan"), 22);
    aluno.greeting();
}