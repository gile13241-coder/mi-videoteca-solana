use anchor_lang::prelude::*;
// ID del Solana Program, se llena automáticamente al hacer el "build"
declare_id!("");

#[program] // Macro que convierte código Rust a Solana. ¡Aquí empieza tu código!
pub mod mi_videoteca {
    use super::*; // Importa todos los structs y enums definidos fuera del módulo

    //////////////////////////// Instrucción: Crear Videoteca /////////////////////////////////////
    /*
    Crea una PDA (Program Derived Address), una cuenta especial en Solana que no necesita
    llave privada para firmar transacciones. Aquí guardaremos la colección de videojuegos.

    La PDA se genera a partir de:
        * Wallet address del dueño
        * Program ID
        * String "videoteca" como semilla

    Parámetros:
        * nombre -> nombre de la videoteca -> String
    */
    pub fn crear_videoteca(context: Context<NuevaVideoteca>, nombre: String) -> Result<()> {
        let owner_id = context.accounts.owner.key(); // Wallet del que llama la instrucción
        msg!("Creando videoteca para owner: {}", owner_id);

        let juegos: Vec<Videojuego> = Vec::new(); // Vector vacío de juegos

        // Guardamos el struct Videoteca en la cuenta PDA
        context.accounts.videoteca.set_inner(Videoteca {
            owner: owner_id,
            nombre,
            juegos,
        });

        Ok(()) // Transacción exitosa
    }

    //////////////////////////// Instrucción: Agregar Videojuego /////////////////////////////////////
    /*
    Agrega un videojuego al vector dentro del struct Videoteca.
    Solo el owner puede agregar juegos a su propia videoteca.

    Parámetros:
        * nombre   -> nombre del juego       -> String
        * genero   -> género del juego       -> String (ej: RPG, FPS, Aventura)
        * anio     -> año de lanzamiento     -> u16
    */
    pub fn agregar_juego(
        context: Context<ModificarVideoteca>,
        nombre: String,
        genero: String,
        anio: u16,
    ) -> Result<()> {
        require!(
            context.accounts.videoteca.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let juego = Videojuego {
            nombre,
            genero,
            anio,
            completado: false, // Por defecto, el juego arranca como no completado
        };

        context.accounts.videoteca.juegos.push(juego);
        msg!("¡Juego agregado exitosamente a tu videoteca!");

        Ok(())
    }

    //////////////////////////// Instrucción: Eliminar Videojuego /////////////////////////////////////
    /*
    Elimina un videojuego buscándolo por nombre.
    Error si no existe o si el vector está vacío.

    Parámetros:
        * nombre -> nombre del juego a eliminar -> String
    */
    pub fn eliminar_juego(context: Context<ModificarVideoteca>, nombre: String) -> Result<()> {
        require!(
            context.accounts.videoteca.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let juegos = &mut context.accounts.videoteca.juegos;

        for i in 0..juegos.len() {
            if juegos[i].nombre == nombre {
                juegos.remove(i);
                msg!("Juego '{}' eliminado de la videoteca.", nombre);
                return Ok(());
            }
        }

        Err(Errores::JuegoNoExiste.into()) // No se encontró el juego
    }

    //////////////////////////// Instrucción: Ver Juegos /////////////////////////////////////
    /*
    Muestra en el log de la transacción todos los juegos de la videoteca.

    Parámetros:
        Ninguno
    */
    pub fn ver_juegos(context: Context<ModificarVideoteca>) -> Result<()> {
        require!(
            context.accounts.videoteca.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        msg!(
            "Tu videoteca contiene: {:#?}",
            context.accounts.videoteca.juegos
        );

        Ok(())
    }

    //////////////////////////// Instrucción: Alternar Completado /////////////////////////////////////
    /*
    Cambia el estado de "completado" de false a true o de true a false.
    Útil para marcar si ya terminaste un juego o no.

    Parámetros:
        * nombre -> nombre del juego -> String
    */
    pub fn alternar_completado(
        context: Context<ModificarVideoteca>,
        nombre: String,
    ) -> Result<()> {
        require!(
            context.accounts.videoteca.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let juegos = &mut context.accounts.videoteca.juegos;

        for i in 0..juegos.len() {
            if juegos[i].nombre == nombre {
                let nuevo_estado = !juegos[i].completado;
                juegos[i].completado = nuevo_estado;
                msg!(
                    "El juego '{}' ahora está marcado como completado: {}",
                    nombre,
                    nuevo_estado
                );
                return Ok(());
            }
        }

        Err(Errores::JuegoNoExiste.into())
    }
}

/*
Códigos de error personalizados.
Cada variante lleva un mensaje descriptivo que aparece en el log cuando ocurre el error.
*/
#[error_code]
pub enum Errores {
    #[msg("Error: no eres el propietario de esta videoteca.")]
    NoEresElOwner,
    #[msg("Error: el videojuego que buscas no existe en la videoteca.")]
    JuegoNoExiste,
}

//////////////////////////// Structs de Datos /////////////////////////////////////

#[account]         // Indica que este struct es una cuenta almacenada en la blockchain
#[derive(InitSpace)] // Calcula automáticamente el espacio necesario en la blockchain
pub struct Videoteca {
    pub owner: Pubkey, // Llave pública del dueño (32 bytes)

    #[max_len(60)]     // Máximo 60 caracteres para el nombre
    pub nombre: String,

    #[max_len(10)]     // Máximo 10 juegos en la colección
    pub juegos: Vec<Videojuego>,
}

/*
Struct secundario que representa un videojuego.
No es una cuenta en sí, se almacena DENTRO de Videoteca.

Atributos derivados:
    * AnchorSerialize   -> para guardar el struct en la cuenta
    * AnchorDeserialize -> para leer su contenido desde la cuenta
    * Clone             -> para copiar valores
    * InitSpace         -> calcula el espacio necesario
    * PartialEq         -> permite comparar con "=="
    * Debug             -> permite imprimirlo con "{:#?}" en logs
*/
#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Videojuego {
    #[max_len(60)]
    pub nombre: String,

    #[max_len(20)]     // Ej: "RPG", "FPS", "Aventura", "Plataformas"
    pub genero: String,

    pub anio: u16,         // Año de lanzamiento (ej: 2023)
    pub completado: bool,  // ¿Ya lo terminaste? true = sí, false = no
}

//////////////////////////// Contextos (Accounts) /////////////////////////////////////

/*
Contexto para CREAR la videoteca.
Define qué cuentas se necesitan al llamar "crear_videoteca".
*/
#[derive(Accounts)]
pub struct NuevaVideoteca<'info> {
    #[account(mut)] // Mutable porque pagará la transacción (cambia su balance)
    pub owner: Signer<'info>,

    #[account(
        init,                                              // Crea la cuenta nueva
        payer = owner,                                     // El owner paga la creación
        space = Videoteca::INIT_SPACE + 8,                 // Espacio reservado en blockchain
        seeds = [b"videoteca", owner.key().as_ref()],      // Semillas para generar la PDA
        bump                                               // Byte extra para encontrar la PDA válida
    )]
    pub videoteca: Account<'info, Videoteca>,

    pub system_program: Program<'info, System>, // Requerido para crear cuentas en Solana
}

/*
Contexto para MODIFICAR la videoteca (agregar, eliminar, ver, alternar).
Se reutiliza para todas las instrucciones que operan sobre juegos ya existentes.
*/
#[derive(Accounts)]
pub struct ModificarVideoteca<'info> {
    pub owner: Signer<'info>, // Firma la transacción, valida identidad

    #[account(mut)] // Mutable porque vamos a modificar el vector de juegos
    pub videoteca: Account<'info, Videoteca>,
}
