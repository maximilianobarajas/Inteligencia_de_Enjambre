use rand::Rng;
//Definimos la funcion objetivo
use std::f64::consts::PI;

fn ackley(xx: &Vec<f64>) -> f64 {
    let a = 20.0;
    let b = 0.2;
    let c = 2.0 * PI;
    let d = xx.len() as f64;

    let sum1: f64 = xx.iter().map(|&x| x.powi(2)).sum();
    let sum2: f64 = xx.iter().map(|&x| (c * x).cos()).sum();

    let term1 = -a * f64::exp(-b * (sum1 / d).sqrt());
    let term2 = -f64::exp(sum2 / d);

    let y = term1 + term2 + a + f64::exp(1.0);
    y
}
struct ParticulaIndividual {
    posicion_actual: Vec<f64>,
    velocidad_actual: Vec<f64>,
    mejor_posicion_conocida: Vec<f64>, // Inicializar con la posición actual
    valor_optimo: f64,
}
impl ParticulaIndividual {
    //Constructor del la particula
    fn new(dim: usize) -> ParticulaIndividual {
        let initial_position: Vec<f64> =
            (0..dim).map(|_| rand::thread_rng().gen_range(-500.0..500.0)).collect();
        ParticulaIndividual {
            posicion_actual: initial_position.clone(),
            velocidad_actual: (0..dim).map(|_| rand::thread_rng().gen_range(-1.0..1.0)).collect(),
            mejor_posicion_conocida: initial_position,
            valor_optimo: 0.0,
        }
    }
}
//PSO
fn pso(
    funcion_objetivo: fn(&Vec<f64>) -> f64,
    dim: usize,
    num_particulas: usize,
    num_iteraciones: usize,
    w: f64,
    c1: f64,
    c2: f64,
) -> (Vec<f64>, f64, Vec<f64>) {
    // Inicializar la parvada
    let mut parvada: Vec<ParticulaIndividual> = (0..num_particulas)
        .map(|_| ParticulaIndividual::new(dim))
        .collect();
    // Mejor posición global
    let mut mejor_posicion_global = vec![0.0; dim];
    let mut mejor_fitness_global = f64::INFINITY;
    // Lista para almacenar el historial de la funcion objetivo
    let mut historial_fitness = Vec::new();
    //Iteramos sobre el numero de iteraciones establecido'
    for _ in 0..num_iteraciones {
        for particula in &mut parvada {
            // Evaluación de la funcion objetivo
            particula.valor_optimo = funcion_objetivo(&particula.posicion_actual);
            // Actualización de la mejor posición personal
            if particula.valor_optimo < funcion_objetivo(&particula.mejor_posicion_conocida) {
                particula.mejor_posicion_conocida = particula.posicion_actual.clone();
            }
            // Actualización de la mejor posición global
            if particula.valor_optimo < mejor_fitness_global {
                mejor_posicion_global = particula.posicion_actual.clone();
                mejor_fitness_global = particula.valor_optimo;
            }
        }
        // Actualización de la posición y velocidad de las partículas
        for particula in &mut parvada {
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

        // Almacenamiento del valor_optimo global en el historial
        historial_fitness.push(mejor_fitness_global);
    }

    (mejor_posicion_global, mejor_fitness_global, historial_fitness)
}

fn main() {
    // Ejemplo de uso con la función de prueba "paraboloide"
    let dim = 2;
    let num_particulas = 900;
    let num_iteraciones = 10000;
    let (mejor_posicion, mejor_fitness, historial_fitness) =
        pso(ackley, dim, num_particulas, num_iteraciones, 0.5, 1.5, 1.5);

    // Imprimir resultados
    println!("Mejor posición: {:?}", mejor_posicion);
    println!("Mejor valor_optimo: {}", mejor_fitness);

    // Graficar el valor_optimo por iteración
    println!("Historial de valor_optimo: {:?}", historial_fitness);
}

