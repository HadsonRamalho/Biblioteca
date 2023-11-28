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

fn mgl_listar(mut livro:Vec<Biblioteca>, T:u32){

    /*let mut i = 0;
    let itos = i.to_string();
    while i < 3{
        i += 1;
        let _itos = i.to_string();
        let t = "Titulo";
        let conc = format!("{}{}", t, itos);
        livro.push(Biblioteca {
            titulo: String::from(conc),
            id: i,
            autor: String::from("Autor{n}"),
        });
        //println!("Titulo: {}\nID: {}\nAutor: {}\n", livro.titulo, livro.id, livro.autor);
    }*/
    let mut i = 0;
    for livro in &livro{
        if i < T{
            i += 1;
            println!("Titulo: {}\nID:{}\nAutor: {}", livro.titulo, livro.id, livro.autor);
        }
    }
}

fn mgl_adicionar(mut livro:Vec<Biblioteca>, mut T:u32){
    println!("\tCadastrando novo livro!");
    println!("Digite o titulo do livro: ");
    let mut titulo = Default::default();
    io::stdin()
        .read_line(&mut titulo)
        .expect("Erro ao ler o titulo do livro");
    println!("Digite o autor do livro: ");
    let mut autor = Default::default();
    io::stdin()
        .read_line(&mut autor)
        .expect("Erro ao ler o autor do livro");
    livro.push(Biblioteca {
        titulo: titulo,
        id: 1,
        autor: autor,});
    T += 1;
}

fn menu_gerencia_livros(){

    let livro : Vec<Biblioteca> = Vec::new();
    let T:u32 = 0;

    println!("1 - Listar todos os livros");
    println!("2 - Adicionar um livro");
    let mut opc = Default::default();
    io::stdin()
        .read_line(&mut opc)
        .expect("Erro ao ler a opção");
    let opc = stoi(opc);
    match opc{
        1 => mgl_listar(livro, T),
        2 => mgl_adicionar(livro, T),
        _ => println!("Opção inválida")
    }
}

fn menu(){
    println!(" Menu Principal\n1 - Buscar livro\n2 - Gerenciar livros\n3 - Sair");
    let mut opc = Default::default();
    io::stdin()
        .read_line(&mut opc)
        .expect("Erro ao obter a opção");

    let opc = stoi(opc);
    while opc != 3{
        match opc{
            1 => println!("ok"),
            2 => menu_gerencia_livros(),
            3 => break,
            _ => println!("Opção inválida")
        }  
    }
    
}
/*

    livro.push(Biblioteca {
        titulo: String::from("Livro2"),
        id: 2,
        autor: String::from("Autor2")});
*/

fn main() {
    menu();
}
