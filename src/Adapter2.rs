
trait SonidoSalvaje{
    fn rugir(&self) -> String;
}

pub struct Oso<'a>{
    nombre: &'a str,
}

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

impl<'a> OsoTeddy<'a>{
    fn sonar(&self) -> String{
        let unir = format!("{}{}", self.nombre, ": Te amo!");
        return unir;
    }
}

pub struct OsoAdapter<'a>{
    teddy: OsoTeddy<'a>,
}


impl<'a> SonidoSalvaje for OsoAdapter<'a>{
    fn rugir(&self) -> String {
        return self.teddy.sonar();
    }
}
