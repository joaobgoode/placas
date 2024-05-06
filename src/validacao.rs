pub mod validacao{
    
    pub fn completa(placa: &str) -> bool {

        let plc = placa.bytes().collect::<Vec<u8>>();

        plc.len() == 7 &&
        plc[0].is_ascii_uppercase() &&
        plc[1].is_ascii_uppercase() &&
        plc[2].is_ascii_uppercase() &&
        plc[3].is_ascii_digit() &&        
        plc[4].is_ascii_uppercase() &&
        plc[4] <= b'J' &&
        plc[5].is_ascii_digit() &&
        plc[6].is_ascii_digit()
    
    }

    #[allow(dead_code)]
    pub fn comeco(placa: &str) -> bool {

        let plc = placa.bytes().collect::<Vec<u8>>();

        plc.len() != 3 &&
        plc[0].is_ascii_uppercase() &&
        plc[1].is_ascii_uppercase() &&
        plc[2].is_ascii_uppercase()

    }

}