import random
import numpy as np
# Definimos la función objetivo
def ackley(xx, a=20, b=0.2, c=2 * np.pi):
    d = len(xx)
  
    sum1 = np.sum(np.array(xx)**2)
    sum2 = np.sum(np.cos(c * np.array(xx)))

    term1 = -a * np.exp(-b * np.sqrt(sum1 / d))
    term2 = -np.exp(sum2 / d)

    y = term1 + term2 + a + np.exp(1)
    return y
# Definimos la estructura de la partícula
class ParticulaIndividual:
    def __init__(self, dim):
        self.posicion_actual = [random.uniform(-500.0, 500.0) for _ in range(dim)]
        self.velocidad_actual = [random.uniform(-1.0, 1.0) for _ in range(dim)]
        self.mejor_posicion_conocida = self.posicion_actual.copy()
        self.valor_optimo = 0.0

# PSO
def pso(funcion_objetivo, dim, num_particulas, num_iteraciones, w, c1, c2):
    # Inicializar la parvada
    parvada = [ParticulaIndividual(dim) for _ in range(num_particulas)]

    # Mejor posición global
    mejor_posicion_global = [0.0] * dim
    mejor_fitness_global = float('inf')

    # Lista para almacenar el historial de la función objetivo
    historial_fitness = []

    # Iteramos sobre el número de iteraciones establecido
    for _ in range(num_iteraciones):
        for particula in parvada:
            # Evaluación de la función objetivo
            particula.valor_optimo = funcion_objetivo(particula.posicion_actual)

            # Actualización de la mejor posición personal
            if particula.valor_optimo < funcion_objetivo(particula.mejor_posicion_conocida):
                particula.mejor_posicion_conocida = particula.posicion_actual.copy()

            # Actualización de la mejor posición global
            if particula.valor_optimo < mejor_fitness_global:
                mejor_posicion_global = particula.posicion_actual.copy()
                mejor_fitness_global = particula.valor_optimo

        # Actualización de la posición y velocidad de las partículas
        for particula in parvada:
            r1 = random.random()
            r2 = random.random()
            for i in range(dim):
                particula.velocidad_actual[i] = (
                    w * particula.velocidad_actual[i] +
                    c1 * r1 * (particula.mejor_posicion_conocida[i] - particula.posicion_actual[i]) +
                    c2 * r2 * (mejor_posicion_global[i] - particula.posicion_actual[i])
                )
                particula.posicion_actual[i] += particula.velocidad_actual[i]

        # Almacenamiento del valor óptimo global en el historial
        historial_fitness.append(mejor_fitness_global)

    return mejor_posicion_global, mejor_fitness_global, historial_fitness

def main():
    # Ejemplo de uso con la función de prueba "paraboloide"
    dim = 2
    num_particulas = 900
    num_iteraciones = 10000
    mejor_posicion, mejor_fitness, historial_fitness = pso(ackley, dim, num_particulas, num_iteraciones, 0.5, 1.5, 1.5)

    # Imprimir resultados
    print("Mejor posición:", mejor_posicion)
    print("Mejor valor óptimo:", mejor_fitness)

    # Graficar el valor óptimo por iteración
    print("Historial de valor óptimo:", historial_fitness)

if __name__ == "__main__":
    main()

