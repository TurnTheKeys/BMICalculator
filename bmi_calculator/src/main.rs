use std::io;

fn main() {
    println!("Hello, this program will calculate your bmi based on given height and weight!");
    println!("=== Height measurements ===");
    println!("Please specify if you are centimeters (cm) or inches (in)");

    let height = get_measurement("height", &["cm", "in"]);
    let height_cm: f32 = convert_height_to_cm (height.0, height.1);

    let weight = get_measurement("weight", &["kg", "ib"]);
    let weight_kg: f32 = convert_weight_to_kg (weight.0, weight.1);

    println!("");
    println!("=== BMI Calculation ===");
    let bmi: f32 = bmi_calculation(height_cm, weight_kg);
    println!("Your bmi is: {}", bmi);
    bmi_categories(bmi);
}

fn get_measurement( measurement_type: &str , valid_units: &[&str]) -> (String, f32) {
    println!("");
    println!("==== {} measurement ====", measurement_type);

    let mut input_unit: String = String:: new();
    loop{
        println!("Please specify your {} in either {} or {}", measurement_type, valid_units[0], valid_units[1]);
        io::stdin().read_line(&mut input_unit).expect("Failed to read line");
        input_unit = input_unit.trim().to_string();
        if valid_units.contains(&&input_unit.as_str()){
            println!("Your chosen unit is: {}", input_unit);
            break;
        }
        else{
            println!("You specified {}, which was not an option. Try again.", input_unit)
        }
    }

    let mut input_value = String::new();
    println!("");
    println!("Please provide your {} measurement in {}", measurement_type, input_unit);
    io::stdin().read_line(&mut input_value).expect("Value could not be read");
    let input_value: f32 = match input_value.trim().parse(){
        Ok(v)=> v,
        Err(_) => {
            println!("This number cannot be used! You are now using 100.00");
            100.00
        }
    };
    (input_unit,input_value)
}

//Converts measurement in cm
fn convert_height_to_cm(unit: String, height: f32) -> f32{
    match unit.as_str(){
        "cm" =>{
            return height;
        }
        "in" =>{
            return height * 2.54;
        }
        _ =>{
            return 0.00;
        }
    }
}

//Converts measurement in cm
fn convert_weight_to_kg(unit: String, weight: f32) -> f32{
    match unit.as_str(){
        "kg" =>{
            return weight;
        }
        "ib" =>{
            return weight * 2.54;
        }
        _ =>{
            return 0.00;
        }
    }
}

// Calculates BMI value
fn bmi_calculation (height_cm: f32, weight_kg: f32) -> f32{
    // kg/m^2
    return weight_kg/((height_cm/100.0).powi(2));
}

//Prints out bmi category based on given bmi value
fn bmi_categories (bmi: f32){
    print!("This is of bmi category: ");
    if bmi < 18.5 {
        println!("underweight");
    } else if bmi >= 18.5 && bmi < 24.9 {
        println!("normal weight");
    } else if bmi >= 24.9 && bmi < 29.9 {
        println!("overweight");
    } else {
        println!("obese");
    }
}
