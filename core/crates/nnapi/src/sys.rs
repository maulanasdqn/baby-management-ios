#![allow(non_camel_case_types, dead_code)]

use std::ffi::c_void;

// Opaque handle types
pub enum ANeuralNetworksModel {}
pub enum ANeuralNetworksCompilation {}
pub enum ANeuralNetworksExecution {}

// Scalar operand type codes
pub const ANEURALNETWORKS_FLOAT32: i32 = 0;
pub const ANEURALNETWORKS_INT32: i32 = 1;
pub const ANEURALNETWORKS_BOOL: i32 = 6; // API 29

// Tensor operand type codes
pub const ANEURALNETWORKS_TENSOR_FLOAT32: i32 = 3;
pub const ANEURALNETWORKS_TENSOR_INT32: i32 = 4;

// Operation codes (API level noted)
pub const ANEURALNETWORKS_ADD: u32 = 0;           // API 27
pub const ANEURALNETWORKS_DIV: u32 = 30;          // API 27
pub const ANEURALNETWORKS_FULLY_CONNECTED: u32 = 9; // API 27
pub const ANEURALNETWORKS_MEAN: u32 = 31;         // API 27
pub const ANEURALNETWORKS_MUL: u32 = 18;          // API 27
pub const ANEURALNETWORKS_RESHAPE: u32 = 22;      // API 27
pub const ANEURALNETWORKS_SOFTMAX: u32 = 25;      // API 27
pub const ANEURALNETWORKS_SUB: u32 = 36;          // API 27
pub const ANEURALNETWORKS_TANH: u32 = 28;         // API 27
pub const ANEURALNETWORKS_TRANSPOSE: u32 = 37;    // API 27
pub const ANEURALNETWORKS_GATHER: u32 = 51;       // API 29
pub const ANEURALNETWORKS_SQRT: u32 = 88;         // API 29
pub const ANEURALNETWORKS_BATCH_MATMUL: u32 = 102; // API 31

// Fused activation
pub const ANEURALNETWORKS_FUSED_NONE: i32 = 0;

// Compilation preferences
pub const ANEURALNETWORKS_PREFER_SUSTAINED_SPEED: i32 = 2;

#[repr(C)]
pub struct ANeuralNetworksOperandType {
    pub type_: i32,
    pub dimension_count: u32,
    pub dimensions: *const u32,
    pub scale: f32,
    pub zero_point: i32,
}

unsafe impl Send for ANeuralNetworksOperandType {}
unsafe impl Sync for ANeuralNetworksOperandType {}

extern "C" {
    pub fn ANeuralNetworksModel_create(model: *mut *mut ANeuralNetworksModel) -> i32;
    pub fn ANeuralNetworksModel_free(model: *mut ANeuralNetworksModel);
    pub fn ANeuralNetworksModel_addOperand(
        model: *mut ANeuralNetworksModel,
        type_: *const ANeuralNetworksOperandType,
    ) -> i32;
    pub fn ANeuralNetworksModel_setOperandValue(
        model: *mut ANeuralNetworksModel,
        index: u32,
        buffer: *const c_void,
        length: usize,
    ) -> i32;
    pub fn ANeuralNetworksModel_addOperation(
        model: *mut ANeuralNetworksModel,
        type_: u32,
        input_count: u32,
        inputs: *const u32,
        output_count: u32,
        outputs: *const u32,
    ) -> i32;
    pub fn ANeuralNetworksModel_identifyInputsAndOutputs(
        model: *mut ANeuralNetworksModel,
        input_count: u32,
        inputs: *const u32,
        output_count: u32,
        outputs: *const u32,
    ) -> i32;
    pub fn ANeuralNetworksModel_finish(model: *mut ANeuralNetworksModel) -> i32;

    pub fn ANeuralNetworksCompilation_create(
        model: *mut ANeuralNetworksModel,
        compilation: *mut *mut ANeuralNetworksCompilation,
    ) -> i32;
    pub fn ANeuralNetworksCompilation_free(compilation: *mut ANeuralNetworksCompilation);
    pub fn ANeuralNetworksCompilation_setPreference(
        compilation: *mut ANeuralNetworksCompilation,
        preference: i32,
    ) -> i32;
    pub fn ANeuralNetworksCompilation_finish(compilation: *mut ANeuralNetworksCompilation) -> i32;

    pub fn ANeuralNetworksExecution_create(
        compilation: *mut ANeuralNetworksCompilation,
        execution: *mut *mut ANeuralNetworksExecution,
    ) -> i32;
    pub fn ANeuralNetworksExecution_free(execution: *mut ANeuralNetworksExecution);
    pub fn ANeuralNetworksExecution_setInput(
        execution: *mut ANeuralNetworksExecution,
        index: i32,
        type_: *const ANeuralNetworksOperandType,
        buffer: *const c_void,
        length: usize,
    ) -> i32;
    pub fn ANeuralNetworksExecution_setOutput(
        execution: *mut ANeuralNetworksExecution,
        index: i32,
        type_: *const ANeuralNetworksOperandType,
        buffer: *mut c_void,
        length: usize,
    ) -> i32;
    pub fn ANeuralNetworksExecution_compute(execution: *mut ANeuralNetworksExecution) -> i32;
}
