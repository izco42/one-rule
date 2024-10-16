import pandas as pd
from sklearn.model_selection import train_test_split
from sklearn.metrics import accuracy_score  
from one_rule_model import OneRuleModel


def main():
    # Cargar el dataset
    data = pd.read_csv('db.csv') 

    # Dividir el dataset en entrenamiento y prueba
    train_data, test_data = train_test_split(data, test_size=0.3, random_state=35)

    # Guardar el test set en un archivo CSV
    test_data.to_csv('test_set.csv', index=False) 

    # Guardar el train set 
    train_data.to_csv('train_set.csv', index=False)

    #crear una instancia de OneRuleModel
    target_column = 4  
    model = OneRuleModel("train_set.csv", target_column)  

    # Hacer predicciones sobre el conjunto de prueba
    X_test = test_data[model.feature_column]  
    y_true = test_data.iloc[:, target_column]  
    y_pred = model.predict(X_test)

    # Calcular la precisión
    accuracy = accuracy_score(y_true, y_pred)
    print(f"La precisión del modelo es: {accuracy:.2f}")

if __name__ == "__main__":
    main()
