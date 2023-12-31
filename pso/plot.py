import pandas as pd
import matplotlib.pyplot as plt
def plot_csv(csv_file_path, x_column, y_column):
    df = pd.read_csv(csv_file_path)
    plt.plot(df[x_column], df[y_column])
    plt.title(f'CSV Plot: {y_column} vs {x_column}')
    plt.xlabel(x_column)
    plt.ylabel(y_column)
    plt.grid(True)
    plt.show()
csv_file_path = 'optimization_history.csv'
x_column = 'iteracion'
y_column = 'valor'
plot_csv(csv_file_path, x_column, y_column)
print("Se generó la gráfica apropiadamente")