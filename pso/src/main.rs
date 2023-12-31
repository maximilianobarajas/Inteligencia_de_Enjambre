use std::f64::consts::PI;
mod pso_lib;
use std::fs;
fn ackley(xx: &Vec<f64>) -> f64 {
    /* Función de prueba Ackley */
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
fn levy(xx: &Vec<f64>) -> f64 {
    /* Funcion de prueba Levy */
    let d = xx.len();
    let w: Vec<f64> = xx.iter().map(|&x| 1.0 + (x - 1.0) / 4.0).collect();

    let term1 = (PI * w[0]).sin().powi(2);
    let term3 = (w[d - 1] - 1.0).powi(2) * (1.0 + (PI * w[d - 1] * 2.0).sin().powi(2));

    let sum: f64 = w.iter().take(d - 1).enumerate().map(|(_i, &wi)| (wi - 1.0).powi(2) * (1.0 + 10.0 * (PI * (wi + 1.0)).sin().powi(2))).sum();

    let y = term1 + sum + term3;
    y
}
fn schaffer2(xx: &Vec<f64>) -> f64 {
    /* Funcion de prueba Schaffer N. 2 */
    let x1 = xx[0];
    let x2 = xx[1];

    let fact1 = (x1.powi(2) - x2.powi(2)).sin().powi(2) - 0.5;
    let fact2 = (1.0 + 0.001 * (x1.powi(2) + x2.powi(2))).powi(2);

    let y = 0.5 + fact1 / fact2;
    y
}

fn main() {
    // Ejemplo de uso con la función de prueba "paraboloide"
    let dim = 2;
    let num_particulas = 90000;
    let num_iteraciones = 100;
    let (mejor_posicion, mejor_fitness, historial_fitness) =
    pso_lib::pso(ackley, dim, num_particulas, num_iteraciones, 0.5, 1.5, 1.5);
    // Imprimir resultados
    println!("Mejor posición: {:?}", mejor_posicion);
    println!("Mejor valor_optimo: {}", mejor_fitness);
    pso_lib::save_optimization_history_to_file(&historial_fitness, "optimization_history.csv").unwrap();
    pso_lib::run_python_script("plot.py");
    match fs::remove_file("optimization_history.csv") {
        Ok(_) => println!("File deleted successfully."),
        Err(e) => eprintln!("Error deleting the file: {}", e),
    }
    let mut historial_optimos_30_iteraciones: Vec<f64> = Vec::new();
    let mut historial_valores_30_iteraciones: Vec<Vec<f64>> = Vec::new();
    for i in 0..30{
        let (mejor_posicion, mejor_fitness, _historial_fitness) =pso_lib::pso(ackley, dim, num_particulas, num_iteraciones, 0.5, 1.5, 1.5);
        println!("iteracion: {} , mejor fitness {} , mejor posicion {:?}",i,mejor_fitness,mejor_posicion);
        historial_optimos_30_iteraciones.push(mejor_fitness);
        historial_valores_30_iteraciones.push(mejor_posicion);
    }
    pso_lib::save_optimization_history_to_file(&historial_optimos_30_iteraciones, "historial_30.csv").unwrap();
    pso_lib::run_python_script("analisis_estadistico.py");
}


