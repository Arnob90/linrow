#[macro_export]
macro_rules! impl_forward_ref_binop {
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
