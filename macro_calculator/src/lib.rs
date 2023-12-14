use json::JsonValue;

pub struct Food {
    pub name: String,
    pub calories: [String; 2],
    pub fats: f64,
    pub carbs: f64,
    pub proteins: f64,
    pub nbr_of_portions: f64,
}

pub fn calculate_macros(foods: Vec<Food>) -> JsonValue {
    let mut total_calories = 0.0;
    let mut total_fats = 0.0;
    let mut total_carbs = 0.0;
    let mut total_proteins = 0.0;

    for food in &foods {
        let calories_kcal = food.calories[1]
            .replace("kcal", "")
            .trim()
            .parse::<f64>()
            .unwrap();
        let portion_factor = food.nbr_of_portions;

        total_calories += calories_kcal * portion_factor;
        total_fats += food.fats * portion_factor;
        total_carbs += food.carbs * portion_factor;
        total_proteins += food.proteins * portion_factor;
    }

    // Round the calculated values to two decimal places
    let total_calories_rounded = (total_calories * 100.0).round() / 100.0;
    let total_fats_rounded = (total_fats * 100.0).round() / 100.0;
    let total_carbs_rounded = (total_carbs * 100.0).round() / 100.0;
    let total_proteins_rounded = (total_proteins * 100.0).round() / 100.0;

    // Create a JSON object with the calculated values
    let result = json::object! {
        "cals" => total_calories_rounded,
        "carbs" => total_carbs_rounded,
        "proteins" => total_proteins_rounded,
        "fats" => total_fats_rounded,
    };

    result
}
