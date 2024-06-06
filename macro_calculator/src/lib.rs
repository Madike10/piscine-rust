extern crate json;

pub struct Food {
    //expected public fields
    pub name: String,
    pub calories: [String; 2],
    pub proteins: f64,
    pub fats: f64,
    pub carbs: f64,
    pub nbr_of_portions: f64,
}

pub fn calculate_macros(foods: Vec<Food>) -> json::JsonValue {
    let mut som_cal = 0.0;
    let mut som_carb = 0.0;
    let mut som_prot = 0.0;
    let mut som_fats = 0.0;
    foods.iter().for_each(|f| {
        som_cal += f.calories[1].trim_end_matches("kcal").parse::<f64>().unwrap() * f.nbr_of_portions;
        som_carb += f.carbs * f.nbr_of_portions;
        som_prot += f.proteins * f.nbr_of_portions;
        som_fats += f.fats * f.nbr_of_portions;
    });
    json::object! {
        cals: (som_cal * 100.0).round() / 100.0,
        carbs: (som_carb * 100.0).round() / 100.0,
        proteins: (som_prot * 100.0).round() / 100.0,
        fats: (som_fats * 100.0).round() / 100.0,
    }
}


