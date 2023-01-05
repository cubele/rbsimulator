use bevy::prelude::warn;
use rand::distributions::Distribution;

pub fn range_rng<T: rand::distributions::uniform::SampleUniform>(min: T, max: T) -> T {
    let num = rand::distributions::Uniform::<T>::new_inclusive(min, max)
        .sample(&mut rand::thread_rng());
    num
}