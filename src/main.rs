#![allow(non_snake_case)]
include!("Facade.rs");


fn main() {

    println!();
    println!();
    //*********Utilizando el patron FACADE********

    println!("********Utilizando FACADE*****\n");

    println!("###Encendiendo piloto automatico###\n");


    let p=FacadePiloto{y:true};
    p.validacion();

    print!("\n");
    println!("###Apagando piloto automatico###\n");

    let p=FacadePiloto{y:false};
    p.validacion();
}

