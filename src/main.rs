use std::io;
use std::io::Read;
use log::error;

struct Biblioteca{
    titulo:String,
    id:u32,
    autor:String
}

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
/*
    livro.push(Biblioteca {
        titulo: String::from("Livro1"),
        id: 1,
        autor: String::from("Autor1"),});
    livro.push(Biblioteca {
        titulo: String::from("Livro2"),
        id: 2,
        autor: String::from("Autor2")});
*/

fn main() {
    let mut livro : Vec<Biblioteca> = Vec::new();

    let mut i = 0;
    let mut n = i.to_string();
    while i < 3{
        i = i + 1;
        let n = i.to_string();
        let t = "Titulo";
        let conc = format!("{}{}", t, n);
        livro.push(Biblioteca {
            titulo: String::from(conc),
            id: i,
            autor: String::from("Autor{n}"),
        });
        //println!("Titulo: {}\nID: {}\nAutor: {}\n", livro.titulo, livro.id, livro.autor);
    }
    for livro in &livro{
        println!("Titulo: {}\nID:{}\nAutor: {}", livro.titulo, livro.id, livro.autor);
    }
    menu();
}