/// Minimum Lua stack available to a C function.
const LUA_MINSTACK: i32 = 20;

/// Mark for precompiled code ('<esc>Lua').
const LUA_SIGNATURE: &'static str = "\x1bLua";

/// Thread error.
enum ThreadError {
    RunError,
    GCMMError,
    OtherError,
    SyntaxError,
    MemoryError,
}

/// Thread status.
enum ThreadStatus {
    Ok,
    Yielded,
    Err(ThreadError),
}

/// Basic types.
enum BasicType {
    TNone,
    TNil,
    TBoolean,
    TLightUserdata,
    TNumber,
    TString,
    TTable,
    TFunction,
    TUserdata,
    TThread,
    TNumTags,
}

/// Bitwise operations.
enum BitwiseOp {
    And,
    Or,
    Xor,
    Not,
}

/// Arithmetic operations.
enum ArithmeticOp {
    Add,
    Sub,
    Mul,
    Mod,
    Pow,
    Div,
    IDiv,
    Shl,
    Shr,
    Unm,
    BitwiseOp(BitwiseOp),
}

/// Logical operations (comparisons).
enum LogicalOp {
    Equal,
    LessThan,
    LessThanOrEqual,
}

/// Operations.
enum LuaOperation {
    Logical(LogicalOp),
    Arithmetic(ArithmeticOp),
}

/// Garbage-Collection functions and options.
enum GCFunc {
    Stop,
    Restart,
    Collect,
    Count,
    CountB,
    Step,
    SetPause,
    SetStepMul,
    IsRunning,
}

/// Event codes.
enum EventCode {
    HookCall,
    HookRet,
    HookLine,
    HookCount,
    HookTailCall,
}

/// Event masks.
enum EventMask {
    MaskCall,
    MaskRet,
    MaskLine,
    MaskCount,
}
