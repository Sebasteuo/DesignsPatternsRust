//Visualización de temperatura, implementado con Observer
trait Observer {
    fn update(&self);
}

// Esto guarda a los observadores y los notifica
trait Observable<'a, T: Observer> {
    fn add_observer(&mut self, observer: &'a T);
    fn delete_observer(&mut self, observer: &'a T);
    fn notify_observers(&self);
}

// Define al Observador y el observable
struct Display {
    name: String,
}
struct Weather<'a, T:'a> {
    temperature: f64,
    observers: Vec<&'a T>
}
impl<'a> Weather<'a, Display> {
    fn set_temperature(&mut self, temperature: f64) {
        self.temperature = temperature;
        self.notify_observers();
    }
}
/*
 * Traits implementations
 */
//impl<U: Observable> Observer<U> for Display<U> {
impl Observer for Display {
    fn update(&self) {
        println!("Display {} actualizado!", self.name);
    }
}
impl Display {
    fn new(name: String) -> Display {
        Display{name: name}
    }
}
impl std::cmp::PartialEq for Display {
    fn eq(&self, other: &Display) -> bool {
        self.name == other.name
    }
}
impl std::fmt::Display for Display {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Display {}", self.name)
    }
}
impl<'a, T: Observer+PartialEq+std::fmt::Display> Observable<'a, T> for Weather<'a, T> {
    fn add_observer(&mut self, observer: &'a T) {
        println!("Añadir Observador({});", observer);
        self.observers.push(observer);
    }
    fn delete_observer(&mut self, observer: &'a T) {
        let mut index = 0;
        let mut found = false;
        for &obs in self.observers.iter() {
            if obs == observer {
                println!("Borrar Observador({});", observer);
                found = true;
                break;
            }
            index += 1;
        }
        if found {
            self.observers.remove(index);
        }
    }
    fn notify_observers(&self) {
        for &observer in self.observers.iter() {
            observer.update();
        }
    }
}