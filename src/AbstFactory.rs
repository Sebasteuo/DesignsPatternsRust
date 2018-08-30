pub struct Refresco{

}

impl Refresco {

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


pub struct pizza{

}

impl pizza{

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

struct abstract_factory{
    orden:i32,

}

impl abstract_factory {
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