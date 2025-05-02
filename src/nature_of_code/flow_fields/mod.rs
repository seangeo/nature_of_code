use crate::nature_of_code::chapter::Chapter;
use crate::nature_of_code::exercise::ExerciseInfo;

mod ex_0_1;

pub const EXERCISES: [ExerciseInfo; 1] = [
    ExerciseInfo {
        name: "Basic Flow Field",
        init_fn: ex_0_1::init,
    },
];

pub fn chapter() -> Chapter {
    Chapter {
        name: "Flow Fields",
        exercises: &EXERCISES,
    }
}
