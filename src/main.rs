mod contagem;
mod importacao;
mod pesquisa;
mod validacao;
pub use crate::validacao::validacao::completa;

fn desenho() {

    println!(
        "\x1b[0;34m{}\x1b[0m",
        " 
    #####################################################################
    #   #####   ###### #     #    #    ####### ######     #    #     #  #
    #  #     # #       ##    #   # #      #    #     #   # #   ##   ##  #
    #  #       #       # #   #  #   #     #    #     #  #   #  # # # #  #
    #   #####  #####   #  #  # #     #    #    ######  #     # #  #  #  #
    #        # #       #   # # #######    #    #   #   ####### #     #  #
    #  #     # #       #    ## #     #    #    #    #  #     # #     #  #
    #   #####   ###### #     # #     #    #    #     # #     # #     #  #
    #                                                                   #
    #  #     #  ###### ######   #####   #####   #####  #     # #        #
    #  ##   ## #       #     # #     # #     # #     # #     # #        #
    #  # # # # #       #     # #       #     # #       #     # #        #
    #  #  #  # #####   ######  #       #     #  #####  #     # #        #
    #  #     # #       #   #   #       #     #       # #     # #        #
    #  #     # #       #    #  #     # #     # #     # #     # #        #
    #  #     #  ###### #     #  #####   #####   #####   #####   ######  # 
    #####################################################################
    "
    );
    
}

fn valida() -> Result<bool, std::io::Error> {

    println!("Digite a placa:");

    let mut placa = String::new();
    std::io::stdin().read_line(&mut placa)?;
    let placa = placa.trim();

    if completa(placa) {

        return Ok(true);
    
    }

    Ok(false)
}

fn placaestado() -> Result<String, std::io::Error> {

    println!("Digite a placa:");

    let mut placa = String::new();
    std::io::stdin().read_line(&mut placa)?;
    let placa = placa.trim();

    if completa(placa) {

        return Ok(pesquisa::pesquisa::estado(placa).to_string());
    
    }

    Ok("Placa inválida".to_string())
}

fn conta() -> Result<String, std::io::Error> {

    println!("Digite o estado:");
    let mut estado = String::new();
    std::io::stdin().read_line(&mut estado)?;

    Ok(estado)

}

fn main() {

    let mut opcao = String::new();

    'menu: loop {

        opcao.clear();

        desenho();
        println!("O que deseja fazer?\n1 - Validar placa\n2 - Pesquisar estado\n3 - Listar placas de um Estado\nOutro - Sair");
        std::io::stdin().read_line(&mut opcao).unwrap();
        let opcao: &str = opcao.trim();

        match opcao {

            "1" => match valida() {

                Ok(res) => {

                    if res {

                        println!("\x1b[0;32m{}\x1b[0m", "Placa válida");

                    } else {
                        
                        println!("\x1b[0;31m{}\x1b[0m", "Placa inválida");

                    }

                }

                Err(_) => println!("Erro ao validar placa"),

            },
            "2" => match placaestado() {
                
                Ok(res) => {
                    
                    if res == "Placa inválida" {
                        
                        println!("\x1b[0;31mPlaca Invalida\x1b[0m");
                    
                    } else {
                        
                        println!("\x1b[0;32m{}\x1b[0m", res);

                    }

                }

                Err(_) => println!("Erro ao pesquisar placa"),
            },
            "3" => match conta() {

                Ok(estado) => {
                
                    print!("\x1B[2J\x1B[1;1H");
                    desenho();
                
                    pesquisa::pesquisa::display(&estado.trim());
                
                }
                
                Err(_) => println!("Erro ao listar placas"),
            
            },
            _ => {
            
                println!("Saindo...");
                break;
            
            }
        }
        
        let mut outra = String::new();
        
        'outro: loop {
        
            outra.clear();
            println!("Deseja fazer outra operação? (s/n)");
        
            std::io::stdin().read_line(&mut outra).unwrap();
            match outra.trim() {
        
                "s" | "S" => {

                    print!("\x1B[2J\x1B[1;1H");
        
                    continue 'menu;
        
                }
        
                "n" | "N" => break 'menu,
                _ => continue 'outro,
        
            }
        }
    }

    println!("Pressione qualquer tecla para sair");
    let mut sair = String::new();
    std::io::stdin().read_line(&mut sair).unwrap();

}
