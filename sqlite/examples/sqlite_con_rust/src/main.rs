use chrono::{DateTime, Utc};
use rusqlite::{Connection, Result};

struct Persona {
    id: i32,
    nombre: String,
    data: Option<Vec<u8>>,
    fecha: Option<DateTime<Utc>>,
}

fn guardar_persona(persona: &Persona, conexion: &Connection) -> Result<()> {
    conexion
        .execute(
            "INSERT INTO PERSONA (nombre, data, fecha) VALUES (?1, ?2, ?3)",
            (&persona.nombre, &persona.data, Utc::now().to_rfc3339()),
        )
        .expect("Error haciendo el registro de una persona");

    Ok(())
}

fn main() -> Result<()> {
    let conexion = Connection::open("my-db.db3").expect("error conectando a sqlite");

    conexion
        .execute(
            "CREATE TABLE IF NOT EXISTS PERSONA(
            id INTEGER PRIMARY KEY,
            nombre TEXT NOT NULL,
            fecha TEXT NULL,
            data BLOB
        )",
            (),
        )
        .expect("Error Creando La Tabla PERSONA");

    guardar_persona(
        &Persona {
            id: 0, // sera colocado automaticamente
            nombre: "Maria".to_string(),
            data: Some(vec![3, 4, 1, 2]), // Este valor definimos que puede ser vacio (None)
            fecha: None,                  // Sera colocado automaticamente
        },
        &conexion,
    )
    .expect("error guardando la persona");

    guardar_persona(
        &Persona {
            id: 0, // sera colocado automaticamente
            nombre: "Juan".to_string(),
            data: Some(vec![10, 11]), // Este valor definimos que puede ser vacio (None)
            fecha: None,              // Sera colocado automaticamente
        },
        &conexion,
    )
    .expect("error guardando la persona");

    // Leyendo los datos
    let mut statement = conexion
        .prepare("SELECT id, nombre, data, fecha FROM PERSONA")
        .expect("No es posible crear el statement");

    let iterador_persona = statement
        .query_map([], |registro| {
            // la fecha del registro de sqlite es text, habra que pasarla a DateTime<Utc>
            // fecha es el cuarto campo del SELECT indice 3
            let fecha_del_registro: String = registro.get(3)?;
            Ok(Persona {
                id: registro.get(0)?,     // id es el primer campo del SELECT indice 0
                nombre: registro.get(1)?, // nombre es el segundo campo del SELECT indice 1
                data: registro.get(2)?,   // data es el tercer campo del SELECT indice 2
                fecha: Some(fecha_del_registro.parse::<DateTime<Utc>>().unwrap()),
            })
        })
        .expect("No fue posible obtener las personas");

    for item_persona in iterador_persona {
        let persona: Persona = item_persona.unwrap();
        println!("--------------------------------------------------------");
        println!("ID: {}", persona.id);
        println!("Nombre: {}", persona.nombre);
        // data puede ser vacio, para evitar un error le pondremos un valor por defecto
        // si viene como None
        println!("Data:{:?}", persona.data.unwrap_or(vec![]));
        println!(
            "Fecha del registro: {}",
            persona.fecha.unwrap().to_rfc3339()
        );
        println!("--------------------------------------------------------");
    }

    Ok(())
}
