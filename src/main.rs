use axum::{routing::get, Router};
use std::{collections::HashSet, env};
use axum::extract::OriginalUri;
use tokio::{net::TcpListener, sync::broadcast, task};

#[tokio::main]
async fn main() {
    // Capturar y filtrar variables de entorno para evitar valores vacíos
    let ports_env = collect_env_vars(&["PORT", "PORTS"]);
    let paths_env = collect_env_vars(&["HEALTHCHECK", "HEALTHCHECK_PATH"]);

    // Formar conjuntos únicos de puertos y paths
    let ports: HashSet<u16> = ports_env
        .into_iter()
        .filter_map(|p| p.parse::<u16>().ok()) // Convertir a u16 si es posible
        .collect();

    let paths: HashSet<String> = paths_env
        .into_iter()
        .map(normalize_path) // Normalizar paths
        .filter(|p| !p.is_empty()) // Filtrar valores vacíos
        .collect();

    // Validar que al menos haya un puerto y un path
    if ports.is_empty() {
        eprintln!("No ports found in the environment variables.");
        return;
    }
    if paths.is_empty() {
        eprintln!("No paths found in the environment variables.");
        return;
    }

    println!(
        "Starting healthchecks on ports: {:?} and paths: {:?}",
        ports, paths
    );

    // Canal para señal de apagado
    let (shutdown_tx, _) = broadcast::channel::<()>(1);

    // Tarea para manejar Ctrl+C
    let shutdown_tx_clone = shutdown_tx.clone();
    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to listen for Ctrl+C");
        println!("Received Ctrl+C, shutting down");
        let _ = shutdown_tx_clone.send(());
    });

    // Vector para almacenar los manejadores de las tareas
    let mut handles = Vec::new();

    // Levantar un servidor por cada puerto y path
    for port in ports {
        let paths = paths.clone(); // Clonar paths para mover al task
        let mut shutdown_rx = shutdown_tx.subscribe(); // Obtener un suscriptor para el canal de apagado

        let handle = task::spawn(async move {
            // Crear un router con un healthcheck en cada path
            let mut router = Router::new();
            for path in &paths {
                router = router.route(&path, get(handle_request));
            }

            // Iniciar servidor
            let addr = format!("0.0.0.0:{}", port);
            let listener = TcpListener::bind(&addr).await.unwrap();
            println!("Listening on {}", addr);

            // Configurar el servidor con apagado elegante
            let server_future = axum::serve(listener, router).with_graceful_shutdown(async move {
                // Esperar la señal de apagado
                let _ = shutdown_rx.recv().await;
            });

            if let Err(err) = server_future.await {
                eprintln!("Error running server on {}: {}", addr, err);
            }
        });

        handles.push(handle);
    }

    // Esperar a que todas las tareas del servidor finalicen
    for handle in handles {
        let _ = handle.await;
    }

    println!("All servers have shut down gracefully");
}

// Función para manejar las solicitudes y registrar el path
async fn handle_request(OriginalUri(uri): OriginalUri) -> &'static str {
    println!("Received request for path: {}", uri.path());
    "OK"
}

// Función para capturar y filtrar variables de entorno como iterador
fn collect_env_vars(keys: &[&str]) -> Vec<String> {
    keys.into_iter()
        .filter_map(|key| env::var(key).ok()) // Capturar valores definidos
        .flat_map(|val| {
            val.split(',')
                .map(|s| s.trim().to_owned())
                .collect::<Vec<String>>()
                .into_iter()
        }) // Dividir por comas y limpiar espacios
        .filter(|s| !s.is_empty()) // Filtrar cadenas vacías
        .collect()
}

// Función para normalizar paths
fn normalize_path(path: String) -> String {
    if path == "/" {
        return path; // El path raíz no se modifica
    }
    path.trim_end_matches('/').to_string() // Eliminar la barra final si existe
}
