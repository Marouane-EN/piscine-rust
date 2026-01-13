use json::object;
#[derive(Clone)]
pub struct Food {
    name: String,
    calories: (String, String),
    fats: f64,
    carbs: f64,
    proteins: f64,
    nbr_of_portions: f64,
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
        food["cals"] = f.calories.1.clone().into();
        food["carbs"] = f.carbs.clone().into();
        food["proteins"] = f.proteins.clone().into();
        food["fats"] = f.fats.clone().into();
    }

    food
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {

    }
}
