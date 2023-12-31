use rand::Rng;
use std::fs::File;
use std::io::{self, Write};
use std::process::Command;
#[derive(Clone)]
//Creamos una estructura para las partículas
pub struct ParticulaIndividual {
    pub posicion_actual: Vec<f64>,
    pub velocidad_actual: Vec<f64>,
    pub mejor_posicion_conocida: Vec<f64>,
    pub valor_optimo: f64,
}
//Creamos el constructor de dicha estructura
impl ParticulaIndividual {
    pub fn new(dim: usize) -> ParticulaIndividual {
        let initial_position: Vec<f64> =
            (0..dim).map(|_| rand::thread_rng().gen_range(-10000.0..10000.0)).collect();
            //Inicializamos la posición de la partícula aleatoriamente en el bloque [-10000 x 10000] * * dim
            //Esta implementación funciona para problemas de n dimensiones
        ParticulaIndividual {
            posicion_actual: initial_position.clone(),
            velocidad_actual: (0..dim).map(|_| rand::thread_rng().gen_range(-1.0..1.0)).collect(),
            mejor_posicion_conocida: initial_position,
            valor_optimo: f64::INFINITY,
        }
    }
}
//Implementamos el método PSO
pub fn pso(
    funcion_objetivo: fn(&Vec<f64>) -> f64,
    dim: usize,
    num_particulas: usize,
    num_iteraciones: usize,
    w: f64,
    c1: f64,
    c2: f64,
) -> (Vec<f64>, f64, Vec<f64>) {
    //Creamos el enjambre como un vector de particulas
    let mut enjambre: Vec<ParticulaIndividual> = (0..num_particulas)
        .map(|_| ParticulaIndividual::new(dim))
        .collect();
    //Inicializamos la mejor posición global en el infinito al igual que el mejor fitness
    let mut mejor_posicion_global = vec![f64::INFINITY; dim];
    let mut mejor_fitness_global = f64::INFINITY;
    //Creamos una lista para almacenar el mejor fitness a lo largo de las iteraciones
    let mut historial_fitness = Vec::new();
    //Iteramos sobre el numero de iteraciones establecido'
    for _ in 0..num_iteraciones {
        for particula in &mut enjambre {
            //Para cada partícula evaluamos la función objetivo
            particula.valor_optimo = funcion_objetivo(&particula.posicion_actual);
            //Decidimos si actualizamos el mejor óptimo conocido de cada partícula
            if particula.valor_optimo < funcion_objetivo(&particula.mejor_posicion_conocida) {
                particula.mejor_posicion_conocida = particula.posicion_actual.clone();
            }
            //Almacenamos los datos en caso de que encontremos un mejor óptimo global
            if particula.valor_optimo < mejor_fitness_global {
                mejor_posicion_global = particula.posicion_actual.clone();
                mejor_fitness_global = particula.valor_optimo;
            }
        }
        // Actualización de la posición y velocidad de las partículas según lo visto en clase
        for particula in &mut enjambre {
            let r1: f64 = rand::thread_rng().gen();
            let r2: f64 = rand::thread_rng().gen();
            for i in 0..dim {
                particula.velocidad_actual[i] =
                    w * particula.velocidad_actual[i]
                        + c1 * r1 * (particula.mejor_posicion_conocida[i] - particula.posicion_actual[i])
                        + c2 * r2 * (mejor_posicion_global[i] - particula.posicion_actual[i]);
                particula.posicion_actual[i] += particula.velocidad_actual[i];
            }
        }
        // Almacenamiento los valores del fitness en el historial
        historial_fitness.push(mejor_fitness_global);
    }
    (mejor_posicion_global, mejor_fitness_global, historial_fitness)
}
//Decidimos crear una función para almacenar en un archivo el historial del fitness
pub fn save_optimization_history_to_file(
    historial_fitness: &Vec<f64>,
    filename: &str,
) -> Result<(), io::Error> {
    let mut file = File::create(filename)?;
    writeln!(file, "iteracion,valor")?;
    for (iteration, value) in historial_fitness.iter().enumerate() {
        writeln!(file, "{},{}", iteration, value)?;
    }
    println!("Optimization history saved to {}", filename);
    Ok(())
}
//De igual manera creamos una función para ejecutar scripts de python con el fin de realizar las gráficas
// además del análisis estadístico.
pub fn run_python_script(script_path: &str) {
    let output = Command::new("python")
        .arg(script_path)
        .output();
    match output {
        Ok(output) if output.status.success() => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            println!("Python script output:\n{}", stdout);
            if !stderr.is_empty() {
                eprintln!("Python script errors:\n{}", stderr);
            }
        }
        _ => {
            // Try python3
            let output_python3 = Command::new("python3")
                .arg(script_path)
                .output()
                .expect("Failed to execute python3 command");

            if output_python3.status.success() {
                let stdout = String::from_utf8_lossy(&output_python3.stdout);
                let stderr = String::from_utf8_lossy(&output_python3.stderr);

                println!("Python3 script output:\n{}", stdout);
                if !stderr.is_empty() {
                    eprintln!("Python3 script errors:\n{}", stderr);
                }
            } else {
                eprintln!("Failed to run Python and Python3 scripts");
            }
        }
    }
}
