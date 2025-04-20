use crate::nature_of_code::chapter::Chapter;
use crate::nature_of_code::exercise::ExerciseInfo;

mod ex_1_1;

pub const EXERCISES: [ExerciseInfo; 1] = [ExerciseInfo {
    name: "1.1 Vector Walker",
    init_fn: ex_1_1::init,
}];

pub fn chapter() -> Chapter {
    Chapter {
        name: "Chapter 1",
        exercises: &EXERCISES,
    }
}
