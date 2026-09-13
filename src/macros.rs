#[macro_export]
macro_rules! impl_forward_ref_binop {
    ($trait:ident, $method:ident, $lhs:ident <$($lt:ident),*>, $rhs:ident <$($rt:ident),*>, $output:ident <$($ot:ident),*> $(where $($bound:tt)+)?) => {
        // 1. Owned * Owned
        impl<$($lt),*> $trait<$rhs<$($rt),*>> for $lhs<$($lt),*>
        $(where $($bound)+)?
        {
            type Output = $output<$($ot),*>;
            #[inline]
            fn $method(self, rhs: $rhs<$($rt),*>) -> Self::Output {
                (&self).$method(&rhs)
            }
        }

        // 2. Owned * Reference
        impl<'b, $($lt),*> $trait<&'b $rhs<$($rt),*>> for $lhs<$($lt),*>
        $(where $($bound)+)?
        {
            type Output = $output<$($ot),*>;
            #[inline]
            fn $method(self, rhs: &'b $rhs<$($rt),*>) -> Self::Output {
                (&self).$method(rhs)
            }
        }

        // 3. Reference * Owned
        impl<'a, $($lt),*> $trait<$rhs<$($rt),*>> for &'a $lhs<$($lt),*>
        $(where $($bound)+)?
        {
            type Output = $output<$($ot),*>;
            #[inline]
            fn $method(self, rhs: $rhs<$($rt),*>) -> Self::Output {
                self.$method(&rhs)
            }
        }
    };
    ($trait:ident, $method:ident, $lhs:ty, $rhs:ty, $output:ty) => {
        // 1. Owned * Owned
        impl $trait<$rhs> for $lhs {
            type Output = $output;
            #[inline]
            fn $method(self, rhs: $rhs) -> $output {
                (&self).$method(&rhs)
            }
        }

        // 2. Owned * Reference
        impl $trait<&$rhs> for $lhs {
            type Output = $output;
            #[inline]
            fn $method(self, rhs: &$rhs) -> $output {
                (&self).$method(rhs)
            }
        }

        // 3. Reference * Owned
        impl $trait<$rhs> for &$lhs {
            type Output = $output;
            #[inline]
            fn $method(self, rhs: $rhs) -> $output {
                self.$method(&rhs)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_forward_ref_assign_op {
    ($trait:ident, $method:ident, $lhs:ident <$($lt:ident),*>, $rhs:ident <$($rt:ident),*> $(where $($bound:tt)+)?) => {
        // Forward: Owned RHS -> Reference RHS
        // This allows: row1 += row2;
        impl<$($lt),*> $trait<$rhs<$($rt),*>> for $lhs<$($lt),*>
        $(where $($bound)+)?
        {
            #[inline]
            fn $method(&mut self, rhs: $rhs<$($rt),*>) {
                self.$method(&rhs)
            }
        }
    };
    ($trait:ident, $method:ident, $lhs:ty, $rhs:ty) => {
        // Forward: Owned RHS -> Reference RHS
        // This allows: row1 += row2;
        impl $trait<$rhs> for $lhs {
            #[inline]
            fn $method(&mut self, rhs: $rhs) {
                self.$method(&rhs)
            }
        }
    };
}

#[macro_export]
macro_rules! def_matrix {
    ( $( [ $( $val:expr ),*$(,)? ] ),* $(,)? ) => {
        Matrix::new(vec![
            $(
                vec![ $( $val ),* ]
            ),*
        ])
    };
}

#[macro_export]
macro_rules! impl_real_scalar {
    ($($t:ty),*) => {
        $(
            impl Metric for $t {
                #[inline(always)]
                fn is_near(&self, other: &Self) -> bool {
                    (*self - *other).abs() <= $crate::utils::get_generic_eps()
                }
            }

            impl HasConj for $t {
                #[inline(always)]
                fn conj(&self) -> Self {
                    *self
                }
            }
        )*
    };
}
