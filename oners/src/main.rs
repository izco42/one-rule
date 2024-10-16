use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use serde_json;
use std::env;
use serde_json::json;

fn one_rule(filename: &str, target_column: usize) -> io::Result<String> {
    let data = load_matrix(filename)?;
    let rule_map = build_rule_map(data, target_column);
    let json_rules = serde_json::to_string(&rule_map)?;
    Ok(json_rules)
} 

fn load_matrix(filename: &str) -> io::Result<Vec<Vec<String>>> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    let mut data: Vec<Vec<String>> = Vec::new();

    // Leer cada línea del archivo y dividirla por comas
    for line in reader.lines() {
        let line = line?;
        let row: Vec<String> = line.split(',').map(String::from).collect();
        data.push(row);
    }

    Ok(data)
}
fn build_rule_map(data: Vec<Vec<String>>, target_column: usize) -> HashMap<String, serde_json::Value> {
    let mut rule_map = HashMap::new();
    let mut best_feature = String::new();
    let mut min_error = std::f64::MAX;

    // Iterar por cada columna (excepto la de la clase)
    for col in 0..data[0].len() {
        if col == target_column { continue; }

        // Generar la tabla de frecuencias
        let mut frequency_table: HashMap<String, HashMap<String, usize>> = HashMap::new();
        for row in &data[1..] {
            let attribute_value = &row[col];
            let class_value = &row[target_column];

            let entry = frequency_table
                .entry(attribute_value.clone())
                .or_insert_with(HashMap::new);
            *entry.entry(class_value.clone()).or_insert(0) += 1;
        }

        // Generar reglas basadas en la frecuencia mayoritaria
        let mut rule: HashMap<String, String> = HashMap::new();
        let mut error = 0;
        for (attribute_value, class_counts) in &frequency_table {
            // Encontrar la clase mayoritaria para cada valor del atributo
            let (predicted_class, _) = class_counts
                .iter()
                .max_by_key(|&(_, count)| count)
                .unwrap();

            rule.insert(attribute_value.clone(), predicted_class.clone());

            // Calcular el error (cuántas veces no coincide la clase)
            for row in &data[1..] {
                if &row[col] == attribute_value && &row[target_column] != predicted_class {
                    error += 1;
                }
            }
        }

        // Calcular el error total para este atributo
        let error_rate = error as f64 / (data.len() - 1) as f64;

        // Guardar la regla con el error más bajo
        if error_rate < min_error {
            min_error = error_rate;
            best_feature = data[0][col].clone(); // Nombre de la característica
            rule_map = rule.into_iter()
                .map(|(k, v)| (k, json!(v)))
                .collect();
        }
    }

    // Agregar la propiedad "feature"
    rule_map.insert("feature".to_string(), json!(best_feature));

    rule_map
}

fn main() {
    let args: Vec<String> = env::args().collect(); 
    if args.len() != 3 {
        eprintln!("Uso: {} <archivo_csv> <columna_target>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    let target_column: usize = args[2].parse().expect("El índice de columna debe ser un número");
    
    match one_rule(filename, target_column) {
        Ok(json_rules) => println!("{}", json_rules),
        Err(e) => eprintln!("Error: {}", e),
    }
}
