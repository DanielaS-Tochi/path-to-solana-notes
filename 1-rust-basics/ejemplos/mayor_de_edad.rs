fn main() {
    println!("{}", es_mayor_de_edad(20));
}

fn es_mayor_de_edad(edad: u8) -> &str {
    if edad >= 18 {
        "Mayor de edad"
    } else {
        "Menor de edad"
    }
}
// Ejemplo de función que determina si una persona es mayor de edad