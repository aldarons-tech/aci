#![allow(
    unreachable_code, dead_code, unused_assignments, unused_mut, unused_variables, non_snake_case,
    non_upper_case_globals, unconditional_recursion, path_statements
)]

pub const PAGE_SIZE: usize = 64 << 10;

pub trait Imports {
    type Memory: Memory;
    fn enlargeMemory(&mut self, context: &mut Context<Self::Memory>) -> i32;
    fn getTotalMemory(&mut self, context: &mut Context<Self::Memory>) -> i32;
    fn abortOnCannotGrowMemory(&mut self, context: &mut Context<Self::Memory>) -> i32;
    fn abortStackOverflow(&mut self, context: &mut Context<Self::Memory>, var0: i32);
    fn nullFunc_ii(&mut self, context: &mut Context<Self::Memory>, var0: i32);
    fn nullFunc_iiii(&mut self, context: &mut Context<Self::Memory>, var0: i32);
    fn ____lock(&mut self, context: &mut Context<Self::Memory>, var0: i32);
    fn ____setErrNo(&mut self, context: &mut Context<Self::Memory>, var0: i32);
    fn ____syscall140(&mut self, context: &mut Context<Self::Memory>, var0: i32, var1: i32) -> i32;
    fn ____syscall146(&mut self, context: &mut Context<Self::Memory>, var0: i32, var1: i32) -> i32;
    fn ____syscall54(&mut self, context: &mut Context<Self::Memory>, var0: i32, var1: i32) -> i32;
    fn ____syscall6(&mut self, context: &mut Context<Self::Memory>, var0: i32, var1: i32) -> i32;
    fn ____unlock(&mut self, context: &mut Context<Self::Memory>, var0: i32);
    fn __emscripten_memcpy_big(&mut self, context: &mut Context<Self::Memory>, var0: i32, var1: i32, var2: i32) -> i32;
    fn memoryBase(&mut self, context: &mut Context<Self::Memory>) -> &mut i32;
    fn tableBase(&mut self, context: &mut Context<Self::Memory>) -> &mut i32;
    fn DYNAMICTOP_PTR(&mut self, context: &mut Context<Self::Memory>) -> &mut i32;
    fn tempDoublePtr(&mut self, context: &mut Context<Self::Memory>) -> &mut i32;
    fn ABORT(&mut self, context: &mut Context<Self::Memory>) -> &mut i32;
    fn STACKTOP(&mut self, context: &mut Context<Self::Memory>) -> &mut i32;
    fn STACK_MAX(&mut self, context: &mut Context<Self::Memory>) -> &mut i32;
    fn NaN(&mut self, context: &mut Context<Self::Memory>) -> &mut f64;
    fn Infinity(&mut self, context: &mut Context<Self::Memory>) -> &mut f64;
}

pub trait Memory {
    fn load8(&mut self, addr: usize) -> u8;
    fn load16(&mut self, addr: usize) -> u16;
    fn load32(&mut self, addr: usize) -> u32;
    fn load64(&mut self, addr: usize) -> u64;

    fn store8(&mut self, addr: usize, val: u8);
    fn store16(&mut self, addr: usize, val: u16);
    fn store32(&mut self, addr: usize, val: u32);
    fn store64(&mut self, addr: usize, val: u64);

    fn store_slice(&mut self, addr: usize, val: &'static [u8]);

    fn grow(&mut self, pages: usize) -> i32;
    fn size(&mut self) -> i32;
}

pub struct Instance<I: Imports<Memory = M>, M: Memory> {
    pub imports: I,
    pub context: Context<M>,
}

pub struct Context<M: Memory> {
    pub memory: M,
    global9: i32,
    global10: i32,
    global11: i32,
    global12: i32,
    global13: i32,
    global14: i32,
    global15: i32,
    global16: i32,
    global17: i32,
    global18: f64,
    global19: f64,
    global20: i32,
    global21: i32,
    global22: i32,
    global23: i32,
    global24: f64,
    global25: i32,
    global26: f32,
    global27: f32,
}

impl<I: Imports<Memory = M>, M: Memory> Instance<I, M> {
    pub fn new(imports: I, mut memory: M) -> Self {
        memory.store_slice(1024, b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x88\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\n\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0");
        let mut instance = Self {
            imports,
            context: Context {
                memory,
                global9: Default::default(),
                global10: Default::default(),
                global11: Default::default(),
                global12: Default::default(),
                global13: Default::default(),
                global14: 0,
                global15: 0,
                global16: 0,
                global17: 0,
                global18: Default::default(),
                global19: Default::default(),
                global20: 0,
                global21: 0,
                global22: 0,
                global23: 0,
                global24: 0,
                global25: 0,
                global26: 0,
                global27: 0,
            },
        };
        instance.context.init_global_values(&mut instance.imports);
        instance
    }
    pub fn ____errno_location(&mut self) -> i32 {
        self.context.func26(&mut self.imports)
    }
    pub fn __fflush(&mut self, var0: i32) -> i32 {
        self.context.func34(&mut self.imports, var0)
    }
    pub fn __free(&mut self, var0: i32) {
        self.context.func22(&mut self.imports, var0)
    }
    pub fn __malloc(&mut self, var0: i32) -> i32 {
        self.context.func21(&mut self.imports, var0)
    }
    pub fn __memcpy(&mut self, var0: i32, var1: i32, var2: i32) -> i32 {
        self.context.func37(&mut self.imports, var0, var1, var2)
    }
    pub fn __memset(&mut self, var0: i32, var1: i32, var2: i32) -> i32 {
        self.context.func38(&mut self.imports, var0, var1, var2)
    }
    pub fn __sbrk(&mut self, var0: i32) -> i32 {
        self.context.func39(&mut self.imports, var0)
    }
    pub fn dynCall_ii(&mut self, var0: i32, var1: i32) -> i32 {
        self.context.func40(&mut self.imports, var0, var1)
    }
    pub fn dynCall_iiii(&mut self, var0: i32, var1: i32, var2: i32, var3: i32) -> i32 {
        self.context.func41(&mut self.imports, var0, var1, var2, var3)
    }
    pub fn establishStackSpace(&mut self, var0: i32, var1: i32) {
        self.context.func17(&mut self.imports, var0, var1)
    }
    pub fn getTempRet0(&mut self) -> i32 {
        self.context.func20(&mut self.imports)
    }
    pub fn runPostSets(&mut self) {
        self.context.func36(&mut self.imports)
    }
    pub fn setTempRet0(&mut self, var0: i32) {
        self.context.func19(&mut self.imports, var0)
    }
    pub fn setThrew(&mut self, var0: i32, var1: i32) {
        self.context.func18(&mut self.imports, var0, var1)
    }
    pub fn stackAlloc(&mut self, var0: i32) -> i32 {
        self.context.func14(&mut self.imports, var0)
    }
    pub fn stackRestore(&mut self, var0: i32) {
        self.context.func16(&mut self.imports, var0)
    }
    pub fn stackSave(&mut self) -> i32 {
        self.context.func15(&mut self.imports)
    }
