use bevy::{
    math::Vec3Swizzles,
    prelude::{Transform, Vec2},
};
use bevy_rapier2d::prelude::{ExternalForce, MassProperties};

use super::NUTONIAN_CONSTANT;

struct GravityInfo {
    transform: Transform,
    mass: MassProperties,
}

fn compute_gravity_force(
    gravity_source: &GravityInfo,
    gravity_target: &GravityInfo,
) -> Option<Box<ExternalForce>> {
    let absolute_force = compute_absolute_force(gravity_source, gravity_target)?;
    let direction = find_force_direction(gravity_source, gravity_target)?;
    let force_vector = direction * absolute_force;
    let center_of_mass = gravity_target.mass.local_center_of_mass;
    let result_force = ExternalForce::at_point(force_vector, center_of_mass, center_of_mass);
    return Some(Box::new(result_force));
}

fn multiply_masses(gravity_source: &GravityInfo, gravity_target: &GravityInfo) -> Option<f32> {
    let source_mass = extract_mass(&gravity_source.mass)?;
    let target_mass = extract_mass(&gravity_target.mass)?;
    let result = source_mass * target_mass;

    return Some(result);
}

fn extract_mass(source: &MassProperties) -> Option<f32> {
    return Some(source.mass);
}

fn compute_distance(gravity_source: &GravityInfo, gravity_target: &GravityInfo) -> Option<f32> {
    let distance = gravity_source
        .transform
        .translation
        .distance(gravity_target.transform.translation);

    return Some(distance);
}

fn compute_absolute_force(source: &GravityInfo, target: &GravityInfo) -> Option<f32> {
    let distance = compute_distance(&source, &target)?;
    let result = NUTONIAN_CONSTANT * multiply_masses(source, target)? / distance.powi(2);
    return Some(result);
}

fn find_force_direction(source: &GravityInfo, target: &GravityInfo) -> Option<Vec2> {
    let source_position = source.transform.translation;
    let target_position = target.transform.translation;
    let vector_distance = source_position - target_position;
    let result = vector_distance.normalize_or_zero();
    return Some(result.xy());
}
