#[macro_export]
macro_rules! enum_flags {
    (pub enum $name:ident : $type:ty { $($flags:ident = $values:expr,)+ } of $($repr:tt)*) => {
        enum_flags!{@impl pub enum, $name, $type, $($flags)+, $($values)+, $($repr)*}
    };

    (enum $name:ident : $type:ty { $($flags:ident = $values:expr,)+ } of $($repr:tt)*) => {
        enum_flags!{@impl enum, $name, $type, $($flags)+, $($values)+, $($repr)*}
    };

    (@impl $($attrs:ident)+, $name:ident, $type: ty, $($flags:ident)+, $($values:expr)+, $($repr:tt)*) => {
        #[allow(dead_code)]
        #[derive(Copy, Clone, PartialEq, Eq)]
        $($repr)*
        $($attrs)+ $name {
            $($flags = 1 << $values,)+

            Nothing = 0,
            All = enum_flags!(@all $type, $($flags)+),
	        Count = enum_flags!(@count $($flags)+)
        }

        impl $name {
            #[inline]
            pub const fn from_mut(x: &'a mut $type) -> &'a mut Self {
                unsafe { &mut *(x as *mut $type as *mut Self) }
            }

            #[inline]
            pub fn add(&mut self, flags: Self) {
	            *self |= (flags & Self::All)
            }

	        #[inline]
	        pub fn remove(&mut self, flags: Self) {
	            *self &= !(flags & Self::All)
            }

	        #[inline]
	        pub fn contain(self, flags: Self) -> bool {
	            (self & flags) != Self::Nothing
            }

	        #[inline]
	        pub fn reset(&mut self) {
	            *self &= !Self::All
            }

	        #[inline]
	        pub fn set(&mut self, flags: Self) {
	            self.reset();
	            self.add(flags)
            }

	        #[inline]
            pub const fn from(x: $type) -> Self {
                unsafe { *(&x as *const $type as *const Self) }
            }
        }

        impl ::core::convert::From <$type> for $name {
            #[inline]
            fn from(x: $type) -> Self {
                unsafe { *(&x as *const $type as *const Self) }
            }
        }

        impl ::core::convert::From <$name> for $type {
	        #[inline]
	        fn from(x: $name) -> Self {
		        unsafe { *(&x as *const $name as *const Self) }
	        }
        }

        impl ::core::ops::Not for $name {
	        type Output = Self;

	        #[inline]
	        fn not(self) -> Self::Output { Self::from(!((self & Self::All) as $type)) }
        }

	    enum_flags!{@bit $name $type, BitOr bitor |}
        enum_flags!{@bit $name $type, BitAnd bitand &}
        enum_flags!{@bit $name $type, BitXor bitxor ^}

        enum_flags!{@bit= $name BitOrAssign bitor_assign |}
        enum_flags!{@bit= $name BitAndAssign bitand_assign &}
        enum_flags!{@bit= $name BitXorAssign bitxor_assign ^}

	    impl ::core::fmt::Display for $name {
	        fn fmt(&self, f: &mut ::core::fmt::Formatter <'_>) -> ::core::fmt::Result {
		        use ::core::fmt::Write;
		        let mut first = true;
		        enum_flags!{@dpy self, first f $($flags)+}
		        if first { f.write_str("Nothing") } else { Ok(()) }
	        }
        }
    };

	(@dpy $self:expr, $first:ident $f:ident $flag:ident) => {
		if $self.contain(Self::$flag) {
			#[allow(unused_assignments)]
			if $first { $first = false } else { $f.write_char('|')? }
			$f.write_str(stringify!($flag))?
		}
	};

	(@dpy $self:expr, $first:ident $f:ident $flag:ident $($flags:ident)+) => {
		enum_flags!(@dpy $self, $first $f $flag);
		enum_flags!(@dpy $self, $first $f $($flags)+);
	};

    (@bit= $name:ident $cls:ident $fun:ident $op:tt) => {
        impl ::core::ops::$cls for $name {
            #[inline]
            fn $fun(&mut self, rhs: Self) { *self = *self $op rhs }
        }
    };

    (@bit $name:ident $type:ty, $cls:ident $fun:ident $op:tt) => {
        impl ::core::ops::$cls for $name {
            type Output = Self;

            #[inline]
            fn $fun(self, rhs: Self) -> Self::Output { Self::from(self as $type $op (rhs as $type & Self::All as $type) as $type) }
        }
    };

	(@all $type:ty, $flag:ident) => {
		Self::$flag as $type
	};

	(@all $type:ty, $flag:ident $($flags:ident)+) => {
		enum_flags!(@all $type, $flag) | enum_flags!(@all $type, $($flags)+)
	};

	(@count $flag:ident) => {
		1
	};

	(@count $flag:ident $($flags:ident)+) => {
		enum_flags!(@count $flag) + enum_flags!(@count $($flags)+)
	};
}
