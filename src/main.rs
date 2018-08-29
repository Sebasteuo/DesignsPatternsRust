#![allow(non_snake_case)]
include!("Facade.rs");
include!("AbstFactory.rs");

fn main() {

    println!();
    println!();
    //*********Utilizando el patron FACADE********

    println!("********Utilizando FACADE*****\n");

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

    let b=abstract_factory{y:1};
    b.elegir_orden();


}

