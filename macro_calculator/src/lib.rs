use json::object;
#[derive(Clone)]
pub struct Food {
    pub name: String,
    pub calories: (String, String),
    pub fats: f64,
    pub carbs: f64,
    pub proteins: f64,
    pub nbr_of_portions: f64,
}

pub fn calculate_macros(foods: &[Food]) -> json::JsonValue {
    let mut food =
        object! {
        "cals":0.0,
        "carbs":0.0,
        "proteins":0.0,
        "fats": 0.0,
    };
    for f in foods {
        let c = f.calories.1.clone();
        let calcule_cals = food["cals"].as_f64().unwrap();
        let calcule_cards = food["carbs"].as_f64().unwrap();
        let calcule_proteins = food["proteins"].as_f64().unwrap();
        let calcule_fats = food["fats"].as_f64().unwrap();

        food["cals"] = (
            calcule_cals +
            ((c[..c.len() - 4].parse::<f64>().unwrap() * 100.0).round() / 100.0) *
                f.nbr_of_portions
        ).into();
        food["carbs"] = (
            calcule_cards +
            ((f.carbs.clone() * 100.0).round() / 100.0) * f.nbr_of_portions
        ).into();
        food["proteins"] = (
            calcule_proteins +
            ((f.proteins.clone() * 100.0).round() / 100.0) * f.nbr_of_portions
        ).into();
        food["fats"] = (
            calcule_fats +
            ((f.fats.clone() * 100.0).round() / 100.0) * f.nbr_of_portions
        ).into();
    }

    food
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
