# 🎮 Mi Videoteca en Solana

Un CRUD on-chain para guardar tu colección de videojuegos en la blockchain de Solana.
Construido con **Rust + Anchor** desde Solana Playground.

---

## ¿Qué puedes hacer?

| Instrucción | Descripción |
|---|---|
| `crear_videoteca` | Crea tu colección personal (PDA) en la blockchain |
| `agregar_juego` | Agrega un juego con nombre, género y año |
| `eliminar_juego` | Elimina un juego por nombre |
| `ver_juegos` | Muestra todos tus juegos en el log |
| `alternar_completado` | Marca un juego como completado o pendiente |

---

## ¿Cómo usarlo en Solana Playground?

### 1️⃣ Importa el proyecto
Ve a [https://beta.solpg.io/](https://beta.solpg.io/) y pega la URL de este repositorio.

### 2️⃣ Conéctate a Devnet
Haz clic en **"Not Connected"** → **Continue**. Se creará una wallet automáticamente.

### 3️⃣ Haz Build
Haz clic en el botón **Build** (ícono de martillo 🔨). Espera a que compile.

### 4️⃣ Haz Deploy
Haz clic en **Deploy**. Esto sube tu programa a la Devnet de Solana.

### 5️⃣ Corre los Tests
Haz clic en el ícono de pruebas (🧪) y corre los tests en orden:
1. `crear_videoteca`
2. `agregar_juego`
3. `alternar_completado`
4. `ver_juegos`
5. `eliminar_juego`

---

## Estructura del proyecto

```
mi-videoteca/
├── src/
│   └── lib.rs              ← Programa principal en Rust (on-chain)
├── tests/
│   └── anchor.test.ts      ← Tests en TypeScript
├── client/
│   └── client.ts           ← Cliente para verificar wallet y videoteca
└── README.md
```

---

## Datos de cada videojuego

```rust
pub struct Videojuego {
    pub nombre: String,     // Ej: "Elden Ring"
    pub genero: String,     // Ej: "RPG", "FPS", "Aventura"
    pub anio: u16,          // Ej: 2022
    pub completado: bool,   // true = completado, false = pendiente
}
```

---

## Seguridad

- Solo el **owner** (dueño) puede agregar, eliminar o modificar juegos.
- La videoteca es una **PDA** única por wallet — nadie más puede acceder a ella.
