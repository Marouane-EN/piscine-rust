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
