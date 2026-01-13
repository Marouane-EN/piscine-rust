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
    println!("================");
    for f in foods {
        let c = f.calories.1.clone();
        let calcule_cals = food["cals"].as_f64().unwrap();
        let calcule_cards = food["carbs"].as_f64().unwrap();
        let calcule_proteins = food["proteins"].as_f64().unwrap();
        let calcule_fats = food["fats"].as_f64().unwrap();
        food["cals"] = (
            calcule_cals +
            c[..c.len() - 4].parse::<f64>().unwrap() * f.nbr_of_portions
        ).into();
        food["carbs"] = (calcule_cards + f.carbs.clone() * f.nbr_of_portions).into();
        food["proteins"] = (calcule_proteins + f.proteins.clone() * f.nbr_of_portions).into();
        food["fats"] = (calcule_fats + f.fats.clone() * f.nbr_of_portions).into();
    }
    food["cals"] = ((food["cals"].as_f64().unwrap() * 100.0).round() / 100.0).into();
    food["carbs"] = ((food["carbs"].as_f64().unwrap() * 100.0).round() / 100.0).into();
    food["proteins"] = ((food["proteins"].as_f64().unwrap() * 100.0).round() / 100.0).into();
    food["fats"] = ((food["fats"].as_f64().unwrap() * 100.0).round() / 100.0).into();
    food
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_macros() {
        let foods = [
            Food {
                name: "big mac".to_owned(),
                calories: ("2133.84kJ".to_owned(), "510kcal".to_owned()),
                proteins: 27.0,
                fats: 26.0,
                carbs: 41.0,
                nbr_of_portions: 2.0,
            },
            Food {
                name: "pizza margherita".to_owned(),
                calories: ("1500.59kJ".to_owned(), "358.65kcal".to_owned()),
                proteins: 13.89,
                fats: 11.21,
                carbs: 49.07,
                nbr_of_portions: 4.9,
            },
        ];

        let result = calculate_macros(&foods);

        // Check that all fields exist
        assert!(result["cals"].is_number());
        assert!(result["carbs"].is_number());
        assert!(result["proteins"].is_number());
        assert!(result["fats"].is_number());

        // Verify calculated values
        // big mac: 510 * 2 = 1020
        // pizza: 358.65 * 4.9 = 1757.385
        // total: 1020 + 1757.385 = 2777.385 (rounded to 2777.39)
        assert_eq!(result["cals"].as_f64().unwrap(), 2777.39);

        // carbs: (41 * 2) + (49.07 * 4.9) = 82 + 240.443 = 322.44
        assert_eq!(result["carbs"].as_f64().unwrap(), 322.44);

        // proteins: (27 * 2) + (13.89 * 4.9) = 54 + 68.061 = 122.06
        assert_eq!(result["proteins"].as_f64().unwrap(), 122.06);

        // fats: (26 * 2) + (11.21 * 4.9) = 52 + 54.929 = 106.93
        assert_eq!(result["fats"].as_f64().unwrap(), 106.93);
    }

    #[test]
    fn test_single_food() {
        let foods = [
            Food {
                name: "apple".to_owned(),
                calories: ("218.45kJ".to_owned(), "52kcal".to_owned()),
                proteins: 0.3,
                fats: 0.2,
                carbs: 14.0,
                nbr_of_portions: 1.0,
            },
        ];

        let result = calculate_macros(&foods);

        assert_eq!(result["cals"].as_f64().unwrap(), 52.0);
        assert_eq!(result["carbs"].as_f64().unwrap(), 14.0);
        assert_eq!(result["proteins"].as_f64().unwrap(), 0.3);
        assert_eq!(result["fats"].as_f64().unwrap(), 0.2);
    }

    #[test]
    fn test_empty_array() {
        let foods: [Food; 0] = [];
        let result = calculate_macros(&foods);

        assert_eq!(result["cals"].as_f64().unwrap(), 0.0);
        assert_eq!(result["carbs"].as_f64().unwrap(), 0.0);
        assert_eq!(result["proteins"].as_f64().unwrap(), 0.0);
        assert_eq!(result["fats"].as_f64().unwrap(), 0.0);
    }
}
