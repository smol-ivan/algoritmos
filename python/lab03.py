import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns
from  sklearn.linear_model import LinearRegression
from numpy import log
import numpy as np
import os
import re

# Estilo
sns.set(style="whitegrid")

# Path to dir
directory = "../resultados/"

promedios = []

for archivo in os.listdir(directory):
    if archivo.endswith(".csv"):
        match = re.search(r"n_(\d+)", archivo)
        if match:
            n= int(match.group(1))
            path = os.path.join(directory, archivo)
            df = pd.read_csv(path)

            df = pd.read_csv(path)
            df_melted = df.melt(id_vars="ejec", var_name="algoritmo", value_name="tiempo")

            plt.figure(figsize=(10, 6))
            sns.scatterplot(data=df_melted, x="ejec", y="tiempo", hue="algoritmo")
            plt.title(f"Dispersión de tiempos para n={n}")
            plt.ylabel("Tiempo (s)")
            plt.xlabel("Repetición")
            
            plt.tight_layout()
            plt.savefig(f"./graficas/n_{n}.png")
            plt.close()

            # Boxplot opcional
            plt.figure(figsize=(8, 5))
            sns.boxplot(data=df_melted, x="algoritmo", y="tiempo")
            plt.title(f"Distribución de tiempos para n={n}")
            plt.ylabel("Tiempo (s)")
            plt.tight_layout()
            plt.savefig(f"./boxplot/n_{n}.png")
            plt.close()

            # Calcular promedios y guardarlos
            promedios.append({
                "n": n,
                "is_t": df["is_t"].mean(),
                "ss_t": df["ss_t"].mean(),
                "bs_t": df["bs_t"].mean()
            })
            # Convertir lista de promedios a DataFrame
df_promedios = pd.DataFrame(promedios).sort_values("n")

# Graficar promedios comparativos
plt.figure(figsize=(10, 6))

for alg in ['is_t', 'ss_t', 'bs_t']:
    x = df_promedios["n"].values
    y = df_promedios[alg].values

    # Solo graficar los puntos originales
    plt.plot(x, y, 'o-', label=f"{alg}")

plt.xlabel("n")
plt.ylabel("tiempo")
plt.title("Tiempo promedio de algoritmos de ordenamiento")
plt.legend()
plt.tight_layout()
plt.savefig("ajuste_polinomico.png")
plt.show()
