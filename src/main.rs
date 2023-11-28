use std::io;
use std::io::Read;

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

fn mgl_listar(){
    let mut livro : Vec<Biblioteca> = Vec::new();

    let mut i = 0;
    let itos = i.to_string();
    while i < 3{
        i += 1;
        let itos = i.to_string();
        let t = "Titulo";
        let a = "Autor";
        let concT = format!("{} {}", t, itos);
        let concA = format!("{} {}", a, itos);
        livro.push(Biblioteca {
            titulo: String::from(concT),
            id: i,
            autor: String::from(concA),
        });
        //println!("Titulo: {}\nID: {}\nAutor: {}\n", livro.titulo, livro.id, livro.autor);
    }
    for livro in &livro{
        println!("Titulo: {}\nID: {}\nAutor: {}", livro.titulo, livro.id, livro.autor);
    }
}

fn menu_gerencia_livros(){
    println!("1 - Listar todos os livros");
    let mut opc = Default::default();
    io::stdin()
        .read_line(&mut opc)
        .expect("Erro ao ler a opção");
    let opc = stoi(opc);
    match opc{
        1 => mgl_listar(),
        _ => println!("Opção inválida")
    }
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
        2 => menu_gerencia_livros(),
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
    menu();
}
