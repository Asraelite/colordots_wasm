use js;

pub type FloatSize = f32;

#[derive(Clone, Copy)]
pub struct Vec2 {
	pub x: FloatSize,
	pub y: FloatSize,
}

const GRAVITATIONAL_FORCE: FloatSize = 1.0;
const GRAVITY_MIN_RADIUS: FloatSize = 3.0;
const PARTICLE_COUNT: u32 = 350;

impl Vec2 {
	pub fn new(x: FloatSize, y: FloatSize) -> Self {
		assert!(!x.is_nan());
		assert!(!y.is_nan());
		Self { x, y }
	}

	pub fn from_polar(angle: FloatSize, magnitude: FloatSize) -> Self {
		Self::new(angle.cos() * magnitude, angle.sin() * magnitude)
	}

	pub fn as_tuple(self) -> (FloatSize, FloatSize) {
		(self.x, self.y)
	}

	pub fn distance_to(self, other: Self) -> FloatSize {
		(self - other.into()).magnitude()
	}

	pub fn angle_to(self, other: Self) -> FloatSize {
		let (diff_x, diff_y) = (self - other).as_tuple();
		diff_y.atan2(diff_x)
	}

	pub fn magnitude(&self) -> FloatSize {
		return (self.x.powi(2) + self.y.powi(2)).sqrt();
	}
}

impl std::ops::Neg for Vec2 {
	type Output = Self;

	fn neg(self) -> Self {
		Self {
			x: -self.x,
			y: -self.y,
		}
	}
}

impl std::ops::Mul<FloatSize> for Vec2 {
	type Output = Self;

	fn mul(self, rhs: FloatSize) -> Self {
		Self {
			x: self.x * rhs,
			y: self.y * rhs,
		}
	}
}

impl std::ops::AddAssign for Vec2 {
	fn add_assign(&mut self, other: Vec2) {
		self.x += other.x;
		self.y += other.y;
	}
}

impl std::ops::Sub for Vec2 {
	type Output = Self;

	fn sub(self, other: Vec2) -> Self {
		Self {
			x: self.x - other.x,
			y: self.y - other.y,
		}
	}
}

pub struct World {
	pub size: Vec2,
	pub particles: Vec<Particle>,
}

impl World {
	pub fn new() -> Self {
		Self {
			size: Vec2::new(0.0, 0.0),
			particles: Vec::new(),
		}
	}

	pub fn init(&mut self, width: FloatSize, height: FloatSize) {
		self.size.x = width;
		self.size.y = height;

		for _ in 0..PARTICLE_COUNT {
			self.create_particle();
		}
	}

	pub fn tick(&mut self) {
		let mut slice = self.particles.as_mut_slice();

		for _ in 0..(slice.len() - 1) {
			let (particle, others) = { slice }.split_at_mut(1);
			particle[0].interact(others);
			slice = others;
		}

		for particle in self.particles.iter_mut() {
			particle.update();
		}
	}

	fn create_particle(&mut self) {
		self.particles.push(Particle::new_random(&self.size));
	}
}

pub struct Particle {
	pub position: Vec2,
	velocity: Vec2,
	next_velocity: Vec2,
	pub heat: FloatSize,
}

impl Particle {
	pub fn new_random(bounds: &Vec2) -> Self {
		let rand_pos_x = (js::math_random() as FloatSize) * bounds.x;
		let rand_pos_y = (js::math_random() as FloatSize) * bounds.y;

		Particle {
			position: Vec2::new(rand_pos_x, rand_pos_y),
			velocity: Vec2::new(0.0, 0.0),
			next_velocity: Vec2::new(0.0, 0.0),
			heat: 0.0,
		}
	}

	pub fn update(&mut self) {
		self.velocity = self.next_velocity;
		self.position += self.velocity;
		self.velocity = self.velocity * 0.98;
	}

	pub fn change_velocity(&mut self, diff: Vec2) {
		self.next_velocity += diff;
	}

	pub fn interact(&mut self, others: &mut [Particle]) {
		for other_particle in others {
			self.simulate_gravity(other_particle);
		}
	}

	pub fn simulate_gravity(&mut self, other_particle: &mut Particle) {
		let distance = self.position.distance_to(other_particle.position);
		let distance = distance.max(GRAVITY_MIN_RADIUS);
		let angle = self.position.angle_to(other_particle.position);
		let force = GRAVITATIONAL_FORCE / distance.powi(2);
		let vel_change = Vec2::from_polar(angle, force);
		self.change_velocity(-vel_change);
		other_particle.change_velocity(vel_change);
	}
}
