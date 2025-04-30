// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }

#![no_std]

use soroban_sdk::{contractimpl, symbol, Env, Symbol, Map};

#[derive(Clone)]
pub struct Usuario {
    pub nombre: Symbol,
    pub apellido_materno: Symbol,
    pub apellido_paterno: Symbol,
    pub usuario: Symbol,
    pub id: u32,
}

pub struct ContratoUsuarios;

#[contractimpl]
impl ContratoUsuarios {
    pub fn inicializar(env: Env) {
        let usuarios: Map<Symbol, Usuario> = Map::new(&env);
        env.storage().instance().set(&symbol!("usuarios"), &usuarios);
    }
// funcion de agregar usuario
    pub fn agregar_usuario(
        env: Env,
        nombre: Symbol,
        apellido_materno: Symbol,
        apellido_paterno: Symbol,
        usuario: Symbol,
        id: u32,
    ) {
        let mut usuarios: Map<Symbol, Usuario> = env
            .storage()
            .instance()
            .get(&symbol!("usuarios"))
            .unwrap_or(Map::new(&env));

        if usuarios.contains_key(usuario.clone()) {
            panic!("El usuario ya existe");
        }

        let nuevo_usuario = Usuario {
            nombre,
            apellido_materno,
            apellido_paterno,
            usuario: usuario.clone(),
            id,
        };

        usuarios.set(usuario.clone(), nuevo_usuario);
        env.storage().instance().set(&symbol!("usuarios"), &usuarios);
    }
// función de obtener usuario
    pub fn obtener_usuario(env: Env, usuario: Symbol) -> Option<Usuario> {
        let usuarios: Map<Symbol, Usuario> = env
            .storage()
            .instance()
            .get(&symbol!("usuarios"))
            .unwrap_or(Map::new(&env));

        usuarios.get(usuario)
    }
}
pub fn obtener_nombre_completo(env: Env, usuario: Symbol) -> Option<Symbol> {
    let usuarios: Map<Symbol, Usuario> = env
        .storage()
        .instance()
        .get(&symbol!("usuarios"))
        .unwrap_or(Map::new(&env));

    if let Some(u) = usuarios.get(usuario) {
        // Unimos nombre + apellidos con espacios
        let nombre_completo = Symbol::from_str(
            &env,
            &format!(
                "{} {} {}",
                u.nombre.to_string(),
                u.apellido_paterno.to_string(),
                u.apellido_materno.to_string()
            ),
        );
        Some(nombre_completo)
    } else {
        None
    }
}
