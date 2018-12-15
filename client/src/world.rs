use js;

pub type FloatSize = f32;

#[derive(Clone, Copy)]
pub struct Vec2 {
	pub x: FloatSize,
	pub y: FloatSize,
}

const GRAVITATIONAL_FORCE: FloatSize = 0.5;
const GRAVITY_MIN_RADIUS: FloatSize = 2.0;
const EM_FORCE: FloatSize = 0.15;
const EM_MAX_RADIUS: FloatSize = 4.0;
const EM_MIN_RADIUS: FloatSize = 2.0;
const FRICTION_FORCE: FloatSize = 0.05;
const FRICTION_MAX_RADIUS: FloatSize = 5.0;
const EDGE_RESTITUTION: FloatSize = 0.8;
const PARTICLE_COUNT: u32 = 800;

impl Vec2 {
	pub fn new(x: FloatSize, y: FloatSize) -> Self {
		assert!(!x.is_nan());
		assert!(!y.is_nan());
		Self { x, y }
	}

	// pub fn from_polar(angle: FloatSize, magnitude: FloatSize) -> Self {
	// 	Self::new(angle.cos() * magnitude, angle.sin() * magnitude)
	// }

	pub fn as_tuple(self) -> (FloatSize, FloatSize) {
		(self.x, self.y)
	}

	pub fn distance_to(self, other: Self) -> FloatSize {
		(self - other.into()).magnitude()
	}

	pub fn sq_distance_to(self, other: Self) -> FloatSize {
		(self - other.into()).sq_magnitude()
	}

	pub fn angle_to(self, other: Self) -> FloatSize {
		let (diff_x, diff_y) = (self - other).as_tuple();
		diff_y.atan2(diff_x)
	}

	pub fn normalize(self, magnitude: FloatSize) -> Self {
		if self.magnitude() == 0.0 {
			self
		} else {
			self * (magnitude / self.magnitude())
		}
	}

	pub fn magnitude(&self) -> FloatSize {
		self.sq_magnitude().sqrt()
	}

	fn sq_magnitude(&self) -> FloatSize {
		(self.x.powi(2) + self.y.powi(2))
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

impl std::ops::MulAssign<FloatSize> for Vec2 {
	fn mul_assign(&mut self, rhs: FloatSize) {
		self.x *= rhs;
		self.y *= rhs;
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
	bounds: Vec2,
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
			bounds: *bounds,
			velocity: Vec2::new(0.0, 0.0),
			next_velocity: Vec2::new(0.0, 0.0),
			heat: 0.0,
		}
	}

	pub fn update(&mut self) {
		self.position += self.next_velocity;

		if self.position.x < 0.0 {
			self.next_velocity.x *= -EDGE_RESTITUTION;
			self.position.x = 0.0;
		}
		if self.position.y < 0.0 {
			self.next_velocity.y *= -EDGE_RESTITUTION;
			self.position.y = 0.0;
		}
		if self.position.x > self.bounds.x {
			self.next_velocity.x *= -EDGE_RESTITUTION;
			self.position.x = self.bounds.x;
		}
		if self.position.y > self.bounds.y {
			self.next_velocity.y *= -EDGE_RESTITUTION;
			self.position.y = self.bounds.y;
		}

		self.velocity = self.next_velocity;
	}

	pub fn change_velocity(&mut self, diff: Vec2) {
		self.next_velocity += diff;
	}

	pub fn interact(&mut self, others: &mut [Particle]) {
		for other_particle in others {
			self.simulate_gravity(other_particle);
			self.simulate_em(other_particle);
			self.simulate_friction(other_particle);
		}
	}

	fn simulate_gravity(&mut self, other_particle: &mut Particle) {
		let sq_distance = self.position.sq_distance_to(other_particle.position);
		let sq_distance = sq_distance.max(GRAVITY_MIN_RADIUS.powi(2));
		let force_mag = GRAVITATIONAL_FORCE / sq_distance;
		let force_vector = (other_particle.position - self.position).normalize(force_mag);
		self.change_velocity(force_vector);
		other_particle.change_velocity(-force_vector);
	}

	fn simulate_em(&mut self, other_particle: &mut Particle) {
		let sq_distance = self.position.sq_distance_to(other_particle.position);
		if sq_distance > EM_MAX_RADIUS.powi(2) {
			return;
		}
		//let sq_distance = sq_distance.max(EM_MIN_RADIUS.powi(2));
		let force_mag = EM_FORCE;// / sq_distance;
		let force_vector = (other_particle.position - self.position).normalize(force_mag);
		self.change_velocity(-force_vector);
		other_particle.change_velocity(force_vector);
	}

	fn simulate_friction(&mut self, other_particle: &mut Particle) {
		let sq_distance = self.position.sq_distance_to(other_particle.position);
		if sq_distance > FRICTION_MAX_RADIUS.powi(2) {
			return;
		};
		let balance = (other_particle.velocity - self.velocity) * FRICTION_FORCE;
		self.change_velocity(balance);
		other_particle.change_velocity(-balance);
	}
}
