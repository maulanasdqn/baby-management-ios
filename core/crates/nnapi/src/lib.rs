pub mod sys;

use anyhow::{anyhow, Result};
use std::ffi::c_void;
use sys::*;

// ── Safe wrappers ────────────────────────────────────────────────────────────

pub struct NnapiModel(*mut ANeuralNetworksModel);
unsafe impl Send for NnapiModel {}

impl NnapiModel {
    pub fn compile(self, preference: i32) -> Result<NnapiCompilation> {
        let mut comp = std::ptr::null_mut();
        nnapi_check(unsafe { ANeuralNetworksCompilation_create(self.0, &mut comp) }, "Compilation_create")?;
        nnapi_check(unsafe { ANeuralNetworksCompilation_setPreference(comp, preference) }, "setPreference")?;
        nnapi_check(unsafe { ANeuralNetworksCompilation_finish(comp) }, "Compilation_finish")?;
        Ok(NnapiCompilation(comp))
    }
}

impl Drop for NnapiModel {
    fn drop(&mut self) { unsafe { ANeuralNetworksModel_free(self.0) } }
}

pub struct NnapiCompilation(*mut ANeuralNetworksCompilation);
unsafe impl Send for NnapiCompilation {}
unsafe impl Sync for NnapiCompilation {}

impl NnapiCompilation {
    pub fn create_execution(&self) -> Result<NnapiExecution> {
        let mut exec = std::ptr::null_mut();
        nnapi_check(unsafe { ANeuralNetworksExecution_create(self.0, &mut exec) }, "Execution_create")?;
        Ok(NnapiExecution(exec))
    }
}

impl Drop for NnapiCompilation {
    fn drop(&mut self) { unsafe { ANeuralNetworksCompilation_free(self.0) } }
}

pub struct NnapiExecution(*mut ANeuralNetworksExecution);

impl NnapiExecution {
    pub fn set_input_i32(&self, index: i32, data: &[i32]) -> Result<()> {
        nnapi_check(unsafe {
            ANeuralNetworksExecution_setInput(
                self.0, index, std::ptr::null(),
                data.as_ptr() as *const c_void,
                data.len() * 4,
            )
        }, "setInput")
    }

    pub fn set_output_f32(&self, index: i32, data: &mut [f32]) -> Result<()> {
        nnapi_check(unsafe {
            ANeuralNetworksExecution_setOutput(
                self.0, index, std::ptr::null(),
                data.as_mut_ptr() as *mut c_void,
                data.len() * 4,
            )
        }, "setOutput")
    }

    pub fn compute(self) -> Result<()> {
        nnapi_check(unsafe { ANeuralNetworksExecution_compute(self.0) }, "compute")
    }
}

impl Drop for NnapiExecution {
    fn drop(&mut self) { unsafe { ANeuralNetworksExecution_free(self.0) } }
}

// ── GraphBuilder ─────────────────────────────────────────────────────────────

pub struct GraphBuilder {
    model: *mut ANeuralNetworksModel,
    next_idx: u32,
    _bufs: Vec<Vec<u8>>, // keep constant data alive until finish()
}

unsafe impl Send for GraphBuilder {}

impl GraphBuilder {
    pub fn new() -> Result<Self> {
        let mut model = std::ptr::null_mut();
        nnapi_check(unsafe { ANeuralNetworksModel_create(&mut model) }, "Model_create")?;
        Ok(Self { model, next_idx: 0, _bufs: Vec::new() })
    }

    // ── Operand allocation ───────────────────────────────────────────────────

    fn alloc(&mut self, type_: i32, dims: &[u32]) -> Result<u32> {
        let t = ANeuralNetworksOperandType {
            type_,
            dimension_count: dims.len() as u32,
            dimensions: if dims.is_empty() { std::ptr::null() } else { dims.as_ptr() },
            scale: 0.0,
            zero_point: 0,
        };
        nnapi_check(unsafe { ANeuralNetworksModel_addOperand(self.model, &t) }, "addOperand")?;
        let idx = self.next_idx;
        self.next_idx += 1;
        Ok(idx)
    }

    pub fn f32_tensor(&mut self, dims: &[u32]) -> Result<u32> {
        self.alloc(ANEURALNETWORKS_TENSOR_FLOAT32, dims)
    }
    pub fn i32_tensor(&mut self, dims: &[u32]) -> Result<u32> {
        self.alloc(ANEURALNETWORKS_TENSOR_INT32, dims)
    }
    pub fn scalar_i32(&mut self) -> Result<u32> { self.alloc(ANEURALNETWORKS_INT32, &[]) }
    pub fn scalar_f32(&mut self) -> Result<u32> { self.alloc(ANEURALNETWORKS_FLOAT32, &[]) }
    pub fn scalar_bool(&mut self) -> Result<u32> { self.alloc(ANEURALNETWORKS_BOOL, &[]) }

    // ── Constant setters ─────────────────────────────────────────────────────

    fn set_value(&mut self, idx: u32, bytes: Vec<u8>) -> Result<()> {
        let len = bytes.len();
        let ptr = bytes.as_ptr() as *const c_void;
        nnapi_check(
            unsafe { ANeuralNetworksModel_setOperandValue(self.model, idx, ptr, len) },
            "setOperandValue",
        )?;
        self._bufs.push(bytes);
        Ok(())
    }

    pub fn set_f32_data(&mut self, idx: u32, data: Vec<f32>) -> Result<()> {
        let bytes: Vec<u8> = data.iter().flat_map(|f| f.to_le_bytes()).collect();
        self.set_value(idx, bytes)
    }

    pub fn set_i32_data(&mut self, idx: u32, data: &[i32]) -> Result<()> {
        let bytes: Vec<u8> = data.iter().flat_map(|v| v.to_le_bytes()).collect();
        self.set_value(idx, bytes)
    }

    pub fn set_bool_data(&mut self, idx: u32, value: bool) -> Result<()> {
        self.set_value(idx, vec![value as u8])
    }

    // ── Constant helpers ─────────────────────────────────────────────────────

    pub fn ci32(&mut self, v: i32) -> Result<u32> {
        let idx = self.scalar_i32()?;
        self.set_i32_data(idx, &[v])?;
        Ok(idx)
    }

    pub fn cf32(&mut self, v: f32) -> Result<u32> {
        let idx = self.scalar_f32()?;
        let bytes = v.to_le_bytes().to_vec();
        self.set_value(idx, bytes)?;
        Ok(idx)
    }

    pub fn cbool(&mut self, v: bool) -> Result<u32> {
        let idx = self.scalar_bool()?;
        self.set_bool_data(idx, v)?;
        Ok(idx)
    }

    pub fn const_f32_tensor(&mut self, dims: &[u32], data: Vec<f32>) -> Result<u32> {
        let idx = self.f32_tensor(dims)?;
        self.set_f32_data(idx, data)?;
        Ok(idx)
    }

    pub fn const_i32_tensor(&mut self, dims: &[u32], data: &[i32]) -> Result<u32> {
        let idx = self.i32_tensor(dims)?;
        self.set_i32_data(idx, data)?;
        Ok(idx)
    }

    // ── Operation builder ────────────────────────────────────────────────────

    fn op(&mut self, op: u32, ins: &[u32], outs: &[u32]) -> Result<()> {
        nnapi_check(unsafe {
            ANeuralNetworksModel_addOperation(
                self.model, op,
                ins.len() as u32, ins.as_ptr(),
                outs.len() as u32, outs.as_ptr(),
            )
        }, "addOperation")
    }

    // ── Arithmetic ───────────────────────────────────────────────────────────

    pub fn add(&mut self, a: u32, b: u32, out_dims: &[u32]) -> Result<u32> {
        let fused = self.ci32(ANEURALNETWORKS_FUSED_NONE)?;
        let out = self.f32_tensor(out_dims)?;
        self.op(ANEURALNETWORKS_ADD, &[a, b, fused], &[out])?;
        Ok(out)
    }

    pub fn sub(&mut self, a: u32, b: u32, out_dims: &[u32]) -> Result<u32> {
        let fused = self.ci32(ANEURALNETWORKS_FUSED_NONE)?;
        let out = self.f32_tensor(out_dims)?;
        self.op(ANEURALNETWORKS_SUB, &[a, b, fused], &[out])?;
        Ok(out)
    }

    pub fn mul(&mut self, a: u32, b: u32, out_dims: &[u32]) -> Result<u32> {
        let fused = self.ci32(ANEURALNETWORKS_FUSED_NONE)?;
        let out = self.f32_tensor(out_dims)?;
        self.op(ANEURALNETWORKS_MUL, &[a, b, fused], &[out])?;
        Ok(out)
    }

    pub fn div(&mut self, a: u32, b: u32, out_dims: &[u32]) -> Result<u32> {
        let fused = self.ci32(ANEURALNETWORKS_FUSED_NONE)?;
        let out = self.f32_tensor(out_dims)?;
        self.op(ANEURALNETWORKS_DIV, &[a, b, fused], &[out])?;
        Ok(out)
    }

    // ── Unary ────────────────────────────────────────────────────────────────

    pub fn sqrt(&mut self, a: u32, out_dims: &[u32]) -> Result<u32> {
        let out = self.f32_tensor(out_dims)?;
        self.op(ANEURALNETWORKS_SQRT, &[a], &[out])?;
        Ok(out)
    }

    pub fn tanh(&mut self, a: u32, out_dims: &[u32]) -> Result<u32> {
        let out = self.f32_tensor(out_dims)?;
        self.op(ANEURALNETWORKS_TANH, &[a], &[out])?;
        Ok(out)
    }

    // ── Reduction ────────────────────────────────────────────────────────────

    pub fn mean(&mut self, a: u32, axes: &[i32], keepdims: bool, out_dims: &[u32]) -> Result<u32> {
        let axes_t = self.const_i32_tensor(&[axes.len() as u32], axes)?;
        let kd = self.ci32(if keepdims { 1 } else { 0 })?;
        let out = self.f32_tensor(out_dims)?;
        self.op(ANEURALNETWORKS_MEAN, &[a, axes_t, kd], &[out])?;
        Ok(out)
    }

    // ── Shape ops ────────────────────────────────────────────────────────────

    pub fn reshape(&mut self, a: u32, shape: &[i32], out_dims: &[u32]) -> Result<u32> {
        let shape_t = self.const_i32_tensor(&[shape.len() as u32], shape)?;
        let out = self.f32_tensor(out_dims)?;
        self.op(ANEURALNETWORKS_RESHAPE, &[a, shape_t], &[out])?;
        Ok(out)
    }

    pub fn transpose(&mut self, a: u32, perm: &[i32], out_dims: &[u32]) -> Result<u32> {
        let perm_t = self.const_i32_tensor(&[perm.len() as u32], perm)?;
        let out = self.f32_tensor(out_dims)?;
        self.op(ANEURALNETWORKS_TRANSPOSE, &[a, perm_t], &[out])?;
        Ok(out)
    }

    pub fn gather(&mut self, table: u32, axis: i32, indices: u32, out_dims: &[u32]) -> Result<u32> {
        let axis_s = self.ci32(axis)?;
        let out = self.f32_tensor(out_dims)?;
        self.op(ANEURALNETWORKS_GATHER, &[table, axis_s, indices], &[out])?;
        Ok(out)
    }

    // ── Linear algebra ───────────────────────────────────────────────────────

    /// weights must be [out_size, in_size] (NNAPI convention: transposed vs GPT-2)
    pub fn fully_connected(&mut self, input: u32, weights: u32, bias: u32, out_dims: &[u32]) -> Result<u32> {
        let fused = self.ci32(ANEURALNETWORKS_FUSED_NONE)?;
        let out = self.f32_tensor(out_dims)?;
        self.op(ANEURALNETWORKS_FULLY_CONNECTED, &[input, weights, bias, fused], &[out])?;
        Ok(out)
    }

    pub fn batch_matmul(&mut self, a: u32, b: u32, adj_x: bool, adj_y: bool, out_dims: &[u32]) -> Result<u32> {
        let ax = self.cbool(adj_x)?;
        let ay = self.cbool(adj_y)?;
        let out = self.f32_tensor(out_dims)?;
        self.op(ANEURALNETWORKS_BATCH_MATMUL, &[a, b, ax, ay], &[out])?;
        Ok(out)
    }

    pub fn softmax(&mut self, a: u32, beta: f32, axis: i32, out_dims: &[u32]) -> Result<u32> {
        let beta_s = self.cf32(beta)?;
        let axis_s = self.ci32(axis)?;
        let out = self.f32_tensor(out_dims)?;
        self.op(ANEURALNETWORKS_SOFTMAX, &[a, beta_s, axis_s], &[out])?;
        Ok(out)
    }

    // ── Finalize ─────────────────────────────────────────────────────────────

    pub fn finish(self, model_inputs: &[u32], model_outputs: &[u32]) -> Result<NnapiModel> {
        nnapi_check(unsafe {
            ANeuralNetworksModel_identifyInputsAndOutputs(
                self.model,
                model_inputs.len() as u32, model_inputs.as_ptr(),
                model_outputs.len() as u32, model_outputs.as_ptr(),
            )
        }, "identifyInputsAndOutputs")?;
        nnapi_check(unsafe { ANeuralNetworksModel_finish(self.model) }, "Model_finish")?;
        // Transfer ownership; _bufs is dropped here but the model has already copied the data
        Ok(NnapiModel(self.model))
    }
}

// ── Utilities ────────────────────────────────────────────────────────────────

fn nnapi_check(ret: i32, ctx: &str) -> Result<()> {
    if ret == 0 { Ok(()) } else { Err(anyhow!("NNAPI {ctx} returned {ret}")) }
}

/// Transpose a [rows × cols] row-major matrix → [cols × rows]
/// Needed because GPT-2 weights are [in, out] but FULLY_CONNECTED wants [out, in].
pub fn transpose_2d(data: &[f32], rows: usize, cols: usize) -> Vec<f32> {
    let mut out = vec![0.0f32; rows * cols];
    for r in 0..rows {
        for c in 0..cols {
            out[c * rows + r] = data[r * cols + c];
        }
    }
    out
}
