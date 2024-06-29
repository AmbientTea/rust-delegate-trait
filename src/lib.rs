pub trait Delegated<T: ?Sized> {
    type DelegateType;

    fn delegate(self) -> Self::DelegateType;
    fn delegate_ref(&self) -> &Self::DelegateType;
    fn delegate_ref_mut(&mut self) -> &mut Self::DelegateType;
}

#[cfg(feature = "macros")]
pub use delegate_trait_proc_macros::delegated;

#[cfg(feature = "macros")]
#[macro_export]
macro_rules! delegate_to_field {
    ($field:ident: $fty:ty as $tr:ident for $struct:ty) => {
        impl Delegated<dyn $tr> for $struct {
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
