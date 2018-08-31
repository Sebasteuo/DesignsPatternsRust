struct PilotoAutomatico {} //Este "struct" hace las veces de clase "PilotoAutomatico"

/* Mediante este "impl" se hace la definición de los métodos que tendría
 la calse PilotoAutomatico */

impl PilotoAutomatico {

    //Estos "fn" funcionan como los métodos de la clase PilotoAutomatico

    pub fn altura_vuelo_aut(&self){
        println!("Ajustando altura crucero del avion automaticamente");
    }

    pub fn velocidad_vuelo_aut(&self){
        println!("Regulando la velocidad de la aeronave");
    }

    pub fn direccion_vuelo_aut(&self){
        println!("Angulos de inclinacion latitudinal y longitudianal establecidos");
    }


    pub fn altura_vuelo_man(&self){
        println!("Piloto favor alcanzar altura crucero de vuelo");
    }

    pub fn velocidad_vuelo_man(&self){
        println!("Velocidad de vuelo regulada manaualmente a partir de este momento");
    }

    pub fn direccion_vuelo_man(&self){
        println!("Direccion por medio de timon del avion");
    }

}

pub struct FacadePiloto{ //Este "struct" vendría siendo la clase Facade para la clase PilotoAutomatio
    y:bool,
}


impl FacadePiloto{ //Se hace la implementación porpiamente del Facade

/* Este "fn" actúa como un método que si lo que recibe un true o un
false activa o desactiva el piloto automatico respectivamente */

    fn validacion(self) {

        let p= PilotoAutomatico{};
        let p1 =&p;
        if true == self.y {
            p1.altura_vuelo_aut();
            p1.velocidad_vuelo_aut();
            p1.direccion_vuelo_aut();
        } else {
            p1.altura_vuelo_man();
            p1.velocidad_vuelo_man();
            p1.direccion_vuelo_man();
        }
    }
}