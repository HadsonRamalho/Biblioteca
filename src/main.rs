use std::fs::File;
use std::io::{self, Write};
use bincode::serialize;
use serde::{Deserialize, Serialize}; // Adicione esta linha

#[derive(Debug, Serialize, Deserialize)] // Adicione Serialize e Deserialize aqui
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

fn exportar_arquivo(biblioteca: &Vec<Livro>) -> io::Result<()> {
    let file_path = "lista_livros.bin";

    // Serializa a lista de livros para um vetor de bytes
    let encoded: Vec<u8> = serialize(biblioteca).map_err(|e| {
        io::Error::new(io::ErrorKind::Other, format!("Failed to serialize: {}", e))
    })?;

    // Abre o arquivo em modo de escrita
    let mut file = File::create(file_path)?;

    // Escreve os bytes no arquivo
    file.write_all(&encoded)?;

    println!("Arquivo {} criado com sucesso.", file_path);

    Ok(())
}


fn mgl_listar(biblioteca:&mut Vec<Livro>){
    println!("Livros cadastrados:");
    for livro in biblioteca{
        println!("Titulo: {}\nID: {}\nAutor: {}", livro.titulo, livro.id, livro.autor);
    }
}

fn mgl_adicionar(biblioteca: &mut Vec<Livro>, t:&mut u32){
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

    let novo_livro = Livro::new(titulo.trim().to_string(), *t, autor.trim().to_string());
    *t += 1;
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
    println!(" Menu Principal\n1 - Buscar livro\n2 - Gerenciar livros\n3 - Exportar lista de livros\n4 - Sair");
    let mut opc = Default::default();
    let mut biblioteca : Vec<Livro> = Vec::new();
    let mut id_contador:u32 = 1;
    io::stdin()
        .read_line(&mut opc)
        .expect("Erro ao obter a opção");
    let mut opc = stoi(opc);
    while opc != 4 {
        println!(" Menu Principal\n1 - Buscar livro\n2 - Gerenciar livros\n3 - Exportar lista de livros\n4 - Sair");
        let mut op:String = Default::default();
        io::stdin()
            .read_line(&mut op)
            .expect("Erro ao obter a opção");
        let op = stoi(op);
        opc = op;
        match opc {
            1 => {
                println!("ok");
                break;
            },
            2 => {
                menu_gerencia_livros(&mut biblioteca, &mut id_contador);
                //break;
            },
            3 => {
                if let Err(e) = exportar_arquivo(&mut biblioteca) {
                    eprintln!("Erro: {}", e);
                }
                break;
            },
            4 => break,
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