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
        food["cals"] = (
            (f.calories.1.clone().parse::<f64>().unwrap() * 100.0).round() / 1000.0
        ).into();
        food["carbs"] = ((f.carbs.clone() * 100.0).round() / 1000.0).into();
        food["proteins"] = ((f.proteins.clone() * 100.0).round() / 1000.0).into();
        food["fats"] = ((f.fats.clone() * 100.0).round() / 1000.0).into();
    }

    food
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
