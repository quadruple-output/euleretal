use super::core::{
    integration_step::builders::{self, Collector},
    Integrator,
};

pub struct Broken {}

impl Broken {
    pub fn new() -> Self {
        Broken {}
    }
}

impl Integrator for Broken {
    fn label(&self) -> String {
        "Broken Euler".to_string()
    }

    fn description(&self) -> String {
        "v' = v + a dt\n\
         s' = s + v dt"
            .to_string()
    }

    fn integrate_step(
        &self,
        s: builders::Position,
        v: builders::Velocity,
        a: builders::Acceleration,
        dt: builders::DtFraction<1, 1>,
        step: &mut builders::Step,
    ) {
        step.compute(v + a * dt);
        step.compute(s + v * dt);
    }
}

#[cfg(test)]
mod test_broken_euler {
    use super::*;
    use crate::core::{Acceleration, Position, StartCondition, Step, Velocity};

    #[test]
    fn first_test() {
        let start = StartCondition::new(
            Position::origin(),
            Velocity::new(0., 0., 0.),
            Acceleration::new(1., 0., 0.),
        );
        let dt = 1.0.into();
        let integrator = Broken::new();
        let field = crate::scenarios::ConstantAcceleration;
        let mut step = Step::new(&start, dt);
        {
            let mut builder = crate::core::integration_step::builders::Step::new(&field, &mut step);
            let ((s, v, a), dt) = (builder.start_values(), builder.dt());
            integrator.integrate_step(s, v, a, dt, &mut builder);
        }

        let (s, v, a) = (start.position(), start.velocity(), start.acceleration());
        let v1 = v + a * dt;
        let s1 = s + v * dt;

        assert!(step.last_v() == v1);
        assert!(step.last_s() == s1);
    }
}

pub struct Euler {}

impl Euler {
    pub fn new() -> Self {
        Euler {}
    }
}

impl Integrator for Euler {
    fn label(&self) -> String {
        "Euler".to_string()
    }

    fn description(&self) -> String {
        "v' = v + a dt\n\
         s' = s + v' dt\n    \
            = s + v dt + a dt²"
            .to_string()
    }

    fn integrate_step(
        &self,
        s: builders::Position,
        v: builders::Velocity,
        a: builders::Acceleration,
        dt: builders::DtFraction<1, 1>,
        step: &mut builders::Step,
    ) {
        let v1 = step.compute(v + a * dt);
        step.compute(s + v1 * dt);
    }
}
