#[cfg(all(feature = "hash", feature = "allocator"))]
mod private {
	use core::hash::Hasher;

	#[cfg(feature = "allocator")]
	use alloc::vec::Vec;

	pub use core::hash::Hash as Hashable;
	pub type Hash = u64;

	pub mod polynomial {

		use super::Hash;

		pub const P_ENGLISH_LOWER_OR_UPPER_ONLY: u64 =  31;
		pub const P_ENGLISH_BOTH:                u64 =  53;
		pub const P_ASCII_ALL:                   u64 = 251;

		pub struct Hasher {
			p: u64,
			bytes: super::Vec <u8>
		}

		impl super::Hasher for Hasher {

			#[allow(arithmetic_overflow)]
			fn finish(&self) -> Hash {
				let mut sum = 0;
				for n in 0..self.bytes.len() as u32 {
					sum += self.bytes[n as usize] as Hash * self.p.pow(n)
				}
				sum
			}

			#[inline]
			fn write(&mut self, bytes: &[u8]) {
				self.bytes.append(&mut super::Vec::from(bytes));
			}
		}

		impl Hasher {

			#[inline]
			pub fn new(p: Hash) -> Self {
				Self {
					bytes: super::Vec::new(),
					p
				}
			}

			#[inline]
			pub fn clear(&mut self) {
				self.bytes.clear()
			}

			#[inline]
			pub fn write_str(&mut self, s: &str) {
				use super::Hasher;
				self.write(s.as_bytes())
			}
		}

		pub fn hash(p: u64, bytes: &[u8]) -> Hash {
			use core::hash::Hasher;
			let mut hasher = self::Hasher::new(p);
			hasher.write(bytes);
			hasher.finish()
		}

	}
}

#[cfg(all(feature = "hash", feature = "allocator"))]
pub use private::*;
