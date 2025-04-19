use crate::nature_of_code::exercise::ExerciseInfo;
use crate::nature_of_code::chapter::Chapter;

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
        name: "0.1 Random Walk",
        init_fn: ex_0_1::init,
    },
    ExerciseInfo {
        name: "0.3 Mouse Following",
        init_fn: ex_0_3::init,
    },
    ExerciseInfo {
        name: "0.4 Gaussian Splatter",
        init_fn: ex_0_4::init,
    },
    ExerciseInfo {
        name: "0.5 Gaussian Distribution",
        init_fn: ex_0_5::init,
    },
    ExerciseInfo {
        name: "0.6 Quadratic Distribution",
        init_fn: ex_0_6::init,
    },
    ExerciseInfo {
        name: "0.7 Perlin Noise Motion",
        init_fn: ex_0_7::init,
    },
    ExerciseInfo {
        name: "0.8 Noise Image Generator",
        init_fn: ex_0_8::init,
    },
    ExerciseInfo {
        name: "0.10 3D Noise Terrain",
        init_fn: ex_0_10::init,
    },
];

pub fn chapter() -> Chapter {
    Chapter {
        name: "Chapter 0",
        exercises: &EXERCISES,
    }
} 