use crate::*;

// This macro expands to all combinations of pairs in the two given sets, for example:
// all_combos!({1, 2}, {a, b}) -> [(1, a), (1, b), (2, a), (2, b)]
// Braces are used to denote arrays because square brackets around a type (e.g. [u16]) get parsed as
// part of the type
// For each combination, the given macro is called.
macro_rules! all_combos {
    // The final match: all_combos!(lh, rh, @macro)
    ($lh:ty, $rh:ty, @$macro:ident) => {
        $macro!($lh, $rh);
    };
    // Matches a plain lh and a single, wrapped rh: all_combos!(lh, {rh}, @macro)
    ($lh:ty, {$rh:ty}, @$macro:ident) => {
        all_combos!($lh, $rh, @$macro);
    };
    // Matches a single, wrapped lh and a plain rh: all_combos!({lh}, rh, @macro)
    ({$lh:ty}, $rh:ty, @$macro:ident) => {
           all_combos!($lh, $rh, @$macro);
    };
    // Matches a single, wrapped lh and a single, wrapped rh: all_combos!({lh}, {rh}, @macro)
    ({$lh:ty}, {$rh:ty}, @$macro:ident) => {
        all_combos!($lh, $rh, @$macro);
    };
    // Matches a plain lh and one+ rh: all_combos!(lh, {rh1, rh2, ...}, @macro)
    ($lh:ty, {$first_rh:ty, $($rest_rh:ty),*}, @$macro:ident) => {
        all_combos!($lh, $first_rh, @$macro);
        all_combos!($lh, {$($rest_rh),*}, @$macro);
    };
    // Matches a single, wrapped lh and one+ rh: all_combos!({lh}, {rh1, ...}, @macro)
    ({$lh:ty}, {$($rh:ty),*}, @$macro:ident) => {
        all_combos!($lh, {$($rh),*}, @$macro)
    };
    // Matches one+ lh and one+ rh: all-combos!({lh1, lh2, ...}, {rh, ...}, @macro)
    ({$first_lh:ty, $($rest_lh:ty),*}, {$($rh:ty),*}, @$macro:ident) => {
        // Expand to the first lh and all of the rh
        all_combos!($first_lh, {$($rh),*}, @$macro);
        // Expand to the rest of the lh and all of the rh
        all_combos!({$($rest_lh),*}, {$($rh),*}, @$macro);
    };
    // Matches one+ lh and a plain rh: all-combos!({lh1, lh2, ...}, rh, @macro)
    ({$first_lh:ty, $($rest_lh:ty),*}, $rh:ty, @$macro:ident) => {
        // Expand to the first lh and the rh
        all_combos!($first_lh, $rh, @$macro);
        // Expand to the rest of the lh and the rh
        all_combos!({$($rest_lh),*}, $rh, @$macro);
    };
}

// impl PartialEq<rh> and PartialOrd<rh> for <lh>
macro_rules! impl_ordering {
    ($lh:ty, $rh:ty) => {
        impl PartialEq<$rh> for $lh {
            fn eq(&self, other: &$rh) -> bool {
                self.0.eq(other)
            }
        }
        impl PartialOrd<$rh> for $lh {
            fn partial_cmp(&self, other: &$rh) -> Option<core::cmp::Ordering> {
                self.0.partial_cmp(other)
            }
        }
    };
}

all_combos!({u1, u2, u3, u4, u5, u6, u7}, u8, @impl_ordering);
all_combos!({u9, u10, u11, u12, u13, u14, u15}, u16, @impl_ordering);
all_combos!({u17, u18, u19, u20, u21, u22, u23}, u32, @impl_ordering);
all_combos!({u24, u25, u26, u27, u28, u29, u30, u31}, u32, @impl_ordering);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ordering() {
        assert!(u1::new(1) > 0u8);
        assert!(u10::new(10) > 0u16);
        assert!(u20::new(10) > 0u32);
    }
}
