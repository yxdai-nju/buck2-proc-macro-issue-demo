use thiserror::Error;

// Define custom error types using thiserror's derive macro
#[derive(Error, Debug)]
enum AppError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Value out of range: {value}, acceptable range is {min}..{max}")]
    OutOfRange {
        value: i32,
        min: i32,
        max: i32,
    }
}

// A function that returns our custom error type
fn process_value(value: i32) -> Result<i32, AppError> {
    if value < 0 {
        return Err(AppError::InvalidInput(format!("Negative value: {}", value)));
    }

    if value < 10 || value > 100 {
        return Err(AppError::OutOfRange {
            value,
            min: 10,
            max: 100,
        });
    }

    // Just a dummy calculation
    Ok(value * 2)
}

fn main() {
    // Test the error handling
    let test_values = [5, 50, -3];

    for &value in &test_values {
        match process_value(value) {
            Ok(result) => println!("Successfully processed {}: result = {}", value, result),
            Err(err) => println!("Error: {}", err),
        }
    }
}
