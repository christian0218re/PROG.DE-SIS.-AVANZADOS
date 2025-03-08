fn main() {
    let x = 5;
    let x = x + 1;

    println!("El valor de x es: {}", x);
    
    let entero: i32 = 42;
    let flotante: f64 = 3.14159;
    let booleano: bool = true;
    let caracter: char = 'a';
    //Tupla 
    let tupla: (i32, f64, char) = (24, 3.14, 'A');
    let arreglo: [i32; 3] = [1, 2, 3];
    println!("El valor de la tupla es: {:?}", tupla);
    println!("El valor de la tupla es: ({}, {}, {})", tupla.0, tupla.1, tupla.2);
}
