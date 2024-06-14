use std::io;

fn main() {
    println!("Hello, this program will calculate your bmi based on given height and weight!");
    println!("=== Height measurements ===");
    println!("Please specify if you are centimeters (cm) or inches (in)");
    let mut given_height_type: String = String::new();
    io::stdin().read_line(&mut given_height_type).expect("Value was unexpected");

    println!("Please specify your measurement in that type");
    let mut given_height_value: String = String::new();
    io::stdin().read_line(&mut given_height_value).expect("Value was unexpected");

    
    let converted_height_cm: f32 = height_conversion(given_height_type, string_to_f32(given_height_value));

    println!("");
    println!("=== Weight measurements ===");
    println!("Please specify if you are measuring in kilograms (kg) or in pounds (ib)");
    let mut given_weight_type: String = String::new();
    io::stdin().read_line(&mut given_weight_type).expect("Value was unexpected");

    println!("Please specify your measurement in that type");
    let mut given_weight_value: String = String::new();
    io::stdin().read_line(&mut given_weight_value).expect("Value was unexpected");
    
    let converted_weight_kg:f32 = weight_conversion(given_weight_type, string_to_f32(given_weight_value));

    println!("");
    println!("=== BMI Calculation ===");
    let bmi: f32 = bmi_calculation(converted_height_cm, converted_weight_kg);
    println!("Your bmi is: {}", bmi);
    bmi_categories(bmi);
}


//Convert from string to f32
fn string_to_f32 (string_value: String) -> f32{
    string_value.trim().parse().unwrap()
}

//Converts weight to kg
fn weight_conversion (measurement_type: String, measurement: f32) -> f32{
    let measurement_type_isolated: &str = measurement_type.trim();
    match measurement_type_isolated{
        "ib" => {
            return measurement * 0.4535; 
        },
        "kg" => {
            return measurement;
        }
        _ => {
            println!("Invalid selection");
            return 0.0;
        }
    }
}

//Converts measurement in cm
fn height_conversion (measurement_type: String, measurement: f32) -> f32{
    let measurement_type_isolated: &str = measurement_type.trim();
    match measurement_type_isolated{
        "in" => {
            return measurement * 2.54; 
        },
        "cm" => {
            return measurement;
        }
        _ => {
            println!("Invalid selection");
            return 0.0;
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
