#![allow(non_snake_case)]
include!("Facade.rs");
include!("AbstFactory.rs");
include!("Builder.rs");

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



    //*********Utilizando el patron ABSTRACT FACTORY********
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



}

//Referencias : Patron Builder: https://goyox86.gitbooks.io/el-libro-de-rust/content/method-syntax.html

