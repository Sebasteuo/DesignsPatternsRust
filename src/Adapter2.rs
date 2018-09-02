
/// Obtiene un sonido de rugido.
trait SonidoSalvaje{
    fn rugir(&self) -> String;
}

pub struct Oso<'a>{
    nombre: &'a str,
}

/**
* Obtiene un sonido salvaje.
*/
impl<'a> SonidoSalvaje for Oso<'a>{
    fn rugir(&self) -> String{
        let rugido: &str = " a rugido";
        let unir = format!("{} {}", self.nombre, rugido);

        return unir;
    }
}

pub struct OsoTeddy<'a>{
    nombre: &'a str,
}

/**
* Obtiene un sonido adorable.
*/
impl<'a> OsoTeddy<'a>{
    fn sonar(&self) -> String{
        let unir = format!("{}{}", self.nombre, ": Te amo!");
        return unir;
    }
}

pub struct OsoAdapter<'a>{
    teddy: OsoTeddy<'a>,
}

/**
* Adapta el sonido adorable para que suene con rugir.
*/
impl<'a> SonidoSalvaje for OsoAdapter<'a>{
    fn rugir(&self) -> String {
        return self.teddy.sonar();
    }
}
