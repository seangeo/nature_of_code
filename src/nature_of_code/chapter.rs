use crate::nature_of_code::exercise::ExerciseInfo;

/// Represents a chapter and its exercises
pub struct Chapter {
    pub name: &'static str,
    pub exercises: &'static [ExerciseInfo],
} 