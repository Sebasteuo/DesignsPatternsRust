pub struct Refresco{ // "Clase" Refresco

}

impl Refresco { //Implementacion de la clase Refresco

    /* Distintos "fn" que funcionan como métodos que muestran
      información sobre distintos tipos de refresco */

    fn coca_cola(&self){
        print!("Su refresco es coca cola\n");
    }


    fn fanta(&self){
        print!("Su refresco es fanta\n");
    }

    fn pepsi(&self){
        print!("Su refresco es pepsi\n");

    }
}


pub struct pizza{ // "Clase" pizza

}

impl pizza{ // Implementacion de la clase pizza

    /* Distintos "fn" que funcionan como métodos que muestran
      información sobre distintos tipos de pizza */

    fn pizza_jyq(&self){
        print!("Su pizza es suprema\n");

    }

    fn pizza_hawaiana(&self){
        print!("Su pizza es hawaina\n");
    }

    fn pizza_suprema(&self){
        print!("Su pizza es suprema\n");

    }
}

struct abstract_factory{ // Esta es el "struct" que servirá como la clase Abstract Factory
    orden:i32,

}

impl abstract_factory { // Se implementa la clase "abstract_factory"

    /* De acuerdo al numero que reciba como parámetro la clase "abtract_factory" se
    elegirá un determinado combo la cual es categoría de una familia de productos y se producirá
    dicho combo*/

    fn elegir_orden(&self){
        if self.orden==1 {
            let x= Refresco{};
            let x1 = &x;

            let y = pizza {};
            let y1 = &y;

            x1.coca_cola();
            y1.pizza_jyq();
        }
        else if self.orden==2 {
            let x= Refresco{};
            let x1 = &x;

            let y = pizza {};
            let y1 = &y;

            x1.fanta();
            y1.pizza_hawaiana();

        }else{
            let x= Refresco{};
            let x1 = &x;

            let y = pizza {};
            let y1 = &y;

            x1.pepsi();
            y1.pizza_suprema();
        }
    }
}