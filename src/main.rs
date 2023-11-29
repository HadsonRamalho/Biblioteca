use std::io;
use std::io::Read;
use std::process::exit;

struct Livro{
    titulo:String,
    id:u32,
    autor:String
}

impl Livro {
    fn new(titulo: String, id: u32, autor: String) -> Livro {
        Livro { titulo, id, autor }
    }
}

fn stoi(y:String) -> i32{ // Transformando uma string numérica em i32
    let y: i32 = match y.trim().parse() {
        Ok(num) => num,
        Err(_) => return 0
    };
    return y
}

fn mgl_listar(biblioteca:&mut Vec<Livro>){
    println!("Livros cadastrados:");
    for livro in biblioteca{
            println!("Titulo: {}\nID: {}\nAutor: {}", livro.titulo, livro.id, livro.autor);
    }
}

fn mgl_adicionar(biblioteca: &mut Vec<Livro>, T:&mut u32){
    println!("\tCadastrando novo livro!");

    println!("Digite o titulo do livro: ");
    let mut titulo = String::new();
    io::stdin()
        .read_line(&mut titulo)
        .expect("Erro ao ler o titulo do livro");

    println!("Digite o autor do livro: ");
    let mut autor = String::new();
    io::stdin()
        .read_line(&mut autor)
        .expect("Erro ao ler o autor do livro");

    let novo_livro = Livro::new(titulo.trim().to_string(), *T, autor.trim().to_string());
    *T += 1;
    biblioteca.push(novo_livro);
    println!("Livro cadastrado!");
}

fn menu_gerencia_livros(biblioteca:&mut Vec<Livro>, id_contador:&mut u32){
    println!("1 - Listar todos os livros");
    println!("2 - Adicionar um livro");
    let mut opc = Default::default();
    io::stdin()
        .read_line(&mut opc)
        .expect("Erro ao ler a opção");
    let opc = stoi(opc);
    match opc{
        1 => mgl_listar(biblioteca),
        2 => mgl_adicionar(biblioteca, id_contador),
        3 => println!("Saindo"),
        _ => println!("Opção inválida")
    }
}

fn menu(){
    println!(" Menu Principal\n1 - Buscar livro\n2 - Gerenciar livros\n3 - Sair");
    let mut opc = Default::default();
    let mut biblioteca : Vec<Livro> = Vec::new();
    let mut id_contador:u32 = 1;
    io::stdin()
        .read_line(&mut opc)
        .expect("Erro ao obter a opção");

    let opc = stoi(opc);
    while opc != 3 {
        match opc {
            1 => println!("ok"),
            2 => menu_gerencia_livros(&mut biblioteca, &mut id_contador),
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
