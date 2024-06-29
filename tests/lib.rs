mod base_case {
    use delegate_trait::*;

    #[delegated]
    pub trait TestTrait {
        type InnerType: Copy;
        const CONST: Self::InnerType;

        fn static_procedure();
        fn static_function(str: String) -> String;

        fn value_procedure(self);
        fn value_function(self, str: &str) -> String;
        fn ref_procedure(&self);
        fn ref_function(&self, str: &str) -> String;
        fn ref_mut_procedure(&mut self);
        fn ref_mut_function(&mut self, str: &str) -> String;
    }

    #[derive(Clone)]
    pub struct A;

    impl TestTrait for A {
        type InnerType = u32;
        const CONST: Self::InnerType = 42;

        fn static_procedure() {}
        fn static_function(str: String) -> String {
            format!("Hello: {str}")
        }

        fn value_procedure(self) {}
        fn value_function(self, str: &str) -> String {
            format!("value function: {str}")
        }
        fn ref_procedure(&self) {}
        fn ref_function(&self, str: &str) -> String {
            format!("ref function: {str}")
        }
        fn ref_mut_procedure(&mut self) {}
        fn ref_mut_function(&mut self, str: &str) -> String {
            format!("ref mut function: {str}")
        }
    }

    #[derive(Clone)]
    pub struct B {
        a: A,
    }

    delegate_to_field!(a: A as DelegatedTestTrait for B);

    #[test]
    fn delegated_calls_work() {
        let mut b = B { a: A };
        b.clone().value_procedure();
        b.ref_procedure();
        b.ref_mut_procedure();
        assert_eq!(b.clone().value_function("abc"), "value function: abc");
        assert_eq!(b.ref_function("abc"), "ref function: abc");
        assert_eq!(b.ref_mut_function("abc"), "ref mut function: abc");
    }

    #[test]
    fn delegated_static_calls() {
        B::static_procedure();
        assert_eq!(B::static_function("abc".into()), "Hello: abc");
    }

    #[test]
    fn delegated_constant() {
        assert_eq!(B::CONST, 42)
    }
}
