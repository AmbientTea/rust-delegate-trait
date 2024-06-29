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
    fn delegated_procedures() {
        let mut b = B { a: A };
        b.ref_procedure();
        b.ref_mut_procedure();
        b.value_procedure();
    }

    #[test]
    fn delegated_functions() {
        let mut b = B { a: A };
        assert_eq!(b.ref_function("abc"), "ref function: abc");
        assert_eq!(b.ref_mut_function("abc"), "ref mut function: abc");
        assert_eq!(b.value_function("abc"), "value function: abc");
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

mod generic_traits {
    use delegate_trait::*;

    #[delegated]
    pub trait TestTrait<T>
    where
        T: Copy,
    {
        const CONST: T;
        type InnerType: From<T>;

        fn extract_t(self) -> T;
        fn extract_t_ref(&self) -> &T;
        fn extract_t_ref_mut(&mut self) -> &mut T;

        fn static_procedure() -> T;
        fn static_function(str: T) -> T;
        fn value_procedure(self);
        fn value_function(self, t: T) -> T;
        fn ref_procedure(&self);
        fn ref_function(&self, t: T) -> T;
        fn ref_mut_procedure(&mut self);
        fn ref_mut_function(&mut self, t: T) -> T;
    }

    pub struct A {
        t: u32,
    }
    impl TestTrait<u32> for A {
        const CONST: u32 = 100;
        type InnerType = u64;

        fn extract_t(self) -> u32 {
            self.t
        }
        fn extract_t_ref(&self) -> &u32 {
            &self.t
        }
        fn extract_t_ref_mut(&mut self) -> &mut u32 {
            &mut self.t
        }

        fn static_procedure() -> u32 {
            42
        }
        fn static_function(t: u32) -> u32 {
            t
        }

        fn value_procedure(self) {}
        fn value_function(self, t: u32) -> u32 {
            t
        }
        fn ref_procedure(&self) {}
        fn ref_function(&self, t: u32) -> u32 {
            t
        }
        fn ref_mut_procedure(&mut self) {}
        fn ref_mut_function(&mut self, t: u32) -> u32 {
            t
        }
    }

    pub struct B {
        a: A,
    }
    delegate_to_field!(a: A as DelegatedTestTrait<u32> for B);

    #[test]
    fn generic_field_access() {
        let mut b = B { a: A { t: 42 } };

        assert_eq!(*b.extract_t_ref(), 42);
        assert_eq!(*b.extract_t_ref_mut(), 42);
        assert_eq!(b.extract_t(), 42);
    }

    #[test]
    fn delegated_procedures() {
        let mut b = B { a: A { t: 42 } };
        b.ref_procedure();
        b.ref_mut_procedure();
        b.value_procedure();
    }

    #[test]
    fn delegated_functions() {
        let mut b = B { a: A { t: 42 } };
        assert_eq!(b.ref_function(42), 42);
        assert_eq!(b.ref_mut_function(42), 42);
        assert_eq!(b.value_function(42), 42);
    }

    #[test]
    fn delegated_static_calls() {
        B::static_procedure();
        assert_eq!(B::static_function(42), 42);
    }

    #[test]
    fn delegated_constant() {
        assert_eq!(B::CONST, 100)
    }
}

mod self_as_argument {
    use delegate_trait::*;

    #[delegated]
    pub trait TestTrait {
        fn consume_self(s: Self);
        fn consume_self_ref(s: &Self);
        fn consume_self_ref_mut(s: &mut Self);
    }

    pub struct A;

    impl TestTrait for A {
        fn consume_self(_s: Self) {}
        fn consume_self_ref(_s: &Self) {}
        fn consume_self_ref_mut(_s: &mut Self) {}
    }

    pub struct B {
        a: A,
    }

    delegate_to_field!(a: A as DelegatedTestTrait for B);
}
