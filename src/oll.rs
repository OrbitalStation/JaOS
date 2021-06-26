use alloc::{
	string::String,
	vec::Vec,
};
pub use alloc::boxed::Box;
use core::{
	fmt::Debug,
	iter::Iterator
};
use crate::{
	tty,
	keyboard,
	print,
	println,
};

use crate::hash::Hash;

pub fn hash(msg: &str) -> Hash {
	crate::hash::polynomial::hash(crate::hash::polynomial::P_ENGLISH_BOTH, msg.as_bytes())
}

pub struct DebugContainerEntry {
	pub name: Hash,
	pub value_ptr: *const (),
	pub function: fn(*const ()),
	pub indent: u8
}

unsafe impl Send for DebugContainerEntry { }

pub struct DebugContainerConstEntry {
	pub name: Hash,
	pub string: String,
	pub indent: u8
}

pub struct DebugContainerForEntry {
	pub name: Hash,
	pub iter: *const (),
	pub index: usize,
	pub function: fn(Hash, *const (), usize)
}

pub trait DebugContainerIteratorTrait {
	fn print(&mut self);

	fn next(&mut self);

	fn name(&self) -> Hash;
}

pub struct DebugContainerIterator <T> where T: Iterator {
	index: usize,
	iter: T,
	name: Hash
}

impl <T: Iterator> DebugContainerIterator <T> {
	pub fn new(iter: T, name: Hash) -> Self {
		Self {
			index: 0,
			iter,
			name
		}
	}
}

impl <T: Iterator + Clone> DebugContainerIteratorTrait for DebugContainerIterator <T> where T::Item: Debug {
	fn print(&mut self) {
		tty::set_color(tty::VGA::make(tty::Color::LightBlue, tty::Color::Default));
		println!("{:?}", self.iter.clone().nth(self.index).unwrap())
	}

	fn next(&mut self) {
		self.index += 1;
	}

	fn name(&self) -> Hash {
		self.name
	}
}

pub struct DebugContainer {
	vec: Vec <DebugContainerEntry>,
	cnt: Vec <DebugContainerConstEntry>,
	its: Vec <Box <dyn DebugContainerIteratorTrait>>,
	indent: u8,
	initial: u8,
	init: bool,
	last_for: bool
}

impl DebugContainer {
	pub const fn new() -> Self {
		Self {
			vec: Vec::new(),
			cnt: Vec::new(),
			its: Vec::new(),
			initial: 0,
			indent: 0,
			init: false,
			last_for: false
		}
	}

	pub fn add(&mut self, name: Hash, value_ptr: *const (), function: fn(*const ())) {
		self.vec.push(DebugContainerEntry {
			name,
			value_ptr,
			function,
			indent: self.indent
		})
	}

	pub fn add_const(&mut self, name: Hash, string: String) {
		self.cnt.push(DebugContainerConstEntry {
			name,
			string,
			indent: self.indent
		})
	}

	pub fn add_iter(&mut self, iter: Box <dyn DebugContainerIteratorTrait>) {
		self.its.push(iter)
	}

	pub fn print(&mut self, name: Hash) -> bool {
		if is_skipping() { return false }
		let mut indent: i16 = -1;
		let mut index = 0;

		for (idx, i) in self.vec.iter().enumerate() {
			if i.name == name && i.indent >= self.initial {
				if i.indent as i16 > indent {
					indent = i.indent as i16;
					index = idx;
				}
			}
		}
		if indent != -1 {
			(self.vec[index].function)(self.vec[index].value_ptr);
			return true
		}
		indent = -1;
		for (idx, i) in self.cnt.iter().enumerate() {
			if i.name == name && i.indent >= self.initial {
				if i.indent as i16 > indent {
					indent = i.indent as i16;
					index = idx;
				}
			}
		}
		if indent != -1 {
			tty::set_color(tty::VGA::make(tty::Color::LightBlue, tty::Color::Default));
			println!("{}", self.cnt[index].string);
			return true
		}
		indent = -1;
		for (idx, i) in self.its.iter().enumerate() {
			if i.name() == name {
				index = idx;
				indent = 0;
			}
		}
		if indent != -1 {
			tty::set_color(tty::VGA::make(tty::Color::Green, tty::Color::Default));
			self.its[index].print();
			return true
		}
		false
	}

	fn _td_template(&mut self, s: &'static str) {
		if is_skipping() { return }
		tty::set_color(tty::VGA::make(tty::Color::LightGreen, tty::Color::Default));
		print!("{}ing sub-block", s);
		if self.indent > 1 {
			let x = self.indent % 10;
			print!("({}-", self.indent);
			if x == 1 {
				print!("st")
			} else if x == 2 {
				print!("nd")
			} else if x == 3 {
				print!("rd")
			} else {
				print!("th")
			}
			print!(")")
		}
		tty::set_color(tty::DEFAULT);
		println!();
	}

	pub fn up(&mut self) {
		self.indent += 1;
		self._td_template("Enter");
	}

	pub fn down(&mut self) {
		self._td_template("Leav");
		let mut i = 0;
		while i < self.vec.len() {
			if self.vec[i].indent == self.indent {
				self.vec.remove(i);
			} else {
				i += 1;
			}
		}
		i = 0;
		while i < self.cnt.len() {
			if self.cnt[i].indent == self.indent {
				self.cnt.remove(i);
			} else {
				i += 1;
			}
		}
		self.indent -= 1;
	}

	pub fn cycle() {
		if is_skipping() { return }
		tty::set_color(tty::VGA::make(tty::Color::LightGreen, tty::Color::Default));
		println!("Loop repeats")
	}

	pub fn clear(&mut self) {
		if is_skipping() { return }
		if self.initial != 0 {
			if self.indent - 1 != self.initial { return }
			self.indent -= 1;
			tty::set_color(tty::VGA::make(tty::Color::Green, tty::Color::Default));
			println!("Control is returned.");
			tty::set_color(tty::DEFAULT);
			self.initial = 0;
		} else {
			if self.indent != self.initial { return }
			tty::set_color(tty::VGA::make(tty::Color::Green, tty::Color::Default));
			println!("Leaving oll.");
			tty::set_color(tty::DEFAULT);
			self.vec.clear();
			self.cnt.clear();
			self.its.clear();
			self.init = false;
			self.initial = 0;
			self.indent = 0;
			unsafe { USING = 0 }
		}
	}

	pub fn hello(&mut self) {
		if is_skipping() { return }
		if self.init {
			self.initial = self.indent;
			self.indent += 1;
			tty::set_color(tty::VGA::make(tty::Color::Green, tty::Color::Default));
			println!("Control was taken by other `debug!` block")
		} else {
			tty::set_color(tty::VGA::make(tty::Color::Green, tty::Color::Default));
			println!("Welcome to the OS-level debugger (oll)!");
			tty::set_color(tty::DEFAULT);
			self.init = true;
			unsafe { USING |= 1 }
		}
	}

	pub fn for_iter(&mut self, name: Hash) {
		if !self.last_for { return }
		for i in self.its.iter_mut() {
			if i.name() == name {
				i.next();
			}
		}
	}

	pub fn lastfor(&mut self, v: bool) {
		self.last_for = v;
	}

	pub fn lname(&self) -> Hash {
		self.its.last().unwrap().name()
	}
}

pub fn take(expr: &str) {
	if is_skipping() { return }
	let w = || {
		tty::set_color(tty::VGA::make(tty::Color::LightGreen, tty::Color::Default));
		print!("Executing ");
		tty::set_color(tty::VGA::make(tty::Color::LightRed, tty::Color::Default));
		print!("`");
		tty::set_color(tty::DEFAULT);
		print!("{}", expr);
		tty::set_color(tty::VGA::make(tty::Color::LightRed, tty::Color::Default));
		println!("`");
	};
	w();
	if expr.starts_with("continue") { unsafe { DC.indent -= 1 } }
	loop {
		tty::set_color(tty::VGA::make(tty::Color::White, tty::Color::Default));
		print!("> ");
		tty::set_color(tty::DEFAULT);
		let mut s = String::new();
		keyboard::readline(&mut s);
		if s == "\n" { continue }
		if s == ":e\n" { break }
		if s.starts_with(":d ") {
			if !unsafe { DC.print(hash(&s[3..s.len() - 1])) } {
				tty::set_color(tty::VGA::make(tty::Color::LightRed, tty::Color::Default));
				println!("There's no variable with such name");
			}
			continue
		}
		if s == ":w\n" {
			w();
			continue
		}
		if s == ":q\n" {
			unsafe { USING |= 2 }
			return
		}
		if s == ":h\n" {
			tty::set_color(tty::VGA::make(tty::Color::Magenta, tty::Color::Default));
			println!("Commands:\n\t:e - execute command\n\t:d <name> - show value of <name>\n\t:q - quit debugger\n\t:h - show this info\n\t:w - show currently executing line");
			continue
		}
		tty::set_color(tty::VGA::make(tty::Color::LightRed, tty::Color::Default));
		println!("Unknown command; try ':h'!");
	}
	tty::set_color(tty::DEFAULT);
}

pub static mut DC: DebugContainer = DebugContainer::new();
pub static mut USING: u8 = 0;

pub fn print <T> (p: *const u8) where T: Debug {
	if is_skipping() { return }
	tty::set_color(tty::VGA::make(tty::Color::LightBlue, tty::Color::Default));
	unsafe { println!("{:?}", *(p as *const T)) }
}

pub fn cycle(n: &'static str, msg: &'static str) {
	if is_skipping() { return }
	tty::set_color(tty::VGA::make(tty::Color::LightGreen, tty::Color::Default));
	print!("Executing {}: ", msg);
	tty::set_color(tty::VGA::make(tty::Color::LightRed, tty::Color::Default));
	print!("`");
	tty::set_color(tty::VGA::make(tty::Color::Yellow, tty::Color::Default));
	print!("{}", n);
	tty::set_color(tty::VGA::make(tty::Color::LightRed, tty::Color::Default));
	println!("`");
}

pub fn cfor(s: Hash) {
	DebugContainer::cycle();
	unsafe { DC.for_iter(s) }
}

#[macro_export]
macro_rules! debug {
	($($t:tt)*) => {
		unsafe { $crate::oll::DC.hello() };
		$crate::__debug_impl!($($t)*);
	}
}

#[macro_export]
macro_rules! __debug_impl {
    (let $let:ident : $type:ty; $($t:tt)*) => {
        $crate::oll::take(stringify!(let $let: $type));
        let $let: $type;
        $crate::__debug_impl!{@end $let, $type, $($t)*}
    };

    (let mut $let:ident : $type:ty; $($t:tt)*) => {
        $crate::oll::take(stringify!(let mut $let: $type));
        let mut $let: $type;
        $crate::__debug_impl!{@end $let, $type, $($t)*}
    };

    (let $let:ident : $type:ty = $val:expr; $($t:tt)*) => {
        $crate::oll::take(stringify!(let $let: $type = $val));
        let $let: $type = $val;
        $crate::__debug_impl!{@end $let, $type, $($t)*}
    };

    (let mut $let:ident : $type:ty = $val:expr; $($t:tt)*) => {
        $crate::oll::take(stringify!(let mut $let: $type = $val));
        let mut $let: $type = $val;
        $crate::__debug_impl!{@end $let, $type, $($t)*}
    };

    (const $let:ident : $type:ty = $val:expr; $($t:tt)*) => {
        $crate::oll::take(stringify!(const $let: $type = $val));
        const $let: $type = $val;
        unsafe { DC }.add_const($crate::oll::hash(stringify!($let)), os::alloc::format!("{:?}", $val));
        $crate::__debug_impl!{$($t)*}
    };

	({ $($tt:tt)* } $($t:tt)*) => {
		{
			unsafe { $crate::oll::DC.up() };
			$crate::__debug_impl!{$($tt)*}
			unsafe { $crate::oll::DC.down() };
		}
		$crate::__debug_impl!{$($t)*}
	};

	(if ($expr:expr) { $($tt:tt)* } $($t:tt)*) => {
		$crate::oll::cycle(stringify!(if ($expr)), "if statement");
		if $expr {
			unsafe { $crate::oll::DC.up() };
			$crate::__debug_impl!{$($tt)*}
			unsafe { $crate::oll::DC.down() };
		}
		$crate::__debug_impl!{$($t)*}
	};

	(loop { $($tt:tt)* } $($t:tt)*) => {
		$crate::oll::cycle("loop", "infinite loop");
		unsafe { $crate::oll::DC.up() };
		loop {
			$crate::__debug_impl!{$($tt)*}
			$crate::oll::DebugContainer::cycle();
		}
		unsafe { $crate::oll::DC.down() };
		$crate::__debug_impl!{$($t)*}
	};

	(while ($expr:expr) { $($tt:tt)* } $($t:tt)*) => {
		$crate::oll::cycle(stringify!(while ($expr)), "while loop");
		unsafe { $crate::oll::DC.up() };
		while $expr {
			$crate::__debug_impl!{$($tt)*}
			$crate::oll::DebugContainer::cycle();
		}
		unsafe { $crate::oll::DC.down() };
		$crate::__debug_impl!{$($t)*}
	};

	(for $name:ident in ($expr:expr) { $($tt:tt)* } $($t:tt)*) => {
		$crate::oll::cycle(stringify!(for $name in ($expr)), "for loop");
		{
			let __debug_expr = $expr;
			unsafe {
				$crate::oll::DC.up();
				$crate::oll::DC.add_iter($crate::oll::Box::new($crate::oll::DebugContainerIterator::new(__debug_expr.clone(), $crate::oll::hash(stringify!($name)))));
				$crate::oll::DC.lastfor(true)
			}
			for $name in __debug_expr {
				$crate::__debug_impl!{$($tt)*}
				$crate::oll::cfor($crate::oll::hash(stringify!($name)));
			}
			unsafe {
				$crate::oll::DC.down();
				$crate::oll::DC.lastfor(false)
			};
		}
		$crate::__debug_impl!{$($t)*}
	};

	(unsafe { $($tt:tt)* } $($t:tt)*) => {
		$crate::oll::cycle("unsafe", "unsafe statement");
		unsafe {
			$crate::oll::DC.up();
			$crate::__debug_impl!{$($tt)*}
			$crate::oll::DebugContainer::cycle();
			$crate::oll::DC.down()
		};
		$crate::__debug_impl!{$($t)*}
	};
	
	(continue $($t:tt)*) => {
		$crate::oll::take("continue");
		$crate::oll::cfor(unsafe { $crate::oll::DC.lname() });
		$crate::__debug_impl!{$($t)*}
	};

    ($expr:expr; $($t:tt)*) => {
        $crate::oll::take(stringify!($expr));
        $expr;
        $crate::__debug_impl!{$($t)*}
    };

    ($expr:expr) => {
        $crate::oll::take(stringify!($expr));
        unsafe { $crate::oll::DC.clear() };
        return $expr;
    };

    (@end $let:ident, $type:ty, $($t:tt)*) => {
        unsafe { $crate::oll::DC.add($crate::oll::hash(stringify!($let)), &$let as *const $type as *const u8, $crate::oll::print::<$type>) };
        $crate::__debug_impl!{$($t)*}
    };

    () => { unsafe { $crate::oll::DC.clear() } }
}

pub fn is_debug_mode_on() -> bool {
	unsafe { USING & 1 != 0 }
}

pub fn is_skipping() -> bool {
	unsafe { USING & 2 != 0 }
}

// SAVE:
/*use alloc::{
	string::String,
	vec::Vec,
};
pub use alloc::boxed::Box;
use core::{
	fmt::Debug,
	iter::Iterator
};
use crate::{
	tty,
	keyboard,
	print,
	println,
};

use crate::hash::polynomial::Hasher;

pub struct DebugContainerEntry {
	pub name: &'static str,
	pub value_ptr: *const u8,
	pub function: fn(*const u8),
	pub indent: u8
}

unsafe impl Send for DebugContainerEntry { }

pub struct DebugContainerConstEntry {
	pub name: &'static str,
	pub string: String,
	pub indent: u8
}

pub struct DebugContainerForEntry {
	pub name: &'static str,
	pub iter: *const (),
	pub index: usize,
	pub function: fn(&'static str, *const (), usize)
}

pub trait DebugContainerIteratorTrait {
	fn print(&mut self);

	fn next(&mut self);

	fn name(&self) -> &'static str;
}

pub struct DebugContainerIterator <T> where T: Iterator {
	index: usize,
	iter: T,
	name: &'static str
}

impl <T: Iterator> DebugContainerIterator <T> {
	pub fn new(iter: T, name: &'static str) -> Self {
		Self {
			index: 0,
			iter,
			name
		}
	}
}

impl <T: Iterator + Clone> DebugContainerIteratorTrait for DebugContainerIterator <T> where T::Item: Debug {
	fn print(&mut self) {
		tty::set_color(tty::VGA::make(tty::Color::Blue, tty::Color::Default));
		println!("{:?}", self.iter.clone().nth(self.index).unwrap())
	}

	fn next(&mut self) {
		self.index += 1;
	}

	fn name(&self) -> &'static str {
		self.name
	}
}

pub struct DebugContainer {
	vec: Vec <DebugContainerEntry>,
	cnt: Vec <DebugContainerConstEntry>,
	its: Vec <Box <dyn DebugContainerIteratorTrait>>,
	indent: u8,
	initial: u8,
	init: bool,
	last_for: bool
}

impl DebugContainer {
	pub const fn new() -> Self {
		Self {
			vec: Vec::new(),
			cnt: Vec::new(),
			its: Vec::new(),
			initial: 0,
			indent: 0,
			init: false,
			last_for: false
		}
	}

	pub fn add(&mut self, name: &'static str, value_ptr: *const u8, function: fn(*const u8)) {
		self.vec.push(DebugContainerEntry {
			name,
			value_ptr,
			function,
			indent: self.indent
		})
	}

	pub fn add_const(&mut self, name: &'static str, string: String) {
		self.cnt.push(DebugContainerConstEntry {
			name,
			string,
			indent: self.indent
		})
	}

	pub fn add_iter(&mut self, iter: Box <dyn DebugContainerIteratorTrait>) {
		self.its.push(iter)
	}

	pub fn print(&mut self, name: &str) -> bool {
		if is_skipping() { return false }
		let mut indent: i16 = -1;
		let mut index = 0;

		for (idx, i) in self.vec.iter().enumerate() {
			if i.name == name && i.indent >= self.initial {
				if i.indent as i16 > indent {
					indent = i.indent as i16;
					index = idx;
				}
			}
		}
		if indent != -1 {
			(self.vec[index].function)(self.vec[index].value_ptr);
			return true
		}
		indent = -1;
		for (idx, i) in self.cnt.iter().enumerate() {
			if i.name == name && i.indent >= self.initial {
				if i.indent as i16 > indent {
					indent = i.indent as i16;
					index = idx;
				}
			}
		}
		if indent != -1 {
			tty::set_color(tty::VGA::make(tty::Color::Green, tty::Color::Default));
			println!("{}", self.cnt[index].string);
			return true
		}
		indent = -1;
		for (idx, i) in self.its.iter().enumerate() {
			if i.name() == name {
				index = idx;
				indent = 0;
			}
		}
		if indent != -1 {
			tty::set_color(tty::VGA::make(tty::Color::Green, tty::Color::Default));
			self.its[index].print();
			return true
		}
		false
	}

	fn _td_template(&mut self, s: &'static str) {
		if is_skipping() { return }
		tty::set_color(tty::VGA::make(tty::Color::LightGreen, tty::Color::Default));
		print!("{}ing sub-block", s);
		if self.indent > 1 {
			let x = self.indent % 10;
			print!("({}-", self.indent);
			if x == 1 {
				print!("st")
			} else if x == 2 {
				print!("nd")
			} else if x == 3 {
				print!("rd")
			} else {
				print!("th")
			}
			print!(")")
		}
		tty::set_color(tty::DEFAULT);
		println!();
	}

	pub fn up(&mut self) {
		self.indent += 1;
		self._td_template("Enter");
	}

	pub fn down(&mut self) {
		self._td_template("Leav");
		let mut i = 0;
		while i < self.vec.len() {
			if self.vec[i].indent == self.indent {
				self.vec.remove(i);
			} else {
				i += 1;
			}
		}
		i = 0;
		while i < self.cnt.len() {
			if self.cnt[i].indent == self.indent {
				self.cnt.remove(i);
			} else {
				i += 1;
			}
		}
		self.indent -= 1;
	}

	pub fn cycle() {
		if is_skipping() { return }
		tty::set_color(tty::VGA::make(tty::Color::LightGreen, tty::Color::Default));
		println!("Loop repeats")
	}

	pub fn clear(&mut self) {
		if is_skipping() { return }
		if self.initial != 0 {
			if self.indent - 1 != self.initial { return }
			self.indent -= 1;
			tty::set_color(tty::VGA::make(tty::Color::Green, tty::Color::Default));
			println!("Control is returned.");
			tty::set_color(tty::DEFAULT);
			self.initial = 0;
		} else {
			if self.indent != self.initial { return }
			tty::set_color(tty::VGA::make(tty::Color::Green, tty::Color::Default));
			println!("Leaving oll.");
			tty::set_color(tty::DEFAULT);
			self.vec.clear();
			self.cnt.clear();
			self.its.clear();
			self.init = false;
			self.initial = 0;
			self.indent = 0;
			unsafe { USING = 0 }
		}
	}

	pub fn hello(&mut self) {
		if is_skipping() { return }
		if self.init {
			self.initial = self.indent;
			self.indent += 1;
			tty::set_color(tty::VGA::make(tty::Color::Green, tty::Color::Default));
			println!("Control was taken by other `debug!` block")
		} else {
			tty::set_color(tty::VGA::make(tty::Color::Green, tty::Color::Default));
			println!("Welcome to the OS-level debugger (oll)!");
			tty::set_color(tty::DEFAULT);
			self.init = true;
			unsafe { USING |= 1 }
		}
	}

	pub fn for_iter(&mut self, name: &'static str) {
		if !self.last_for { return }
		for i in self.its.iter_mut() {
			if i.name() == name {
				i.next();
			}
		}
	}

	pub fn lastfor(&mut self, v: bool) {
		self.last_for = v;
	}

	pub fn lname(&'a self) -> &'a str {
		self.its.last().unwrap().name()
	}
}

pub fn take(expr: &str) {
	if is_skipping() { return }
	let w = || {
		tty::set_color(tty::VGA::make(tty::Color::LightGreen, tty::Color::Default));
		print!("Executing ");
		tty::set_color(tty::VGA::make(tty::Color::LightRed, tty::Color::Default));
		print!("`");
		tty::set_color(tty::DEFAULT);
		print!("{}", expr);
		tty::set_color(tty::VGA::make(tty::Color::LightRed, tty::Color::Default));
		println!("`");
	};
	w();
	if expr.starts_with("continue") { unsafe { DC.indent -= 1 } }
	loop {
		tty::set_color(tty::VGA::make(tty::Color::White, tty::Color::Default));
		print!("> ");
		tty::set_color(tty::DEFAULT);
		let mut s = String::new();
		keyboard::readline(&mut s);
		if s == "\n" { continue }
		if s == ":e\n" { break }
		if s.starts_with(":d ") {
			if !unsafe { DC.print(&s[3..s.len() - 1]) } {
				tty::set_color(tty::VGA::make(tty::Color::LightRed, tty::Color::Default));
				println!("There's no variable with such name");
			}
			continue
		}
		if s == ":w\n" {
			w();
			continue
		}
		if s == ":q\n" {
			unsafe { USING |= 2 }
			return
		}
		if s == ":h\n" {
			tty::set_color(tty::VGA::make(tty::Color::Magenta, tty::Color::Default));
			println!("Commands:\n\t:e - execute command\n\t:d <name> - show value of <name>\n\t:q - quit debugger\n\t:h - show this info\n\t:w - show currently executing line");
			continue
		}
		tty::set_color(tty::VGA::make(tty::Color::LightRed, tty::Color::Default));
		println!("Unknown command; try ':h'!");
	}
	tty::set_color(tty::DEFAULT);
}

pub static mut DC: DebugContainer = DebugContainer::new();
pub static mut USING: u8 = 0;

pub fn print <T> (p: *const u8) where T: Debug {
	if is_skipping() { return }
	tty::set_color(tty::VGA::make(tty::Color::Blue, tty::Color::Default));
	unsafe { println!("{:?}", *(p as *const T)) }
}

pub fn cycle(n: &'static str, msg: &'static str) {
	if is_skipping() { return }
	tty::set_color(tty::VGA::make(tty::Color::LightGreen, tty::Color::Default));
	print!("Executing {}: ", msg);
	tty::set_color(tty::VGA::make(tty::Color::LightRed, tty::Color::Default));
	print!("`");
	tty::set_color(tty::VGA::make(tty::Color::Yellow, tty::Color::Default));
	print!("{}", n);
	tty::set_color(tty::VGA::make(tty::Color::LightRed, tty::Color::Default));
	println!("`");
}

pub fn cfor(s: &'static str) {
	DebugContainer::cycle();
	unsafe { DC.for_iter(s) }
}

#[macro_export]
macro_rules! debug {
	($($t:tt)*) => {
		unsafe { $crate::oll::DC.hello() };
		$crate::__debug_impl!($($t)*);
	}
}

#[macro_export]
macro_rules! __debug_impl {
    (let $let:ident : $type:ty; $($t:tt)*) => {
        $crate::oll::take(stringify!(let $let: $type));
        let $let: $type;
        $crate::__debug_impl!{@end $let, $type, $($t)*}
    };

    (let mut $let:ident : $type:ty; $($t:tt)*) => {
        $crate::oll::take(stringify!(let mut $let: $type));
        let mut $let: $type;
        $crate::__debug_impl!{@end $let, $type, $($t)*}
    };

    (let $let:ident : $type:ty = $val:expr; $($t:tt)*) => {
        $crate::oll::take(stringify!(let $let: $type = $val));
        let $let: $type = $val;
        $crate::__debug_impl!{@end $let, $type, $($t)*}
    };

    (let mut $let:ident : $type:ty = $val:expr; $($t:tt)*) => {
        $crate::oll::take(stringify!(let mut $let: $type = $val));
        let mut $let: $type = $val;
        $crate::__debug_impl!{@end $let, $type, $($t)*}
    };

    (const $let:ident : $type:ty = $val:expr; $($t:tt)*) => {
        $crate::oll::take(stringify!(const $let: $type = $val));
        const $let: $type = $val;
        unsafe { DC }.add_const(stringify!($let), os::alloc::format!("{:?}", $val));
        $crate::__debug_impl!{$($t)*}
    };

	({ $($tt:tt)* } $($t:tt)*) => {
		{
			unsafe { $crate::oll::DC.up() };
			$crate::__debug_impl!{$($tt)*}
			unsafe { $crate::oll::DC.down() };
		}
		$crate::__debug_impl!{$($t)*}
	};

	(if ($expr:expr) { $($tt:tt)* } $($t:tt)*) => {
		$crate::oll::cycle(stringify!(if ($expr)), "if statement");
		if $expr {
			unsafe { $crate::oll::DC.up() };
			$crate::__debug_impl!{$($tt)*}
			unsafe { $crate::oll::DC.down() };
		}
		$crate::__debug_impl!{$($t)*}
	};

	(loop { $($tt:tt)* } $($t:tt)*) => {
		$crate::oll::cycle("loop", "infinite loop");
		unsafe { $crate::oll::DC.up() };
		loop {
			$crate::__debug_impl!{$($tt)*}
			$crate::oll::DebugContainer::cycle();
		}
		unsafe { $crate::oll::DC.down() };
		$crate::__debug_impl!{$($t)*}
	};

	(while ($expr:expr) { $($tt:tt)* } $($t:tt)*) => {
		$crate::oll::cycle(stringify!(while ($expr)), "while loop");
		unsafe { $crate::oll::DC.up() };
		while $expr {
			$crate::__debug_impl!{$($tt)*}
			$crate::oll::DebugContainer::cycle();
		}
		unsafe { $crate::oll::DC.down() };
		$crate::__debug_impl!{$($t)*}
	};

	(for $name:ident in ($expr:expr) { $($tt:tt)* } $($t:tt)*) => {
		$crate::oll::cycle(stringify!(for $name in ($expr)), "for loop");
		{
			let __debug_expr = $expr;
			unsafe {
				$crate::oll::DC.up();
				$crate::oll::DC.add_iter($crate::oll::Box::new($crate::oll::DebugContainerIterator::new(__debug_expr.clone(), stringify!($name))));
				$crate::oll::DC.lastfor(true)
			}
			for $name in __debug_expr {
				$crate::__debug_impl!{$($tt)*}
				$crate::oll::cfor(stringify!($name));
			}
			unsafe {
				$crate::oll::DC.down();
				$crate::oll::DC.lastfor(false)
			};
		}
		$crate::__debug_impl!{$($t)*}
	};

	(unsafe { $($tt:tt)* } $($t:tt)*) => {
		$crate::oll::cycle("unsafe", "unsafe statement");
		unsafe {
			$crate::oll::DC.up();
			$crate::__debug_impl!{$($tt)*}
			$crate::oll::DebugContainer::cycle();
			$crate::oll::DC.down()
		};
		$crate::__debug_impl!{$($t)*}
	};

	(continue $($t:tt)*) => {
		$crate::oll::take("continue");
		$crate::oll::cfor(unsafe { $crate::oll::DC.lname() });
		$crate::__debug_impl!{$($t)*}
	};

    ($expr:expr; $($t:tt)*) => {
        $crate::oll::take(stringify!($expr));
        $expr;
        $crate::__debug_impl!{$($t)*}
    };

    ($expr:expr) => {
        $crate::oll::take(stringify!($expr));
        unsafe { $crate::oll::DC.clear() };
        return $expr;
    };

    (@end $let:ident, $type:ty, $($t:tt)*) => {
        unsafe { $crate::oll::DC.add(stringify!($let), &$let as *const $type as *const u8, $crate::oll::print::<$type>) };
        $crate::__debug_impl!{$($t)*}
    };

    () => { unsafe { $crate::oll::DC.clear() } }
}

pub fn is_debug_mode_on() -> bool {
	unsafe { USING & 1 != 0 }
}

pub fn is_skipping() -> bool {
	unsafe { USING & 2 != 0 }
}
*/
