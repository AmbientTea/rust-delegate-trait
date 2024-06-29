mod base_case {
    use delegate_trait::*;

    #[delegated]
    pub trait TestTrait {
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

    delegate_to_field!(a: A as TestTrait for B);

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
}
