use crate::nature_of_code::exercise::ExerciseInfo;

pub mod ex_0_1;
pub mod ex_0_3;
pub mod ex_0_4;
pub mod ex_0_5;
pub mod ex_0_6;
pub mod ex_0_7;
pub mod ex_0_8;
pub mod ex_0_10;

pub const EXERCISES: [ExerciseInfo; 8] = [
    ExerciseInfo {
        name: "Exercise 0.1",
        init_fn: ex_0_1::init,
    },
    ExerciseInfo {
        name: "Exercise 0.3",
        init_fn: ex_0_3::init,
    },
    ExerciseInfo {
        name: "Exercise 0.4",
        init_fn: ex_0_4::init,
    },
    ExerciseInfo {
        name: "Exercise 0.5",
        init_fn: ex_0_5::init,
    },
    ExerciseInfo {
        name: "Exercise 0.6",
        init_fn: ex_0_6::init,
    },
    ExerciseInfo {
        name: "Exercise 0.7",
        init_fn: ex_0_7::init,
    },
    ExerciseInfo {
        name: "Exercise 0.8",
        init_fn: ex_0_8::init,
    },
    ExerciseInfo {
        name: "Exercise 0.10",
        init_fn: ex_0_10::init,
    },
]; 