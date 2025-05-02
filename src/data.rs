// This module defines the Student struct and loads student data
// from a CSV file, separating features from performance scores.

use csv::ReaderBuilder;
use std::error::Error;

/// Struct representing a student. It contains:
/// - `features`: a numeric vector of lifestyle and academic factors
/// - `score`: the final exam score
#[derive(Debug, Clone)]
pub struct Student {
    pub features: Vec<f64>,
    pub score: f64,
}

/// Loads student data from a CSV file and converts each row into a Student struct.

pub fn load_students_from_csv(path: &str) -> Result<Vec<Student>, Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new().has_headers(true).from_path(path)?;
    let mut students = Vec::new();

    for result in rdr.records() {
        let record = result?;
        let mut features = Vec::new();

        // Directly parse numeric fields
        let age = record.get(1).unwrap_or("0").parse::<f64>().unwrap_or(0.0);
        let study_hours = record.get(3).unwrap_or("0").parse::<f64>().unwrap_or(0.0);
        let social_media = record.get(4).unwrap_or("0").parse::<f64>().unwrap_or(0.0);
        let netflix = record.get(5).unwrap_or("0").parse::<f64>().unwrap_or(0.0);

        // Convert Yes/No to 1.0/0.0 for binary fields
        let part_time_job = match record.get(6).unwrap_or("").to_lowercase().as_str() {
            "yes" => 1.0,
            "no" => 0.0,
            _ => 0.0,
        };

        let attendance = record.get(7).unwrap_or("0").parse::<f64>().unwrap_or(0.0);
        let sleep_hours = record.get(8).unwrap_or("0").parse::<f64>().unwrap_or(0.0);

        // Map categorical diet quality to ordinal scale
        let diet_quality = match record.get(9).unwrap_or("").to_lowercase().as_str() {
            "poor" => 1.0,
            "fair" => 2.0,
            "good" => 3.0,
            _ => 0.0,
        };

        let exercise = record.get(10).unwrap_or("0").parse::<f64>().unwrap_or(0.0);

        // Encode education level into a numeric scale
        let parental_edu = match record.get(11).unwrap_or("").to_lowercase().as_str() {
            "none" => 0.0,
            "high school" => 1.0,
            "bachelor" => 2.0,
            "master" => 3.0,
            "phd" => 4.0,
            _ => 0.0,
        };

        // Map internet quality into ordinal values
        let internet_quality = match record.get(12).unwrap_or("").to_lowercase().as_str() {
            "poor" => 1.0,
            "average" => 2.0,
            "good" => 3.0,
            _ => 0.0,
        };

        let mental_health = record.get(13).unwrap_or("0").parse::<f64>().unwrap_or(0.0);

        // Encode extracurricular participation (Yes/No) to 1.0/0.0
        let extracurricular = match record.get(14).unwrap_or("").to_lowercase().as_str() {
            "yes" => 1.0,
            "no" => 0.0,
            _ => 0.0,
        };

        // Final exam score as the ground-truth output
        let score = record.get(15).unwrap_or("0").parse::<f64>().unwrap_or(0.0);

        // Push all features into the student's vector in order
        features.push(age);
        features.push(study_hours);
        features.push(social_media);
        features.push(netflix);
        features.push(part_time_job);
        features.push(attendance);
        features.push(sleep_hours);
        features.push(diet_quality);
        features.push(exercise);
        features.push(parental_edu);
        features.push(internet_quality);
        features.push(mental_health);
        features.push(extracurricular);

        // Store the fully parsed student
        students.push(Student { features, score });
    }
    Ok(students)
}
