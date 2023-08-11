pub mod parser {
    use crate::bftype::bferror;
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum TOKEN {
        Increment, // +
        Decrement, // -
        MoveLeft,  // <
        MoveRight, // >
        Input,     // ,
        Output,    // .
        LeftLoop,  // [
        RightLoop, // ]
    }

    pub fn parse(str: &str) -> Result<Vec<TOKEN>, bferror::error::CompileError> {
        let mut tokens = vec![];
        let mut stack: Vec<(u32, u32, u32)> = vec![];
        let mut line = 1;
        let mut col = 0;
        for c in str.chars() {
            match c {
                '+' => tokens.push(TOKEN::Increment),
                '-' => tokens.push(TOKEN::Decrement),
                '<' => tokens.push(TOKEN::MoveLeft),
                '>' => tokens.push(TOKEN::MoveRight),
                ',' => tokens.push(TOKEN::Input),
                '.' => tokens.push(TOKEN::Output),
                '[' => {
                    stack.push((line, col, tokens.len() as u32));
                    tokens.push(TOKEN::LeftLoop);
                }
                ']' => {
                    stack.pop().ok_or(bferror::error::CompileError {
                        line,
                        col,
                        kind: bferror::error::CompileErrorKind::UnexpectedRightBracket,
                    })?;
                    tokens.push(TOKEN::RightLoop);
                }
                '\n' => {
                    line += 1;
                    col = 0;
                }
                _ => (),
            }
            col += 1;
        }
        if let Some((line, col, _)) = stack.pop() {
            return Err(bferror::error::CompileError {
                line,
                col,
                kind: bferror::error::CompileErrorKind::UnclosedLeftBracket,
            });
        }
        return Ok(tokens);
    }
}

pub mod ir {
    use crate::bftype::bferror;
    use crate::bfparser::frontend::parser::TOKEN;
    use std::cell::Ref;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum BFIR {
        Add(u8),                  // + (u8)
        Sub(u8),                  // - (u8)
        MoveLeft(u32),            // < (u32)
        MoveRight(u32),           // > (u32)
        Input,                    // ,
        Output,                   // .
        Loop(RefCell<Vec<BFIR>>), // [ (Vec<BFIR>)]
    }

    pub struct IRStruct {
        tokens: RefCell<Vec<TOKEN>>,
        tmp_results: RefCell<Vec<RefCell<Vec<BFIR>>>>,
        result: RefCell<Vec<BFIR>>,
    }

    trait IRInterface {
        fn new(tokens: &Vec<TOKEN>) -> Self;
        fn init(&self, tokens: &Vec<TOKEN>);
        fn clear(&self, clear_result: bool, clear_tokens: bool);
        fn get_tokens(&self) -> Ref<'_, Vec<TOKEN>>;
        fn get_result(&self) -> Ref<'_, Vec<BFIR>>;
        fn stack_start(&self);
        fn stack_end(&self) -> Result<(), bferror::error::RuntimeError>;
        fn tmp_push(&self, ir: BFIR) -> Result<(), bferror::error::RuntimeError>;
    }

    impl IRInterface for IRStruct {
        fn new(tokens: &Vec<TOKEN>) -> Self {
            IRStruct {
                tokens: RefCell::new(tokens.clone()),
                tmp_results: RefCell::new(vec![]),
                result: RefCell::new(vec![]),
            }
        }

        fn init(&self, tokens: &Vec<TOKEN>) {
            self.clear(true, true);
            tokens
                .iter()
                .for_each(|x| self.tokens.borrow_mut().push(x.clone()));
        }

        fn clear(&self, clear_result: bool, clear_tokens: bool) {
            if clear_tokens {
                self.tokens.borrow_mut().clear();
            }
            if clear_result {
                self.result.borrow_mut().clear();
            }
            self.tmp_results.borrow_mut().clear();
            self.tokens.borrow_mut().shrink_to_fit();
            self.result.borrow_mut().shrink_to_fit();
            self.tmp_results.borrow_mut().shrink_to_fit();
        }

        fn get_tokens(&self) -> Ref<'_, Vec<TOKEN>> {
            return self.tokens.borrow();
        }

        fn get_result(&self) -> Ref<'_, Vec<BFIR>> {
            return self.result.borrow();
        }

        fn stack_start(&self) {
            self.tmp_results.borrow_mut().push(RefCell::new(vec![]));
        }

        fn stack_end(&self) -> Result<(), bferror::error::RuntimeError> {
            let last = self
                .tmp_results
                .borrow_mut()
                .pop()
                .ok_or(bferror::error::RuntimeError {
                    index: 0,
                    kind: bferror::error::RuntimeErrorKind::OutOfRange,
                })?;
            let len = self.tmp_results.borrow().len();
            if len == 0 {
                self.result.borrow_mut().push(BFIR::Loop(last));
                self.clear(false, true);
                return Ok(());
            }
            self.tmp_results
                .borrow_mut()
                .get_mut(len - 1)
                .unwrap()
                .borrow_mut()
                .push(BFIR::Loop(last));
            return Ok(());
        }

        fn tmp_push(&self, ir: BFIR) -> Result<(), bferror::error::RuntimeError> {
            let len = self.tmp_results.borrow().len();
            if len == 0 {
                return Err(bferror::error::RuntimeError {
                    index: 0,
                    kind: bferror::error::RuntimeErrorKind::OutOfRange,
                });
            }
            self.tmp_results
                .borrow_mut()
                .get_mut(len - 1)
                .unwrap()
                .borrow_mut()
                .push(ir);
            return Ok(());
        }
    }

    fn reduce_duplicate_updown(
        mut index: usize,
        ir_struct: Rc<IRStruct>,
    ) -> Result<usize, bferror::error::RuntimeError> {
        let mut count: i8 = 0;
        let tokens = ir_struct.get_tokens();
        let len = tokens.len();
        while index < len {
            match tokens[index] {
                TOKEN::Increment => {
                    count += 1;
                    index += 1;
                }
                TOKEN::Decrement => {
                    count -= 1;
                    index += 1;
                }
                _ => break,
            }
        }
        if count > 0 {
            ir_struct.tmp_push(BFIR::Add(count as u8))?;
        } else if count < 0 {
            ir_struct.tmp_push(BFIR::Sub((-count) as u8))?;
        }
        return Ok(index);
    }

    fn reduce_duplicate_leftright(
        mut index: usize,
        ir_struct: Rc<IRStruct>,
    ) -> Result<usize, bferror::error::RuntimeError> {
        let mut count = 0;
        let tokens = ir_struct.get_tokens();
        let len = tokens.len();
        while index < len {
            match tokens[index] {
                TOKEN::MoveRight => {
                    count += 1;
                    index += 1;
                }
                TOKEN::MoveLeft => {
                    count -= 1;
                    index += 1;
                }
                _ => break,
            }
        }
        if count > 0 {
            ir_struct.tmp_push(BFIR::MoveRight(count as u32))?;
        } else if count < 0 {
            ir_struct.tmp_push(BFIR::MoveLeft((-count) as u32))?;
        }
        return Ok(index);
    }

    fn reduce_io(
        mut index: usize,
        ir_struct: Rc<IRStruct>,
    ) -> Result<usize, bferror::error::RuntimeError> {
        let tokens = ir_struct.get_tokens();
        let len = tokens.len();
        while index < len {
            match tokens[index] {
                TOKEN::Input => {
                    ir_struct.tmp_push(BFIR::Input)?;
                    index += 1;
                }
                TOKEN::Output => {
                    ir_struct.tmp_push(BFIR::Output)?;
                    index += 1;
                }
                _ => break,
            }
        }
        return Ok(index);
    }

    fn reduce_loop(
        mut index: usize,
        ir_struct: Rc<IRStruct>,
    ) -> Result<usize, bferror::error::RuntimeError> {
        let tokens = ir_struct.get_tokens();
        let len = tokens.len();
        index += 1; // jump over '['
        ir_struct.stack_start();
        while index < len {
            match tokens[index] {
                TOKEN::RightLoop => {
                    index += 1;
                    break;
                }
                _ => match normal(index, ir_struct.clone()) {
                    Ok(i) => index = i,
                    Err(e) => return Err(e),
                },
            }
        }
        ir_struct.stack_end()?;
        return Ok(index);
    }

    fn normal(
        mut index: usize,
        ir_struct: Rc<IRStruct>,
    ) -> Result<usize, bferror::error::RuntimeError> {
        match ir_struct.get_tokens()[index] {
            TOKEN::Increment => match reduce_duplicate_updown(index, ir_struct.clone()) {
                Ok(i) => index = i,
                Err(e) => return Err(e),
            },
            TOKEN::Decrement => match reduce_duplicate_updown(index, ir_struct.clone()) {
                Ok(i) => index = i,
                Err(e) => return Err(e),
            },
            TOKEN::MoveRight => match reduce_duplicate_leftright(index, ir_struct.clone()) {
                Ok(i) => index = i,
                Err(e) => return Err(e),
            },
            TOKEN::MoveLeft => match reduce_duplicate_leftright(index, ir_struct.clone()) {
                Ok(i) => index = i,
                Err(e) => return Err(e),
            },
            TOKEN::Input => match reduce_io(index, ir_struct.clone()) {
                Ok(i) => index = i,
                Err(e) => return Err(e),
            },
            TOKEN::Output => match reduce_io(index, ir_struct.clone()) {
                Ok(i) => index = i,
                Err(e) => return Err(e),
            },
            TOKEN::LeftLoop => match reduce_loop(index, ir_struct.clone()) {
                Ok(i) => index = i,
                Err(e) => return Err(e),
            },
            TOKEN::RightLoop => {
                return Err(bferror::error::RuntimeError {
                    index,
                    kind: bferror::error::RuntimeErrorKind::OutOfRange,
                });
            }
        }
        return Ok(index);
    }

    pub fn transfer_to_ir(tokens: &Vec<TOKEN>) -> Result<Vec<BFIR>, bferror::error::RuntimeError> {
        let ir_struct = Rc::new(IRStruct::new(tokens));
        let mut index = 0;
        let len = tokens.len();
        ir_struct.stack_start();
        while index < len {
            match normal(index, ir_struct.clone()) {
                Ok(i) => index = i,
                Err(e) => return Err(e),
            }
        }
        ir_struct.stack_end()?;
        return Ok(ir_struct.get_result().clone());
    }
}
