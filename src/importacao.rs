pub mod get {

    pub const PLACAS: [&'static str; 181] = const_str::split!(include_str!("./placas.txt"),"\r\n");
    
}