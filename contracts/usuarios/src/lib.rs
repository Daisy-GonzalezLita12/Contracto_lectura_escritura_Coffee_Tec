#![no_std]

use soroban_sdk::{contractimpl, contracttype, symbol, Env, Symbol, Map, Vec};

#[derive(Clone, Debug)]
#[contracttype]
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
    // Inicializar el almacenamiento
    pub fn inicializar(env: Env) {
        let usuarios: Map<Symbol, Usuario> = Map::new(&env);
        env.storage().instance().set(&symbol!("usuarios"), &usuarios);
    }

    // Agregar un nuevo usuario
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

    // Obtener datos de un usuario
    pub fn obtener_usuario(env: Env, usuario: Symbol) -> Option<Usuario> {
        let usuarios: Map<Symbol, Usuario> = env
            .storage()
            .instance()
            .get(&symbol!("usuarios"))
            .unwrap_or(Map::new(&env));

        usuarios.get(usuario)
    }

    // Obtener nombre completo
    pub fn obtener_nombre_completo(env: Env, usuario: Symbol) -> Option<Symbol> {
        let usuarios: Map<Symbol, Usuario> = env
            .storage()
            .instance()
            .get(&symbol!("usuarios"))
            .unwrap_or(Map::new(&env));

        if let Some(u) = usuarios.get(usuario) {
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

    // Actualizar datos del usuario
    pub fn actualizar_usuario(
        env: Env,
        usuario: Symbol,
        nuevo_nombre: Symbol,
        nuevo_apellido_materno: Symbol,
        nuevo_apellido_paterno: Symbol,
        nuevo_id: u32,
    ) {
        let mut usuarios: Map<Symbol, Usuario> = env
            .storage()
            .instance()
            .get(&symbol!("usuarios"))
            .unwrap_or(Map::new(&env));

        if !usuarios.contains_key(usuario.clone()) {
            panic!("El usuario no existe");
        }

        let usuario_actualizado = Usuario {
            nombre: nuevo_nombre,
            apellido_materno: nuevo_apellido_materno,
            apellido_paterno: nuevo_apellido_paterno,
            usuario: usuario.clone(),
            id: nuevo_id,
        };

        usuarios.set(usuario.clone(), usuario_actualizado);
        env.storage().instance().set(&symbol!("usuarios"), &usuarios);
    }

    // Eliminar usuario
    pub fn eliminar_usuario(env: Env, usuario: Symbol) {
        let mut usuarios: Map<Symbol, Usuario> = env
            .storage()
            .instance()
            .get(&symbol!("usuarios"))
            .unwrap_or(Map::new(&env));

        if !usuarios.contains_key(usuario.clone()) {
            panic!("El usuario no existe");
        }

        usuarios.remove(usuario.clone());
        env.storage().instance().set(&symbol!("usuarios"), &usuarios);
    }

    // Listar todos los usuarios
    pub fn listar_usuarios(env: Env) -> Vec<Usuario> {
        let usuarios: Map<Symbol, Usuario> = env
            .storage()
            .instance()
            .get(&symbol!("usuarios"))
            .unwrap_or(Map::new(&env));

        let mut lista_usuarios = Vec::new(&env);

        for key in usuarios.keys() {
            if let Some(usuario) = usuarios.get(key.clone()) {
                lista_usuarios.push_back(usuario);
            }
        }

        lista_usuarios
    }
}
