pub mod error {
    use std::fmt;

    #[derive(Debug, thiserror::Error)]
    pub enum CompileErrorKind {
        #[error("Unclosed left bracket")]
        UnclosedLeftBracket,
        #[error("Unexpected right bracket")]
        UnexpectedRightBracket,
    }

    #[derive(Debug)]
    pub struct CompileError {
        pub line: u32,
        pub col: u32,
        pub kind: CompileErrorKind,
    }

    #[derive(Debug, thiserror::Error)]
    pub enum RuntimeErrorKind {
        #[error("IO Error")]
        IO,
        #[error("Out of range")]
        OutOfRange,
        #[error("Memory error")]
        Memory,
        #[error("Unknown error")]
        Unknown,
    }

    #[derive(Debug)]
    pub struct RuntimeError {
        pub index: usize,
        pub kind: RuntimeErrorKind,
    }

    impl fmt::Display for CompileError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{} at line {}:{}", self.kind, self.line, self.col)
        }
    }

    impl fmt::Display for RuntimeError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{} at index {}", self.kind, self.index)
        }
    }
}
