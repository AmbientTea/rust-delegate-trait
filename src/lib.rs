pub use delegate_trait_proc_macros::delegated;

#[macro_export]
macro_rules! delegate_to_field {
    ($field:ident: $fty:ty as $tr:ident for $struct:ty) => {
        impl $tr for $struct {
            type DelegateType = $fty;
            fn delegate(self) -> $fty {
                self.$field
            }
            fn delegate_ref(&self) -> &$fty {
                &self.$field
            }
            fn delegate_ref_mut(&mut self) -> &mut $fty {
                &mut self.$field
            }
        }
    };
}
