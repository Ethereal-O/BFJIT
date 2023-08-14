pub mod warn {
    use std::fmt;

    #[derive(Debug, thiserror::Error)]
    #[allow(dead_code)]
    pub enum RuntimeWarnKind {
        #[error("Parse input error, using default value: STDIN")]
        ParseInputWarn,
        #[error("Parse output error, using default value: STDOUT")]
        ParseOutputWarn,
    }

    #[derive(Debug)]
    pub struct RuntimeWarn {
        pub kind: RuntimeWarnKind,
    }

    impl fmt::Display for RuntimeWarn {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.kind)
        }
    }
}
