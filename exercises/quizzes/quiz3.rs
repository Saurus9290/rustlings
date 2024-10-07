use std::fmt::Display;

// Make the ReportCard struct generic over the grade type `T`
struct ReportCard<T> {
    grade: T,
    student_name: String,
    student_age: u8,
}

// Implement print method for ReportCard where `T` implements the Display trait
impl<T: Display> ReportCard<T> {
    fn print(&self) -> String {
        format!(
            "{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, &self.grade,
        )
    }
}

fn main() {
    // Experimentation
    let numeric_report_card = ReportCard {
        grade: 3.7,
        student_name: "Alice".to_string(),
        student_age: 13,
    };
    println!("{}", numeric_report_card.print());

    let alphabetic_report_card = ReportCard {
        grade: "A",
        student_name: "Bob".to_string(),
        student_age: 14,
    };
    println!("{}", alphabetic_report_card.print());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: 2.1,
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1",
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        let report_card = ReportCard {
            grade: "A+",
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+",
        );
    }
}
