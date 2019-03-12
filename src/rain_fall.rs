use std::collections::HashMap;

fn get_record(dr0: &str) ->HashMap<&str, Vec<f64>>{
    let mut record: HashMap<&str, Vec<f64>> = HashMap::new();
    for i in dr0.lines() {
        if let [key, value] = i.split(':').collect::<Vec<&str>>().as_slice() {
            let b: Vec<_> = value
                .split(',')
                .map(|j| {
                    j.split_whitespace().collect::<Vec<&str>>()[1]
                        .parse::<f64>()
                        .unwrap()
                })
                .collect();
            record.insert(key, b);
        }
    }
    record
}
fn mean(town: &str, strng: &str) -> f64 {
    if let Some(values) = get_record(strng).get(town){
        values.iter().sum::<f64>() / values.len() as f64
    }else{
        -1.0
    }
}

fn variance(town: &str, strng: &str) -> f64 {
    if let Some(values) = get_record(strng).get(town){
        values.iter().map(|value| (value - mean(town, strng)).powi(2)).sum::<f64>() / values.len() as f64
    }else{
        -1.0
    }
}
