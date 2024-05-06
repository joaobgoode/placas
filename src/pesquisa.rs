pub mod pesquisa {

    pub use crate::contagem::contagem;
    pub use crate::importacao::get;
    use colored::*;

    pub fn estado(placainserida: &str) -> &str {

        let selecionado = &get::PLACAS
                                    .iter()
                                    .find(|placa| placa[4..=6] >= placainserida[0..3] && placa[0..=2] <= placainserida[0..3]);
        
        
        if selecionado.is_none() {

            return "Placa não encontrada";
        
        }

        &selecionado.unwrap()[8..]
    }

    pub fn todas(estado: &str) -> Vec<&&str>{

        get::PLACAS
            .iter()
            .filter(|&placa| placa[8..] == *estado)
            .collect::<Vec<&&str>>()

    }

    pub fn display(estado: &str) {

        const ESTADOS: [&str; 27] = ["Acre", "Alagoas", "Amapá", "Amazonas", "Bahia", "Ceará", "Distrito Federal", "Espírito Santo", "Goiás", "Maranhão", "Mato Grosso", "Mato Grosso do Sul", "Minas Gerais", "Pará", "Paraíba", "Paraná", "Pernambuco", "Piauí", "Rio de Janeiro", "Rio Grande do Norte", "Rio Grande do Sul", "Rondônia", "Roraima", "Santa Catarina", "São Paulo", "Sergipe", "Tocantins"];
        
        if !ESTADOS.contains(&estado) {

            println!("Estado não encontrado.");

            return;
        
        }

        println!("Intervalo  \t\tNº de Letras\tNº de Placas");

        let sum: i32 = todas(estado).iter().map(|plc| {

            let divs = &plc.split(' ').collect::<Vec<&str>>()[0..3];

            let n = contagem::quantidade(plc);

            println!("{} → {}\t\t{}\t\t{}", divs[0], divs[1], n, n * 10000);

            n
            
        }).sum();

        println!("Total de letras:\t{}", sum);
        println!("Total de placas:\t{}", (sum * 10000).to_string().green());

    }

}
