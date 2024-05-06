pub mod contagem {

    pub fn para_numero(placa: &str) -> i32 {

        let plc = placa.bytes().collect::<Vec<u8>>();

        (plc[0] as i32 - 65)*26*26 + (plc[1] as i32 - 65)*26 + (plc[2] as i32 - 65)

    }
    
    pub fn quantidade(intervalo: &str) -> i32 {

        let campos: Vec<&str> = intervalo.split(" ").collect();

        para_numero(campos[1]) - para_numero(campos[0]) + 1

    }

}
