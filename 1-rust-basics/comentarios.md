# Comentarios en Rust

Los comentarios te ayudan a documentar tu código y dejar anotaciones para vos o para otras personas.

---

## 🔹 Comentarios de una línea

Empiezan con `//` y terminan al final de la línea.

```rust
// Este es un comentario simple
let x = 5;


🔹 Comentarios multilínea
Empiezan con /* y terminan con */.

/*
Esto es un comentario
de varias líneas.
*/

🔹 Comentarios de documentación
Se escriben con triple barra /// y se colocan justo antes de funciones, structs o módulos. Se usan para generar documentación automática con cargo doc.

/// Esta función suma dos números
fn sumar(a: i32, b: i32) -> i32 {
    a + b
}

📌 Los comentarios de documentación pueden incluir markdown para mejorar su visualización.

✅ Tareas sugeridas
Agregar comentarios en tus funciones personales.

Usar /// para documentar una función pública.

Crear un bloque TODO con algo a mejorar en el código.

```

