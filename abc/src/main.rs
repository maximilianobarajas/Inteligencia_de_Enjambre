use std::f64::consts::PI;
use rand::Rng;

const NP: usize = 600;
const FOOD_NUMBER: usize = NP / 2;
const LIMIT: usize = 1000;
const MAX_CYCLE: usize = 3000;
const D: usize = 2;
const LB: f64 = -10000.12;
const UB: f64 = 10000.12;
const RUNTIME: usize = 30;

type Solution = Vec<f64>;

struct ABC {
    foods: Vec<Solution>,
    f: Vec<f64>,
    fitness: Vec<f64>,
    trial: Vec<usize>,
    prob: Vec<f64>,
    solution: Solution,
    obj_val_sol: f64,
    fitness_sol: f64,
    neighbour: usize,
    param2change: usize,
    global_min: f64,
    global_params: Solution,
    global_mins: Vec<f64>,
    r: f64,
}

impl ABC {
    fn new() -> ABC {
        ABC {
            foods: vec![vec![0.0; D]; FOOD_NUMBER],
            f: vec![0.0; FOOD_NUMBER],
            fitness: vec![0.0; FOOD_NUMBER],
            trial: vec![0; FOOD_NUMBER],
            prob: vec![0.0; FOOD_NUMBER],
            solution: vec![0.0; D],
            obj_val_sol: 0.0,
            fitness_sol: 0.0,
            neighbour: 0,
            param2change: 0,
            global_min: 0.0,
            global_params: vec![0.0; D],
            global_mins: vec![0.0; RUNTIME],
            r: 0.0,
        }
    }

    fn init(&mut self, index: usize) {
        for j in 0..D {
            self.r = rand::thread_rng().gen_range(0.0..1.0);
            self.foods[index][j] = self.r * (UB - LB) + LB;
            self.solution[j] = self.foods[index][j];
        }
        self.f[index] = function(&self.solution);
        self.fitness[index] = calculate_fitness(self.f[index]);
        self.trial[index] = 0;
    }

    fn initial(&mut self) {
        for i in 0..FOOD_NUMBER {
            self.init(i);
        }
        self.global_min = self.f[0];
        self.global_params = self.foods[0].clone();
    }

    fn send_employed_bees(&mut self) {
        for i in 0..FOOD_NUMBER {
            self.r = rand::thread_rng().gen_range(0.0..1.0);
            self.param2change = (self.r * D as f64) as usize;
            self.r = rand::thread_rng().gen_range(0.0..1.0);
            self.neighbour = (self.r * FOOD_NUMBER as f64) as usize;

            while self.neighbour == i {
                self.r = rand::thread_rng().gen_range(0.0..1.0);
                self.neighbour = (self.r * FOOD_NUMBER as f64) as usize;
            }

            for j in 0..D {
                self.solution[j] = self.foods[i][j];
            }

            self.r = rand::thread_rng().gen_range(0.0..1.0);
            self.solution[self.param2change] =
                self.foods[i][self.param2change]
                    + (self.foods[i][self.param2change] - self.foods[self.neighbour][self.param2change])
                        * (self.r - 0.5) * 2.0;

            if self.solution[self.param2change] < LB {
                self.solution[self.param2change] = LB;
            }
            if self.solution[self.param2change] > UB {
                self.solution[self.param2change] = UB;
            }

            self.obj_val_sol = function(&self.solution);
            self.fitness_sol = calculate_fitness(self.obj_val_sol);

            if self.fitness_sol > self.fitness[i] {
                self.trial[i] = 0;
                self.foods[i] = self.solution.clone();
                self.f[i] = self.obj_val_sol;
                self.fitness[i] = self.fitness_sol;
            } else {
                self.trial[i] += 1;
            }
        }
    }

    fn calculate_probabilities(&mut self) {
        let max_fit = self.fitness.iter().fold(f64::NEG_INFINITY, |max, &x| max.max(x));

        for i in 0..FOOD_NUMBER {
            self.prob[i] = 0.9 * (self.fitness[i] / max_fit) + 0.1;
        }
    }

    fn send_onlooker_bees(&mut self) {
        let mut i = 0;
        let mut t = 0;

        while t < FOOD_NUMBER {
            self.r = rand::thread_rng().gen_range(0.0..1.0);
            if self.r < self.prob[i] {
                t += 1;
                self.r = rand::thread_rng().gen_range(0.0..1.0);
                self.param2change = (self.r * D as f64) as usize;

                self.r = rand::thread_rng().gen_range(0.0..1.0);
                self.neighbour = (self.r * FOOD_NUMBER as f64) as usize;

                while self.neighbour == i {
                    self.r = rand::thread_rng().gen_range(0.0..1.0);
                    self.neighbour = (self.r * FOOD_NUMBER as f64) as usize;
                }

                for j in 0..D {
                    self.solution[j] = self.foods[i][j];
                }

                self.r = rand::thread_rng().gen_range(0.0..1.0);
                self.solution[self.param2change] =
                    self.foods[i][self.param2change]
                        + (self.foods[i][self.param2change] - self.foods[self.neighbour][self.param2change])
                            * (self.r - 0.5) * 2.0;

                if self.solution[self.param2change] < LB {
                    self.solution[self.param2change] = LB;
                }
                if self.solution[self.param2change] > UB {
                    self.solution[self.param2change] = UB;
                }

                self.obj_val_sol = function(&self.solution);
                self.fitness_sol = calculate_fitness(self.obj_val_sol);

                if self.fitness_sol > self.fitness[i] {
                    self.trial[i] = 0;
                    self.foods[i] = self.solution.clone();
                    self.f[i] = self.obj_val_sol;
                    self.fitness[i] = self.fitness_sol;
                } else {
                    self.trial[i] += 1;
                }
            }

            i += 1;
            if i == FOOD_NUMBER {
                i = 0;
            }
        }
    }

    fn send_scout_bees(&mut self) {
        let mut max_trial_index = 0;

        for i in 1..FOOD_NUMBER {
            if self.trial[i] > self.trial[max_trial_index] {
                max_trial_index = i;
            }
        }

        if self.trial[max_trial_index] >= LIMIT {
            self.init(max_trial_index);
        }
    }

    fn memorize_best_source(&mut self) {
        let mut i = 0;
        let mut j = 0;

        while j < FOOD_NUMBER {
            if self.f[i] < self.global_min {
                self.global_min = self.f[i];
                for k in 0..D {
                    self.global_params[k] = self.foods[i][k];
                }
            }
            i += 1;
            if i == FOOD_NUMBER {
                i = 0;
                j += 1;
            }
        }
    }

    fn main_algorithm(&mut self) {
        let mut iter = 0;
        let mut run = 0;
        let mut j = 0;
        let mut mean = 0.0;

        for _ in 0..RUNTIME {
            self.initial();
            self.memorize_best_source();

            while iter < MAX_CYCLE {
                self.send_employed_bees();
                self.calculate_probabilities();
                self.send_onlooker_bees();
                self.memorize_best_source();
                self.send_scout_bees();
                iter += 1;
            }

            for k in 0..D {
                println!("GlobalParam[{}]: {}", k + 1, self.global_params[k]);
            }

            println!("{}. run: {}", run + 1, self.global_min);
            self.global_mins[run] = self.global_min;
            mean += self.global_min;

            iter = 0;
            run += 1;
        }

        mean /= RUNTIME as f64;
        println!("Means of {} runs: {}", RUNTIME, mean);
    }
}

fn function(sol: &Solution) -> f64 {
    // Your objective function implementation here (e.g., Ackley)
    // Replace the example function with your desired function
    //sol.iter().map(|&x| x * x).sum()
    ackley(sol)
}

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

fn calculate_fitness(fun: f64) -> f64 {
    if fun >= 0.0 {
        1.0 / (fun + 1.0)
    } else {
        1.0 + f64::abs(fun)
    }
}

fn main() {
    let mut abc = ABC::new();
    abc.main_algorithm();
}
