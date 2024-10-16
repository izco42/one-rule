import subprocess
import json

class OneRuleModel:
    def __init__(self, dataset, target_column):
        self.dataset = dataset
        self.target_column = target_column
        self.rules = self.load_rules()
        self.feature_column = self.rules.pop('feature', None)  
        print("Reglas cargadas:", self.rules)
        print("feature escogida:", self.feature_column)

    def load_rules(self):
        result = subprocess.run(
            [f"./oners", self.dataset, str(self.target_column)] , 
            capture_output=True,
            text=True,
            check=True
        )
        rules_json = result.stdout.strip()
        return json.loads(rules_json)
    
    def predict(self, X):
        return [self.rules.get(x, "Desconocido") for x in X]

