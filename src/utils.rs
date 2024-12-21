use rand::{thread_rng, Rng};

pub fn generate_x_position() -> f32 {
    let mut rng = thread_rng();
    let random_number = rng.gen_range(-250..=250);
    random_number as f32
}
