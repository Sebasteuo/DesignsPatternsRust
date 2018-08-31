struct Cuadrado { // "Clase" Cuadrado
    x: f64,
    y: f64,
    lado: f64,
}

impl Cuadrado { // Se implementa la clase "Cuadrado"
    fn area(&self) -> f64 { // Se ha definido el método area() en Cuadrado
        std::f64::consts::PI * (self.lado * self.lado)
    }
}

struct ConstructorCuadrado { //Lo que se ha hecho es crear otra struct, ConstructorCuadrado
    x: f64,
    y: f64,
    lado: f64,
}
//podemos usar los métodos en ConstructorCuadrado para crear Cuadrados con las dimensiones que se quieran.

impl ConstructorCuadrado {
    fn new() -> ConstructorCuadrado {
        ConstructorCuadrado { x: 0.0, y: 0.0, lado: 5.0, }
    }

    fn x(&mut self, coordenada: f64) -> &mut ConstructorCuadrado {
        self.x = coordenada;
        self
    }

    fn y(&mut self, coordenada: f64) -> &mut ConstructorCuadrado {
        self.y = coordenada;
        self
    }

    fn lado(&mut self, lado: f64) -> &mut ConstructorCuadrado {
        self.lado = lado;
        self
    }
    /*Se agregó otro método en ConstructorCuadrado: finalizar().
    Este método crea nuestro Cuadrado desde el constructor.
    */
    fn finalizar(&self) -> Cuadrado {
        Cuadrado { x: self.x, y: self.y, lado: self.lado }
    }
}
