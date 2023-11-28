use std::io;
use std::io::Read;
use log::error;


fn stoi(y:String) -> i32{ // Transformando uma string numérica em i32
    let y: i32 = match y.trim().parse() {
        Ok(num) => num,
        Err(_) => return 0
    };
    return y
}

fn menu(){
    println!(" Menu Principal\n1 - Buscar livro\n2 - Gerenciar livros");
    let mut opc = Default::default();
    io::stdin()
        .read_line(&mut opc)
        .expect("Erro ao obter a opção");

    let opc = stoi(opc);

    match opc{
        1 => println!("ok"),
        _ => println!("Opção inválida")
    }
}
fn main() {
    menu();
}