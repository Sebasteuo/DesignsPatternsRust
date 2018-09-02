#![allow(non_snake_case)]
include!("Facade.rs");
include!("AbstFactory.rs");
include!("Builder.rs");
include!("Adapter2.rs");

fn main() {

    println!();
    println!();
    //*********Utilizando el patron FACADE********

    println!("********Utilizando el patron FACADE*****\n");

    println!("###Encendiendo piloto automatico###\n");


    let a=FacadePiloto{y:true};
    a.validacion();

    print!("\n");
    println!("###Apagando piloto automatico###\n");

    let a=FacadePiloto{y:false};
    a.validacion();

    print!("\n");
    print!("\n");


    //*********Utilizando el patron ABSTRACT FACTORY********

    println!("*********Utilizando el patron ABSTRACT FACTORY********\n");
    //@@@@@ COMBO#1 @@@@@
    println!("@@@@@ COMBO#1 LISTO @@@@@");
    let b=abstract_factory{orden:1};
    b.elegir_orden();
    println!();

    //@@@@@ COMBO#2 @@@@@
    println!("@@@@@ COMBO#2 LISTO @@@@@");
    let b=abstract_factory{orden:2};
    b.elegir_orden();
    println!();

    //@@@@@ COMBO#2 @@@@@
    println!("@@@@@ COMBO#1 LISTO @@@@@");
    let b=abstract_factory{orden:3};
    b.elegir_orden();
    print!("\n");
    print!("\n");



    //*********Utilizando el patron BUILDER********
    println!("*********Utilizando el patron BUILDER********\n");
    print!("\n");

    let c = ConstructorCuadrado::new()
        .x(1.0)
        .y(2.0)
        .lado(2.0)
        .finalizar();

    println!("area: {}", c.area());
    println!("x: {}", c.x);
    println!("y: {}", c.y);


    //***********Utilizando el patron Observer**********

    println!("*********Utilizando el patron OBSERVER********\n\n");

    let display = Display::new("COMPURADORA".to_string());
    let display2 = Display::new("COMPURADORA2".to_string());
    let mut weather = Weather{temperature: 19.0, observers: Vec::new()};
    weather.add_observer(&display);
    weather.add_observer(&display2);
    weather.set_temperature(20.0);
    weather.delete_observer(&display2);
    weather.set_temperature(21.0);

    //***********Utilizando el patron ADAPTER**********
    println!("*********Utilizando el patron ADAPTER********\n\n");

    let oso = Oso{
        nombre: "Paco"
    };

    let oso_teddy = OsoTeddy{
        nombre: "Ted"
    };

    println!("{}", oso_teddy.sonar());

    let oso_adapter = OsoAdapter{
        teddy: oso_teddy
    };

    println!("{}", oso.rugir(oso_teddy));
    println!("{} || con adapter", oso_adapter.rugir());

}

//Referencias : Patron Builder: https://goyox86.gitbooks.io/el-libro-de-rust/content/method-syntax.html
//              Patron Observer : https://github.com/eliovir/rust-examples/blob/master/design_pattern-observer.rs

