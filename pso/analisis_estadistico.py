import pandas as pd
import matplotlib.pyplot as plt
from statistics import stdev,mean
csv_file_path = 'historial_30.csv'
x_column = 'iteracion'
y_column = 'valor'
df = pd.read_csv(csv_file_path)
print("La media fue: ", mean(df['valor']))
print("La desviacion estandar fue: ", stdev(df['valor']))
plt.boxplot(df['valor'])
plt.show()
