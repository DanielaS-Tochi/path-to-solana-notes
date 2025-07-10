# Tipos Compuestos en Rust

En Rust, los **tipos compuestos** permiten agrupar múltiples valores en una sola estructura. Los principales son:

---

## 🟣 Tuplas

Las tuplas pueden contener elementos de diferentes tipos.  
Se accede mediante desestructuración o con índices: `persona.0`, `persona.1`.  
📌 Las tuplas tienen tamaño fijo.

```rust
let persona = ("Daniela", 34);
let (nombre, edad) = persona;
println!("Nombre: {}, Edad: {}", nombre, edad);

// Output: Nombre: Daniela, Edad: 34

