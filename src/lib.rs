/// Marks a trait as delegatable.
///
/// Generates a Delegated<TraitName> trait of the following for:
/// ```
///  pub trait DelegatedExampleTrait {
///      type DelegateType;
///
///      fn delegate(self) -> Self::DelegateType;
///      fn delegate_ref(&self) -> &Self::DelegateType;
///      fn delegate_ref_mut(&mut self) -> &mut Self::DelegateType;
///  }
/// ```
///
/// Types implementing `DelegatedExampleTrait` will automatically implement `ExampleTrait`.
///
/// All item types within the delegated trait are supported, except for item-level macro invocations,
/// since they opaque to the macro.
pub use delegate_trait_proc_macros::delegated;

/// Delegates implementation of delegatable traits to structure's fields.
///
/// Delegatee fields are marked with `delegate` attribute. Delegating multiple traits
/// and to multiple fields is supported, as well as delegating to tuple struct fields.
///
/// # Examples
///
/// ```
/// use delegate_trait::*;
///
/// #[delegated]
/// pub trait ExampleTrait {
///     fn foo(&self) -> u32;
/// }
///
/// impl ExampleTrait for u32 {
///     fn foo(&self) -> u32 {
///         *self
///     }
/// }
///
/// #[derive(Delegating)]
/// pub struct ExampleStruct {
///     #[delegate(DelegatedExampleTrait)]
///     pub field1: u32,
///     pub field2: String
/// }
///
/// #[derive(Delegating)]
/// pub struct ExampleTuple(#[delegate(DelegatedExampleTrait)] pub u32);
///
/// assert_eq!(ExampleStruct {field1: 42, field2: "".into() }.foo(), 42);
/// assert_eq!(ExampleTuple(42).foo(), 42);
/// ```
pub use delegate_trait_proc_macros::Delegating;
