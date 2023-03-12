
#[cfg(ocvrs_has_module_calib3d)]
mod calib3d_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfLMSolver = core::Ptr<dyn crate::calib3d::LMSolver>;
	
	ptr_extern! { dyn crate::calib3d::LMSolver,
		cv_PtrOfLMSolver_delete, cv_PtrOfLMSolver_get_inner_ptr, cv_PtrOfLMSolver_get_inner_ptr_mut
	}
	
	impl core::Ptr<dyn crate::calib3d::LMSolver> {
		#[inline] pub fn as_raw_PtrOfLMSolver(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfLMSolver(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::calib3d::LMSolverConst for core::Ptr<dyn crate::calib3d::LMSolver> {
		#[inline] fn as_raw_LMSolver(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::calib3d::LMSolver for core::Ptr<dyn crate::calib3d::LMSolver> {
		#[inline] fn as_raw_mut_LMSolver(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<dyn crate::calib3d::LMSolver> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<dyn crate::calib3d::LMSolver> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfLMSolver_Callback = core::Ptr<dyn crate::calib3d::LMSolver_Callback>;
	
	ptr_extern! { dyn crate::calib3d::LMSolver_Callback,
		cv_PtrOfLMSolver_Callback_delete, cv_PtrOfLMSolver_Callback_get_inner_ptr, cv_PtrOfLMSolver_Callback_get_inner_ptr_mut
	}
	
	impl core::Ptr<dyn crate::calib3d::LMSolver_Callback> {
		#[inline] pub fn as_raw_PtrOfLMSolver_Callback(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfLMSolver_Callback(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::calib3d::LMSolver_CallbackConst for core::Ptr<dyn crate::calib3d::LMSolver_Callback> {
		#[inline] fn as_raw_LMSolver_Callback(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::calib3d::LMSolver_Callback for core::Ptr<dyn crate::calib3d::LMSolver_Callback> {
		#[inline] fn as_raw_mut_LMSolver_Callback(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfStereoBM = core::Ptr<dyn crate::calib3d::StereoBM>;
	
	ptr_extern! { dyn crate::calib3d::StereoBM,
		cv_PtrOfStereoBM_delete, cv_PtrOfStereoBM_get_inner_ptr, cv_PtrOfStereoBM_get_inner_ptr_mut
	}
	
	impl core::Ptr<dyn crate::calib3d::StereoBM> {
		#[inline] pub fn as_raw_PtrOfStereoBM(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfStereoBM(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::calib3d::StereoBMConst for core::Ptr<dyn crate::calib3d::StereoBM> {
		#[inline] fn as_raw_StereoBM(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::calib3d::StereoBM for core::Ptr<dyn crate::calib3d::StereoBM> {
		#[inline] fn as_raw_mut_StereoBM(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<dyn crate::calib3d::StereoBM> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<dyn crate::calib3d::StereoBM> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::calib3d::StereoMatcherConst for core::Ptr<dyn crate::calib3d::StereoBM> {
		#[inline] fn as_raw_StereoMatcher(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::calib3d::StereoMatcher for core::Ptr<dyn crate::calib3d::StereoBM> {
		#[inline] fn as_raw_mut_StereoMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfStereoSGBM = core::Ptr<dyn crate::calib3d::StereoSGBM>;
	
	ptr_extern! { dyn crate::calib3d::StereoSGBM,
		cv_PtrOfStereoSGBM_delete, cv_PtrOfStereoSGBM_get_inner_ptr, cv_PtrOfStereoSGBM_get_inner_ptr_mut
	}
	
	impl core::Ptr<dyn crate::calib3d::StereoSGBM> {
		#[inline] pub fn as_raw_PtrOfStereoSGBM(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfStereoSGBM(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::calib3d::StereoSGBMConst for core::Ptr<dyn crate::calib3d::StereoSGBM> {
		#[inline] fn as_raw_StereoSGBM(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::calib3d::StereoSGBM for core::Ptr<dyn crate::calib3d::StereoSGBM> {
		#[inline] fn as_raw_mut_StereoSGBM(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<dyn crate::calib3d::StereoSGBM> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<dyn crate::calib3d::StereoSGBM> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::calib3d::StereoMatcherConst for core::Ptr<dyn crate::calib3d::StereoSGBM> {
		#[inline] fn as_raw_StereoMatcher(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::calib3d::StereoMatcher for core::Ptr<dyn crate::calib3d::StereoSGBM> {
		#[inline] fn as_raw_mut_StereoMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(ocvrs_has_module_calib3d)]
pub use calib3d_types::*;

#[cfg(ocvrs_has_module_core)]
mod core_types {
	use crate::{mod_prelude::*, core, types, sys};

	impl core::GpuMat_AllocatorConst for types::AbstractRefMut<'static, dyn core::GpuMat_Allocator> {
		#[inline] fn as_raw_GpuMat_Allocator(&self) -> extern_send!(Self) { self.as_raw() }
	}
	
	impl core::GpuMat_Allocator for types::AbstractRefMut<'static, dyn core::GpuMat_Allocator> {
		#[inline] fn as_raw_mut_GpuMat_Allocator(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl core::MatOpConst for types::AbstractRefMut<'static, dyn core::MatOp> {
		#[inline] fn as_raw_MatOp(&self) -> extern_send!(Self) { self.as_raw() }
	}
	
	impl core::MatOp for types::AbstractRefMut<'static, dyn core::MatOp> {
		#[inline] fn as_raw_mut_MatOp(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	pub type PtrOfConjGradSolver = core::Ptr<dyn core::ConjGradSolver>;
	
	ptr_extern! { dyn core::ConjGradSolver,
		cv_PtrOfConjGradSolver_delete, cv_PtrOfConjGradSolver_get_inner_ptr, cv_PtrOfConjGradSolver_get_inner_ptr_mut
	}
	
	impl core::Ptr<dyn core::ConjGradSolver> {
		#[inline] pub fn as_raw_PtrOfConjGradSolver(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfConjGradSolver(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl core::ConjGradSolverConst for core::Ptr<dyn core::ConjGradSolver> {
		#[inline] fn as_raw_ConjGradSolver(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::ConjGradSolver for core::Ptr<dyn core::ConjGradSolver> {
		#[inline] fn as_raw_mut_ConjGradSolver(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<dyn core::ConjGradSolver> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<dyn core::ConjGradSolver> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::MinProblemSolverConst for core::Ptr<dyn core::ConjGradSolver> {
		#[inline] fn as_raw_MinProblemSolver(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::MinProblemSolver for core::Ptr<dyn core::ConjGradSolver> {
		#[inline] fn as_raw_mut_MinProblemSolver(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDownhillSolver = core::Ptr<dyn core::DownhillSolver>;
	
	ptr_extern! { dyn core::DownhillSolver,
		cv_PtrOfDownhillSolver_delete, cv_PtrOfDownhillSolver_get_inner_ptr, cv_PtrOfDownhillSolver_get_inner_ptr_mut
	}
	
	impl core::Ptr<dyn core::DownhillSolver> {
		#[inline] pub fn as_raw_PtrOfDownhillSolver(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDownhillSolver(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl core::DownhillSolverConst for core::Ptr<dyn core::DownhillSolver> {
		#[inline] fn as_raw_DownhillSolver(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::DownhillSolver for core::Ptr<dyn core::DownhillSolver> {
		#[inline] fn as_raw_mut_DownhillSolver(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<dyn core::DownhillSolver> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<dyn core::DownhillSolver> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::MinProblemSolverConst for core::Ptr<dyn core::DownhillSolver> {
		#[inline] fn as_raw_MinProblemSolver(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::MinProblemSolver for core::Ptr<dyn core::DownhillSolver> {
		#[inline] fn as_raw_mut_MinProblemSolver(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfFileStorage = core::Ptr<core::FileStorage>;
	
	ptr_extern! { core::FileStorage,
		cv_PtrOfFileStorage_delete, cv_PtrOfFileStorage_get_inner_ptr, cv_PtrOfFileStorage_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { core::FileStorage, cv_PtrOfFileStorage_new }
	
	impl core::Ptr<core::FileStorage> {
		#[inline] pub fn as_raw_PtrOfFileStorage(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFileStorage(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl core::FileStorageTraitConst for core::Ptr<core::FileStorage> {
		#[inline] fn as_raw_FileStorage(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::FileStorageTrait for core::Ptr<core::FileStorage> {
		#[inline] fn as_raw_mut_FileStorage(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfFormatted = core::Ptr<dyn core::Formatted>;
	
	ptr_extern! { dyn core::Formatted,
		cv_PtrOfFormatted_delete, cv_PtrOfFormatted_get_inner_ptr, cv_PtrOfFormatted_get_inner_ptr_mut
	}
	
	impl core::Ptr<dyn core::Formatted> {
		#[inline] pub fn as_raw_PtrOfFormatted(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFormatted(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl core::FormattedConst for core::Ptr<dyn core::Formatted> {
		#[inline] fn as_raw_Formatted(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::Formatted for core::Ptr<dyn core::Formatted> {
		#[inline] fn as_raw_mut_Formatted(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfFormatter = core::Ptr<dyn core::Formatter>;
	
	ptr_extern! { dyn core::Formatter,
		cv_PtrOfFormatter_delete, cv_PtrOfFormatter_get_inner_ptr, cv_PtrOfFormatter_get_inner_ptr_mut
	}
	
	impl core::Ptr<dyn core::Formatter> {
		#[inline] pub fn as_raw_PtrOfFormatter(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFormatter(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl core::FormatterConst for core::Ptr<dyn core::Formatter> {
		#[inline] fn as_raw_Formatter(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::Formatter for core::Ptr<dyn core::Formatter> {
		#[inline] fn as_raw_mut_Formatter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfGpuMat_Allocator = core::Ptr<dyn core::GpuMat_Allocator>;
	
	ptr_extern! { dyn core::GpuMat_Allocator,
		cv_PtrOfGpuMat_Allocator_delete, cv_PtrOfGpuMat_Allocator_get_inner_ptr, cv_PtrOfGpuMat_Allocator_get_inner_ptr_mut
	}
	
	impl core::Ptr<dyn core::GpuMat_Allocator> {
		#[inline] pub fn as_raw_PtrOfGpuMat_Allocator(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfGpuMat_Allocator(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl core::GpuMat_AllocatorConst for core::Ptr<dyn core::GpuMat_Allocator> {
		#[inline] fn as_raw_GpuMat_Allocator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::GpuMat_Allocator for core::Ptr<dyn core::GpuMat_Allocator> {
		#[inline] fn as_raw_mut_GpuMat_Allocator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfMinProblemSolver_Function = core::Ptr<dyn core::MinProblemSolver_Function>;
	
	ptr_extern! { dyn core::MinProblemSolver_Function,
		cv_PtrOfMinProblemSolver_Function_delete, cv_PtrOfMinProblemSolver_Function_get_inner_ptr, cv_PtrOfMinProblemSolver_Function_get_inner_ptr_mut
	}
	
	impl core::Ptr<dyn core::MinProblemSolver_Function> {
		#[inline] pub fn as_raw_PtrOfMinProblemSolver_Function(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMinProblemSolver_Function(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl core::MinProblemSolver_FunctionConst for core::Ptr<dyn core::MinProblemSolver_Function> {
		#[inline] fn as_raw_MinProblemSolver_Function(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::MinProblemSolver_Function for core::Ptr<dyn core::MinProblemSolver_Function> {
		#[inline] fn as_raw_mut_MinProblemSolver_Function(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfOriginalClassName = core::Ptr<core::OriginalClassName>;
	
	ptr_extern! { core::OriginalClassName,
		cv_PtrOfOriginalClassName_delete, cv_PtrOfOriginalClassName_get_inner_ptr, cv_PtrOfOriginalClassName_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { core::OriginalClassName, cv_PtrOfOriginalClassName_new }
	
	impl core::Ptr<core::OriginalClassName> {
		#[inline] pub fn as_raw_PtrOfOriginalClassName(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfOriginalClassName(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl core::OriginalClassNameTraitConst for core::Ptr<core::OriginalClassName> {
		#[inline] fn as_raw_OriginalClassName(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::OriginalClassNameTrait for core::Ptr<core::OriginalClassName> {
		#[inline] fn as_raw_mut_OriginalClassName(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOff32 = core::Ptr<f32>;
	
	ptr_extern! { f32,
		cv_PtrOff32_delete, cv_PtrOff32_get_inner_ptr, cv_PtrOff32_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { f32, cv_PtrOff32_new }
	
	impl core::Ptr<f32> {
		#[inline] pub fn as_raw_PtrOff32(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOff32(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	pub type TupleOfRect_i32 = core::Tuple<(core::Rect, i32)>;
	
	impl core::Tuple<(core::Rect, i32)> {
		pub fn as_raw_TupleOfRect_i32(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_TupleOfRect_i32(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	tuple_extern! { (core::Rect, i32),
		cv_TupleOfRect_i32_new, cv_TupleOfRect_i32_delete,
		0 = arg: core::Rect, get_0 via cv_TupleOfRect_i32_get_0,
		1 = arg_1: i32, get_1 via cv_TupleOfRect_i32_get_1
	}
	
	pub type TupleOfUMat_u8 = core::Tuple<(core::UMat, u8)>;
	
	impl core::Tuple<(core::UMat, u8)> {
		pub fn as_raw_TupleOfUMat_u8(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_TupleOfUMat_u8(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	tuple_extern! { (core::UMat, u8),
		cv_TupleOfUMat_u8_new, cv_TupleOfUMat_u8_delete,
		0 = arg: core::UMat, get_0 via cv_TupleOfUMat_u8_get_0,
		1 = arg_1: u8, get_1 via cv_TupleOfUMat_u8_get_1
	}
	
	pub type TupleOfi32_f32 = core::Tuple<(i32, f32)>;
	
	impl core::Tuple<(i32, f32)> {
		pub fn as_raw_TupleOfi32_f32(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_TupleOfi32_f32(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	tuple_extern! { (i32, f32),
		cv_TupleOfi32_f32_new, cv_TupleOfi32_f32_delete,
		0 = arg: i32, get_0 via cv_TupleOfi32_f32_get_0,
		1 = arg_1: f32, get_1 via cv_TupleOfi32_f32_get_1
	}
	
	pub type VectorOfDMatch = core::Vector<core::DMatch>;
	
	impl core::Vector<core::DMatch> {
		pub fn as_raw_VectorOfDMatch(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfDMatch(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::DMatch,
		cv_VectorOfDMatch_new, cv_VectorOfDMatch_delete,
		cv_VectorOfDMatch_len, cv_VectorOfDMatch_is_empty,
		cv_VectorOfDMatch_capacity, cv_VectorOfDMatch_shrink_to_fit,
		cv_VectorOfDMatch_reserve, cv_VectorOfDMatch_remove,
		cv_VectorOfDMatch_swap, cv_VectorOfDMatch_clear,
		cv_VectorOfDMatch_get, cv_VectorOfDMatch_set,
		cv_VectorOfDMatch_push, cv_VectorOfDMatch_insert,
	}
	vector_copy_non_bool! { core::DMatch,
		cv_VectorOfDMatch_data, cv_VectorOfDMatch_data_mut, cv_VectorOfDMatch_from_slice,
		cv_VectorOfDMatch_clone,
	}
	
	pub type VectorOfGpuMat = core::Vector<core::GpuMat>;
	
	impl core::Vector<core::GpuMat> {
		pub fn as_raw_VectorOfGpuMat(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfGpuMat(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::GpuMat,
		cv_VectorOfGpuMat_new, cv_VectorOfGpuMat_delete,
		cv_VectorOfGpuMat_len, cv_VectorOfGpuMat_is_empty,
		cv_VectorOfGpuMat_capacity, cv_VectorOfGpuMat_shrink_to_fit,
		cv_VectorOfGpuMat_reserve, cv_VectorOfGpuMat_remove,
		cv_VectorOfGpuMat_swap, cv_VectorOfGpuMat_clear,
		cv_VectorOfGpuMat_get, cv_VectorOfGpuMat_set,
		cv_VectorOfGpuMat_push, cv_VectorOfGpuMat_insert,
	}
	vector_non_copy_or_bool! { clone core::GpuMat }
	
	pub type VectorOfKeyPoint = core::Vector<core::KeyPoint>;
	
	impl core::Vector<core::KeyPoint> {
		pub fn as_raw_VectorOfKeyPoint(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfKeyPoint(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::KeyPoint,
		cv_VectorOfKeyPoint_new, cv_VectorOfKeyPoint_delete,
		cv_VectorOfKeyPoint_len, cv_VectorOfKeyPoint_is_empty,
		cv_VectorOfKeyPoint_capacity, cv_VectorOfKeyPoint_shrink_to_fit,
		cv_VectorOfKeyPoint_reserve, cv_VectorOfKeyPoint_remove,
		cv_VectorOfKeyPoint_swap, cv_VectorOfKeyPoint_clear,
		cv_VectorOfKeyPoint_get, cv_VectorOfKeyPoint_set,
		cv_VectorOfKeyPoint_push, cv_VectorOfKeyPoint_insert,
	}
	vector_non_copy_or_bool! { core::KeyPoint }
	
	pub type VectorOfMat = core::Vector<core::Mat>;
	
	impl core::Vector<core::Mat> {
		pub fn as_raw_VectorOfMat(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfMat(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Mat,
		cv_VectorOfMat_new, cv_VectorOfMat_delete,
		cv_VectorOfMat_len, cv_VectorOfMat_is_empty,
		cv_VectorOfMat_capacity, cv_VectorOfMat_shrink_to_fit,
		cv_VectorOfMat_reserve, cv_VectorOfMat_remove,
		cv_VectorOfMat_swap, cv_VectorOfMat_clear,
		cv_VectorOfMat_get, cv_VectorOfMat_set,
		cv_VectorOfMat_push, cv_VectorOfMat_insert,
	}
	vector_non_copy_or_bool! { clone core::Mat }
	
	pub type VectorOfPlatformInfo = core::Vector<core::PlatformInfo>;
	
	impl core::Vector<core::PlatformInfo> {
		pub fn as_raw_VectorOfPlatformInfo(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfPlatformInfo(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::PlatformInfo,
		cv_VectorOfPlatformInfo_new, cv_VectorOfPlatformInfo_delete,
		cv_VectorOfPlatformInfo_len, cv_VectorOfPlatformInfo_is_empty,
		cv_VectorOfPlatformInfo_capacity, cv_VectorOfPlatformInfo_shrink_to_fit,
		cv_VectorOfPlatformInfo_reserve, cv_VectorOfPlatformInfo_remove,
		cv_VectorOfPlatformInfo_swap, cv_VectorOfPlatformInfo_clear,
		cv_VectorOfPlatformInfo_get, cv_VectorOfPlatformInfo_set,
		cv_VectorOfPlatformInfo_push, cv_VectorOfPlatformInfo_insert,
	}
	vector_non_copy_or_bool! { core::PlatformInfo }
	
	pub type VectorOfPoint = core::Vector<core::Point>;
	
	impl core::Vector<core::Point> {
		pub fn as_raw_VectorOfPoint(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfPoint(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Point,
		cv_VectorOfPoint_new, cv_VectorOfPoint_delete,
		cv_VectorOfPoint_len, cv_VectorOfPoint_is_empty,
		cv_VectorOfPoint_capacity, cv_VectorOfPoint_shrink_to_fit,
		cv_VectorOfPoint_reserve, cv_VectorOfPoint_remove,
		cv_VectorOfPoint_swap, cv_VectorOfPoint_clear,
		cv_VectorOfPoint_get, cv_VectorOfPoint_set,
		cv_VectorOfPoint_push, cv_VectorOfPoint_insert,
	}
	vector_copy_non_bool! { core::Point,
		cv_VectorOfPoint_data, cv_VectorOfPoint_data_mut, cv_VectorOfPoint_from_slice,
		cv_VectorOfPoint_clone,
	}
	
	extern "C" {
		fn cv_VectorOfPoint_input_array(instance: extern_send!(core::Vector<core::Point>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputArray)>);
		fn cv_VectorOfPoint_output_array(instance: extern_send!(mut core::Vector<core::Point>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_OutputArray)>);
		fn cv_VectorOfPoint_input_output_array(instance: extern_send!(mut core::Vector<core::Point>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputOutputArray)>);
	}
	
	impl core::ToInputArray for core::Vector<core::Point> {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfPoint_input_array(self.as_raw_VectorOfPoint(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for core::Vector<core::Point> {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfPoint_output_array(self.as_raw_mut_VectorOfPoint(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for core::Vector<core::Point> {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfPoint_input_output_array(self.as_raw_mut_VectorOfPoint(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { core::Vector<core::Point> }
	
	pub type VectorOfPoint2d = core::Vector<core::Point2d>;
	
	impl core::Vector<core::Point2d> {
		pub fn as_raw_VectorOfPoint2d(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfPoint2d(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Point2d,
		cv_VectorOfPoint2d_new, cv_VectorOfPoint2d_delete,
		cv_VectorOfPoint2d_len, cv_VectorOfPoint2d_is_empty,
		cv_VectorOfPoint2d_capacity, cv_VectorOfPoint2d_shrink_to_fit,
		cv_VectorOfPoint2d_reserve, cv_VectorOfPoint2d_remove,
		cv_VectorOfPoint2d_swap, cv_VectorOfPoint2d_clear,
		cv_VectorOfPoint2d_get, cv_VectorOfPoint2d_set,
		cv_VectorOfPoint2d_push, cv_VectorOfPoint2d_insert,
	}
	vector_copy_non_bool! { core::Point2d,
		cv_VectorOfPoint2d_data, cv_VectorOfPoint2d_data_mut, cv_VectorOfPoint2d_from_slice,
		cv_VectorOfPoint2d_clone,
	}
	
	extern "C" {
		fn cv_VectorOfPoint2d_input_array(instance: extern_send!(core::Vector<core::Point2d>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputArray)>);
		fn cv_VectorOfPoint2d_output_array(instance: extern_send!(mut core::Vector<core::Point2d>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_OutputArray)>);
		fn cv_VectorOfPoint2d_input_output_array(instance: extern_send!(mut core::Vector<core::Point2d>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputOutputArray)>);
	}
	
	impl core::ToInputArray for core::Vector<core::Point2d> {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfPoint2d_input_array(self.as_raw_VectorOfPoint2d(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for core::Vector<core::Point2d> {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfPoint2d_output_array(self.as_raw_mut_VectorOfPoint2d(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for core::Vector<core::Point2d> {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfPoint2d_input_output_array(self.as_raw_mut_VectorOfPoint2d(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { core::Vector<core::Point2d> }
	
	pub type VectorOfPoint2f = core::Vector<core::Point2f>;
	
	impl core::Vector<core::Point2f> {
		pub fn as_raw_VectorOfPoint2f(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfPoint2f(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Point2f,
		cv_VectorOfPoint2f_new, cv_VectorOfPoint2f_delete,
		cv_VectorOfPoint2f_len, cv_VectorOfPoint2f_is_empty,
		cv_VectorOfPoint2f_capacity, cv_VectorOfPoint2f_shrink_to_fit,
		cv_VectorOfPoint2f_reserve, cv_VectorOfPoint2f_remove,
		cv_VectorOfPoint2f_swap, cv_VectorOfPoint2f_clear,
		cv_VectorOfPoint2f_get, cv_VectorOfPoint2f_set,
		cv_VectorOfPoint2f_push, cv_VectorOfPoint2f_insert,
	}
	vector_copy_non_bool! { core::Point2f,
		cv_VectorOfPoint2f_data, cv_VectorOfPoint2f_data_mut, cv_VectorOfPoint2f_from_slice,
		cv_VectorOfPoint2f_clone,
	}
	
	extern "C" {
		fn cv_VectorOfPoint2f_input_array(instance: extern_send!(core::Vector<core::Point2f>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputArray)>);
		fn cv_VectorOfPoint2f_output_array(instance: extern_send!(mut core::Vector<core::Point2f>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_OutputArray)>);
		fn cv_VectorOfPoint2f_input_output_array(instance: extern_send!(mut core::Vector<core::Point2f>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputOutputArray)>);
	}
	
	impl core::ToInputArray for core::Vector<core::Point2f> {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfPoint2f_input_array(self.as_raw_VectorOfPoint2f(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for core::Vector<core::Point2f> {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfPoint2f_output_array(self.as_raw_mut_VectorOfPoint2f(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for core::Vector<core::Point2f> {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfPoint2f_input_output_array(self.as_raw_mut_VectorOfPoint2f(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { core::Vector<core::Point2f> }
	
	pub type VectorOfPoint3d = core::Vector<core::Point3d>;
	
	impl core::Vector<core::Point3d> {
		pub fn as_raw_VectorOfPoint3d(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfPoint3d(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Point3d,
		cv_VectorOfPoint3d_new, cv_VectorOfPoint3d_delete,
		cv_VectorOfPoint3d_len, cv_VectorOfPoint3d_is_empty,
		cv_VectorOfPoint3d_capacity, cv_VectorOfPoint3d_shrink_to_fit,
		cv_VectorOfPoint3d_reserve, cv_VectorOfPoint3d_remove,
		cv_VectorOfPoint3d_swap, cv_VectorOfPoint3d_clear,
		cv_VectorOfPoint3d_get, cv_VectorOfPoint3d_set,
		cv_VectorOfPoint3d_push, cv_VectorOfPoint3d_insert,
	}
	vector_copy_non_bool! { core::Point3d,
		cv_VectorOfPoint3d_data, cv_VectorOfPoint3d_data_mut, cv_VectorOfPoint3d_from_slice,
		cv_VectorOfPoint3d_clone,
	}
	
	extern "C" {
		fn cv_VectorOfPoint3d_input_array(instance: extern_send!(core::Vector<core::Point3d>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputArray)>);
		fn cv_VectorOfPoint3d_output_array(instance: extern_send!(mut core::Vector<core::Point3d>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_OutputArray)>);
		fn cv_VectorOfPoint3d_input_output_array(instance: extern_send!(mut core::Vector<core::Point3d>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputOutputArray)>);
	}
	
	impl core::ToInputArray for core::Vector<core::Point3d> {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfPoint3d_input_array(self.as_raw_VectorOfPoint3d(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for core::Vector<core::Point3d> {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfPoint3d_output_array(self.as_raw_mut_VectorOfPoint3d(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for core::Vector<core::Point3d> {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfPoint3d_input_output_array(self.as_raw_mut_VectorOfPoint3d(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { core::Vector<core::Point3d> }
	
	pub type VectorOfPoint3f = core::Vector<core::Point3f>;
	
	impl core::Vector<core::Point3f> {
		pub fn as_raw_VectorOfPoint3f(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfPoint3f(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Point3f,
		cv_VectorOfPoint3f_new, cv_VectorOfPoint3f_delete,
		cv_VectorOfPoint3f_len, cv_VectorOfPoint3f_is_empty,
		cv_VectorOfPoint3f_capacity, cv_VectorOfPoint3f_shrink_to_fit,
		cv_VectorOfPoint3f_reserve, cv_VectorOfPoint3f_remove,
		cv_VectorOfPoint3f_swap, cv_VectorOfPoint3f_clear,
		cv_VectorOfPoint3f_get, cv_VectorOfPoint3f_set,
		cv_VectorOfPoint3f_push, cv_VectorOfPoint3f_insert,
	}
	vector_copy_non_bool! { core::Point3f,
		cv_VectorOfPoint3f_data, cv_VectorOfPoint3f_data_mut, cv_VectorOfPoint3f_from_slice,
		cv_VectorOfPoint3f_clone,
	}
	
	extern "C" {
		fn cv_VectorOfPoint3f_input_array(instance: extern_send!(core::Vector<core::Point3f>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputArray)>);
		fn cv_VectorOfPoint3f_output_array(instance: extern_send!(mut core::Vector<core::Point3f>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_OutputArray)>);
		fn cv_VectorOfPoint3f_input_output_array(instance: extern_send!(mut core::Vector<core::Point3f>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputOutputArray)>);
	}
	
	impl core::ToInputArray for core::Vector<core::Point3f> {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfPoint3f_input_array(self.as_raw_VectorOfPoint3f(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for core::Vector<core::Point3f> {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfPoint3f_output_array(self.as_raw_mut_VectorOfPoint3f(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for core::Vector<core::Point3f> {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfPoint3f_input_output_array(self.as_raw_mut_VectorOfPoint3f(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { core::Vector<core::Point3f> }
	
	pub type VectorOfPoint3i = core::Vector<core::Point3i>;
	
	impl core::Vector<core::Point3i> {
		pub fn as_raw_VectorOfPoint3i(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfPoint3i(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Point3i,
		cv_VectorOfPoint3i_new, cv_VectorOfPoint3i_delete,
		cv_VectorOfPoint3i_len, cv_VectorOfPoint3i_is_empty,
		cv_VectorOfPoint3i_capacity, cv_VectorOfPoint3i_shrink_to_fit,
		cv_VectorOfPoint3i_reserve, cv_VectorOfPoint3i_remove,
		cv_VectorOfPoint3i_swap, cv_VectorOfPoint3i_clear,
		cv_VectorOfPoint3i_get, cv_VectorOfPoint3i_set,
		cv_VectorOfPoint3i_push, cv_VectorOfPoint3i_insert,
	}
	vector_copy_non_bool! { core::Point3i,
		cv_VectorOfPoint3i_data, cv_VectorOfPoint3i_data_mut, cv_VectorOfPoint3i_from_slice,
		cv_VectorOfPoint3i_clone,
	}
	
	extern "C" {
		fn cv_VectorOfPoint3i_input_array(instance: extern_send!(core::Vector<core::Point3i>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputArray)>);
		fn cv_VectorOfPoint3i_output_array(instance: extern_send!(mut core::Vector<core::Point3i>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_OutputArray)>);
		fn cv_VectorOfPoint3i_input_output_array(instance: extern_send!(mut core::Vector<core::Point3i>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputOutputArray)>);
	}
	
	impl core::ToInputArray for core::Vector<core::Point3i> {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfPoint3i_input_array(self.as_raw_VectorOfPoint3i(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for core::Vector<core::Point3i> {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfPoint3i_output_array(self.as_raw_mut_VectorOfPoint3i(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for core::Vector<core::Point3i> {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfPoint3i_input_output_array(self.as_raw_mut_VectorOfPoint3i(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { core::Vector<core::Point3i> }
	
	pub type VectorOfRange = core::Vector<core::Range>;
	
	impl core::Vector<core::Range> {
		pub fn as_raw_VectorOfRange(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfRange(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Range,
		cv_VectorOfRange_new, cv_VectorOfRange_delete,
		cv_VectorOfRange_len, cv_VectorOfRange_is_empty,
		cv_VectorOfRange_capacity, cv_VectorOfRange_shrink_to_fit,
		cv_VectorOfRange_reserve, cv_VectorOfRange_remove,
		cv_VectorOfRange_swap, cv_VectorOfRange_clear,
		cv_VectorOfRange_get, cv_VectorOfRange_set,
		cv_VectorOfRange_push, cv_VectorOfRange_insert,
	}
	vector_non_copy_or_bool! { core::Range }
	
	pub type VectorOfRect = core::Vector<core::Rect>;
	
	impl core::Vector<core::Rect> {
		pub fn as_raw_VectorOfRect(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfRect(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Rect,
		cv_VectorOfRect_new, cv_VectorOfRect_delete,
		cv_VectorOfRect_len, cv_VectorOfRect_is_empty,
		cv_VectorOfRect_capacity, cv_VectorOfRect_shrink_to_fit,
		cv_VectorOfRect_reserve, cv_VectorOfRect_remove,
		cv_VectorOfRect_swap, cv_VectorOfRect_clear,
		cv_VectorOfRect_get, cv_VectorOfRect_set,
		cv_VectorOfRect_push, cv_VectorOfRect_insert,
	}
	vector_copy_non_bool! { core::Rect,
		cv_VectorOfRect_data, cv_VectorOfRect_data_mut, cv_VectorOfRect_from_slice,
		cv_VectorOfRect_clone,
	}
	
	extern "C" {
		fn cv_VectorOfRect_input_array(instance: extern_send!(core::Vector<core::Rect>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputArray)>);
		fn cv_VectorOfRect_output_array(instance: extern_send!(mut core::Vector<core::Rect>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_OutputArray)>);
		fn cv_VectorOfRect_input_output_array(instance: extern_send!(mut core::Vector<core::Rect>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputOutputArray)>);
	}
	
	impl core::ToInputArray for core::Vector<core::Rect> {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfRect_input_array(self.as_raw_VectorOfRect(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for core::Vector<core::Rect> {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfRect_output_array(self.as_raw_mut_VectorOfRect(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for core::Vector<core::Rect> {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfRect_input_output_array(self.as_raw_mut_VectorOfRect(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { core::Vector<core::Rect> }
	
	pub type VectorOfRect2d = core::Vector<core::Rect2d>;
	
	impl core::Vector<core::Rect2d> {
		pub fn as_raw_VectorOfRect2d(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfRect2d(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Rect2d,
		cv_VectorOfRect2d_new, cv_VectorOfRect2d_delete,
		cv_VectorOfRect2d_len, cv_VectorOfRect2d_is_empty,
		cv_VectorOfRect2d_capacity, cv_VectorOfRect2d_shrink_to_fit,
		cv_VectorOfRect2d_reserve, cv_VectorOfRect2d_remove,
		cv_VectorOfRect2d_swap, cv_VectorOfRect2d_clear,
		cv_VectorOfRect2d_get, cv_VectorOfRect2d_set,
		cv_VectorOfRect2d_push, cv_VectorOfRect2d_insert,
	}
	vector_copy_non_bool! { core::Rect2d,
		cv_VectorOfRect2d_data, cv_VectorOfRect2d_data_mut, cv_VectorOfRect2d_from_slice,
		cv_VectorOfRect2d_clone,
	}
	
	extern "C" {
		fn cv_VectorOfRect2d_input_array(instance: extern_send!(core::Vector<core::Rect2d>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputArray)>);
		fn cv_VectorOfRect2d_output_array(instance: extern_send!(mut core::Vector<core::Rect2d>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_OutputArray)>);
		fn cv_VectorOfRect2d_input_output_array(instance: extern_send!(mut core::Vector<core::Rect2d>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputOutputArray)>);
	}
	
	impl core::ToInputArray for core::Vector<core::Rect2d> {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfRect2d_input_array(self.as_raw_VectorOfRect2d(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for core::Vector<core::Rect2d> {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfRect2d_output_array(self.as_raw_mut_VectorOfRect2d(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for core::Vector<core::Rect2d> {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfRect2d_input_output_array(self.as_raw_mut_VectorOfRect2d(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { core::Vector<core::Rect2d> }
	
	pub type VectorOfRotatedRect = core::Vector<core::RotatedRect>;
	
	impl core::Vector<core::RotatedRect> {
		pub fn as_raw_VectorOfRotatedRect(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfRotatedRect(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::RotatedRect,
		cv_VectorOfRotatedRect_new, cv_VectorOfRotatedRect_delete,
		cv_VectorOfRotatedRect_len, cv_VectorOfRotatedRect_is_empty,
		cv_VectorOfRotatedRect_capacity, cv_VectorOfRotatedRect_shrink_to_fit,
		cv_VectorOfRotatedRect_reserve, cv_VectorOfRotatedRect_remove,
		cv_VectorOfRotatedRect_swap, cv_VectorOfRotatedRect_clear,
		cv_VectorOfRotatedRect_get, cv_VectorOfRotatedRect_set,
		cv_VectorOfRotatedRect_push, cv_VectorOfRotatedRect_insert,
	}
	vector_non_copy_or_bool! { core::RotatedRect }
	
	pub type VectorOfScalar = core::Vector<core::Scalar>;
	
	impl core::Vector<core::Scalar> {
		pub fn as_raw_VectorOfScalar(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfScalar(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Scalar,
		cv_VectorOfScalar_new, cv_VectorOfScalar_delete,
		cv_VectorOfScalar_len, cv_VectorOfScalar_is_empty,
		cv_VectorOfScalar_capacity, cv_VectorOfScalar_shrink_to_fit,
		cv_VectorOfScalar_reserve, cv_VectorOfScalar_remove,
		cv_VectorOfScalar_swap, cv_VectorOfScalar_clear,
		cv_VectorOfScalar_get, cv_VectorOfScalar_set,
		cv_VectorOfScalar_push, cv_VectorOfScalar_insert,
	}
	vector_copy_non_bool! { core::Scalar,
		cv_VectorOfScalar_data, cv_VectorOfScalar_data_mut, cv_VectorOfScalar_from_slice,
		cv_VectorOfScalar_clone,
	}
	
	extern "C" {
		fn cv_VectorOfScalar_input_array(instance: extern_send!(core::Vector<core::Scalar>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputArray)>);
		fn cv_VectorOfScalar_output_array(instance: extern_send!(mut core::Vector<core::Scalar>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_OutputArray)>);
		fn cv_VectorOfScalar_input_output_array(instance: extern_send!(mut core::Vector<core::Scalar>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputOutputArray)>);
	}
	
	impl core::ToInputArray for core::Vector<core::Scalar> {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfScalar_input_array(self.as_raw_VectorOfScalar(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for core::Vector<core::Scalar> {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfScalar_output_array(self.as_raw_mut_VectorOfScalar(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for core::Vector<core::Scalar> {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfScalar_input_output_array(self.as_raw_mut_VectorOfScalar(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { core::Vector<core::Scalar> }
	
	pub type VectorOfSize = core::Vector<core::Size>;
	
	impl core::Vector<core::Size> {
		pub fn as_raw_VectorOfSize(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfSize(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Size,
		cv_VectorOfSize_new, cv_VectorOfSize_delete,
		cv_VectorOfSize_len, cv_VectorOfSize_is_empty,
		cv_VectorOfSize_capacity, cv_VectorOfSize_shrink_to_fit,
		cv_VectorOfSize_reserve, cv_VectorOfSize_remove,
		cv_VectorOfSize_swap, cv_VectorOfSize_clear,
		cv_VectorOfSize_get, cv_VectorOfSize_set,
		cv_VectorOfSize_push, cv_VectorOfSize_insert,
	}
	vector_copy_non_bool! { core::Size,
		cv_VectorOfSize_data, cv_VectorOfSize_data_mut, cv_VectorOfSize_from_slice,
		cv_VectorOfSize_clone,
	}
	
	extern "C" {
		fn cv_VectorOfSize_input_array(instance: extern_send!(core::Vector<core::Size>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputArray)>);
		fn cv_VectorOfSize_output_array(instance: extern_send!(mut core::Vector<core::Size>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_OutputArray)>);
		fn cv_VectorOfSize_input_output_array(instance: extern_send!(mut core::Vector<core::Size>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputOutputArray)>);
	}
	
	impl core::ToInputArray for core::Vector<core::Size> {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfSize_input_array(self.as_raw_VectorOfSize(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for core::Vector<core::Size> {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfSize_output_array(self.as_raw_mut_VectorOfSize(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for core::Vector<core::Size> {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfSize_input_output_array(self.as_raw_mut_VectorOfSize(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { core::Vector<core::Size> }
	
	pub type VectorOfString = core::Vector<String>;
	
	impl core::Vector<String> {
		pub fn as_raw_VectorOfString(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfString(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { String,
		cv_VectorOfString_new, cv_VectorOfString_delete,
		cv_VectorOfString_len, cv_VectorOfString_is_empty,
		cv_VectorOfString_capacity, cv_VectorOfString_shrink_to_fit,
		cv_VectorOfString_reserve, cv_VectorOfString_remove,
		cv_VectorOfString_swap, cv_VectorOfString_clear,
		cv_VectorOfString_get, cv_VectorOfString_set,
		cv_VectorOfString_push, cv_VectorOfString_insert,
	}
	vector_non_copy_or_bool! { String }
	
	pub type VectorOfTupleOfUMat_u8 = core::Vector<core::Tuple<(core::UMat, u8)>>;
	
	impl core::Vector<core::Tuple<(core::UMat, u8)>> {
		pub fn as_raw_VectorOfTupleOfUMat_u8(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfTupleOfUMat_u8(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Tuple<(core::UMat, u8)>,
		cv_VectorOfTupleOfUMat_u8_new, cv_VectorOfTupleOfUMat_u8_delete,
		cv_VectorOfTupleOfUMat_u8_len, cv_VectorOfTupleOfUMat_u8_is_empty,
		cv_VectorOfTupleOfUMat_u8_capacity, cv_VectorOfTupleOfUMat_u8_shrink_to_fit,
		cv_VectorOfTupleOfUMat_u8_reserve, cv_VectorOfTupleOfUMat_u8_remove,
		cv_VectorOfTupleOfUMat_u8_swap, cv_VectorOfTupleOfUMat_u8_clear,
		cv_VectorOfTupleOfUMat_u8_get, cv_VectorOfTupleOfUMat_u8_set,
		cv_VectorOfTupleOfUMat_u8_push, cv_VectorOfTupleOfUMat_u8_insert,
	}
	vector_non_copy_or_bool! { core::Tuple<(core::UMat, u8)> }
	
	pub type VectorOfUMat = core::Vector<core::UMat>;
	
	impl core::Vector<core::UMat> {
		pub fn as_raw_VectorOfUMat(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfUMat(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::UMat,
		cv_VectorOfUMat_new, cv_VectorOfUMat_delete,
		cv_VectorOfUMat_len, cv_VectorOfUMat_is_empty,
		cv_VectorOfUMat_capacity, cv_VectorOfUMat_shrink_to_fit,
		cv_VectorOfUMat_reserve, cv_VectorOfUMat_remove,
		cv_VectorOfUMat_swap, cv_VectorOfUMat_clear,
		cv_VectorOfUMat_get, cv_VectorOfUMat_set,
		cv_VectorOfUMat_push, cv_VectorOfUMat_insert,
	}
	vector_non_copy_or_bool! { clone core::UMat }
	
	pub type VectorOfVec2d = core::Vector<core::Vec2d>;
	
	impl core::Vector<core::Vec2d> {
		pub fn as_raw_VectorOfVec2d(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfVec2d(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vec2d,
		cv_VectorOfVec2d_new, cv_VectorOfVec2d_delete,
		cv_VectorOfVec2d_len, cv_VectorOfVec2d_is_empty,
		cv_VectorOfVec2d_capacity, cv_VectorOfVec2d_shrink_to_fit,
		cv_VectorOfVec2d_reserve, cv_VectorOfVec2d_remove,
		cv_VectorOfVec2d_swap, cv_VectorOfVec2d_clear,
		cv_VectorOfVec2d_get, cv_VectorOfVec2d_set,
		cv_VectorOfVec2d_push, cv_VectorOfVec2d_insert,
	}
	vector_copy_non_bool! { core::Vec2d,
		cv_VectorOfVec2d_data, cv_VectorOfVec2d_data_mut, cv_VectorOfVec2d_from_slice,
		cv_VectorOfVec2d_clone,
	}
	
	extern "C" {
		fn cv_VectorOfVec2d_input_array(instance: extern_send!(core::Vector<core::Vec2d>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputArray)>);
		fn cv_VectorOfVec2d_output_array(instance: extern_send!(mut core::Vector<core::Vec2d>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_OutputArray)>);
		fn cv_VectorOfVec2d_input_output_array(instance: extern_send!(mut core::Vector<core::Vec2d>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputOutputArray)>);
	}
	
	impl core::ToInputArray for core::Vector<core::Vec2d> {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVec2d_input_array(self.as_raw_VectorOfVec2d(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for core::Vector<core::Vec2d> {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVec2d_output_array(self.as_raw_mut_VectorOfVec2d(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for core::Vector<core::Vec2d> {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVec2d_input_output_array(self.as_raw_mut_VectorOfVec2d(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { core::Vector<core::Vec2d> }
	
	pub type VectorOfVec2f = core::Vector<core::Vec2f>;
	
	impl core::Vector<core::Vec2f> {
		pub fn as_raw_VectorOfVec2f(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfVec2f(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vec2f,
		cv_VectorOfVec2f_new, cv_VectorOfVec2f_delete,
		cv_VectorOfVec2f_len, cv_VectorOfVec2f_is_empty,
		cv_VectorOfVec2f_capacity, cv_VectorOfVec2f_shrink_to_fit,
		cv_VectorOfVec2f_reserve, cv_VectorOfVec2f_remove,
		cv_VectorOfVec2f_swap, cv_VectorOfVec2f_clear,
		cv_VectorOfVec2f_get, cv_VectorOfVec2f_set,
		cv_VectorOfVec2f_push, cv_VectorOfVec2f_insert,
	}
	vector_copy_non_bool! { core::Vec2f,
		cv_VectorOfVec2f_data, cv_VectorOfVec2f_data_mut, cv_VectorOfVec2f_from_slice,
		cv_VectorOfVec2f_clone,
	}
	
	extern "C" {
		fn cv_VectorOfVec2f_input_array(instance: extern_send!(core::Vector<core::Vec2f>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputArray)>);
		fn cv_VectorOfVec2f_output_array(instance: extern_send!(mut core::Vector<core::Vec2f>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_OutputArray)>);
		fn cv_VectorOfVec2f_input_output_array(instance: extern_send!(mut core::Vector<core::Vec2f>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputOutputArray)>);
	}
	
	impl core::ToInputArray for core::Vector<core::Vec2f> {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVec2f_input_array(self.as_raw_VectorOfVec2f(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for core::Vector<core::Vec2f> {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVec2f_output_array(self.as_raw_mut_VectorOfVec2f(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for core::Vector<core::Vec2f> {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVec2f_input_output_array(self.as_raw_mut_VectorOfVec2f(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { core::Vector<core::Vec2f> }
	
	pub type VectorOfVec3d = core::Vector<core::Vec3d>;
	
	impl core::Vector<core::Vec3d> {
		pub fn as_raw_VectorOfVec3d(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfVec3d(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vec3d,
		cv_VectorOfVec3d_new, cv_VectorOfVec3d_delete,
		cv_VectorOfVec3d_len, cv_VectorOfVec3d_is_empty,
		cv_VectorOfVec3d_capacity, cv_VectorOfVec3d_shrink_to_fit,
		cv_VectorOfVec3d_reserve, cv_VectorOfVec3d_remove,
		cv_VectorOfVec3d_swap, cv_VectorOfVec3d_clear,
		cv_VectorOfVec3d_get, cv_VectorOfVec3d_set,
		cv_VectorOfVec3d_push, cv_VectorOfVec3d_insert,
	}
	vector_copy_non_bool! { core::Vec3d,
		cv_VectorOfVec3d_data, cv_VectorOfVec3d_data_mut, cv_VectorOfVec3d_from_slice,
		cv_VectorOfVec3d_clone,
	}
	
	extern "C" {
		fn cv_VectorOfVec3d_input_array(instance: extern_send!(core::Vector<core::Vec3d>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputArray)>);
		fn cv_VectorOfVec3d_output_array(instance: extern_send!(mut core::Vector<core::Vec3d>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_OutputArray)>);
		fn cv_VectorOfVec3d_input_output_array(instance: extern_send!(mut core::Vector<core::Vec3d>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputOutputArray)>);
	}
	
	impl core::ToInputArray for core::Vector<core::Vec3d> {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVec3d_input_array(self.as_raw_VectorOfVec3d(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for core::Vector<core::Vec3d> {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVec3d_output_array(self.as_raw_mut_VectorOfVec3d(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for core::Vector<core::Vec3d> {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVec3d_input_output_array(self.as_raw_mut_VectorOfVec3d(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { core::Vector<core::Vec3d> }
	
	pub type VectorOfVec3f = core::Vector<core::Vec3f>;
	
	impl core::Vector<core::Vec3f> {
		pub fn as_raw_VectorOfVec3f(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfVec3f(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vec3f,
		cv_VectorOfVec3f_new, cv_VectorOfVec3f_delete,
		cv_VectorOfVec3f_len, cv_VectorOfVec3f_is_empty,
		cv_VectorOfVec3f_capacity, cv_VectorOfVec3f_shrink_to_fit,
		cv_VectorOfVec3f_reserve, cv_VectorOfVec3f_remove,
		cv_VectorOfVec3f_swap, cv_VectorOfVec3f_clear,
		cv_VectorOfVec3f_get, cv_VectorOfVec3f_set,
		cv_VectorOfVec3f_push, cv_VectorOfVec3f_insert,
	}
	vector_copy_non_bool! { core::Vec3f,
		cv_VectorOfVec3f_data, cv_VectorOfVec3f_data_mut, cv_VectorOfVec3f_from_slice,
		cv_VectorOfVec3f_clone,
	}
	
	extern "C" {
		fn cv_VectorOfVec3f_input_array(instance: extern_send!(core::Vector<core::Vec3f>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputArray)>);
		fn cv_VectorOfVec3f_output_array(instance: extern_send!(mut core::Vector<core::Vec3f>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_OutputArray)>);
		fn cv_VectorOfVec3f_input_output_array(instance: extern_send!(mut core::Vector<core::Vec3f>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputOutputArray)>);
	}
	
	impl core::ToInputArray for core::Vector<core::Vec3f> {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVec3f_input_array(self.as_raw_VectorOfVec3f(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for core::Vector<core::Vec3f> {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVec3f_output_array(self.as_raw_mut_VectorOfVec3f(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for core::Vector<core::Vec3f> {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVec3f_input_output_array(self.as_raw_mut_VectorOfVec3f(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { core::Vector<core::Vec3f> }
	
	pub type VectorOfVec4f = core::Vector<core::Vec4f>;
	
	impl core::Vector<core::Vec4f> {
		pub fn as_raw_VectorOfVec4f(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfVec4f(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vec4f,
		cv_VectorOfVec4f_new, cv_VectorOfVec4f_delete,
		cv_VectorOfVec4f_len, cv_VectorOfVec4f_is_empty,
		cv_VectorOfVec4f_capacity, cv_VectorOfVec4f_shrink_to_fit,
		cv_VectorOfVec4f_reserve, cv_VectorOfVec4f_remove,
		cv_VectorOfVec4f_swap, cv_VectorOfVec4f_clear,
		cv_VectorOfVec4f_get, cv_VectorOfVec4f_set,
		cv_VectorOfVec4f_push, cv_VectorOfVec4f_insert,
	}
	vector_copy_non_bool! { core::Vec4f,
		cv_VectorOfVec4f_data, cv_VectorOfVec4f_data_mut, cv_VectorOfVec4f_from_slice,
		cv_VectorOfVec4f_clone,
	}
	
	extern "C" {
		fn cv_VectorOfVec4f_input_array(instance: extern_send!(core::Vector<core::Vec4f>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputArray)>);
		fn cv_VectorOfVec4f_output_array(instance: extern_send!(mut core::Vector<core::Vec4f>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_OutputArray)>);
		fn cv_VectorOfVec4f_input_output_array(instance: extern_send!(mut core::Vector<core::Vec4f>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputOutputArray)>);
	}
	
	impl core::ToInputArray for core::Vector<core::Vec4f> {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVec4f_input_array(self.as_raw_VectorOfVec4f(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for core::Vector<core::Vec4f> {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVec4f_output_array(self.as_raw_mut_VectorOfVec4f(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for core::Vector<core::Vec4f> {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVec4f_input_output_array(self.as_raw_mut_VectorOfVec4f(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { core::Vector<core::Vec4f> }
	
	pub type VectorOfVec4i = core::Vector<core::Vec4i>;
	
	impl core::Vector<core::Vec4i> {
		pub fn as_raw_VectorOfVec4i(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfVec4i(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vec4i,
		cv_VectorOfVec4i_new, cv_VectorOfVec4i_delete,
		cv_VectorOfVec4i_len, cv_VectorOfVec4i_is_empty,
		cv_VectorOfVec4i_capacity, cv_VectorOfVec4i_shrink_to_fit,
		cv_VectorOfVec4i_reserve, cv_VectorOfVec4i_remove,
		cv_VectorOfVec4i_swap, cv_VectorOfVec4i_clear,
		cv_VectorOfVec4i_get, cv_VectorOfVec4i_set,
		cv_VectorOfVec4i_push, cv_VectorOfVec4i_insert,
	}
	vector_copy_non_bool! { core::Vec4i,
		cv_VectorOfVec4i_data, cv_VectorOfVec4i_data_mut, cv_VectorOfVec4i_from_slice,
		cv_VectorOfVec4i_clone,
	}
	
	extern "C" {
		fn cv_VectorOfVec4i_input_array(instance: extern_send!(core::Vector<core::Vec4i>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputArray)>);
		fn cv_VectorOfVec4i_output_array(instance: extern_send!(mut core::Vector<core::Vec4i>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_OutputArray)>);
		fn cv_VectorOfVec4i_input_output_array(instance: extern_send!(mut core::Vector<core::Vec4i>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputOutputArray)>);
	}
	
	impl core::ToInputArray for core::Vector<core::Vec4i> {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVec4i_input_array(self.as_raw_VectorOfVec4i(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for core::Vector<core::Vec4i> {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVec4i_output_array(self.as_raw_mut_VectorOfVec4i(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for core::Vector<core::Vec4i> {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVec4i_input_output_array(self.as_raw_mut_VectorOfVec4i(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { core::Vector<core::Vec4i> }
	
	pub type VectorOfVec6f = core::Vector<core::Vec6f>;
	
	impl core::Vector<core::Vec6f> {
		pub fn as_raw_VectorOfVec6f(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfVec6f(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vec6f,
		cv_VectorOfVec6f_new, cv_VectorOfVec6f_delete,
		cv_VectorOfVec6f_len, cv_VectorOfVec6f_is_empty,
		cv_VectorOfVec6f_capacity, cv_VectorOfVec6f_shrink_to_fit,
		cv_VectorOfVec6f_reserve, cv_VectorOfVec6f_remove,
		cv_VectorOfVec6f_swap, cv_VectorOfVec6f_clear,
		cv_VectorOfVec6f_get, cv_VectorOfVec6f_set,
		cv_VectorOfVec6f_push, cv_VectorOfVec6f_insert,
	}
	vector_copy_non_bool! { core::Vec6f,
		cv_VectorOfVec6f_data, cv_VectorOfVec6f_data_mut, cv_VectorOfVec6f_from_slice,
		cv_VectorOfVec6f_clone,
	}
	
	extern "C" {
		fn cv_VectorOfVec6f_input_array(instance: extern_send!(core::Vector<core::Vec6f>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputArray)>);
		fn cv_VectorOfVec6f_output_array(instance: extern_send!(mut core::Vector<core::Vec6f>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_OutputArray)>);
		fn cv_VectorOfVec6f_input_output_array(instance: extern_send!(mut core::Vector<core::Vec6f>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputOutputArray)>);
	}
	
	impl core::ToInputArray for core::Vector<core::Vec6f> {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVec6f_input_array(self.as_raw_VectorOfVec6f(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for core::Vector<core::Vec6f> {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVec6f_output_array(self.as_raw_mut_VectorOfVec6f(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for core::Vector<core::Vec6f> {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVec6f_input_output_array(self.as_raw_mut_VectorOfVec6f(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { core::Vector<core::Vec6f> }
	
	pub type VectorOfVectorOfDMatch = core::Vector<core::Vector<core::DMatch>>;
	
	impl core::Vector<core::Vector<core::DMatch>> {
		pub fn as_raw_VectorOfVectorOfDMatch(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfDMatch(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector<core::DMatch>,
		cv_VectorOfVectorOfDMatch_new, cv_VectorOfVectorOfDMatch_delete,
		cv_VectorOfVectorOfDMatch_len, cv_VectorOfVectorOfDMatch_is_empty,
		cv_VectorOfVectorOfDMatch_capacity, cv_VectorOfVectorOfDMatch_shrink_to_fit,
		cv_VectorOfVectorOfDMatch_reserve, cv_VectorOfVectorOfDMatch_remove,
		cv_VectorOfVectorOfDMatch_swap, cv_VectorOfVectorOfDMatch_clear,
		cv_VectorOfVectorOfDMatch_get, cv_VectorOfVectorOfDMatch_set,
		cv_VectorOfVectorOfDMatch_push, cv_VectorOfVectorOfDMatch_insert,
	}
	vector_non_copy_or_bool! { clone core::Vector<core::DMatch> }
	
	pub type VectorOfVectorOfKeyPoint = core::Vector<core::Vector<core::KeyPoint>>;
	
	impl core::Vector<core::Vector<core::KeyPoint>> {
		pub fn as_raw_VectorOfVectorOfKeyPoint(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfKeyPoint(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector<core::KeyPoint>,
		cv_VectorOfVectorOfKeyPoint_new, cv_VectorOfVectorOfKeyPoint_delete,
		cv_VectorOfVectorOfKeyPoint_len, cv_VectorOfVectorOfKeyPoint_is_empty,
		cv_VectorOfVectorOfKeyPoint_capacity, cv_VectorOfVectorOfKeyPoint_shrink_to_fit,
		cv_VectorOfVectorOfKeyPoint_reserve, cv_VectorOfVectorOfKeyPoint_remove,
		cv_VectorOfVectorOfKeyPoint_swap, cv_VectorOfVectorOfKeyPoint_clear,
		cv_VectorOfVectorOfKeyPoint_get, cv_VectorOfVectorOfKeyPoint_set,
		cv_VectorOfVectorOfKeyPoint_push, cv_VectorOfVectorOfKeyPoint_insert,
	}
	vector_non_copy_or_bool! { core::Vector<core::KeyPoint> }
	
	pub type VectorOfVectorOfMat = core::Vector<core::Vector<core::Mat>>;
	
	impl core::Vector<core::Vector<core::Mat>> {
		pub fn as_raw_VectorOfVectorOfMat(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfMat(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector<core::Mat>,
		cv_VectorOfVectorOfMat_new, cv_VectorOfVectorOfMat_delete,
		cv_VectorOfVectorOfMat_len, cv_VectorOfVectorOfMat_is_empty,
		cv_VectorOfVectorOfMat_capacity, cv_VectorOfVectorOfMat_shrink_to_fit,
		cv_VectorOfVectorOfMat_reserve, cv_VectorOfVectorOfMat_remove,
		cv_VectorOfVectorOfMat_swap, cv_VectorOfVectorOfMat_clear,
		cv_VectorOfVectorOfMat_get, cv_VectorOfVectorOfMat_set,
		cv_VectorOfVectorOfMat_push, cv_VectorOfVectorOfMat_insert,
	}
	vector_non_copy_or_bool! { clone core::Vector<core::Mat> }
	
	pub type VectorOfVectorOfPoint = core::Vector<core::Vector<core::Point>>;
	
	impl core::Vector<core::Vector<core::Point>> {
		pub fn as_raw_VectorOfVectorOfPoint(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfPoint(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector<core::Point>,
		cv_VectorOfVectorOfPoint_new, cv_VectorOfVectorOfPoint_delete,
		cv_VectorOfVectorOfPoint_len, cv_VectorOfVectorOfPoint_is_empty,
		cv_VectorOfVectorOfPoint_capacity, cv_VectorOfVectorOfPoint_shrink_to_fit,
		cv_VectorOfVectorOfPoint_reserve, cv_VectorOfVectorOfPoint_remove,
		cv_VectorOfVectorOfPoint_swap, cv_VectorOfVectorOfPoint_clear,
		cv_VectorOfVectorOfPoint_get, cv_VectorOfVectorOfPoint_set,
		cv_VectorOfVectorOfPoint_push, cv_VectorOfVectorOfPoint_insert,
	}
	vector_non_copy_or_bool! { clone core::Vector<core::Point> }
	
	extern "C" {
		fn cv_VectorOfVectorOfPoint_input_array(instance: extern_send!(core::Vector<core::Vector<core::Point>>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputArray)>);
		fn cv_VectorOfVectorOfPoint_output_array(instance: extern_send!(mut core::Vector<core::Vector<core::Point>>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_OutputArray)>);
		fn cv_VectorOfVectorOfPoint_input_output_array(instance: extern_send!(mut core::Vector<core::Vector<core::Point>>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputOutputArray)>);
	}
	
	impl core::ToInputArray for core::Vector<core::Vector<core::Point>> {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfPoint_input_array(self.as_raw_VectorOfVectorOfPoint(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for core::Vector<core::Vector<core::Point>> {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfPoint_output_array(self.as_raw_mut_VectorOfVectorOfPoint(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for core::Vector<core::Vector<core::Point>> {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfPoint_input_output_array(self.as_raw_mut_VectorOfVectorOfPoint(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { core::Vector<core::Vector<core::Point>> }
	
	pub type VectorOfVectorOfPoint2f = core::Vector<core::Vector<core::Point2f>>;
	
	impl core::Vector<core::Vector<core::Point2f>> {
		pub fn as_raw_VectorOfVectorOfPoint2f(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfPoint2f(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector<core::Point2f>,
		cv_VectorOfVectorOfPoint2f_new, cv_VectorOfVectorOfPoint2f_delete,
		cv_VectorOfVectorOfPoint2f_len, cv_VectorOfVectorOfPoint2f_is_empty,
		cv_VectorOfVectorOfPoint2f_capacity, cv_VectorOfVectorOfPoint2f_shrink_to_fit,
		cv_VectorOfVectorOfPoint2f_reserve, cv_VectorOfVectorOfPoint2f_remove,
		cv_VectorOfVectorOfPoint2f_swap, cv_VectorOfVectorOfPoint2f_clear,
		cv_VectorOfVectorOfPoint2f_get, cv_VectorOfVectorOfPoint2f_set,
		cv_VectorOfVectorOfPoint2f_push, cv_VectorOfVectorOfPoint2f_insert,
	}
	vector_non_copy_or_bool! { clone core::Vector<core::Point2f> }
	
	extern "C" {
		fn cv_VectorOfVectorOfPoint2f_input_array(instance: extern_send!(core::Vector<core::Vector<core::Point2f>>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputArray)>);
		fn cv_VectorOfVectorOfPoint2f_output_array(instance: extern_send!(mut core::Vector<core::Vector<core::Point2f>>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_OutputArray)>);
		fn cv_VectorOfVectorOfPoint2f_input_output_array(instance: extern_send!(mut core::Vector<core::Vector<core::Point2f>>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputOutputArray)>);
	}
	
	impl core::ToInputArray for core::Vector<core::Vector<core::Point2f>> {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfPoint2f_input_array(self.as_raw_VectorOfVectorOfPoint2f(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for core::Vector<core::Vector<core::Point2f>> {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfPoint2f_output_array(self.as_raw_mut_VectorOfVectorOfPoint2f(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for core::Vector<core::Vector<core::Point2f>> {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfPoint2f_input_output_array(self.as_raw_mut_VectorOfVectorOfPoint2f(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { core::Vector<core::Vector<core::Point2f>> }
	
	pub type VectorOfVectorOfPoint3d = core::Vector<core::Vector<core::Point3d>>;
	
	impl core::Vector<core::Vector<core::Point3d>> {
		pub fn as_raw_VectorOfVectorOfPoint3d(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfPoint3d(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector<core::Point3d>,
		cv_VectorOfVectorOfPoint3d_new, cv_VectorOfVectorOfPoint3d_delete,
		cv_VectorOfVectorOfPoint3d_len, cv_VectorOfVectorOfPoint3d_is_empty,
		cv_VectorOfVectorOfPoint3d_capacity, cv_VectorOfVectorOfPoint3d_shrink_to_fit,
		cv_VectorOfVectorOfPoint3d_reserve, cv_VectorOfVectorOfPoint3d_remove,
		cv_VectorOfVectorOfPoint3d_swap, cv_VectorOfVectorOfPoint3d_clear,
		cv_VectorOfVectorOfPoint3d_get, cv_VectorOfVectorOfPoint3d_set,
		cv_VectorOfVectorOfPoint3d_push, cv_VectorOfVectorOfPoint3d_insert,
	}
	vector_non_copy_or_bool! { clone core::Vector<core::Point3d> }
	
	extern "C" {
		fn cv_VectorOfVectorOfPoint3d_input_array(instance: extern_send!(core::Vector<core::Vector<core::Point3d>>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputArray)>);
		fn cv_VectorOfVectorOfPoint3d_output_array(instance: extern_send!(mut core::Vector<core::Vector<core::Point3d>>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_OutputArray)>);
		fn cv_VectorOfVectorOfPoint3d_input_output_array(instance: extern_send!(mut core::Vector<core::Vector<core::Point3d>>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputOutputArray)>);
	}
	
	impl core::ToInputArray for core::Vector<core::Vector<core::Point3d>> {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfPoint3d_input_array(self.as_raw_VectorOfVectorOfPoint3d(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for core::Vector<core::Vector<core::Point3d>> {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfPoint3d_output_array(self.as_raw_mut_VectorOfVectorOfPoint3d(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for core::Vector<core::Vector<core::Point3d>> {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfPoint3d_input_output_array(self.as_raw_mut_VectorOfVectorOfPoint3d(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { core::Vector<core::Vector<core::Point3d>> }
	
	pub type VectorOfVectorOfPoint3f = core::Vector<core::Vector<core::Point3f>>;
	
	impl core::Vector<core::Vector<core::Point3f>> {
		pub fn as_raw_VectorOfVectorOfPoint3f(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfPoint3f(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector<core::Point3f>,
		cv_VectorOfVectorOfPoint3f_new, cv_VectorOfVectorOfPoint3f_delete,
		cv_VectorOfVectorOfPoint3f_len, cv_VectorOfVectorOfPoint3f_is_empty,
		cv_VectorOfVectorOfPoint3f_capacity, cv_VectorOfVectorOfPoint3f_shrink_to_fit,
		cv_VectorOfVectorOfPoint3f_reserve, cv_VectorOfVectorOfPoint3f_remove,
		cv_VectorOfVectorOfPoint3f_swap, cv_VectorOfVectorOfPoint3f_clear,
		cv_VectorOfVectorOfPoint3f_get, cv_VectorOfVectorOfPoint3f_set,
		cv_VectorOfVectorOfPoint3f_push, cv_VectorOfVectorOfPoint3f_insert,
	}
	vector_non_copy_or_bool! { clone core::Vector<core::Point3f> }
	
	extern "C" {
		fn cv_VectorOfVectorOfPoint3f_input_array(instance: extern_send!(core::Vector<core::Vector<core::Point3f>>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputArray)>);
		fn cv_VectorOfVectorOfPoint3f_output_array(instance: extern_send!(mut core::Vector<core::Vector<core::Point3f>>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_OutputArray)>);
		fn cv_VectorOfVectorOfPoint3f_input_output_array(instance: extern_send!(mut core::Vector<core::Vector<core::Point3f>>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputOutputArray)>);
	}
	
	impl core::ToInputArray for core::Vector<core::Vector<core::Point3f>> {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfPoint3f_input_array(self.as_raw_VectorOfVectorOfPoint3f(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for core::Vector<core::Vector<core::Point3f>> {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfPoint3f_output_array(self.as_raw_mut_VectorOfVectorOfPoint3f(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for core::Vector<core::Vector<core::Point3f>> {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfPoint3f_input_output_array(self.as_raw_mut_VectorOfVectorOfPoint3f(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { core::Vector<core::Vector<core::Point3f>> }
	
	pub type VectorOfVectorOfPoint3i = core::Vector<core::Vector<core::Point3i>>;
	
	impl core::Vector<core::Vector<core::Point3i>> {
		pub fn as_raw_VectorOfVectorOfPoint3i(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfPoint3i(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector<core::Point3i>,
		cv_VectorOfVectorOfPoint3i_new, cv_VectorOfVectorOfPoint3i_delete,
		cv_VectorOfVectorOfPoint3i_len, cv_VectorOfVectorOfPoint3i_is_empty,
		cv_VectorOfVectorOfPoint3i_capacity, cv_VectorOfVectorOfPoint3i_shrink_to_fit,
		cv_VectorOfVectorOfPoint3i_reserve, cv_VectorOfVectorOfPoint3i_remove,
		cv_VectorOfVectorOfPoint3i_swap, cv_VectorOfVectorOfPoint3i_clear,
		cv_VectorOfVectorOfPoint3i_get, cv_VectorOfVectorOfPoint3i_set,
		cv_VectorOfVectorOfPoint3i_push, cv_VectorOfVectorOfPoint3i_insert,
	}
	vector_non_copy_or_bool! { clone core::Vector<core::Point3i> }
	
	extern "C" {
		fn cv_VectorOfVectorOfPoint3i_input_array(instance: extern_send!(core::Vector<core::Vector<core::Point3i>>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputArray)>);
		fn cv_VectorOfVectorOfPoint3i_output_array(instance: extern_send!(mut core::Vector<core::Vector<core::Point3i>>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_OutputArray)>);
		fn cv_VectorOfVectorOfPoint3i_input_output_array(instance: extern_send!(mut core::Vector<core::Vector<core::Point3i>>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputOutputArray)>);
	}
	
	impl core::ToInputArray for core::Vector<core::Vector<core::Point3i>> {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfPoint3i_input_array(self.as_raw_VectorOfVectorOfPoint3i(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for core::Vector<core::Vector<core::Point3i>> {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfPoint3i_output_array(self.as_raw_mut_VectorOfVectorOfPoint3i(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for core::Vector<core::Vector<core::Point3i>> {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfPoint3i_input_output_array(self.as_raw_mut_VectorOfVectorOfPoint3i(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { core::Vector<core::Vector<core::Point3i>> }
	
	pub type VectorOfVectorOfRange = core::Vector<core::Vector<core::Range>>;
	
	impl core::Vector<core::Vector<core::Range>> {
		pub fn as_raw_VectorOfVectorOfRange(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfRange(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector<core::Range>,
		cv_VectorOfVectorOfRange_new, cv_VectorOfVectorOfRange_delete,
		cv_VectorOfVectorOfRange_len, cv_VectorOfVectorOfRange_is_empty,
		cv_VectorOfVectorOfRange_capacity, cv_VectorOfVectorOfRange_shrink_to_fit,
		cv_VectorOfVectorOfRange_reserve, cv_VectorOfVectorOfRange_remove,
		cv_VectorOfVectorOfRange_swap, cv_VectorOfVectorOfRange_clear,
		cv_VectorOfVectorOfRange_get, cv_VectorOfVectorOfRange_set,
		cv_VectorOfVectorOfRange_push, cv_VectorOfVectorOfRange_insert,
	}
	vector_non_copy_or_bool! { core::Vector<core::Range> }
	
	pub type VectorOfVectorOfVec3f = core::Vector<core::Vector<core::Vec3f>>;
	
	impl core::Vector<core::Vector<core::Vec3f>> {
		pub fn as_raw_VectorOfVectorOfVec3f(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfVec3f(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector<core::Vec3f>,
		cv_VectorOfVectorOfVec3f_new, cv_VectorOfVectorOfVec3f_delete,
		cv_VectorOfVectorOfVec3f_len, cv_VectorOfVectorOfVec3f_is_empty,
		cv_VectorOfVectorOfVec3f_capacity, cv_VectorOfVectorOfVec3f_shrink_to_fit,
		cv_VectorOfVectorOfVec3f_reserve, cv_VectorOfVectorOfVec3f_remove,
		cv_VectorOfVectorOfVec3f_swap, cv_VectorOfVectorOfVec3f_clear,
		cv_VectorOfVectorOfVec3f_get, cv_VectorOfVectorOfVec3f_set,
		cv_VectorOfVectorOfVec3f_push, cv_VectorOfVectorOfVec3f_insert,
	}
	vector_non_copy_or_bool! { clone core::Vector<core::Vec3f> }
	
	extern "C" {
		fn cv_VectorOfVectorOfVec3f_input_array(instance: extern_send!(core::Vector<core::Vector<core::Vec3f>>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputArray)>);
		fn cv_VectorOfVectorOfVec3f_output_array(instance: extern_send!(mut core::Vector<core::Vector<core::Vec3f>>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_OutputArray)>);
		fn cv_VectorOfVectorOfVec3f_input_output_array(instance: extern_send!(mut core::Vector<core::Vector<core::Vec3f>>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputOutputArray)>);
	}
	
	impl core::ToInputArray for core::Vector<core::Vector<core::Vec3f>> {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfVec3f_input_array(self.as_raw_VectorOfVectorOfVec3f(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for core::Vector<core::Vector<core::Vec3f>> {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfVec3f_output_array(self.as_raw_mut_VectorOfVectorOfVec3f(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for core::Vector<core::Vector<core::Vec3f>> {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfVec3f_input_output_array(self.as_raw_mut_VectorOfVectorOfVec3f(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { core::Vector<core::Vector<core::Vec3f>> }
	
	pub type VectorOfVectorOff32 = core::Vector<core::Vector<f32>>;
	
	impl core::Vector<core::Vector<f32>> {
		pub fn as_raw_VectorOfVectorOff32(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOff32(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector<f32>,
		cv_VectorOfVectorOff32_new, cv_VectorOfVectorOff32_delete,
		cv_VectorOfVectorOff32_len, cv_VectorOfVectorOff32_is_empty,
		cv_VectorOfVectorOff32_capacity, cv_VectorOfVectorOff32_shrink_to_fit,
		cv_VectorOfVectorOff32_reserve, cv_VectorOfVectorOff32_remove,
		cv_VectorOfVectorOff32_swap, cv_VectorOfVectorOff32_clear,
		cv_VectorOfVectorOff32_get, cv_VectorOfVectorOff32_set,
		cv_VectorOfVectorOff32_push, cv_VectorOfVectorOff32_insert,
	}
	vector_non_copy_or_bool! { clone core::Vector<f32> }
	
	extern "C" {
		fn cv_VectorOfVectorOff32_input_array(instance: extern_send!(core::Vector<core::Vector<f32>>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputArray)>);
		fn cv_VectorOfVectorOff32_output_array(instance: extern_send!(mut core::Vector<core::Vector<f32>>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_OutputArray)>);
		fn cv_VectorOfVectorOff32_input_output_array(instance: extern_send!(mut core::Vector<core::Vector<f32>>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputOutputArray)>);
	}
	
	impl core::ToInputArray for core::Vector<core::Vector<f32>> {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOff32_input_array(self.as_raw_VectorOfVectorOff32(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for core::Vector<core::Vector<f32>> {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOff32_output_array(self.as_raw_mut_VectorOfVectorOff32(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for core::Vector<core::Vector<f32>> {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOff32_input_output_array(self.as_raw_mut_VectorOfVectorOff32(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { core::Vector<core::Vector<f32>> }
	
	pub type VectorOfVectorOff64 = core::Vector<core::Vector<f64>>;
	
	impl core::Vector<core::Vector<f64>> {
		pub fn as_raw_VectorOfVectorOff64(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOff64(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector<f64>,
		cv_VectorOfVectorOff64_new, cv_VectorOfVectorOff64_delete,
		cv_VectorOfVectorOff64_len, cv_VectorOfVectorOff64_is_empty,
		cv_VectorOfVectorOff64_capacity, cv_VectorOfVectorOff64_shrink_to_fit,
		cv_VectorOfVectorOff64_reserve, cv_VectorOfVectorOff64_remove,
		cv_VectorOfVectorOff64_swap, cv_VectorOfVectorOff64_clear,
		cv_VectorOfVectorOff64_get, cv_VectorOfVectorOff64_set,
		cv_VectorOfVectorOff64_push, cv_VectorOfVectorOff64_insert,
	}
	vector_non_copy_or_bool! { clone core::Vector<f64> }
	
	extern "C" {
		fn cv_VectorOfVectorOff64_input_array(instance: extern_send!(core::Vector<core::Vector<f64>>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputArray)>);
		fn cv_VectorOfVectorOff64_output_array(instance: extern_send!(mut core::Vector<core::Vector<f64>>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_OutputArray)>);
		fn cv_VectorOfVectorOff64_input_output_array(instance: extern_send!(mut core::Vector<core::Vector<f64>>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputOutputArray)>);
	}
	
	impl core::ToInputArray for core::Vector<core::Vector<f64>> {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOff64_input_array(self.as_raw_VectorOfVectorOff64(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for core::Vector<core::Vector<f64>> {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOff64_output_array(self.as_raw_mut_VectorOfVectorOff64(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for core::Vector<core::Vector<f64>> {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOff64_input_output_array(self.as_raw_mut_VectorOfVectorOff64(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { core::Vector<core::Vector<f64>> }
	
	pub type VectorOfVectorOfi32 = core::Vector<core::Vector<i32>>;
	
	impl core::Vector<core::Vector<i32>> {
		pub fn as_raw_VectorOfVectorOfi32(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfi32(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector<i32>,
		cv_VectorOfVectorOfi32_new, cv_VectorOfVectorOfi32_delete,
		cv_VectorOfVectorOfi32_len, cv_VectorOfVectorOfi32_is_empty,
		cv_VectorOfVectorOfi32_capacity, cv_VectorOfVectorOfi32_shrink_to_fit,
		cv_VectorOfVectorOfi32_reserve, cv_VectorOfVectorOfi32_remove,
		cv_VectorOfVectorOfi32_swap, cv_VectorOfVectorOfi32_clear,
		cv_VectorOfVectorOfi32_get, cv_VectorOfVectorOfi32_set,
		cv_VectorOfVectorOfi32_push, cv_VectorOfVectorOfi32_insert,
	}
	vector_non_copy_or_bool! { clone core::Vector<i32> }
	
	extern "C" {
		fn cv_VectorOfVectorOfi32_input_array(instance: extern_send!(core::Vector<core::Vector<i32>>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputArray)>);
		fn cv_VectorOfVectorOfi32_output_array(instance: extern_send!(mut core::Vector<core::Vector<i32>>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_OutputArray)>);
		fn cv_VectorOfVectorOfi32_input_output_array(instance: extern_send!(mut core::Vector<core::Vector<i32>>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputOutputArray)>);
	}
	
	impl core::ToInputArray for core::Vector<core::Vector<i32>> {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfi32_input_array(self.as_raw_VectorOfVectorOfi32(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for core::Vector<core::Vector<i32>> {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfi32_output_array(self.as_raw_mut_VectorOfVectorOfi32(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for core::Vector<core::Vector<i32>> {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfi32_input_output_array(self.as_raw_mut_VectorOfVectorOfi32(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { core::Vector<core::Vector<i32>> }
	
	pub type VectorOfVectorOfi8 = core::Vector<core::Vector<i8>>;
	
	impl core::Vector<core::Vector<i8>> {
		pub fn as_raw_VectorOfVectorOfi8(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfi8(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector<i8>,
		cv_VectorOfVectorOfi8_new, cv_VectorOfVectorOfi8_delete,
		cv_VectorOfVectorOfi8_len, cv_VectorOfVectorOfi8_is_empty,
		cv_VectorOfVectorOfi8_capacity, cv_VectorOfVectorOfi8_shrink_to_fit,
		cv_VectorOfVectorOfi8_reserve, cv_VectorOfVectorOfi8_remove,
		cv_VectorOfVectorOfi8_swap, cv_VectorOfVectorOfi8_clear,
		cv_VectorOfVectorOfi8_get, cv_VectorOfVectorOfi8_set,
		cv_VectorOfVectorOfi8_push, cv_VectorOfVectorOfi8_insert,
	}
	vector_non_copy_or_bool! { clone core::Vector<i8> }
	
	extern "C" {
		fn cv_VectorOfVectorOfi8_input_array(instance: extern_send!(core::Vector<core::Vector<i8>>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputArray)>);
		fn cv_VectorOfVectorOfi8_output_array(instance: extern_send!(mut core::Vector<core::Vector<i8>>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_OutputArray)>);
		fn cv_VectorOfVectorOfi8_input_output_array(instance: extern_send!(mut core::Vector<core::Vector<i8>>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputOutputArray)>);
	}
	
	impl core::ToInputArray for core::Vector<core::Vector<i8>> {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfi8_input_array(self.as_raw_VectorOfVectorOfi8(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for core::Vector<core::Vector<i8>> {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfi8_output_array(self.as_raw_mut_VectorOfVectorOfi8(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for core::Vector<core::Vector<i8>> {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfi8_input_output_array(self.as_raw_mut_VectorOfVectorOfi8(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { core::Vector<core::Vector<i8>> }
	
	pub type VectorOfVectorOfu8 = core::Vector<core::Vector<u8>>;
	
	impl core::Vector<core::Vector<u8>> {
		pub fn as_raw_VectorOfVectorOfu8(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfu8(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector<u8>,
		cv_VectorOfVectorOfu8_new, cv_VectorOfVectorOfu8_delete,
		cv_VectorOfVectorOfu8_len, cv_VectorOfVectorOfu8_is_empty,
		cv_VectorOfVectorOfu8_capacity, cv_VectorOfVectorOfu8_shrink_to_fit,
		cv_VectorOfVectorOfu8_reserve, cv_VectorOfVectorOfu8_remove,
		cv_VectorOfVectorOfu8_swap, cv_VectorOfVectorOfu8_clear,
		cv_VectorOfVectorOfu8_get, cv_VectorOfVectorOfu8_set,
		cv_VectorOfVectorOfu8_push, cv_VectorOfVectorOfu8_insert,
	}
	vector_non_copy_or_bool! { clone core::Vector<u8> }
	
	extern "C" {
		fn cv_VectorOfVectorOfu8_input_array(instance: extern_send!(core::Vector<core::Vector<u8>>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputArray)>);
		fn cv_VectorOfVectorOfu8_output_array(instance: extern_send!(mut core::Vector<core::Vector<u8>>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_OutputArray)>);
		fn cv_VectorOfVectorOfu8_input_output_array(instance: extern_send!(mut core::Vector<core::Vector<u8>>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputOutputArray)>);
	}
	
	impl core::ToInputArray for core::Vector<core::Vector<u8>> {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfu8_input_array(self.as_raw_VectorOfVectorOfu8(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for core::Vector<core::Vector<u8>> {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfu8_output_array(self.as_raw_mut_VectorOfVectorOfu8(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for core::Vector<core::Vector<u8>> {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfu8_input_output_array(self.as_raw_mut_VectorOfVectorOfu8(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { core::Vector<core::Vector<u8>> }
	
	pub type VectorOfbool = core::Vector<bool>;
	
	impl core::Vector<bool> {
		pub fn as_raw_VectorOfbool(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfbool(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { bool,
		cv_VectorOfbool_new, cv_VectorOfbool_delete,
		cv_VectorOfbool_len, cv_VectorOfbool_is_empty,
		cv_VectorOfbool_capacity, cv_VectorOfbool_shrink_to_fit,
		cv_VectorOfbool_reserve, cv_VectorOfbool_remove,
		cv_VectorOfbool_swap, cv_VectorOfbool_clear,
		cv_VectorOfbool_get, cv_VectorOfbool_set,
		cv_VectorOfbool_push, cv_VectorOfbool_insert,
	}
	vector_non_copy_or_bool! { clone bool }
	
	pub type VectorOff32 = core::Vector<f32>;
	
	impl core::Vector<f32> {
		pub fn as_raw_VectorOff32(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOff32(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { f32,
		cv_VectorOff32_new, cv_VectorOff32_delete,
		cv_VectorOff32_len, cv_VectorOff32_is_empty,
		cv_VectorOff32_capacity, cv_VectorOff32_shrink_to_fit,
		cv_VectorOff32_reserve, cv_VectorOff32_remove,
		cv_VectorOff32_swap, cv_VectorOff32_clear,
		cv_VectorOff32_get, cv_VectorOff32_set,
		cv_VectorOff32_push, cv_VectorOff32_insert,
	}
	vector_copy_non_bool! { f32,
		cv_VectorOff32_data, cv_VectorOff32_data_mut, cv_VectorOff32_from_slice,
		cv_VectorOff32_clone,
	}
	
	extern "C" {
		fn cv_VectorOff32_input_array(instance: extern_send!(core::Vector<f32>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputArray)>);
		fn cv_VectorOff32_output_array(instance: extern_send!(mut core::Vector<f32>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_OutputArray)>);
		fn cv_VectorOff32_input_output_array(instance: extern_send!(mut core::Vector<f32>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputOutputArray)>);
	}
	
	impl core::ToInputArray for core::Vector<f32> {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOff32_input_array(self.as_raw_VectorOff32(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for core::Vector<f32> {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOff32_output_array(self.as_raw_mut_VectorOff32(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for core::Vector<f32> {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOff32_input_output_array(self.as_raw_mut_VectorOff32(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { core::Vector<f32> }
	
	pub type VectorOff64 = core::Vector<f64>;
	
	impl core::Vector<f64> {
		pub fn as_raw_VectorOff64(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOff64(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { f64,
		cv_VectorOff64_new, cv_VectorOff64_delete,
		cv_VectorOff64_len, cv_VectorOff64_is_empty,
		cv_VectorOff64_capacity, cv_VectorOff64_shrink_to_fit,
		cv_VectorOff64_reserve, cv_VectorOff64_remove,
		cv_VectorOff64_swap, cv_VectorOff64_clear,
		cv_VectorOff64_get, cv_VectorOff64_set,
		cv_VectorOff64_push, cv_VectorOff64_insert,
	}
	vector_copy_non_bool! { f64,
		cv_VectorOff64_data, cv_VectorOff64_data_mut, cv_VectorOff64_from_slice,
		cv_VectorOff64_clone,
	}
	
	extern "C" {
		fn cv_VectorOff64_input_array(instance: extern_send!(core::Vector<f64>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputArray)>);
		fn cv_VectorOff64_output_array(instance: extern_send!(mut core::Vector<f64>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_OutputArray)>);
		fn cv_VectorOff64_input_output_array(instance: extern_send!(mut core::Vector<f64>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputOutputArray)>);
	}
	
	impl core::ToInputArray for core::Vector<f64> {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOff64_input_array(self.as_raw_VectorOff64(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for core::Vector<f64> {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOff64_output_array(self.as_raw_mut_VectorOff64(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for core::Vector<f64> {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOff64_input_output_array(self.as_raw_mut_VectorOff64(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { core::Vector<f64> }
	
	pub type VectorOfi32 = core::Vector<i32>;
	
	impl core::Vector<i32> {
		pub fn as_raw_VectorOfi32(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfi32(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { i32,
		cv_VectorOfi32_new, cv_VectorOfi32_delete,
		cv_VectorOfi32_len, cv_VectorOfi32_is_empty,
		cv_VectorOfi32_capacity, cv_VectorOfi32_shrink_to_fit,
		cv_VectorOfi32_reserve, cv_VectorOfi32_remove,
		cv_VectorOfi32_swap, cv_VectorOfi32_clear,
		cv_VectorOfi32_get, cv_VectorOfi32_set,
		cv_VectorOfi32_push, cv_VectorOfi32_insert,
	}
	vector_copy_non_bool! { i32,
		cv_VectorOfi32_data, cv_VectorOfi32_data_mut, cv_VectorOfi32_from_slice,
		cv_VectorOfi32_clone,
	}
	
	extern "C" {
		fn cv_VectorOfi32_input_array(instance: extern_send!(core::Vector<i32>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputArray)>);
		fn cv_VectorOfi32_output_array(instance: extern_send!(mut core::Vector<i32>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_OutputArray)>);
		fn cv_VectorOfi32_input_output_array(instance: extern_send!(mut core::Vector<i32>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputOutputArray)>);
	}
	
	impl core::ToInputArray for core::Vector<i32> {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfi32_input_array(self.as_raw_VectorOfi32(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for core::Vector<i32> {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfi32_output_array(self.as_raw_mut_VectorOfi32(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for core::Vector<i32> {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfi32_input_output_array(self.as_raw_mut_VectorOfi32(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { core::Vector<i32> }
	
	pub type VectorOfi8 = core::Vector<i8>;
	
	impl core::Vector<i8> {
		pub fn as_raw_VectorOfi8(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfi8(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { i8,
		cv_VectorOfi8_new, cv_VectorOfi8_delete,
		cv_VectorOfi8_len, cv_VectorOfi8_is_empty,
		cv_VectorOfi8_capacity, cv_VectorOfi8_shrink_to_fit,
		cv_VectorOfi8_reserve, cv_VectorOfi8_remove,
		cv_VectorOfi8_swap, cv_VectorOfi8_clear,
		cv_VectorOfi8_get, cv_VectorOfi8_set,
		cv_VectorOfi8_push, cv_VectorOfi8_insert,
	}
	vector_copy_non_bool! { i8,
		cv_VectorOfi8_data, cv_VectorOfi8_data_mut, cv_VectorOfi8_from_slice,
		cv_VectorOfi8_clone,
	}
	
	extern "C" {
		fn cv_VectorOfi8_input_array(instance: extern_send!(core::Vector<i8>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputArray)>);
		fn cv_VectorOfi8_output_array(instance: extern_send!(mut core::Vector<i8>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_OutputArray)>);
		fn cv_VectorOfi8_input_output_array(instance: extern_send!(mut core::Vector<i8>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputOutputArray)>);
	}
	
	impl core::ToInputArray for core::Vector<i8> {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfi8_input_array(self.as_raw_VectorOfi8(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for core::Vector<i8> {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfi8_output_array(self.as_raw_mut_VectorOfi8(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for core::Vector<i8> {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfi8_input_output_array(self.as_raw_mut_VectorOfi8(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { core::Vector<i8> }
	
	pub type VectorOfsize_t = core::Vector<size_t>;
	
	impl core::Vector<size_t> {
		pub fn as_raw_VectorOfsize_t(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfsize_t(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { size_t,
		cv_VectorOfsize_t_new, cv_VectorOfsize_t_delete,
		cv_VectorOfsize_t_len, cv_VectorOfsize_t_is_empty,
		cv_VectorOfsize_t_capacity, cv_VectorOfsize_t_shrink_to_fit,
		cv_VectorOfsize_t_reserve, cv_VectorOfsize_t_remove,
		cv_VectorOfsize_t_swap, cv_VectorOfsize_t_clear,
		cv_VectorOfsize_t_get, cv_VectorOfsize_t_set,
		cv_VectorOfsize_t_push, cv_VectorOfsize_t_insert,
	}
	vector_copy_non_bool! { size_t,
		cv_VectorOfsize_t_data, cv_VectorOfsize_t_data_mut, cv_VectorOfsize_t_from_slice,
		cv_VectorOfsize_t_clone,
	}
	
	pub type VectorOfu8 = core::Vector<u8>;
	
	impl core::Vector<u8> {
		pub fn as_raw_VectorOfu8(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfu8(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { u8,
		cv_VectorOfu8_new, cv_VectorOfu8_delete,
		cv_VectorOfu8_len, cv_VectorOfu8_is_empty,
		cv_VectorOfu8_capacity, cv_VectorOfu8_shrink_to_fit,
		cv_VectorOfu8_reserve, cv_VectorOfu8_remove,
		cv_VectorOfu8_swap, cv_VectorOfu8_clear,
		cv_VectorOfu8_get, cv_VectorOfu8_set,
		cv_VectorOfu8_push, cv_VectorOfu8_insert,
	}
	vector_copy_non_bool! { u8,
		cv_VectorOfu8_data, cv_VectorOfu8_data_mut, cv_VectorOfu8_from_slice,
		cv_VectorOfu8_clone,
	}
	
	extern "C" {
		fn cv_VectorOfu8_input_array(instance: extern_send!(core::Vector<u8>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputArray)>);
		fn cv_VectorOfu8_output_array(instance: extern_send!(mut core::Vector<u8>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_OutputArray)>);
		fn cv_VectorOfu8_input_output_array(instance: extern_send!(mut core::Vector<u8>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputOutputArray)>);
	}
	
	impl core::ToInputArray for core::Vector<u8> {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfu8_input_array(self.as_raw_VectorOfu8(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for core::Vector<u8> {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfu8_output_array(self.as_raw_mut_VectorOfu8(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for core::Vector<u8> {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfu8_input_output_array(self.as_raw_mut_VectorOfu8(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { core::Vector<u8> }
	
}
#[cfg(ocvrs_has_module_core)]
pub use core_types::*;

#[cfg(ocvrs_has_module_dnn)]
mod dnn_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfAbsLayer = core::Ptr<crate::dnn::AbsLayer>;
	
	ptr_extern! { crate::dnn::AbsLayer,
		cv_PtrOfAbsLayer_delete, cv_PtrOfAbsLayer_get_inner_ptr, cv_PtrOfAbsLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::AbsLayer, cv_PtrOfAbsLayer_new }
	
	impl core::Ptr<crate::dnn::AbsLayer> {
		#[inline] pub fn as_raw_PtrOfAbsLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfAbsLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::AbsLayerTraitConst for core::Ptr<crate::dnn::AbsLayer> {
		#[inline] fn as_raw_AbsLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::AbsLayerTrait for core::Ptr<crate::dnn::AbsLayer> {
		#[inline] fn as_raw_mut_AbsLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::AbsLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::AbsLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::AbsLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::AbsLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::AbsLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::AbsLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfAccumLayer = core::Ptr<crate::dnn::AccumLayer>;
	
	ptr_extern! { crate::dnn::AccumLayer,
		cv_PtrOfAccumLayer_delete, cv_PtrOfAccumLayer_get_inner_ptr, cv_PtrOfAccumLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::AccumLayer, cv_PtrOfAccumLayer_new }
	
	impl core::Ptr<crate::dnn::AccumLayer> {
		#[inline] pub fn as_raw_PtrOfAccumLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfAccumLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::AccumLayerTraitConst for core::Ptr<crate::dnn::AccumLayer> {
		#[inline] fn as_raw_AccumLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::AccumLayerTrait for core::Ptr<crate::dnn::AccumLayer> {
		#[inline] fn as_raw_mut_AccumLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::AccumLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::AccumLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::AccumLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::AccumLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfAcosLayer = core::Ptr<crate::dnn::AcosLayer>;
	
	ptr_extern! { crate::dnn::AcosLayer,
		cv_PtrOfAcosLayer_delete, cv_PtrOfAcosLayer_get_inner_ptr, cv_PtrOfAcosLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::AcosLayer, cv_PtrOfAcosLayer_new }
	
	impl core::Ptr<crate::dnn::AcosLayer> {
		#[inline] pub fn as_raw_PtrOfAcosLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfAcosLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::AcosLayerTraitConst for core::Ptr<crate::dnn::AcosLayer> {
		#[inline] fn as_raw_AcosLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::AcosLayerTrait for core::Ptr<crate::dnn::AcosLayer> {
		#[inline] fn as_raw_mut_AcosLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::AcosLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::AcosLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::AcosLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::AcosLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::AcosLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::AcosLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfAcoshLayer = core::Ptr<crate::dnn::AcoshLayer>;
	
	ptr_extern! { crate::dnn::AcoshLayer,
		cv_PtrOfAcoshLayer_delete, cv_PtrOfAcoshLayer_get_inner_ptr, cv_PtrOfAcoshLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::AcoshLayer, cv_PtrOfAcoshLayer_new }
	
	impl core::Ptr<crate::dnn::AcoshLayer> {
		#[inline] pub fn as_raw_PtrOfAcoshLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfAcoshLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::AcoshLayerTraitConst for core::Ptr<crate::dnn::AcoshLayer> {
		#[inline] fn as_raw_AcoshLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::AcoshLayerTrait for core::Ptr<crate::dnn::AcoshLayer> {
		#[inline] fn as_raw_mut_AcoshLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::AcoshLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::AcoshLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::AcoshLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::AcoshLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::AcoshLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::AcoshLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfActivationLayer = core::Ptr<crate::dnn::ActivationLayer>;
	
	ptr_extern! { crate::dnn::ActivationLayer,
		cv_PtrOfActivationLayer_delete, cv_PtrOfActivationLayer_get_inner_ptr, cv_PtrOfActivationLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::ActivationLayer, cv_PtrOfActivationLayer_new }
	
	impl core::Ptr<crate::dnn::ActivationLayer> {
		#[inline] pub fn as_raw_PtrOfActivationLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfActivationLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::ActivationLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::ActivationLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ActivationLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ActivationLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ActivationLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ActivationLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfActivationLayerInt8 = core::Ptr<crate::dnn::ActivationLayerInt8>;
	
	ptr_extern! { crate::dnn::ActivationLayerInt8,
		cv_PtrOfActivationLayerInt8_delete, cv_PtrOfActivationLayerInt8_get_inner_ptr, cv_PtrOfActivationLayerInt8_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::ActivationLayerInt8, cv_PtrOfActivationLayerInt8_new }
	
	impl core::Ptr<crate::dnn::ActivationLayerInt8> {
		#[inline] pub fn as_raw_PtrOfActivationLayerInt8(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfActivationLayerInt8(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerInt8TraitConst for core::Ptr<crate::dnn::ActivationLayerInt8> {
		#[inline] fn as_raw_ActivationLayerInt8(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerInt8Trait for core::Ptr<crate::dnn::ActivationLayerInt8> {
		#[inline] fn as_raw_mut_ActivationLayerInt8(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ActivationLayerInt8> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ActivationLayerInt8> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::ActivationLayerInt8> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::ActivationLayerInt8> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ActivationLayerInt8> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ActivationLayerInt8> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfArgLayer = core::Ptr<crate::dnn::ArgLayer>;
	
	ptr_extern! { crate::dnn::ArgLayer,
		cv_PtrOfArgLayer_delete, cv_PtrOfArgLayer_get_inner_ptr, cv_PtrOfArgLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::ArgLayer, cv_PtrOfArgLayer_new }
	
	impl core::Ptr<crate::dnn::ArgLayer> {
		#[inline] pub fn as_raw_PtrOfArgLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfArgLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::ArgLayerTraitConst for core::Ptr<crate::dnn::ArgLayer> {
		#[inline] fn as_raw_ArgLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ArgLayerTrait for core::Ptr<crate::dnn::ArgLayer> {
		#[inline] fn as_raw_mut_ArgLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ArgLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ArgLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ArgLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ArgLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfAsinLayer = core::Ptr<crate::dnn::AsinLayer>;
	
	ptr_extern! { crate::dnn::AsinLayer,
		cv_PtrOfAsinLayer_delete, cv_PtrOfAsinLayer_get_inner_ptr, cv_PtrOfAsinLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::AsinLayer, cv_PtrOfAsinLayer_new }
	
	impl core::Ptr<crate::dnn::AsinLayer> {
		#[inline] pub fn as_raw_PtrOfAsinLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfAsinLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::AsinLayerTraitConst for core::Ptr<crate::dnn::AsinLayer> {
		#[inline] fn as_raw_AsinLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::AsinLayerTrait for core::Ptr<crate::dnn::AsinLayer> {
		#[inline] fn as_raw_mut_AsinLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::AsinLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::AsinLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::AsinLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::AsinLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::AsinLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::AsinLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfAsinhLayer = core::Ptr<crate::dnn::AsinhLayer>;
	
	ptr_extern! { crate::dnn::AsinhLayer,
		cv_PtrOfAsinhLayer_delete, cv_PtrOfAsinhLayer_get_inner_ptr, cv_PtrOfAsinhLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::AsinhLayer, cv_PtrOfAsinhLayer_new }
	
	impl core::Ptr<crate::dnn::AsinhLayer> {
		#[inline] pub fn as_raw_PtrOfAsinhLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfAsinhLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::AsinhLayerTraitConst for core::Ptr<crate::dnn::AsinhLayer> {
		#[inline] fn as_raw_AsinhLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::AsinhLayerTrait for core::Ptr<crate::dnn::AsinhLayer> {
		#[inline] fn as_raw_mut_AsinhLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::AsinhLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::AsinhLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::AsinhLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::AsinhLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::AsinhLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::AsinhLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfAtanLayer = core::Ptr<crate::dnn::AtanLayer>;
	
	ptr_extern! { crate::dnn::AtanLayer,
		cv_PtrOfAtanLayer_delete, cv_PtrOfAtanLayer_get_inner_ptr, cv_PtrOfAtanLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::AtanLayer, cv_PtrOfAtanLayer_new }
	
	impl core::Ptr<crate::dnn::AtanLayer> {
		#[inline] pub fn as_raw_PtrOfAtanLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfAtanLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::AtanLayerTraitConst for core::Ptr<crate::dnn::AtanLayer> {
		#[inline] fn as_raw_AtanLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::AtanLayerTrait for core::Ptr<crate::dnn::AtanLayer> {
		#[inline] fn as_raw_mut_AtanLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::AtanLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::AtanLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::AtanLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::AtanLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::AtanLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::AtanLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfAtanhLayer = core::Ptr<crate::dnn::AtanhLayer>;
	
	ptr_extern! { crate::dnn::AtanhLayer,
		cv_PtrOfAtanhLayer_delete, cv_PtrOfAtanhLayer_get_inner_ptr, cv_PtrOfAtanhLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::AtanhLayer, cv_PtrOfAtanhLayer_new }
	
	impl core::Ptr<crate::dnn::AtanhLayer> {
		#[inline] pub fn as_raw_PtrOfAtanhLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfAtanhLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::AtanhLayerTraitConst for core::Ptr<crate::dnn::AtanhLayer> {
		#[inline] fn as_raw_AtanhLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::AtanhLayerTrait for core::Ptr<crate::dnn::AtanhLayer> {
		#[inline] fn as_raw_mut_AtanhLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::AtanhLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::AtanhLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::AtanhLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::AtanhLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::AtanhLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::AtanhLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfBNLLLayer = core::Ptr<crate::dnn::BNLLLayer>;
	
	ptr_extern! { crate::dnn::BNLLLayer,
		cv_PtrOfBNLLLayer_delete, cv_PtrOfBNLLLayer_get_inner_ptr, cv_PtrOfBNLLLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::BNLLLayer, cv_PtrOfBNLLLayer_new }
	
	impl core::Ptr<crate::dnn::BNLLLayer> {
		#[inline] pub fn as_raw_PtrOfBNLLLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBNLLLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::BNLLLayerTraitConst for core::Ptr<crate::dnn::BNLLLayer> {
		#[inline] fn as_raw_BNLLLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::BNLLLayerTrait for core::Ptr<crate::dnn::BNLLLayer> {
		#[inline] fn as_raw_mut_BNLLLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::BNLLLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::BNLLLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::BNLLLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::BNLLLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::BNLLLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::BNLLLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfBackendNode = core::Ptr<crate::dnn::BackendNode>;
	
	ptr_extern! { crate::dnn::BackendNode,
		cv_PtrOfBackendNode_delete, cv_PtrOfBackendNode_get_inner_ptr, cv_PtrOfBackendNode_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::BackendNode, cv_PtrOfBackendNode_new }
	
	impl core::Ptr<crate::dnn::BackendNode> {
		#[inline] pub fn as_raw_PtrOfBackendNode(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBackendNode(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::BackendNodeTraitConst for core::Ptr<crate::dnn::BackendNode> {
		#[inline] fn as_raw_BackendNode(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::BackendNodeTrait for core::Ptr<crate::dnn::BackendNode> {
		#[inline] fn as_raw_mut_BackendNode(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfBackendWrapper = core::Ptr<dyn crate::dnn::BackendWrapper>;
	
	ptr_extern! { dyn crate::dnn::BackendWrapper,
		cv_PtrOfBackendWrapper_delete, cv_PtrOfBackendWrapper_get_inner_ptr, cv_PtrOfBackendWrapper_get_inner_ptr_mut
	}
	
	impl core::Ptr<dyn crate::dnn::BackendWrapper> {
		#[inline] pub fn as_raw_PtrOfBackendWrapper(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBackendWrapper(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::BackendWrapperConst for core::Ptr<dyn crate::dnn::BackendWrapper> {
		#[inline] fn as_raw_BackendWrapper(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::BackendWrapper for core::Ptr<dyn crate::dnn::BackendWrapper> {
		#[inline] fn as_raw_mut_BackendWrapper(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfBaseConvolutionLayer = core::Ptr<crate::dnn::BaseConvolutionLayer>;
	
	ptr_extern! { crate::dnn::BaseConvolutionLayer,
		cv_PtrOfBaseConvolutionLayer_delete, cv_PtrOfBaseConvolutionLayer_get_inner_ptr, cv_PtrOfBaseConvolutionLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::BaseConvolutionLayer, cv_PtrOfBaseConvolutionLayer_new }
	
	impl core::Ptr<crate::dnn::BaseConvolutionLayer> {
		#[inline] pub fn as_raw_PtrOfBaseConvolutionLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBaseConvolutionLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::BaseConvolutionLayerTraitConst for core::Ptr<crate::dnn::BaseConvolutionLayer> {
		#[inline] fn as_raw_BaseConvolutionLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::BaseConvolutionLayerTrait for core::Ptr<crate::dnn::BaseConvolutionLayer> {
		#[inline] fn as_raw_mut_BaseConvolutionLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::BaseConvolutionLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::BaseConvolutionLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::BaseConvolutionLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::BaseConvolutionLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfBatchNormLayer = core::Ptr<crate::dnn::BatchNormLayer>;
	
	ptr_extern! { crate::dnn::BatchNormLayer,
		cv_PtrOfBatchNormLayer_delete, cv_PtrOfBatchNormLayer_get_inner_ptr, cv_PtrOfBatchNormLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::BatchNormLayer, cv_PtrOfBatchNormLayer_new }
	
	impl core::Ptr<crate::dnn::BatchNormLayer> {
		#[inline] pub fn as_raw_PtrOfBatchNormLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBatchNormLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::BatchNormLayerTraitConst for core::Ptr<crate::dnn::BatchNormLayer> {
		#[inline] fn as_raw_BatchNormLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::BatchNormLayerTrait for core::Ptr<crate::dnn::BatchNormLayer> {
		#[inline] fn as_raw_mut_BatchNormLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::BatchNormLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::BatchNormLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::BatchNormLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::BatchNormLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::BatchNormLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::BatchNormLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfBatchNormLayerInt8 = core::Ptr<crate::dnn::BatchNormLayerInt8>;
	
	ptr_extern! { crate::dnn::BatchNormLayerInt8,
		cv_PtrOfBatchNormLayerInt8_delete, cv_PtrOfBatchNormLayerInt8_get_inner_ptr, cv_PtrOfBatchNormLayerInt8_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::BatchNormLayerInt8, cv_PtrOfBatchNormLayerInt8_new }
	
	impl core::Ptr<crate::dnn::BatchNormLayerInt8> {
		#[inline] pub fn as_raw_PtrOfBatchNormLayerInt8(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBatchNormLayerInt8(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::BatchNormLayerInt8TraitConst for core::Ptr<crate::dnn::BatchNormLayerInt8> {
		#[inline] fn as_raw_BatchNormLayerInt8(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::BatchNormLayerInt8Trait for core::Ptr<crate::dnn::BatchNormLayerInt8> {
		#[inline] fn as_raw_mut_BatchNormLayerInt8(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::BatchNormLayerInt8> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::BatchNormLayerInt8> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::BatchNormLayerInt8> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::BatchNormLayerInt8> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::BatchNormLayerTraitConst for core::Ptr<crate::dnn::BatchNormLayerInt8> {
		#[inline] fn as_raw_BatchNormLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::BatchNormLayerTrait for core::Ptr<crate::dnn::BatchNormLayerInt8> {
		#[inline] fn as_raw_mut_BatchNormLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::BatchNormLayerInt8> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::BatchNormLayerInt8> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCeilLayer = core::Ptr<crate::dnn::CeilLayer>;
	
	ptr_extern! { crate::dnn::CeilLayer,
		cv_PtrOfCeilLayer_delete, cv_PtrOfCeilLayer_get_inner_ptr, cv_PtrOfCeilLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::CeilLayer, cv_PtrOfCeilLayer_new }
	
	impl core::Ptr<crate::dnn::CeilLayer> {
		#[inline] pub fn as_raw_PtrOfCeilLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCeilLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::CeilLayerTraitConst for core::Ptr<crate::dnn::CeilLayer> {
		#[inline] fn as_raw_CeilLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::CeilLayerTrait for core::Ptr<crate::dnn::CeilLayer> {
		#[inline] fn as_raw_mut_CeilLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::CeilLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::CeilLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::CeilLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::CeilLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::CeilLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::CeilLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCeluLayer = core::Ptr<crate::dnn::CeluLayer>;
	
	ptr_extern! { crate::dnn::CeluLayer,
		cv_PtrOfCeluLayer_delete, cv_PtrOfCeluLayer_get_inner_ptr, cv_PtrOfCeluLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::CeluLayer, cv_PtrOfCeluLayer_new }
	
	impl core::Ptr<crate::dnn::CeluLayer> {
		#[inline] pub fn as_raw_PtrOfCeluLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCeluLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::CeluLayerTraitConst for core::Ptr<crate::dnn::CeluLayer> {
		#[inline] fn as_raw_CeluLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::CeluLayerTrait for core::Ptr<crate::dnn::CeluLayer> {
		#[inline] fn as_raw_mut_CeluLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::CeluLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::CeluLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::CeluLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::CeluLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::CeluLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::CeluLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfConcatLayer = core::Ptr<crate::dnn::ConcatLayer>;
	
	ptr_extern! { crate::dnn::ConcatLayer,
		cv_PtrOfConcatLayer_delete, cv_PtrOfConcatLayer_get_inner_ptr, cv_PtrOfConcatLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::ConcatLayer, cv_PtrOfConcatLayer_new }
	
	impl core::Ptr<crate::dnn::ConcatLayer> {
		#[inline] pub fn as_raw_PtrOfConcatLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfConcatLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::ConcatLayerTraitConst for core::Ptr<crate::dnn::ConcatLayer> {
		#[inline] fn as_raw_ConcatLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ConcatLayerTrait for core::Ptr<crate::dnn::ConcatLayer> {
		#[inline] fn as_raw_mut_ConcatLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ConcatLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ConcatLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ConcatLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ConcatLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCorrelationLayer = core::Ptr<crate::dnn::CorrelationLayer>;
	
	ptr_extern! { crate::dnn::CorrelationLayer,
		cv_PtrOfCorrelationLayer_delete, cv_PtrOfCorrelationLayer_get_inner_ptr, cv_PtrOfCorrelationLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::CorrelationLayer, cv_PtrOfCorrelationLayer_new }
	
	impl core::Ptr<crate::dnn::CorrelationLayer> {
		#[inline] pub fn as_raw_PtrOfCorrelationLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCorrelationLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::CorrelationLayerTraitConst for core::Ptr<crate::dnn::CorrelationLayer> {
		#[inline] fn as_raw_CorrelationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::CorrelationLayerTrait for core::Ptr<crate::dnn::CorrelationLayer> {
		#[inline] fn as_raw_mut_CorrelationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::CorrelationLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::CorrelationLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::CorrelationLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::CorrelationLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCosLayer = core::Ptr<crate::dnn::CosLayer>;
	
	ptr_extern! { crate::dnn::CosLayer,
		cv_PtrOfCosLayer_delete, cv_PtrOfCosLayer_get_inner_ptr, cv_PtrOfCosLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::CosLayer, cv_PtrOfCosLayer_new }
	
	impl core::Ptr<crate::dnn::CosLayer> {
		#[inline] pub fn as_raw_PtrOfCosLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCosLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::CosLayerTraitConst for core::Ptr<crate::dnn::CosLayer> {
		#[inline] fn as_raw_CosLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::CosLayerTrait for core::Ptr<crate::dnn::CosLayer> {
		#[inline] fn as_raw_mut_CosLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::CosLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::CosLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::CosLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::CosLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::CosLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::CosLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCoshLayer = core::Ptr<crate::dnn::CoshLayer>;
	
	ptr_extern! { crate::dnn::CoshLayer,
		cv_PtrOfCoshLayer_delete, cv_PtrOfCoshLayer_get_inner_ptr, cv_PtrOfCoshLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::CoshLayer, cv_PtrOfCoshLayer_new }
	
	impl core::Ptr<crate::dnn::CoshLayer> {
		#[inline] pub fn as_raw_PtrOfCoshLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCoshLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::CoshLayerTraitConst for core::Ptr<crate::dnn::CoshLayer> {
		#[inline] fn as_raw_CoshLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::CoshLayerTrait for core::Ptr<crate::dnn::CoshLayer> {
		#[inline] fn as_raw_mut_CoshLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::CoshLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::CoshLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::CoshLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::CoshLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::CoshLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::CoshLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCumSumLayer = core::Ptr<crate::dnn::CumSumLayer>;
	
	ptr_extern! { crate::dnn::CumSumLayer,
		cv_PtrOfCumSumLayer_delete, cv_PtrOfCumSumLayer_get_inner_ptr, cv_PtrOfCumSumLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::CumSumLayer, cv_PtrOfCumSumLayer_new }
	
	impl core::Ptr<crate::dnn::CumSumLayer> {
		#[inline] pub fn as_raw_PtrOfCumSumLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCumSumLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::CumSumLayerTraitConst for core::Ptr<crate::dnn::CumSumLayer> {
		#[inline] fn as_raw_CumSumLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::CumSumLayerTrait for core::Ptr<crate::dnn::CumSumLayer> {
		#[inline] fn as_raw_mut_CumSumLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::CumSumLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::CumSumLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::CumSumLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::CumSumLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDataAugmentationLayer = core::Ptr<crate::dnn::DataAugmentationLayer>;
	
	ptr_extern! { crate::dnn::DataAugmentationLayer,
		cv_PtrOfDataAugmentationLayer_delete, cv_PtrOfDataAugmentationLayer_get_inner_ptr, cv_PtrOfDataAugmentationLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::DataAugmentationLayer, cv_PtrOfDataAugmentationLayer_new }
	
	impl core::Ptr<crate::dnn::DataAugmentationLayer> {
		#[inline] pub fn as_raw_PtrOfDataAugmentationLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDataAugmentationLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::DataAugmentationLayerTraitConst for core::Ptr<crate::dnn::DataAugmentationLayer> {
		#[inline] fn as_raw_DataAugmentationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::DataAugmentationLayerTrait for core::Ptr<crate::dnn::DataAugmentationLayer> {
		#[inline] fn as_raw_mut_DataAugmentationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::DataAugmentationLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::DataAugmentationLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::DataAugmentationLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::DataAugmentationLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDequantizeLayer = core::Ptr<crate::dnn::DequantizeLayer>;
	
	ptr_extern! { crate::dnn::DequantizeLayer,
		cv_PtrOfDequantizeLayer_delete, cv_PtrOfDequantizeLayer_get_inner_ptr, cv_PtrOfDequantizeLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::DequantizeLayer, cv_PtrOfDequantizeLayer_new }
	
	impl core::Ptr<crate::dnn::DequantizeLayer> {
		#[inline] pub fn as_raw_PtrOfDequantizeLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDequantizeLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::DequantizeLayerTraitConst for core::Ptr<crate::dnn::DequantizeLayer> {
		#[inline] fn as_raw_DequantizeLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::DequantizeLayerTrait for core::Ptr<crate::dnn::DequantizeLayer> {
		#[inline] fn as_raw_mut_DequantizeLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::DequantizeLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::DequantizeLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::DequantizeLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::DequantizeLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDetectionOutputLayer = core::Ptr<crate::dnn::DetectionOutputLayer>;
	
	ptr_extern! { crate::dnn::DetectionOutputLayer,
		cv_PtrOfDetectionOutputLayer_delete, cv_PtrOfDetectionOutputLayer_get_inner_ptr, cv_PtrOfDetectionOutputLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::DetectionOutputLayer, cv_PtrOfDetectionOutputLayer_new }
	
	impl core::Ptr<crate::dnn::DetectionOutputLayer> {
		#[inline] pub fn as_raw_PtrOfDetectionOutputLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetectionOutputLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::DetectionOutputLayerTraitConst for core::Ptr<crate::dnn::DetectionOutputLayer> {
		#[inline] fn as_raw_DetectionOutputLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::DetectionOutputLayerTrait for core::Ptr<crate::dnn::DetectionOutputLayer> {
		#[inline] fn as_raw_mut_DetectionOutputLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::DetectionOutputLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::DetectionOutputLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::DetectionOutputLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::DetectionOutputLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfELULayer = core::Ptr<crate::dnn::ELULayer>;
	
	ptr_extern! { crate::dnn::ELULayer,
		cv_PtrOfELULayer_delete, cv_PtrOfELULayer_get_inner_ptr, cv_PtrOfELULayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::ELULayer, cv_PtrOfELULayer_new }
	
	impl core::Ptr<crate::dnn::ELULayer> {
		#[inline] pub fn as_raw_PtrOfELULayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfELULayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::ELULayerTraitConst for core::Ptr<crate::dnn::ELULayer> {
		#[inline] fn as_raw_ELULayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ELULayerTrait for core::Ptr<crate::dnn::ELULayer> {
		#[inline] fn as_raw_mut_ELULayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ELULayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ELULayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::ELULayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::ELULayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ELULayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ELULayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfEltwiseLayer = core::Ptr<crate::dnn::EltwiseLayer>;
	
	ptr_extern! { crate::dnn::EltwiseLayer,
		cv_PtrOfEltwiseLayer_delete, cv_PtrOfEltwiseLayer_get_inner_ptr, cv_PtrOfEltwiseLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::EltwiseLayer, cv_PtrOfEltwiseLayer_new }
	
	impl core::Ptr<crate::dnn::EltwiseLayer> {
		#[inline] pub fn as_raw_PtrOfEltwiseLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfEltwiseLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::EltwiseLayerTraitConst for core::Ptr<crate::dnn::EltwiseLayer> {
		#[inline] fn as_raw_EltwiseLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::EltwiseLayerTrait for core::Ptr<crate::dnn::EltwiseLayer> {
		#[inline] fn as_raw_mut_EltwiseLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::EltwiseLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::EltwiseLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::EltwiseLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::EltwiseLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfEltwiseLayerInt8 = core::Ptr<crate::dnn::EltwiseLayerInt8>;
	
	ptr_extern! { crate::dnn::EltwiseLayerInt8,
		cv_PtrOfEltwiseLayerInt8_delete, cv_PtrOfEltwiseLayerInt8_get_inner_ptr, cv_PtrOfEltwiseLayerInt8_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::EltwiseLayerInt8, cv_PtrOfEltwiseLayerInt8_new }
	
	impl core::Ptr<crate::dnn::EltwiseLayerInt8> {
		#[inline] pub fn as_raw_PtrOfEltwiseLayerInt8(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfEltwiseLayerInt8(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::EltwiseLayerInt8TraitConst for core::Ptr<crate::dnn::EltwiseLayerInt8> {
		#[inline] fn as_raw_EltwiseLayerInt8(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::EltwiseLayerInt8Trait for core::Ptr<crate::dnn::EltwiseLayerInt8> {
		#[inline] fn as_raw_mut_EltwiseLayerInt8(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::EltwiseLayerInt8> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::EltwiseLayerInt8> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::EltwiseLayerInt8> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::EltwiseLayerInt8> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfErfLayer = core::Ptr<crate::dnn::ErfLayer>;
	
	ptr_extern! { crate::dnn::ErfLayer,
		cv_PtrOfErfLayer_delete, cv_PtrOfErfLayer_get_inner_ptr, cv_PtrOfErfLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::ErfLayer, cv_PtrOfErfLayer_new }
	
	impl core::Ptr<crate::dnn::ErfLayer> {
		#[inline] pub fn as_raw_PtrOfErfLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfErfLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::ErfLayerTraitConst for core::Ptr<crate::dnn::ErfLayer> {
		#[inline] fn as_raw_ErfLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ErfLayerTrait for core::Ptr<crate::dnn::ErfLayer> {
		#[inline] fn as_raw_mut_ErfLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ErfLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ErfLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::ErfLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::ErfLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ErfLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ErfLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfExpLayer = core::Ptr<crate::dnn::ExpLayer>;
	
	ptr_extern! { crate::dnn::ExpLayer,
		cv_PtrOfExpLayer_delete, cv_PtrOfExpLayer_get_inner_ptr, cv_PtrOfExpLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::ExpLayer, cv_PtrOfExpLayer_new }
	
	impl core::Ptr<crate::dnn::ExpLayer> {
		#[inline] pub fn as_raw_PtrOfExpLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfExpLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::ExpLayerTraitConst for core::Ptr<crate::dnn::ExpLayer> {
		#[inline] fn as_raw_ExpLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ExpLayerTrait for core::Ptr<crate::dnn::ExpLayer> {
		#[inline] fn as_raw_mut_ExpLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ExpLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ExpLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::ExpLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::ExpLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ExpLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ExpLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfFlattenLayer = core::Ptr<crate::dnn::FlattenLayer>;
	
	ptr_extern! { crate::dnn::FlattenLayer,
		cv_PtrOfFlattenLayer_delete, cv_PtrOfFlattenLayer_get_inner_ptr, cv_PtrOfFlattenLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::FlattenLayer, cv_PtrOfFlattenLayer_new }
	
	impl core::Ptr<crate::dnn::FlattenLayer> {
		#[inline] pub fn as_raw_PtrOfFlattenLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFlattenLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::FlattenLayerTraitConst for core::Ptr<crate::dnn::FlattenLayer> {
		#[inline] fn as_raw_FlattenLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::FlattenLayerTrait for core::Ptr<crate::dnn::FlattenLayer> {
		#[inline] fn as_raw_mut_FlattenLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::FlattenLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::FlattenLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::FlattenLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::FlattenLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfFloorLayer = core::Ptr<crate::dnn::FloorLayer>;
	
	ptr_extern! { crate::dnn::FloorLayer,
		cv_PtrOfFloorLayer_delete, cv_PtrOfFloorLayer_get_inner_ptr, cv_PtrOfFloorLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::FloorLayer, cv_PtrOfFloorLayer_new }
	
	impl core::Ptr<crate::dnn::FloorLayer> {
		#[inline] pub fn as_raw_PtrOfFloorLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFloorLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::FloorLayerTraitConst for core::Ptr<crate::dnn::FloorLayer> {
		#[inline] fn as_raw_FloorLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::FloorLayerTrait for core::Ptr<crate::dnn::FloorLayer> {
		#[inline] fn as_raw_mut_FloorLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::FloorLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::FloorLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::FloorLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::FloorLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::FloorLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::FloorLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfFlowWarpLayer = core::Ptr<crate::dnn::FlowWarpLayer>;
	
	ptr_extern! { crate::dnn::FlowWarpLayer,
		cv_PtrOfFlowWarpLayer_delete, cv_PtrOfFlowWarpLayer_get_inner_ptr, cv_PtrOfFlowWarpLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::FlowWarpLayer, cv_PtrOfFlowWarpLayer_new }
	
	impl core::Ptr<crate::dnn::FlowWarpLayer> {
		#[inline] pub fn as_raw_PtrOfFlowWarpLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFlowWarpLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::FlowWarpLayerTraitConst for core::Ptr<crate::dnn::FlowWarpLayer> {
		#[inline] fn as_raw_FlowWarpLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::FlowWarpLayerTrait for core::Ptr<crate::dnn::FlowWarpLayer> {
		#[inline] fn as_raw_mut_FlowWarpLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::FlowWarpLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::FlowWarpLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::FlowWarpLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::FlowWarpLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfGRULayer = core::Ptr<crate::dnn::GRULayer>;
	
	ptr_extern! { crate::dnn::GRULayer,
		cv_PtrOfGRULayer_delete, cv_PtrOfGRULayer_get_inner_ptr, cv_PtrOfGRULayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::GRULayer, cv_PtrOfGRULayer_new }
	
	impl core::Ptr<crate::dnn::GRULayer> {
		#[inline] pub fn as_raw_PtrOfGRULayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfGRULayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::GRULayerTraitConst for core::Ptr<crate::dnn::GRULayer> {
		#[inline] fn as_raw_GRULayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::GRULayerTrait for core::Ptr<crate::dnn::GRULayer> {
		#[inline] fn as_raw_mut_GRULayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::GRULayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::GRULayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::GRULayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::GRULayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfHardSigmoidLayer = core::Ptr<crate::dnn::HardSigmoidLayer>;
	
	ptr_extern! { crate::dnn::HardSigmoidLayer,
		cv_PtrOfHardSigmoidLayer_delete, cv_PtrOfHardSigmoidLayer_get_inner_ptr, cv_PtrOfHardSigmoidLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::HardSigmoidLayer, cv_PtrOfHardSigmoidLayer_new }
	
	impl core::Ptr<crate::dnn::HardSigmoidLayer> {
		#[inline] pub fn as_raw_PtrOfHardSigmoidLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfHardSigmoidLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::HardSigmoidLayerTraitConst for core::Ptr<crate::dnn::HardSigmoidLayer> {
		#[inline] fn as_raw_HardSigmoidLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::HardSigmoidLayerTrait for core::Ptr<crate::dnn::HardSigmoidLayer> {
		#[inline] fn as_raw_mut_HardSigmoidLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::HardSigmoidLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::HardSigmoidLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::HardSigmoidLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::HardSigmoidLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::HardSigmoidLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::HardSigmoidLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfHardSwishLayer = core::Ptr<crate::dnn::HardSwishLayer>;
	
	ptr_extern! { crate::dnn::HardSwishLayer,
		cv_PtrOfHardSwishLayer_delete, cv_PtrOfHardSwishLayer_get_inner_ptr, cv_PtrOfHardSwishLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::HardSwishLayer, cv_PtrOfHardSwishLayer_new }
	
	impl core::Ptr<crate::dnn::HardSwishLayer> {
		#[inline] pub fn as_raw_PtrOfHardSwishLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfHardSwishLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::HardSwishLayerTraitConst for core::Ptr<crate::dnn::HardSwishLayer> {
		#[inline] fn as_raw_HardSwishLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::HardSwishLayerTrait for core::Ptr<crate::dnn::HardSwishLayer> {
		#[inline] fn as_raw_mut_HardSwishLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::HardSwishLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::HardSwishLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::HardSwishLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::HardSwishLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::HardSwishLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::HardSwishLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfInnerProductLayer = core::Ptr<crate::dnn::InnerProductLayer>;
	
	ptr_extern! { crate::dnn::InnerProductLayer,
		cv_PtrOfInnerProductLayer_delete, cv_PtrOfInnerProductLayer_get_inner_ptr, cv_PtrOfInnerProductLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::InnerProductLayer, cv_PtrOfInnerProductLayer_new }
	
	impl core::Ptr<crate::dnn::InnerProductLayer> {
		#[inline] pub fn as_raw_PtrOfInnerProductLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfInnerProductLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::InnerProductLayerTraitConst for core::Ptr<crate::dnn::InnerProductLayer> {
		#[inline] fn as_raw_InnerProductLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::InnerProductLayerTrait for core::Ptr<crate::dnn::InnerProductLayer> {
		#[inline] fn as_raw_mut_InnerProductLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::InnerProductLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::InnerProductLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::InnerProductLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::InnerProductLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfInnerProductLayerInt8 = core::Ptr<crate::dnn::InnerProductLayerInt8>;
	
	ptr_extern! { crate::dnn::InnerProductLayerInt8,
		cv_PtrOfInnerProductLayerInt8_delete, cv_PtrOfInnerProductLayerInt8_get_inner_ptr, cv_PtrOfInnerProductLayerInt8_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::InnerProductLayerInt8, cv_PtrOfInnerProductLayerInt8_new }
	
	impl core::Ptr<crate::dnn::InnerProductLayerInt8> {
		#[inline] pub fn as_raw_PtrOfInnerProductLayerInt8(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfInnerProductLayerInt8(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::InnerProductLayerInt8TraitConst for core::Ptr<crate::dnn::InnerProductLayerInt8> {
		#[inline] fn as_raw_InnerProductLayerInt8(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::InnerProductLayerInt8Trait for core::Ptr<crate::dnn::InnerProductLayerInt8> {
		#[inline] fn as_raw_mut_InnerProductLayerInt8(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::InnerProductLayerInt8> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::InnerProductLayerInt8> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::InnerProductLayerTraitConst for core::Ptr<crate::dnn::InnerProductLayerInt8> {
		#[inline] fn as_raw_InnerProductLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::InnerProductLayerTrait for core::Ptr<crate::dnn::InnerProductLayerInt8> {
		#[inline] fn as_raw_mut_InnerProductLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::InnerProductLayerInt8> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::InnerProductLayerInt8> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfLRNLayer = core::Ptr<crate::dnn::LRNLayer>;
	
	ptr_extern! { crate::dnn::LRNLayer,
		cv_PtrOfLRNLayer_delete, cv_PtrOfLRNLayer_get_inner_ptr, cv_PtrOfLRNLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::LRNLayer, cv_PtrOfLRNLayer_new }
	
	impl core::Ptr<crate::dnn::LRNLayer> {
		#[inline] pub fn as_raw_PtrOfLRNLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfLRNLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::LRNLayerTraitConst for core::Ptr<crate::dnn::LRNLayer> {
		#[inline] fn as_raw_LRNLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LRNLayerTrait for core::Ptr<crate::dnn::LRNLayer> {
		#[inline] fn as_raw_mut_LRNLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::LRNLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::LRNLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::LRNLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::LRNLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfLSTMLayer = core::Ptr<dyn crate::dnn::LSTMLayer>;
	
	ptr_extern! { dyn crate::dnn::LSTMLayer,
		cv_PtrOfLSTMLayer_delete, cv_PtrOfLSTMLayer_get_inner_ptr, cv_PtrOfLSTMLayer_get_inner_ptr_mut
	}
	
	impl core::Ptr<dyn crate::dnn::LSTMLayer> {
		#[inline] pub fn as_raw_PtrOfLSTMLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfLSTMLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::LSTMLayerConst for core::Ptr<dyn crate::dnn::LSTMLayer> {
		#[inline] fn as_raw_LSTMLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LSTMLayer for core::Ptr<dyn crate::dnn::LSTMLayer> {
		#[inline] fn as_raw_mut_LSTMLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<dyn crate::dnn::LSTMLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<dyn crate::dnn::LSTMLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<dyn crate::dnn::LSTMLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<dyn crate::dnn::LSTMLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfLayer = core::Ptr<crate::dnn::Layer>;
	
	ptr_extern! { crate::dnn::Layer,
		cv_PtrOfLayer_delete, cv_PtrOfLayer_get_inner_ptr, cv_PtrOfLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::Layer, cv_PtrOfLayer_new }
	
	impl core::Ptr<crate::dnn::Layer> {
		#[inline] pub fn as_raw_PtrOfLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::Layer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::Layer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::Layer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::Layer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfLogLayer = core::Ptr<crate::dnn::LogLayer>;
	
	ptr_extern! { crate::dnn::LogLayer,
		cv_PtrOfLogLayer_delete, cv_PtrOfLogLayer_get_inner_ptr, cv_PtrOfLogLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::LogLayer, cv_PtrOfLogLayer_new }
	
	impl core::Ptr<crate::dnn::LogLayer> {
		#[inline] pub fn as_raw_PtrOfLogLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfLogLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::LogLayerTraitConst for core::Ptr<crate::dnn::LogLayer> {
		#[inline] fn as_raw_LogLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LogLayerTrait for core::Ptr<crate::dnn::LogLayer> {
		#[inline] fn as_raw_mut_LogLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::LogLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::LogLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::LogLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::LogLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::LogLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::LogLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfMVNLayer = core::Ptr<crate::dnn::MVNLayer>;
	
	ptr_extern! { crate::dnn::MVNLayer,
		cv_PtrOfMVNLayer_delete, cv_PtrOfMVNLayer_get_inner_ptr, cv_PtrOfMVNLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::MVNLayer, cv_PtrOfMVNLayer_new }
	
	impl core::Ptr<crate::dnn::MVNLayer> {
		#[inline] pub fn as_raw_PtrOfMVNLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMVNLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::MVNLayerTraitConst for core::Ptr<crate::dnn::MVNLayer> {
		#[inline] fn as_raw_MVNLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::MVNLayerTrait for core::Ptr<crate::dnn::MVNLayer> {
		#[inline] fn as_raw_mut_MVNLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::MVNLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::MVNLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::MVNLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::MVNLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfMaxUnpoolLayer = core::Ptr<crate::dnn::MaxUnpoolLayer>;
	
	ptr_extern! { crate::dnn::MaxUnpoolLayer,
		cv_PtrOfMaxUnpoolLayer_delete, cv_PtrOfMaxUnpoolLayer_get_inner_ptr, cv_PtrOfMaxUnpoolLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::MaxUnpoolLayer, cv_PtrOfMaxUnpoolLayer_new }
	
	impl core::Ptr<crate::dnn::MaxUnpoolLayer> {
		#[inline] pub fn as_raw_PtrOfMaxUnpoolLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMaxUnpoolLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::MaxUnpoolLayerTraitConst for core::Ptr<crate::dnn::MaxUnpoolLayer> {
		#[inline] fn as_raw_MaxUnpoolLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::MaxUnpoolLayerTrait for core::Ptr<crate::dnn::MaxUnpoolLayer> {
		#[inline] fn as_raw_mut_MaxUnpoolLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::MaxUnpoolLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::MaxUnpoolLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::MaxUnpoolLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::MaxUnpoolLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfMishLayer = core::Ptr<crate::dnn::MishLayer>;
	
	ptr_extern! { crate::dnn::MishLayer,
		cv_PtrOfMishLayer_delete, cv_PtrOfMishLayer_get_inner_ptr, cv_PtrOfMishLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::MishLayer, cv_PtrOfMishLayer_new }
	
	impl core::Ptr<crate::dnn::MishLayer> {
		#[inline] pub fn as_raw_PtrOfMishLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMishLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::MishLayerTraitConst for core::Ptr<crate::dnn::MishLayer> {
		#[inline] fn as_raw_MishLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::MishLayerTrait for core::Ptr<crate::dnn::MishLayer> {
		#[inline] fn as_raw_mut_MishLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::MishLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::MishLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::MishLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::MishLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::MishLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::MishLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfNormalizeBBoxLayer = core::Ptr<crate::dnn::NormalizeBBoxLayer>;
	
	ptr_extern! { crate::dnn::NormalizeBBoxLayer,
		cv_PtrOfNormalizeBBoxLayer_delete, cv_PtrOfNormalizeBBoxLayer_get_inner_ptr, cv_PtrOfNormalizeBBoxLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::NormalizeBBoxLayer, cv_PtrOfNormalizeBBoxLayer_new }
	
	impl core::Ptr<crate::dnn::NormalizeBBoxLayer> {
		#[inline] pub fn as_raw_PtrOfNormalizeBBoxLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfNormalizeBBoxLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::NormalizeBBoxLayerTraitConst for core::Ptr<crate::dnn::NormalizeBBoxLayer> {
		#[inline] fn as_raw_NormalizeBBoxLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::NormalizeBBoxLayerTrait for core::Ptr<crate::dnn::NormalizeBBoxLayer> {
		#[inline] fn as_raw_mut_NormalizeBBoxLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::NormalizeBBoxLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::NormalizeBBoxLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::NormalizeBBoxLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::NormalizeBBoxLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfNotLayer = core::Ptr<crate::dnn::NotLayer>;
	
	ptr_extern! { crate::dnn::NotLayer,
		cv_PtrOfNotLayer_delete, cv_PtrOfNotLayer_get_inner_ptr, cv_PtrOfNotLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::NotLayer, cv_PtrOfNotLayer_new }
	
	impl core::Ptr<crate::dnn::NotLayer> {
		#[inline] pub fn as_raw_PtrOfNotLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfNotLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::NotLayerTraitConst for core::Ptr<crate::dnn::NotLayer> {
		#[inline] fn as_raw_NotLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::NotLayerTrait for core::Ptr<crate::dnn::NotLayer> {
		#[inline] fn as_raw_mut_NotLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::NotLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::NotLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::NotLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::NotLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::NotLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::NotLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfPaddingLayer = core::Ptr<crate::dnn::PaddingLayer>;
	
	ptr_extern! { crate::dnn::PaddingLayer,
		cv_PtrOfPaddingLayer_delete, cv_PtrOfPaddingLayer_get_inner_ptr, cv_PtrOfPaddingLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::PaddingLayer, cv_PtrOfPaddingLayer_new }
	
	impl core::Ptr<crate::dnn::PaddingLayer> {
		#[inline] pub fn as_raw_PtrOfPaddingLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfPaddingLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::PaddingLayerTraitConst for core::Ptr<crate::dnn::PaddingLayer> {
		#[inline] fn as_raw_PaddingLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::PaddingLayerTrait for core::Ptr<crate::dnn::PaddingLayer> {
		#[inline] fn as_raw_mut_PaddingLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::PaddingLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::PaddingLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::PaddingLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::PaddingLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfPermuteLayer = core::Ptr<crate::dnn::PermuteLayer>;
	
	ptr_extern! { crate::dnn::PermuteLayer,
		cv_PtrOfPermuteLayer_delete, cv_PtrOfPermuteLayer_get_inner_ptr, cv_PtrOfPermuteLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::PermuteLayer, cv_PtrOfPermuteLayer_new }
	
	impl core::Ptr<crate::dnn::PermuteLayer> {
		#[inline] pub fn as_raw_PtrOfPermuteLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfPermuteLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::PermuteLayerTraitConst for core::Ptr<crate::dnn::PermuteLayer> {
		#[inline] fn as_raw_PermuteLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::PermuteLayerTrait for core::Ptr<crate::dnn::PermuteLayer> {
		#[inline] fn as_raw_mut_PermuteLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::PermuteLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::PermuteLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::PermuteLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::PermuteLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfPoolingLayer = core::Ptr<crate::dnn::PoolingLayer>;
	
	ptr_extern! { crate::dnn::PoolingLayer,
		cv_PtrOfPoolingLayer_delete, cv_PtrOfPoolingLayer_get_inner_ptr, cv_PtrOfPoolingLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::PoolingLayer, cv_PtrOfPoolingLayer_new }
	
	impl core::Ptr<crate::dnn::PoolingLayer> {
		#[inline] pub fn as_raw_PtrOfPoolingLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfPoolingLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::PoolingLayerTraitConst for core::Ptr<crate::dnn::PoolingLayer> {
		#[inline] fn as_raw_PoolingLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::PoolingLayerTrait for core::Ptr<crate::dnn::PoolingLayer> {
		#[inline] fn as_raw_mut_PoolingLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::PoolingLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::PoolingLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::PoolingLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::PoolingLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfPoolingLayerInt8 = core::Ptr<crate::dnn::PoolingLayerInt8>;
	
	ptr_extern! { crate::dnn::PoolingLayerInt8,
		cv_PtrOfPoolingLayerInt8_delete, cv_PtrOfPoolingLayerInt8_get_inner_ptr, cv_PtrOfPoolingLayerInt8_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::PoolingLayerInt8, cv_PtrOfPoolingLayerInt8_new }
	
	impl core::Ptr<crate::dnn::PoolingLayerInt8> {
		#[inline] pub fn as_raw_PtrOfPoolingLayerInt8(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfPoolingLayerInt8(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::PoolingLayerInt8TraitConst for core::Ptr<crate::dnn::PoolingLayerInt8> {
		#[inline] fn as_raw_PoolingLayerInt8(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::PoolingLayerInt8Trait for core::Ptr<crate::dnn::PoolingLayerInt8> {
		#[inline] fn as_raw_mut_PoolingLayerInt8(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::PoolingLayerInt8> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::PoolingLayerInt8> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::PoolingLayerInt8> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::PoolingLayerInt8> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::PoolingLayerTraitConst for core::Ptr<crate::dnn::PoolingLayerInt8> {
		#[inline] fn as_raw_PoolingLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::PoolingLayerTrait for core::Ptr<crate::dnn::PoolingLayerInt8> {
		#[inline] fn as_raw_mut_PoolingLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfPowerLayer = core::Ptr<crate::dnn::PowerLayer>;
	
	ptr_extern! { crate::dnn::PowerLayer,
		cv_PtrOfPowerLayer_delete, cv_PtrOfPowerLayer_get_inner_ptr, cv_PtrOfPowerLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::PowerLayer, cv_PtrOfPowerLayer_new }
	
	impl core::Ptr<crate::dnn::PowerLayer> {
		#[inline] pub fn as_raw_PtrOfPowerLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfPowerLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::PowerLayerTraitConst for core::Ptr<crate::dnn::PowerLayer> {
		#[inline] fn as_raw_PowerLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::PowerLayerTrait for core::Ptr<crate::dnn::PowerLayer> {
		#[inline] fn as_raw_mut_PowerLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::PowerLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::PowerLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::PowerLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::PowerLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::PowerLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::PowerLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfPriorBoxLayer = core::Ptr<crate::dnn::PriorBoxLayer>;
	
	ptr_extern! { crate::dnn::PriorBoxLayer,
		cv_PtrOfPriorBoxLayer_delete, cv_PtrOfPriorBoxLayer_get_inner_ptr, cv_PtrOfPriorBoxLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::PriorBoxLayer, cv_PtrOfPriorBoxLayer_new }
	
	impl core::Ptr<crate::dnn::PriorBoxLayer> {
		#[inline] pub fn as_raw_PtrOfPriorBoxLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfPriorBoxLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::PriorBoxLayerTraitConst for core::Ptr<crate::dnn::PriorBoxLayer> {
		#[inline] fn as_raw_PriorBoxLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::PriorBoxLayerTrait for core::Ptr<crate::dnn::PriorBoxLayer> {
		#[inline] fn as_raw_mut_PriorBoxLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::PriorBoxLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::PriorBoxLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::PriorBoxLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::PriorBoxLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfProposalLayer = core::Ptr<crate::dnn::ProposalLayer>;
	
	ptr_extern! { crate::dnn::ProposalLayer,
		cv_PtrOfProposalLayer_delete, cv_PtrOfProposalLayer_get_inner_ptr, cv_PtrOfProposalLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::ProposalLayer, cv_PtrOfProposalLayer_new }
	
	impl core::Ptr<crate::dnn::ProposalLayer> {
		#[inline] pub fn as_raw_PtrOfProposalLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfProposalLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::ProposalLayerTraitConst for core::Ptr<crate::dnn::ProposalLayer> {
		#[inline] fn as_raw_ProposalLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ProposalLayerTrait for core::Ptr<crate::dnn::ProposalLayer> {
		#[inline] fn as_raw_mut_ProposalLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ProposalLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ProposalLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ProposalLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ProposalLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfQuantizeLayer = core::Ptr<crate::dnn::QuantizeLayer>;
	
	ptr_extern! { crate::dnn::QuantizeLayer,
		cv_PtrOfQuantizeLayer_delete, cv_PtrOfQuantizeLayer_get_inner_ptr, cv_PtrOfQuantizeLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::QuantizeLayer, cv_PtrOfQuantizeLayer_new }
	
	impl core::Ptr<crate::dnn::QuantizeLayer> {
		#[inline] pub fn as_raw_PtrOfQuantizeLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfQuantizeLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::QuantizeLayerTraitConst for core::Ptr<crate::dnn::QuantizeLayer> {
		#[inline] fn as_raw_QuantizeLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::QuantizeLayerTrait for core::Ptr<crate::dnn::QuantizeLayer> {
		#[inline] fn as_raw_mut_QuantizeLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::QuantizeLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::QuantizeLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::QuantizeLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::QuantizeLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfRNNLayer = core::Ptr<dyn crate::dnn::RNNLayer>;
	
	ptr_extern! { dyn crate::dnn::RNNLayer,
		cv_PtrOfRNNLayer_delete, cv_PtrOfRNNLayer_get_inner_ptr, cv_PtrOfRNNLayer_get_inner_ptr_mut
	}
	
	impl core::Ptr<dyn crate::dnn::RNNLayer> {
		#[inline] pub fn as_raw_PtrOfRNNLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfRNNLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::RNNLayerConst for core::Ptr<dyn crate::dnn::RNNLayer> {
		#[inline] fn as_raw_RNNLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::RNNLayer for core::Ptr<dyn crate::dnn::RNNLayer> {
		#[inline] fn as_raw_mut_RNNLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<dyn crate::dnn::RNNLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<dyn crate::dnn::RNNLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<dyn crate::dnn::RNNLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<dyn crate::dnn::RNNLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfReLU6Layer = core::Ptr<crate::dnn::ReLU6Layer>;
	
	ptr_extern! { crate::dnn::ReLU6Layer,
		cv_PtrOfReLU6Layer_delete, cv_PtrOfReLU6Layer_get_inner_ptr, cv_PtrOfReLU6Layer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::ReLU6Layer, cv_PtrOfReLU6Layer_new }
	
	impl core::Ptr<crate::dnn::ReLU6Layer> {
		#[inline] pub fn as_raw_PtrOfReLU6Layer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfReLU6Layer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::ReLU6LayerTraitConst for core::Ptr<crate::dnn::ReLU6Layer> {
		#[inline] fn as_raw_ReLU6Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ReLU6LayerTrait for core::Ptr<crate::dnn::ReLU6Layer> {
		#[inline] fn as_raw_mut_ReLU6Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ReLU6Layer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ReLU6Layer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::ReLU6Layer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::ReLU6Layer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ReLU6Layer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ReLU6Layer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfReLULayer = core::Ptr<crate::dnn::ReLULayer>;
	
	ptr_extern! { crate::dnn::ReLULayer,
		cv_PtrOfReLULayer_delete, cv_PtrOfReLULayer_get_inner_ptr, cv_PtrOfReLULayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::ReLULayer, cv_PtrOfReLULayer_new }
	
	impl core::Ptr<crate::dnn::ReLULayer> {
		#[inline] pub fn as_raw_PtrOfReLULayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfReLULayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::ReLULayerTraitConst for core::Ptr<crate::dnn::ReLULayer> {
		#[inline] fn as_raw_ReLULayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ReLULayerTrait for core::Ptr<crate::dnn::ReLULayer> {
		#[inline] fn as_raw_mut_ReLULayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ReLULayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ReLULayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::ReLULayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::ReLULayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ReLULayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ReLULayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfReciprocalLayer = core::Ptr<crate::dnn::ReciprocalLayer>;
	
	ptr_extern! { crate::dnn::ReciprocalLayer,
		cv_PtrOfReciprocalLayer_delete, cv_PtrOfReciprocalLayer_get_inner_ptr, cv_PtrOfReciprocalLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::ReciprocalLayer, cv_PtrOfReciprocalLayer_new }
	
	impl core::Ptr<crate::dnn::ReciprocalLayer> {
		#[inline] pub fn as_raw_PtrOfReciprocalLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfReciprocalLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::ReciprocalLayerTraitConst for core::Ptr<crate::dnn::ReciprocalLayer> {
		#[inline] fn as_raw_ReciprocalLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ReciprocalLayerTrait for core::Ptr<crate::dnn::ReciprocalLayer> {
		#[inline] fn as_raw_mut_ReciprocalLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ReciprocalLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ReciprocalLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::ReciprocalLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::ReciprocalLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ReciprocalLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ReciprocalLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfReduceLayer = core::Ptr<crate::dnn::ReduceLayer>;
	
	ptr_extern! { crate::dnn::ReduceLayer,
		cv_PtrOfReduceLayer_delete, cv_PtrOfReduceLayer_get_inner_ptr, cv_PtrOfReduceLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::ReduceLayer, cv_PtrOfReduceLayer_new }
	
	impl core::Ptr<crate::dnn::ReduceLayer> {
		#[inline] pub fn as_raw_PtrOfReduceLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfReduceLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::ReduceLayerTraitConst for core::Ptr<crate::dnn::ReduceLayer> {
		#[inline] fn as_raw_ReduceLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ReduceLayerTrait for core::Ptr<crate::dnn::ReduceLayer> {
		#[inline] fn as_raw_mut_ReduceLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ReduceLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ReduceLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ReduceLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ReduceLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfReduceLayerInt8 = core::Ptr<crate::dnn::ReduceLayerInt8>;
	
	ptr_extern! { crate::dnn::ReduceLayerInt8,
		cv_PtrOfReduceLayerInt8_delete, cv_PtrOfReduceLayerInt8_get_inner_ptr, cv_PtrOfReduceLayerInt8_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::ReduceLayerInt8, cv_PtrOfReduceLayerInt8_new }
	
	impl core::Ptr<crate::dnn::ReduceLayerInt8> {
		#[inline] pub fn as_raw_PtrOfReduceLayerInt8(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfReduceLayerInt8(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::ReduceLayerInt8TraitConst for core::Ptr<crate::dnn::ReduceLayerInt8> {
		#[inline] fn as_raw_ReduceLayerInt8(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ReduceLayerInt8Trait for core::Ptr<crate::dnn::ReduceLayerInt8> {
		#[inline] fn as_raw_mut_ReduceLayerInt8(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ReduceLayerInt8> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ReduceLayerInt8> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ReduceLayerInt8> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ReduceLayerInt8> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ReduceLayerTraitConst for core::Ptr<crate::dnn::ReduceLayerInt8> {
		#[inline] fn as_raw_ReduceLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ReduceLayerTrait for core::Ptr<crate::dnn::ReduceLayerInt8> {
		#[inline] fn as_raw_mut_ReduceLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfRegionLayer = core::Ptr<crate::dnn::RegionLayer>;
	
	ptr_extern! { crate::dnn::RegionLayer,
		cv_PtrOfRegionLayer_delete, cv_PtrOfRegionLayer_get_inner_ptr, cv_PtrOfRegionLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::RegionLayer, cv_PtrOfRegionLayer_new }
	
	impl core::Ptr<crate::dnn::RegionLayer> {
		#[inline] pub fn as_raw_PtrOfRegionLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfRegionLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::RegionLayerTraitConst for core::Ptr<crate::dnn::RegionLayer> {
		#[inline] fn as_raw_RegionLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::RegionLayerTrait for core::Ptr<crate::dnn::RegionLayer> {
		#[inline] fn as_raw_mut_RegionLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::RegionLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::RegionLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::RegionLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::RegionLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfReorgLayer = core::Ptr<crate::dnn::ReorgLayer>;
	
	ptr_extern! { crate::dnn::ReorgLayer,
		cv_PtrOfReorgLayer_delete, cv_PtrOfReorgLayer_get_inner_ptr, cv_PtrOfReorgLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::ReorgLayer, cv_PtrOfReorgLayer_new }
	
	impl core::Ptr<crate::dnn::ReorgLayer> {
		#[inline] pub fn as_raw_PtrOfReorgLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfReorgLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::ReorgLayerTraitConst for core::Ptr<crate::dnn::ReorgLayer> {
		#[inline] fn as_raw_ReorgLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ReorgLayerTrait for core::Ptr<crate::dnn::ReorgLayer> {
		#[inline] fn as_raw_mut_ReorgLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ReorgLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ReorgLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ReorgLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ReorgLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfRequantizeLayer = core::Ptr<crate::dnn::RequantizeLayer>;
	
	ptr_extern! { crate::dnn::RequantizeLayer,
		cv_PtrOfRequantizeLayer_delete, cv_PtrOfRequantizeLayer_get_inner_ptr, cv_PtrOfRequantizeLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::RequantizeLayer, cv_PtrOfRequantizeLayer_new }
	
	impl core::Ptr<crate::dnn::RequantizeLayer> {
		#[inline] pub fn as_raw_PtrOfRequantizeLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfRequantizeLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::RequantizeLayerTraitConst for core::Ptr<crate::dnn::RequantizeLayer> {
		#[inline] fn as_raw_RequantizeLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::RequantizeLayerTrait for core::Ptr<crate::dnn::RequantizeLayer> {
		#[inline] fn as_raw_mut_RequantizeLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::RequantizeLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::RequantizeLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::RequantizeLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::RequantizeLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfReshapeLayer = core::Ptr<crate::dnn::ReshapeLayer>;
	
	ptr_extern! { crate::dnn::ReshapeLayer,
		cv_PtrOfReshapeLayer_delete, cv_PtrOfReshapeLayer_get_inner_ptr, cv_PtrOfReshapeLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::ReshapeLayer, cv_PtrOfReshapeLayer_new }
	
	impl core::Ptr<crate::dnn::ReshapeLayer> {
		#[inline] pub fn as_raw_PtrOfReshapeLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfReshapeLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::ReshapeLayerTraitConst for core::Ptr<crate::dnn::ReshapeLayer> {
		#[inline] fn as_raw_ReshapeLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ReshapeLayerTrait for core::Ptr<crate::dnn::ReshapeLayer> {
		#[inline] fn as_raw_mut_ReshapeLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ReshapeLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ReshapeLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ReshapeLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ReshapeLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfResizeLayer = core::Ptr<crate::dnn::ResizeLayer>;
	
	ptr_extern! { crate::dnn::ResizeLayer,
		cv_PtrOfResizeLayer_delete, cv_PtrOfResizeLayer_get_inner_ptr, cv_PtrOfResizeLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::ResizeLayer, cv_PtrOfResizeLayer_new }
	
	impl core::Ptr<crate::dnn::ResizeLayer> {
		#[inline] pub fn as_raw_PtrOfResizeLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfResizeLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::ResizeLayerTraitConst for core::Ptr<crate::dnn::ResizeLayer> {
		#[inline] fn as_raw_ResizeLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ResizeLayerTrait for core::Ptr<crate::dnn::ResizeLayer> {
		#[inline] fn as_raw_mut_ResizeLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ResizeLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ResizeLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ResizeLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ResizeLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfRoundLayer = core::Ptr<crate::dnn::RoundLayer>;
	
	ptr_extern! { crate::dnn::RoundLayer,
		cv_PtrOfRoundLayer_delete, cv_PtrOfRoundLayer_get_inner_ptr, cv_PtrOfRoundLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::RoundLayer, cv_PtrOfRoundLayer_new }
	
	impl core::Ptr<crate::dnn::RoundLayer> {
		#[inline] pub fn as_raw_PtrOfRoundLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfRoundLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::RoundLayerTraitConst for core::Ptr<crate::dnn::RoundLayer> {
		#[inline] fn as_raw_RoundLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::RoundLayerTrait for core::Ptr<crate::dnn::RoundLayer> {
		#[inline] fn as_raw_mut_RoundLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::RoundLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::RoundLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::RoundLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::RoundLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::RoundLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::RoundLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfScaleLayer = core::Ptr<crate::dnn::ScaleLayer>;
	
	ptr_extern! { crate::dnn::ScaleLayer,
		cv_PtrOfScaleLayer_delete, cv_PtrOfScaleLayer_get_inner_ptr, cv_PtrOfScaleLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::ScaleLayer, cv_PtrOfScaleLayer_new }
	
	impl core::Ptr<crate::dnn::ScaleLayer> {
		#[inline] pub fn as_raw_PtrOfScaleLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfScaleLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::ScaleLayerTraitConst for core::Ptr<crate::dnn::ScaleLayer> {
		#[inline] fn as_raw_ScaleLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ScaleLayerTrait for core::Ptr<crate::dnn::ScaleLayer> {
		#[inline] fn as_raw_mut_ScaleLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ScaleLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ScaleLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ScaleLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ScaleLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfScaleLayerInt8 = core::Ptr<crate::dnn::ScaleLayerInt8>;
	
	ptr_extern! { crate::dnn::ScaleLayerInt8,
		cv_PtrOfScaleLayerInt8_delete, cv_PtrOfScaleLayerInt8_get_inner_ptr, cv_PtrOfScaleLayerInt8_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::ScaleLayerInt8, cv_PtrOfScaleLayerInt8_new }
	
	impl core::Ptr<crate::dnn::ScaleLayerInt8> {
		#[inline] pub fn as_raw_PtrOfScaleLayerInt8(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfScaleLayerInt8(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::ScaleLayerInt8TraitConst for core::Ptr<crate::dnn::ScaleLayerInt8> {
		#[inline] fn as_raw_ScaleLayerInt8(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ScaleLayerInt8Trait for core::Ptr<crate::dnn::ScaleLayerInt8> {
		#[inline] fn as_raw_mut_ScaleLayerInt8(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ScaleLayerInt8> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ScaleLayerInt8> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ScaleLayerInt8> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ScaleLayerInt8> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ScaleLayerTraitConst for core::Ptr<crate::dnn::ScaleLayerInt8> {
		#[inline] fn as_raw_ScaleLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ScaleLayerTrait for core::Ptr<crate::dnn::ScaleLayerInt8> {
		#[inline] fn as_raw_mut_ScaleLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSeluLayer = core::Ptr<crate::dnn::SeluLayer>;
	
	ptr_extern! { crate::dnn::SeluLayer,
		cv_PtrOfSeluLayer_delete, cv_PtrOfSeluLayer_get_inner_ptr, cv_PtrOfSeluLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::SeluLayer, cv_PtrOfSeluLayer_new }
	
	impl core::Ptr<crate::dnn::SeluLayer> {
		#[inline] pub fn as_raw_PtrOfSeluLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSeluLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::SeluLayerTraitConst for core::Ptr<crate::dnn::SeluLayer> {
		#[inline] fn as_raw_SeluLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::SeluLayerTrait for core::Ptr<crate::dnn::SeluLayer> {
		#[inline] fn as_raw_mut_SeluLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::SeluLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::SeluLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::SeluLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::SeluLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::SeluLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::SeluLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfShrinkLayer = core::Ptr<crate::dnn::ShrinkLayer>;
	
	ptr_extern! { crate::dnn::ShrinkLayer,
		cv_PtrOfShrinkLayer_delete, cv_PtrOfShrinkLayer_get_inner_ptr, cv_PtrOfShrinkLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::ShrinkLayer, cv_PtrOfShrinkLayer_new }
	
	impl core::Ptr<crate::dnn::ShrinkLayer> {
		#[inline] pub fn as_raw_PtrOfShrinkLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfShrinkLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::ShrinkLayerTraitConst for core::Ptr<crate::dnn::ShrinkLayer> {
		#[inline] fn as_raw_ShrinkLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ShrinkLayerTrait for core::Ptr<crate::dnn::ShrinkLayer> {
		#[inline] fn as_raw_mut_ShrinkLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ShrinkLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ShrinkLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::ShrinkLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::ShrinkLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ShrinkLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ShrinkLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSigmoidLayer = core::Ptr<crate::dnn::SigmoidLayer>;
	
	ptr_extern! { crate::dnn::SigmoidLayer,
		cv_PtrOfSigmoidLayer_delete, cv_PtrOfSigmoidLayer_get_inner_ptr, cv_PtrOfSigmoidLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::SigmoidLayer, cv_PtrOfSigmoidLayer_new }
	
	impl core::Ptr<crate::dnn::SigmoidLayer> {
		#[inline] pub fn as_raw_PtrOfSigmoidLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSigmoidLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::SigmoidLayerTraitConst for core::Ptr<crate::dnn::SigmoidLayer> {
		#[inline] fn as_raw_SigmoidLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::SigmoidLayerTrait for core::Ptr<crate::dnn::SigmoidLayer> {
		#[inline] fn as_raw_mut_SigmoidLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::SigmoidLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::SigmoidLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::SigmoidLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::SigmoidLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::SigmoidLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::SigmoidLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSignLayer = core::Ptr<crate::dnn::SignLayer>;
	
	ptr_extern! { crate::dnn::SignLayer,
		cv_PtrOfSignLayer_delete, cv_PtrOfSignLayer_get_inner_ptr, cv_PtrOfSignLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::SignLayer, cv_PtrOfSignLayer_new }
	
	impl core::Ptr<crate::dnn::SignLayer> {
		#[inline] pub fn as_raw_PtrOfSignLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSignLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::SignLayerTraitConst for core::Ptr<crate::dnn::SignLayer> {
		#[inline] fn as_raw_SignLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::SignLayerTrait for core::Ptr<crate::dnn::SignLayer> {
		#[inline] fn as_raw_mut_SignLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::SignLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::SignLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::SignLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::SignLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::SignLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::SignLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSinLayer = core::Ptr<crate::dnn::SinLayer>;
	
	ptr_extern! { crate::dnn::SinLayer,
		cv_PtrOfSinLayer_delete, cv_PtrOfSinLayer_get_inner_ptr, cv_PtrOfSinLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::SinLayer, cv_PtrOfSinLayer_new }
	
	impl core::Ptr<crate::dnn::SinLayer> {
		#[inline] pub fn as_raw_PtrOfSinLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSinLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::SinLayerTraitConst for core::Ptr<crate::dnn::SinLayer> {
		#[inline] fn as_raw_SinLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::SinLayerTrait for core::Ptr<crate::dnn::SinLayer> {
		#[inline] fn as_raw_mut_SinLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::SinLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::SinLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::SinLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::SinLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::SinLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::SinLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSinhLayer = core::Ptr<crate::dnn::SinhLayer>;
	
	ptr_extern! { crate::dnn::SinhLayer,
		cv_PtrOfSinhLayer_delete, cv_PtrOfSinhLayer_get_inner_ptr, cv_PtrOfSinhLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::SinhLayer, cv_PtrOfSinhLayer_new }
	
	impl core::Ptr<crate::dnn::SinhLayer> {
		#[inline] pub fn as_raw_PtrOfSinhLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSinhLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::SinhLayerTraitConst for core::Ptr<crate::dnn::SinhLayer> {
		#[inline] fn as_raw_SinhLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::SinhLayerTrait for core::Ptr<crate::dnn::SinhLayer> {
		#[inline] fn as_raw_mut_SinhLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::SinhLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::SinhLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::SinhLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::SinhLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::SinhLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::SinhLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSliceLayer = core::Ptr<crate::dnn::SliceLayer>;
	
	ptr_extern! { crate::dnn::SliceLayer,
		cv_PtrOfSliceLayer_delete, cv_PtrOfSliceLayer_get_inner_ptr, cv_PtrOfSliceLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::SliceLayer, cv_PtrOfSliceLayer_new }
	
	impl core::Ptr<crate::dnn::SliceLayer> {
		#[inline] pub fn as_raw_PtrOfSliceLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSliceLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::SliceLayerTraitConst for core::Ptr<crate::dnn::SliceLayer> {
		#[inline] fn as_raw_SliceLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::SliceLayerTrait for core::Ptr<crate::dnn::SliceLayer> {
		#[inline] fn as_raw_mut_SliceLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::SliceLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::SliceLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::SliceLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::SliceLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSoftmaxLayer = core::Ptr<crate::dnn::SoftmaxLayer>;
	
	ptr_extern! { crate::dnn::SoftmaxLayer,
		cv_PtrOfSoftmaxLayer_delete, cv_PtrOfSoftmaxLayer_get_inner_ptr, cv_PtrOfSoftmaxLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::SoftmaxLayer, cv_PtrOfSoftmaxLayer_new }
	
	impl core::Ptr<crate::dnn::SoftmaxLayer> {
		#[inline] pub fn as_raw_PtrOfSoftmaxLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSoftmaxLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::SoftmaxLayerTraitConst for core::Ptr<crate::dnn::SoftmaxLayer> {
		#[inline] fn as_raw_SoftmaxLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::SoftmaxLayerTrait for core::Ptr<crate::dnn::SoftmaxLayer> {
		#[inline] fn as_raw_mut_SoftmaxLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::SoftmaxLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::SoftmaxLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::SoftmaxLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::SoftmaxLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSoftmaxLayerInt8 = core::Ptr<crate::dnn::SoftmaxLayerInt8>;
	
	ptr_extern! { crate::dnn::SoftmaxLayerInt8,
		cv_PtrOfSoftmaxLayerInt8_delete, cv_PtrOfSoftmaxLayerInt8_get_inner_ptr, cv_PtrOfSoftmaxLayerInt8_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::SoftmaxLayerInt8, cv_PtrOfSoftmaxLayerInt8_new }
	
	impl core::Ptr<crate::dnn::SoftmaxLayerInt8> {
		#[inline] pub fn as_raw_PtrOfSoftmaxLayerInt8(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSoftmaxLayerInt8(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::SoftmaxLayerInt8TraitConst for core::Ptr<crate::dnn::SoftmaxLayerInt8> {
		#[inline] fn as_raw_SoftmaxLayerInt8(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::SoftmaxLayerInt8Trait for core::Ptr<crate::dnn::SoftmaxLayerInt8> {
		#[inline] fn as_raw_mut_SoftmaxLayerInt8(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::SoftmaxLayerInt8> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::SoftmaxLayerInt8> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::SoftmaxLayerInt8> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::SoftmaxLayerInt8> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::SoftmaxLayerTraitConst for core::Ptr<crate::dnn::SoftmaxLayerInt8> {
		#[inline] fn as_raw_SoftmaxLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::SoftmaxLayerTrait for core::Ptr<crate::dnn::SoftmaxLayerInt8> {
		#[inline] fn as_raw_mut_SoftmaxLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSoftplusLayer = core::Ptr<crate::dnn::SoftplusLayer>;
	
	ptr_extern! { crate::dnn::SoftplusLayer,
		cv_PtrOfSoftplusLayer_delete, cv_PtrOfSoftplusLayer_get_inner_ptr, cv_PtrOfSoftplusLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::SoftplusLayer, cv_PtrOfSoftplusLayer_new }
	
	impl core::Ptr<crate::dnn::SoftplusLayer> {
		#[inline] pub fn as_raw_PtrOfSoftplusLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSoftplusLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::SoftplusLayerTraitConst for core::Ptr<crate::dnn::SoftplusLayer> {
		#[inline] fn as_raw_SoftplusLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::SoftplusLayerTrait for core::Ptr<crate::dnn::SoftplusLayer> {
		#[inline] fn as_raw_mut_SoftplusLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::SoftplusLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::SoftplusLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::SoftplusLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::SoftplusLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::SoftplusLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::SoftplusLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSoftsignLayer = core::Ptr<crate::dnn::SoftsignLayer>;
	
	ptr_extern! { crate::dnn::SoftsignLayer,
		cv_PtrOfSoftsignLayer_delete, cv_PtrOfSoftsignLayer_get_inner_ptr, cv_PtrOfSoftsignLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::SoftsignLayer, cv_PtrOfSoftsignLayer_new }
	
	impl core::Ptr<crate::dnn::SoftsignLayer> {
		#[inline] pub fn as_raw_PtrOfSoftsignLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSoftsignLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::SoftsignLayerTraitConst for core::Ptr<crate::dnn::SoftsignLayer> {
		#[inline] fn as_raw_SoftsignLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::SoftsignLayerTrait for core::Ptr<crate::dnn::SoftsignLayer> {
		#[inline] fn as_raw_mut_SoftsignLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::SoftsignLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::SoftsignLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::SoftsignLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::SoftsignLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::SoftsignLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::SoftsignLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSplitLayer = core::Ptr<crate::dnn::SplitLayer>;
	
	ptr_extern! { crate::dnn::SplitLayer,
		cv_PtrOfSplitLayer_delete, cv_PtrOfSplitLayer_get_inner_ptr, cv_PtrOfSplitLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::SplitLayer, cv_PtrOfSplitLayer_new }
	
	impl core::Ptr<crate::dnn::SplitLayer> {
		#[inline] pub fn as_raw_PtrOfSplitLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSplitLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::SplitLayerTraitConst for core::Ptr<crate::dnn::SplitLayer> {
		#[inline] fn as_raw_SplitLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::SplitLayerTrait for core::Ptr<crate::dnn::SplitLayer> {
		#[inline] fn as_raw_mut_SplitLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::SplitLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::SplitLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::SplitLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::SplitLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSqrtLayer = core::Ptr<crate::dnn::SqrtLayer>;
	
	ptr_extern! { crate::dnn::SqrtLayer,
		cv_PtrOfSqrtLayer_delete, cv_PtrOfSqrtLayer_get_inner_ptr, cv_PtrOfSqrtLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::SqrtLayer, cv_PtrOfSqrtLayer_new }
	
	impl core::Ptr<crate::dnn::SqrtLayer> {
		#[inline] pub fn as_raw_PtrOfSqrtLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSqrtLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::SqrtLayerTraitConst for core::Ptr<crate::dnn::SqrtLayer> {
		#[inline] fn as_raw_SqrtLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::SqrtLayerTrait for core::Ptr<crate::dnn::SqrtLayer> {
		#[inline] fn as_raw_mut_SqrtLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::SqrtLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::SqrtLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::SqrtLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::SqrtLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::SqrtLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::SqrtLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSwishLayer = core::Ptr<crate::dnn::SwishLayer>;
	
	ptr_extern! { crate::dnn::SwishLayer,
		cv_PtrOfSwishLayer_delete, cv_PtrOfSwishLayer_get_inner_ptr, cv_PtrOfSwishLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::SwishLayer, cv_PtrOfSwishLayer_new }
	
	impl core::Ptr<crate::dnn::SwishLayer> {
		#[inline] pub fn as_raw_PtrOfSwishLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSwishLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::SwishLayerTraitConst for core::Ptr<crate::dnn::SwishLayer> {
		#[inline] fn as_raw_SwishLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::SwishLayerTrait for core::Ptr<crate::dnn::SwishLayer> {
		#[inline] fn as_raw_mut_SwishLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::SwishLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::SwishLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::SwishLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::SwishLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::SwishLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::SwishLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfTanHLayer = core::Ptr<crate::dnn::TanHLayer>;
	
	ptr_extern! { crate::dnn::TanHLayer,
		cv_PtrOfTanHLayer_delete, cv_PtrOfTanHLayer_get_inner_ptr, cv_PtrOfTanHLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::TanHLayer, cv_PtrOfTanHLayer_new }
	
	impl core::Ptr<crate::dnn::TanHLayer> {
		#[inline] pub fn as_raw_PtrOfTanHLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfTanHLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::TanHLayerTraitConst for core::Ptr<crate::dnn::TanHLayer> {
		#[inline] fn as_raw_TanHLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::TanHLayerTrait for core::Ptr<crate::dnn::TanHLayer> {
		#[inline] fn as_raw_mut_TanHLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::TanHLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::TanHLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::TanHLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::TanHLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::TanHLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::TanHLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfTanLayer = core::Ptr<crate::dnn::TanLayer>;
	
	ptr_extern! { crate::dnn::TanLayer,
		cv_PtrOfTanLayer_delete, cv_PtrOfTanLayer_get_inner_ptr, cv_PtrOfTanLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::TanLayer, cv_PtrOfTanLayer_new }
	
	impl core::Ptr<crate::dnn::TanLayer> {
		#[inline] pub fn as_raw_PtrOfTanLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfTanLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::TanLayerTraitConst for core::Ptr<crate::dnn::TanLayer> {
		#[inline] fn as_raw_TanLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::TanLayerTrait for core::Ptr<crate::dnn::TanLayer> {
		#[inline] fn as_raw_mut_TanLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::TanLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::TanLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::TanLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::TanLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::TanLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::TanLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfThresholdedReluLayer = core::Ptr<crate::dnn::ThresholdedReluLayer>;
	
	ptr_extern! { crate::dnn::ThresholdedReluLayer,
		cv_PtrOfThresholdedReluLayer_delete, cv_PtrOfThresholdedReluLayer_get_inner_ptr, cv_PtrOfThresholdedReluLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::ThresholdedReluLayer, cv_PtrOfThresholdedReluLayer_new }
	
	impl core::Ptr<crate::dnn::ThresholdedReluLayer> {
		#[inline] pub fn as_raw_PtrOfThresholdedReluLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfThresholdedReluLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::ThresholdedReluLayerTraitConst for core::Ptr<crate::dnn::ThresholdedReluLayer> {
		#[inline] fn as_raw_ThresholdedReluLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ThresholdedReluLayerTrait for core::Ptr<crate::dnn::ThresholdedReluLayer> {
		#[inline] fn as_raw_mut_ThresholdedReluLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ThresholdedReluLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ThresholdedReluLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::ThresholdedReluLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::ThresholdedReluLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ThresholdedReluLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ThresholdedReluLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type TupleOfBackend_Target = core::Tuple<(crate::dnn::Backend, crate::dnn::Target)>;
	
	impl core::Tuple<(crate::dnn::Backend, crate::dnn::Target)> {
		pub fn as_raw_TupleOfBackend_Target(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_TupleOfBackend_Target(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	tuple_extern! { (crate::dnn::Backend, crate::dnn::Target),
		cv_TupleOfBackend_Target_new, cv_TupleOfBackend_Target_delete,
		0 = arg: crate::dnn::Backend, get_0 via cv_TupleOfBackend_Target_get_0,
		1 = arg_1: crate::dnn::Target, get_1 via cv_TupleOfBackend_Target_get_1
	}
	
	pub type VectorOfMatShape = core::Vector<crate::dnn::MatShape>;
	
	impl core::Vector<crate::dnn::MatShape> {
		pub fn as_raw_VectorOfMatShape(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfMatShape(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	pub type VectorOfPtrOfBackendNode = core::Vector<core::Ptr<crate::dnn::BackendNode>>;
	
	impl core::Vector<core::Ptr<crate::dnn::BackendNode>> {
		pub fn as_raw_VectorOfPtrOfBackendNode(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfPtrOfBackendNode(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Ptr<crate::dnn::BackendNode>,
		cv_VectorOfPtrOfBackendNode_new, cv_VectorOfPtrOfBackendNode_delete,
		cv_VectorOfPtrOfBackendNode_len, cv_VectorOfPtrOfBackendNode_is_empty,
		cv_VectorOfPtrOfBackendNode_capacity, cv_VectorOfPtrOfBackendNode_shrink_to_fit,
		cv_VectorOfPtrOfBackendNode_reserve, cv_VectorOfPtrOfBackendNode_remove,
		cv_VectorOfPtrOfBackendNode_swap, cv_VectorOfPtrOfBackendNode_clear,
		cv_VectorOfPtrOfBackendNode_get, cv_VectorOfPtrOfBackendNode_set,
		cv_VectorOfPtrOfBackendNode_push, cv_VectorOfPtrOfBackendNode_insert,
	}
	vector_non_copy_or_bool! { core::Ptr<crate::dnn::BackendNode> }
	
	pub type VectorOfPtrOfBackendWrapper = core::Vector<core::Ptr<dyn crate::dnn::BackendWrapper>>;
	
	impl core::Vector<core::Ptr<dyn crate::dnn::BackendWrapper>> {
		pub fn as_raw_VectorOfPtrOfBackendWrapper(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfPtrOfBackendWrapper(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Ptr<dyn crate::dnn::BackendWrapper>,
		cv_VectorOfPtrOfBackendWrapper_new, cv_VectorOfPtrOfBackendWrapper_delete,
		cv_VectorOfPtrOfBackendWrapper_len, cv_VectorOfPtrOfBackendWrapper_is_empty,
		cv_VectorOfPtrOfBackendWrapper_capacity, cv_VectorOfPtrOfBackendWrapper_shrink_to_fit,
		cv_VectorOfPtrOfBackendWrapper_reserve, cv_VectorOfPtrOfBackendWrapper_remove,
		cv_VectorOfPtrOfBackendWrapper_swap, cv_VectorOfPtrOfBackendWrapper_clear,
		cv_VectorOfPtrOfBackendWrapper_get, cv_VectorOfPtrOfBackendWrapper_set,
		cv_VectorOfPtrOfBackendWrapper_push, cv_VectorOfPtrOfBackendWrapper_insert,
	}
	vector_non_copy_or_bool! { core::Ptr<dyn crate::dnn::BackendWrapper> }
	
	pub type VectorOfPtrOfLayer = core::Vector<core::Ptr<crate::dnn::Layer>>;
	
	impl core::Vector<core::Ptr<crate::dnn::Layer>> {
		pub fn as_raw_VectorOfPtrOfLayer(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfPtrOfLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Ptr<crate::dnn::Layer>,
		cv_VectorOfPtrOfLayer_new, cv_VectorOfPtrOfLayer_delete,
		cv_VectorOfPtrOfLayer_len, cv_VectorOfPtrOfLayer_is_empty,
		cv_VectorOfPtrOfLayer_capacity, cv_VectorOfPtrOfLayer_shrink_to_fit,
		cv_VectorOfPtrOfLayer_reserve, cv_VectorOfPtrOfLayer_remove,
		cv_VectorOfPtrOfLayer_swap, cv_VectorOfPtrOfLayer_clear,
		cv_VectorOfPtrOfLayer_get, cv_VectorOfPtrOfLayer_set,
		cv_VectorOfPtrOfLayer_push, cv_VectorOfPtrOfLayer_insert,
	}
	vector_non_copy_or_bool! { core::Ptr<crate::dnn::Layer> }
	
	pub type VectorOfTarget = core::Vector<crate::dnn::Target>;
	
	impl core::Vector<crate::dnn::Target> {
		pub fn as_raw_VectorOfTarget(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfTarget(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { crate::dnn::Target,
		cv_VectorOfTarget_new, cv_VectorOfTarget_delete,
		cv_VectorOfTarget_len, cv_VectorOfTarget_is_empty,
		cv_VectorOfTarget_capacity, cv_VectorOfTarget_shrink_to_fit,
		cv_VectorOfTarget_reserve, cv_VectorOfTarget_remove,
		cv_VectorOfTarget_swap, cv_VectorOfTarget_clear,
		cv_VectorOfTarget_get, cv_VectorOfTarget_set,
		cv_VectorOfTarget_push, cv_VectorOfTarget_insert,
	}
	vector_copy_non_bool! { crate::dnn::Target,
		cv_VectorOfTarget_data, cv_VectorOfTarget_data_mut, cv_VectorOfTarget_from_slice,
		cv_VectorOfTarget_clone,
	}
	
	pub type VectorOfTupleOfBackend_Target = core::Vector<core::Tuple<(crate::dnn::Backend, crate::dnn::Target)>>;
	
	impl core::Vector<core::Tuple<(crate::dnn::Backend, crate::dnn::Target)>> {
		pub fn as_raw_VectorOfTupleOfBackend_Target(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfTupleOfBackend_Target(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Tuple<(crate::dnn::Backend, crate::dnn::Target)>,
		cv_VectorOfTupleOfBackend_Target_new, cv_VectorOfTupleOfBackend_Target_delete,
		cv_VectorOfTupleOfBackend_Target_len, cv_VectorOfTupleOfBackend_Target_is_empty,
		cv_VectorOfTupleOfBackend_Target_capacity, cv_VectorOfTupleOfBackend_Target_shrink_to_fit,
		cv_VectorOfTupleOfBackend_Target_reserve, cv_VectorOfTupleOfBackend_Target_remove,
		cv_VectorOfTupleOfBackend_Target_swap, cv_VectorOfTupleOfBackend_Target_clear,
		cv_VectorOfTupleOfBackend_Target_get, cv_VectorOfTupleOfBackend_Target_set,
		cv_VectorOfTupleOfBackend_Target_push, cv_VectorOfTupleOfBackend_Target_insert,
	}
	vector_non_copy_or_bool! { core::Tuple<(crate::dnn::Backend, crate::dnn::Target)> }
	
	pub type VectorOfVectorOfMatShape = core::Vector<core::Vector<crate::dnn::MatShape>>;
	
	impl core::Vector<core::Vector<crate::dnn::MatShape>> {
		pub fn as_raw_VectorOfVectorOfMatShape(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfMatShape(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector<crate::dnn::MatShape>,
		cv_VectorOfVectorOfMatShape_new, cv_VectorOfVectorOfMatShape_delete,
		cv_VectorOfVectorOfMatShape_len, cv_VectorOfVectorOfMatShape_is_empty,
		cv_VectorOfVectorOfMatShape_capacity, cv_VectorOfVectorOfMatShape_shrink_to_fit,
		cv_VectorOfVectorOfMatShape_reserve, cv_VectorOfVectorOfMatShape_remove,
		cv_VectorOfVectorOfMatShape_swap, cv_VectorOfVectorOfMatShape_clear,
		cv_VectorOfVectorOfMatShape_get, cv_VectorOfVectorOfMatShape_set,
		cv_VectorOfVectorOfMatShape_push, cv_VectorOfVectorOfMatShape_insert,
	}
	vector_non_copy_or_bool! { core::Vector<crate::dnn::MatShape> }
	
}
#[cfg(ocvrs_has_module_dnn)]
pub use dnn_types::*;

#[cfg(ocvrs_has_module_features2d)]
mod features2d_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfAKAZE = core::Ptr<dyn crate::features2d::AKAZE>;
	
	ptr_extern! { dyn crate::features2d::AKAZE,
		cv_PtrOfAKAZE_delete, cv_PtrOfAKAZE_get_inner_ptr, cv_PtrOfAKAZE_get_inner_ptr_mut
	}
	
	impl core::Ptr<dyn crate::features2d::AKAZE> {
		#[inline] pub fn as_raw_PtrOfAKAZE(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfAKAZE(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::features2d::AKAZEConst for core::Ptr<dyn crate::features2d::AKAZE> {
		#[inline] fn as_raw_AKAZE(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::AKAZE for core::Ptr<dyn crate::features2d::AKAZE> {
		#[inline] fn as_raw_mut_AKAZE(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<dyn crate::features2d::AKAZE> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<dyn crate::features2d::AKAZE> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTraitConst for core::Ptr<dyn crate::features2d::AKAZE> {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::Feature2DTrait for core::Ptr<dyn crate::features2d::AKAZE> {
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfAKAZE, core::Ptr<crate::features2d::Feature2D>, cv_PtrOfAKAZE_to_PtrOfFeature2D }
	
	pub type PtrOfAffineFeature = core::Ptr<dyn crate::features2d::AffineFeature>;
	
	ptr_extern! { dyn crate::features2d::AffineFeature,
		cv_PtrOfAffineFeature_delete, cv_PtrOfAffineFeature_get_inner_ptr, cv_PtrOfAffineFeature_get_inner_ptr_mut
	}
	
	impl core::Ptr<dyn crate::features2d::AffineFeature> {
		#[inline] pub fn as_raw_PtrOfAffineFeature(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfAffineFeature(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::features2d::AffineFeatureConst for core::Ptr<dyn crate::features2d::AffineFeature> {
		#[inline] fn as_raw_AffineFeature(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::AffineFeature for core::Ptr<dyn crate::features2d::AffineFeature> {
		#[inline] fn as_raw_mut_AffineFeature(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<dyn crate::features2d::AffineFeature> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<dyn crate::features2d::AffineFeature> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTraitConst for core::Ptr<dyn crate::features2d::AffineFeature> {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::Feature2DTrait for core::Ptr<dyn crate::features2d::AffineFeature> {
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfAffineFeature, core::Ptr<crate::features2d::Feature2D>, cv_PtrOfAffineFeature_to_PtrOfFeature2D }
	
	pub type PtrOfAgastFeatureDetector = core::Ptr<dyn crate::features2d::AgastFeatureDetector>;
	
	ptr_extern! { dyn crate::features2d::AgastFeatureDetector,
		cv_PtrOfAgastFeatureDetector_delete, cv_PtrOfAgastFeatureDetector_get_inner_ptr, cv_PtrOfAgastFeatureDetector_get_inner_ptr_mut
	}
	
	impl core::Ptr<dyn crate::features2d::AgastFeatureDetector> {
		#[inline] pub fn as_raw_PtrOfAgastFeatureDetector(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfAgastFeatureDetector(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::features2d::AgastFeatureDetectorConst for core::Ptr<dyn crate::features2d::AgastFeatureDetector> {
		#[inline] fn as_raw_AgastFeatureDetector(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::AgastFeatureDetector for core::Ptr<dyn crate::features2d::AgastFeatureDetector> {
		#[inline] fn as_raw_mut_AgastFeatureDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<dyn crate::features2d::AgastFeatureDetector> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<dyn crate::features2d::AgastFeatureDetector> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTraitConst for core::Ptr<dyn crate::features2d::AgastFeatureDetector> {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::Feature2DTrait for core::Ptr<dyn crate::features2d::AgastFeatureDetector> {
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfAgastFeatureDetector, core::Ptr<crate::features2d::Feature2D>, cv_PtrOfAgastFeatureDetector_to_PtrOfFeature2D }
	
	pub type PtrOfBFMatcher = core::Ptr<crate::features2d::BFMatcher>;
	
	ptr_extern! { crate::features2d::BFMatcher,
		cv_PtrOfBFMatcher_delete, cv_PtrOfBFMatcher_get_inner_ptr, cv_PtrOfBFMatcher_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::features2d::BFMatcher, cv_PtrOfBFMatcher_new }
	
	impl core::Ptr<crate::features2d::BFMatcher> {
		#[inline] pub fn as_raw_PtrOfBFMatcher(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBFMatcher(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::features2d::BFMatcherTraitConst for core::Ptr<crate::features2d::BFMatcher> {
		#[inline] fn as_raw_BFMatcher(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::BFMatcherTrait for core::Ptr<crate::features2d::BFMatcher> {
		#[inline] fn as_raw_mut_BFMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::features2d::BFMatcher> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::features2d::BFMatcher> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::DescriptorMatcherConst for core::Ptr<crate::features2d::BFMatcher> {
		#[inline] fn as_raw_DescriptorMatcher(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::DescriptorMatcher for core::Ptr<crate::features2d::BFMatcher> {
		#[inline] fn as_raw_mut_DescriptorMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfBRISK = core::Ptr<crate::features2d::BRISK>;
	
	ptr_extern! { crate::features2d::BRISK,
		cv_PtrOfBRISK_delete, cv_PtrOfBRISK_get_inner_ptr, cv_PtrOfBRISK_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::features2d::BRISK, cv_PtrOfBRISK_new }
	
	impl core::Ptr<crate::features2d::BRISK> {
		#[inline] pub fn as_raw_PtrOfBRISK(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBRISK(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::features2d::BRISKTraitConst for core::Ptr<crate::features2d::BRISK> {
		#[inline] fn as_raw_BRISK(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::BRISKTrait for core::Ptr<crate::features2d::BRISK> {
		#[inline] fn as_raw_mut_BRISK(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::features2d::BRISK> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::features2d::BRISK> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTraitConst for core::Ptr<crate::features2d::BRISK> {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::Feature2DTrait for core::Ptr<crate::features2d::BRISK> {
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfBRISK, core::Ptr<crate::features2d::Feature2D>, cv_PtrOfBRISK_to_PtrOfFeature2D }
	
	pub type PtrOfDescriptorMatcher = core::Ptr<dyn crate::features2d::DescriptorMatcher>;
	
	ptr_extern! { dyn crate::features2d::DescriptorMatcher,
		cv_PtrOfDescriptorMatcher_delete, cv_PtrOfDescriptorMatcher_get_inner_ptr, cv_PtrOfDescriptorMatcher_get_inner_ptr_mut
	}
	
	impl core::Ptr<dyn crate::features2d::DescriptorMatcher> {
		#[inline] pub fn as_raw_PtrOfDescriptorMatcher(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDescriptorMatcher(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::features2d::DescriptorMatcherConst for core::Ptr<dyn crate::features2d::DescriptorMatcher> {
		#[inline] fn as_raw_DescriptorMatcher(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::DescriptorMatcher for core::Ptr<dyn crate::features2d::DescriptorMatcher> {
		#[inline] fn as_raw_mut_DescriptorMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<dyn crate::features2d::DescriptorMatcher> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<dyn crate::features2d::DescriptorMatcher> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfFastFeatureDetector = core::Ptr<dyn crate::features2d::FastFeatureDetector>;
	
	ptr_extern! { dyn crate::features2d::FastFeatureDetector,
		cv_PtrOfFastFeatureDetector_delete, cv_PtrOfFastFeatureDetector_get_inner_ptr, cv_PtrOfFastFeatureDetector_get_inner_ptr_mut
	}
	
	impl core::Ptr<dyn crate::features2d::FastFeatureDetector> {
		#[inline] pub fn as_raw_PtrOfFastFeatureDetector(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFastFeatureDetector(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::features2d::FastFeatureDetectorConst for core::Ptr<dyn crate::features2d::FastFeatureDetector> {
		#[inline] fn as_raw_FastFeatureDetector(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::FastFeatureDetector for core::Ptr<dyn crate::features2d::FastFeatureDetector> {
		#[inline] fn as_raw_mut_FastFeatureDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<dyn crate::features2d::FastFeatureDetector> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<dyn crate::features2d::FastFeatureDetector> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTraitConst for core::Ptr<dyn crate::features2d::FastFeatureDetector> {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::Feature2DTrait for core::Ptr<dyn crate::features2d::FastFeatureDetector> {
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfFastFeatureDetector, core::Ptr<crate::features2d::Feature2D>, cv_PtrOfFastFeatureDetector_to_PtrOfFeature2D }
	
	pub type PtrOfFeature2D = core::Ptr<crate::features2d::Feature2D>;
	
	ptr_extern! { crate::features2d::Feature2D,
		cv_PtrOfFeature2D_delete, cv_PtrOfFeature2D_get_inner_ptr, cv_PtrOfFeature2D_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::features2d::Feature2D, cv_PtrOfFeature2D_new }
	
	impl core::Ptr<crate::features2d::Feature2D> {
		#[inline] pub fn as_raw_PtrOfFeature2D(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFeature2D(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTraitConst for core::Ptr<crate::features2d::Feature2D> {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::Feature2DTrait for core::Ptr<crate::features2d::Feature2D> {
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::features2d::Feature2D> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::features2d::Feature2D> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfFlannBasedMatcher = core::Ptr<crate::features2d::FlannBasedMatcher>;
	
	ptr_extern! { crate::features2d::FlannBasedMatcher,
		cv_PtrOfFlannBasedMatcher_delete, cv_PtrOfFlannBasedMatcher_get_inner_ptr, cv_PtrOfFlannBasedMatcher_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::features2d::FlannBasedMatcher, cv_PtrOfFlannBasedMatcher_new }
	
	impl core::Ptr<crate::features2d::FlannBasedMatcher> {
		#[inline] pub fn as_raw_PtrOfFlannBasedMatcher(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFlannBasedMatcher(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::features2d::FlannBasedMatcherTraitConst for core::Ptr<crate::features2d::FlannBasedMatcher> {
		#[inline] fn as_raw_FlannBasedMatcher(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::FlannBasedMatcherTrait for core::Ptr<crate::features2d::FlannBasedMatcher> {
		#[inline] fn as_raw_mut_FlannBasedMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::features2d::FlannBasedMatcher> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::features2d::FlannBasedMatcher> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::DescriptorMatcherConst for core::Ptr<crate::features2d::FlannBasedMatcher> {
		#[inline] fn as_raw_DescriptorMatcher(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::DescriptorMatcher for core::Ptr<crate::features2d::FlannBasedMatcher> {
		#[inline] fn as_raw_mut_DescriptorMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfGFTTDetector = core::Ptr<dyn crate::features2d::GFTTDetector>;
	
	ptr_extern! { dyn crate::features2d::GFTTDetector,
		cv_PtrOfGFTTDetector_delete, cv_PtrOfGFTTDetector_get_inner_ptr, cv_PtrOfGFTTDetector_get_inner_ptr_mut
	}
	
	impl core::Ptr<dyn crate::features2d::GFTTDetector> {
		#[inline] pub fn as_raw_PtrOfGFTTDetector(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfGFTTDetector(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::features2d::GFTTDetectorConst for core::Ptr<dyn crate::features2d::GFTTDetector> {
		#[inline] fn as_raw_GFTTDetector(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::GFTTDetector for core::Ptr<dyn crate::features2d::GFTTDetector> {
		#[inline] fn as_raw_mut_GFTTDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<dyn crate::features2d::GFTTDetector> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<dyn crate::features2d::GFTTDetector> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTraitConst for core::Ptr<dyn crate::features2d::GFTTDetector> {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::Feature2DTrait for core::Ptr<dyn crate::features2d::GFTTDetector> {
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfGFTTDetector, core::Ptr<crate::features2d::Feature2D>, cv_PtrOfGFTTDetector_to_PtrOfFeature2D }
	
	pub type PtrOfKAZE = core::Ptr<dyn crate::features2d::KAZE>;
	
	ptr_extern! { dyn crate::features2d::KAZE,
		cv_PtrOfKAZE_delete, cv_PtrOfKAZE_get_inner_ptr, cv_PtrOfKAZE_get_inner_ptr_mut
	}
	
	impl core::Ptr<dyn crate::features2d::KAZE> {
		#[inline] pub fn as_raw_PtrOfKAZE(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfKAZE(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::features2d::KAZEConst for core::Ptr<dyn crate::features2d::KAZE> {
		#[inline] fn as_raw_KAZE(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::KAZE for core::Ptr<dyn crate::features2d::KAZE> {
		#[inline] fn as_raw_mut_KAZE(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<dyn crate::features2d::KAZE> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<dyn crate::features2d::KAZE> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTraitConst for core::Ptr<dyn crate::features2d::KAZE> {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::Feature2DTrait for core::Ptr<dyn crate::features2d::KAZE> {
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfKAZE, core::Ptr<crate::features2d::Feature2D>, cv_PtrOfKAZE_to_PtrOfFeature2D }
	
	pub type PtrOfMSER = core::Ptr<dyn crate::features2d::MSER>;
	
	ptr_extern! { dyn crate::features2d::MSER,
		cv_PtrOfMSER_delete, cv_PtrOfMSER_get_inner_ptr, cv_PtrOfMSER_get_inner_ptr_mut
	}
	
	impl core::Ptr<dyn crate::features2d::MSER> {
		#[inline] pub fn as_raw_PtrOfMSER(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMSER(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::features2d::MSERConst for core::Ptr<dyn crate::features2d::MSER> {
		#[inline] fn as_raw_MSER(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::MSER for core::Ptr<dyn crate::features2d::MSER> {
		#[inline] fn as_raw_mut_MSER(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<dyn crate::features2d::MSER> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<dyn crate::features2d::MSER> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTraitConst for core::Ptr<dyn crate::features2d::MSER> {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::Feature2DTrait for core::Ptr<dyn crate::features2d::MSER> {
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfMSER, core::Ptr<crate::features2d::Feature2D>, cv_PtrOfMSER_to_PtrOfFeature2D }
	
	pub type PtrOfORB = core::Ptr<dyn crate::features2d::ORB>;
	
	ptr_extern! { dyn crate::features2d::ORB,
		cv_PtrOfORB_delete, cv_PtrOfORB_get_inner_ptr, cv_PtrOfORB_get_inner_ptr_mut
	}
	
	impl core::Ptr<dyn crate::features2d::ORB> {
		#[inline] pub fn as_raw_PtrOfORB(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfORB(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::features2d::ORBConst for core::Ptr<dyn crate::features2d::ORB> {
		#[inline] fn as_raw_ORB(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::ORB for core::Ptr<dyn crate::features2d::ORB> {
		#[inline] fn as_raw_mut_ORB(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<dyn crate::features2d::ORB> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<dyn crate::features2d::ORB> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTraitConst for core::Ptr<dyn crate::features2d::ORB> {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::Feature2DTrait for core::Ptr<dyn crate::features2d::ORB> {
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfORB, core::Ptr<crate::features2d::Feature2D>, cv_PtrOfORB_to_PtrOfFeature2D }
	
	pub type PtrOfSIFT = core::Ptr<crate::features2d::SIFT>;
	
	ptr_extern! { crate::features2d::SIFT,
		cv_PtrOfSIFT_delete, cv_PtrOfSIFT_get_inner_ptr, cv_PtrOfSIFT_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::features2d::SIFT, cv_PtrOfSIFT_new }
	
	impl core::Ptr<crate::features2d::SIFT> {
		#[inline] pub fn as_raw_PtrOfSIFT(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSIFT(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::features2d::SIFTTraitConst for core::Ptr<crate::features2d::SIFT> {
		#[inline] fn as_raw_SIFT(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::SIFTTrait for core::Ptr<crate::features2d::SIFT> {
		#[inline] fn as_raw_mut_SIFT(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::features2d::SIFT> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::features2d::SIFT> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTraitConst for core::Ptr<crate::features2d::SIFT> {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::Feature2DTrait for core::Ptr<crate::features2d::SIFT> {
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfSIFT, core::Ptr<crate::features2d::Feature2D>, cv_PtrOfSIFT_to_PtrOfFeature2D }
	
	pub type PtrOfSimpleBlobDetector = core::Ptr<crate::features2d::SimpleBlobDetector>;
	
	ptr_extern! { crate::features2d::SimpleBlobDetector,
		cv_PtrOfSimpleBlobDetector_delete, cv_PtrOfSimpleBlobDetector_get_inner_ptr, cv_PtrOfSimpleBlobDetector_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::features2d::SimpleBlobDetector, cv_PtrOfSimpleBlobDetector_new }
	
	impl core::Ptr<crate::features2d::SimpleBlobDetector> {
		#[inline] pub fn as_raw_PtrOfSimpleBlobDetector(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSimpleBlobDetector(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::features2d::SimpleBlobDetectorTraitConst for core::Ptr<crate::features2d::SimpleBlobDetector> {
		#[inline] fn as_raw_SimpleBlobDetector(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::SimpleBlobDetectorTrait for core::Ptr<crate::features2d::SimpleBlobDetector> {
		#[inline] fn as_raw_mut_SimpleBlobDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::features2d::SimpleBlobDetector> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::features2d::SimpleBlobDetector> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTraitConst for core::Ptr<crate::features2d::SimpleBlobDetector> {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::Feature2DTrait for core::Ptr<crate::features2d::SimpleBlobDetector> {
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfSimpleBlobDetector, core::Ptr<crate::features2d::Feature2D>, cv_PtrOfSimpleBlobDetector_to_PtrOfFeature2D }
	
}
#[cfg(ocvrs_has_module_features2d)]
pub use features2d_types::*;

#[cfg(ocvrs_has_module_flann)]
mod flann_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfIndexParams = core::Ptr<crate::flann::IndexParams>;
	
	ptr_extern! { crate::flann::IndexParams,
		cv_PtrOfIndexParams_delete, cv_PtrOfIndexParams_get_inner_ptr, cv_PtrOfIndexParams_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::flann::IndexParams, cv_PtrOfIndexParams_new }
	
	impl core::Ptr<crate::flann::IndexParams> {
		#[inline] pub fn as_raw_PtrOfIndexParams(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfIndexParams(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::flann::IndexParamsTraitConst for core::Ptr<crate::flann::IndexParams> {
		#[inline] fn as_raw_IndexParams(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::flann::IndexParamsTrait for core::Ptr<crate::flann::IndexParams> {
		#[inline] fn as_raw_mut_IndexParams(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSearchParams = core::Ptr<crate::flann::SearchParams>;
	
	ptr_extern! { crate::flann::SearchParams,
		cv_PtrOfSearchParams_delete, cv_PtrOfSearchParams_get_inner_ptr, cv_PtrOfSearchParams_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::flann::SearchParams, cv_PtrOfSearchParams_new }
	
	impl core::Ptr<crate::flann::SearchParams> {
		#[inline] pub fn as_raw_PtrOfSearchParams(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSearchParams(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::flann::SearchParamsTraitConst for core::Ptr<crate::flann::SearchParams> {
		#[inline] fn as_raw_SearchParams(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::flann::SearchParamsTrait for core::Ptr<crate::flann::SearchParams> {
		#[inline] fn as_raw_mut_SearchParams(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::flann::IndexParamsTraitConst for core::Ptr<crate::flann::SearchParams> {
		#[inline] fn as_raw_IndexParams(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::flann::IndexParamsTrait for core::Ptr<crate::flann::SearchParams> {
		#[inline] fn as_raw_mut_IndexParams(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type VectorOfFlannIndexType = core::Vector<crate::flann::FlannIndexType>;
	
	impl core::Vector<crate::flann::FlannIndexType> {
		pub fn as_raw_VectorOfFlannIndexType(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfFlannIndexType(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { crate::flann::FlannIndexType,
		cv_VectorOfFlannIndexType_new, cv_VectorOfFlannIndexType_delete,
		cv_VectorOfFlannIndexType_len, cv_VectorOfFlannIndexType_is_empty,
		cv_VectorOfFlannIndexType_capacity, cv_VectorOfFlannIndexType_shrink_to_fit,
		cv_VectorOfFlannIndexType_reserve, cv_VectorOfFlannIndexType_remove,
		cv_VectorOfFlannIndexType_swap, cv_VectorOfFlannIndexType_clear,
		cv_VectorOfFlannIndexType_get, cv_VectorOfFlannIndexType_set,
		cv_VectorOfFlannIndexType_push, cv_VectorOfFlannIndexType_insert,
	}
	vector_copy_non_bool! { crate::flann::FlannIndexType,
		cv_VectorOfFlannIndexType_data, cv_VectorOfFlannIndexType_data_mut, cv_VectorOfFlannIndexType_from_slice,
		cv_VectorOfFlannIndexType_clone,
	}
	
}
#[cfg(ocvrs_has_module_flann)]
pub use flann_types::*;

#[cfg(ocvrs_has_module_gapi)]
mod gapi_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type TupleOfGBackend_GKernelImpl = core::Tuple<(crate::gapi::GBackend, crate::gapi::GKernelImpl)>;
	
	impl core::Tuple<(crate::gapi::GBackend, crate::gapi::GKernelImpl)> {
		pub fn as_raw_TupleOfGBackend_GKernelImpl(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_TupleOfGBackend_GKernelImpl(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	tuple_extern! { (crate::gapi::GBackend, crate::gapi::GKernelImpl),
		cv_TupleOfGBackend_GKernelImpl_new, cv_TupleOfGBackend_GKernelImpl_delete,
		0 = arg: crate::gapi::GBackend, get_0 via cv_TupleOfGBackend_GKernelImpl_get_0,
		1 = arg_1: crate::gapi::GKernelImpl, get_1 via cv_TupleOfGBackend_GKernelImpl_get_1
	}
	
	pub type TupleOfGMat_GMat = core::Tuple<(crate::gapi::GMat, crate::gapi::GMat)>;
	
	impl core::Tuple<(crate::gapi::GMat, crate::gapi::GMat)> {
		pub fn as_raw_TupleOfGMat_GMat(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_TupleOfGMat_GMat(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	tuple_extern! { (crate::gapi::GMat, crate::gapi::GMat),
		cv_TupleOfGMat_GMat_new, cv_TupleOfGMat_GMat_delete,
		0 = arg: crate::gapi::GMat, get_0 via cv_TupleOfGMat_GMat_get_0,
		1 = arg_1: crate::gapi::GMat, get_1 via cv_TupleOfGMat_GMat_get_1
	}
	
	pub type TupleOfGMat_GMat_GMat = core::Tuple<(crate::gapi::GMat, crate::gapi::GMat, crate::gapi::GMat)>;
	
	impl core::Tuple<(crate::gapi::GMat, crate::gapi::GMat, crate::gapi::GMat)> {
		pub fn as_raw_TupleOfGMat_GMat_GMat(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_TupleOfGMat_GMat_GMat(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	tuple_extern! { (crate::gapi::GMat, crate::gapi::GMat, crate::gapi::GMat),
		cv_TupleOfGMat_GMat_GMat_new, cv_TupleOfGMat_GMat_GMat_delete,
		0 = arg: crate::gapi::GMat, get_0 via cv_TupleOfGMat_GMat_GMat_get_0,
		1 = arg_1: crate::gapi::GMat, get_1 via cv_TupleOfGMat_GMat_GMat_get_1,
		2 = arg_2: crate::gapi::GMat, get_2 via cv_TupleOfGMat_GMat_GMat_get_2
	}
	
	pub type TupleOfGMat_GMat_GMat_GMat = core::Tuple<(crate::gapi::GMat, crate::gapi::GMat, crate::gapi::GMat, crate::gapi::GMat)>;
	
	impl core::Tuple<(crate::gapi::GMat, crate::gapi::GMat, crate::gapi::GMat, crate::gapi::GMat)> {
		pub fn as_raw_TupleOfGMat_GMat_GMat_GMat(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_TupleOfGMat_GMat_GMat_GMat(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	tuple_extern! { (crate::gapi::GMat, crate::gapi::GMat, crate::gapi::GMat, crate::gapi::GMat),
		cv_TupleOfGMat_GMat_GMat_GMat_new, cv_TupleOfGMat_GMat_GMat_GMat_delete,
		0 = arg: crate::gapi::GMat, get_0 via cv_TupleOfGMat_GMat_GMat_GMat_get_0,
		1 = arg_1: crate::gapi::GMat, get_1 via cv_TupleOfGMat_GMat_GMat_GMat_get_1,
		2 = arg_2: crate::gapi::GMat, get_2 via cv_TupleOfGMat_GMat_GMat_GMat_get_2,
		3 = arg_3: crate::gapi::GMat, get_3 via cv_TupleOfGMat_GMat_GMat_GMat_get_3
	}
	
	pub type TupleOfGMat_GScalar = core::Tuple<(crate::gapi::GMat, crate::gapi::GScalar)>;
	
	impl core::Tuple<(crate::gapi::GMat, crate::gapi::GScalar)> {
		pub fn as_raw_TupleOfGMat_GScalar(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_TupleOfGMat_GScalar(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	tuple_extern! { (crate::gapi::GMat, crate::gapi::GScalar),
		cv_TupleOfGMat_GScalar_new, cv_TupleOfGMat_GScalar_delete,
		0 = arg: crate::gapi::GMat, get_0 via cv_TupleOfGMat_GScalar_get_0,
		1 = arg_1: crate::gapi::GScalar, get_1 via cv_TupleOfGMat_GScalar_get_1
	}
	
	pub type VectorOfGArg = core::Vector<crate::gapi::GArg>;
	
	impl core::Vector<crate::gapi::GArg> {
		pub fn as_raw_VectorOfGArg(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfGArg(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { crate::gapi::GArg,
		cv_VectorOfGArg_new, cv_VectorOfGArg_delete,
		cv_VectorOfGArg_len, cv_VectorOfGArg_is_empty,
		cv_VectorOfGArg_capacity, cv_VectorOfGArg_shrink_to_fit,
		cv_VectorOfGArg_reserve, cv_VectorOfGArg_remove,
		cv_VectorOfGArg_swap, cv_VectorOfGArg_clear,
		cv_VectorOfGArg_get, cv_VectorOfGArg_set,
		cv_VectorOfGArg_push, cv_VectorOfGArg_insert,
	}
	vector_non_copy_or_bool! { crate::gapi::GArg }
	
	pub type VectorOfGBackend = core::Vector<crate::gapi::GBackend>;
	
	impl core::Vector<crate::gapi::GBackend> {
		pub fn as_raw_VectorOfGBackend(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfGBackend(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { crate::gapi::GBackend,
		cv_VectorOfGBackend_new, cv_VectorOfGBackend_delete,
		cv_VectorOfGBackend_len, cv_VectorOfGBackend_is_empty,
		cv_VectorOfGBackend_capacity, cv_VectorOfGBackend_shrink_to_fit,
		cv_VectorOfGBackend_reserve, cv_VectorOfGBackend_remove,
		cv_VectorOfGBackend_swap, cv_VectorOfGBackend_clear,
		cv_VectorOfGBackend_get, cv_VectorOfGBackend_set,
		cv_VectorOfGBackend_push, cv_VectorOfGBackend_insert,
	}
	vector_non_copy_or_bool! { crate::gapi::GBackend }
	
	pub type VectorOfGCompileArg = core::Vector<crate::gapi::GCompileArg>;
	
	impl core::Vector<crate::gapi::GCompileArg> {
		pub fn as_raw_VectorOfGCompileArg(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfGCompileArg(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { crate::gapi::GCompileArg,
		cv_VectorOfGCompileArg_new, cv_VectorOfGCompileArg_delete,
		cv_VectorOfGCompileArg_len, cv_VectorOfGCompileArg_is_empty,
		cv_VectorOfGCompileArg_capacity, cv_VectorOfGCompileArg_shrink_to_fit,
		cv_VectorOfGCompileArg_reserve, cv_VectorOfGCompileArg_remove,
		cv_VectorOfGCompileArg_swap, cv_VectorOfGCompileArg_clear,
		cv_VectorOfGCompileArg_get, cv_VectorOfGCompileArg_set,
		cv_VectorOfGCompileArg_push, cv_VectorOfGCompileArg_insert,
	}
	vector_non_copy_or_bool! { crate::gapi::GCompileArg }
	
	pub type VectorOfGMat = core::Vector<crate::gapi::GMat>;
	
	impl core::Vector<crate::gapi::GMat> {
		pub fn as_raw_VectorOfGMat(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfGMat(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { crate::gapi::GMat,
		cv_VectorOfGMat_new, cv_VectorOfGMat_delete,
		cv_VectorOfGMat_len, cv_VectorOfGMat_is_empty,
		cv_VectorOfGMat_capacity, cv_VectorOfGMat_shrink_to_fit,
		cv_VectorOfGMat_reserve, cv_VectorOfGMat_remove,
		cv_VectorOfGMat_swap, cv_VectorOfGMat_clear,
		cv_VectorOfGMat_get, cv_VectorOfGMat_set,
		cv_VectorOfGMat_push, cv_VectorOfGMat_insert,
	}
	vector_non_copy_or_bool! { crate::gapi::GMat }
	
	pub type VectorOfGRunArg = core::Vector<crate::gapi::GRunArg>;
	
	impl core::Vector<crate::gapi::GRunArg> {
		pub fn as_raw_VectorOfGRunArg(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfGRunArg(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { crate::gapi::GRunArg,
		cv_VectorOfGRunArg_new, cv_VectorOfGRunArg_delete,
		cv_VectorOfGRunArg_len, cv_VectorOfGRunArg_is_empty,
		cv_VectorOfGRunArg_capacity, cv_VectorOfGRunArg_shrink_to_fit,
		cv_VectorOfGRunArg_reserve, cv_VectorOfGRunArg_remove,
		cv_VectorOfGRunArg_swap, cv_VectorOfGRunArg_clear,
		cv_VectorOfGRunArg_get, cv_VectorOfGRunArg_set,
		cv_VectorOfGRunArg_push, cv_VectorOfGRunArg_insert,
	}
	vector_non_copy_or_bool! { crate::gapi::GRunArg }
	
	pub type VectorOfGShape = core::Vector<crate::gapi::GShape>;
	
	impl core::Vector<crate::gapi::GShape> {
		pub fn as_raw_VectorOfGShape(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfGShape(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { crate::gapi::GShape,
		cv_VectorOfGShape_new, cv_VectorOfGShape_delete,
		cv_VectorOfGShape_len, cv_VectorOfGShape_is_empty,
		cv_VectorOfGShape_capacity, cv_VectorOfGShape_shrink_to_fit,
		cv_VectorOfGShape_reserve, cv_VectorOfGShape_remove,
		cv_VectorOfGShape_swap, cv_VectorOfGShape_clear,
		cv_VectorOfGShape_get, cv_VectorOfGShape_set,
		cv_VectorOfGShape_push, cv_VectorOfGShape_insert,
	}
	vector_copy_non_bool! { crate::gapi::GShape,
		cv_VectorOfGShape_data, cv_VectorOfGShape_data_mut, cv_VectorOfGShape_from_slice,
		cv_VectorOfGShape_clone,
	}
	
	pub type VectorOfGTransform = core::Vector<crate::gapi::GTransform>;
	
	impl core::Vector<crate::gapi::GTransform> {
		pub fn as_raw_VectorOfGTransform(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfGTransform(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { crate::gapi::GTransform,
		cv_VectorOfGTransform_new, cv_VectorOfGTransform_delete,
		cv_VectorOfGTransform_len, cv_VectorOfGTransform_is_empty,
		cv_VectorOfGTransform_capacity, cv_VectorOfGTransform_shrink_to_fit,
		cv_VectorOfGTransform_reserve, cv_VectorOfGTransform_remove,
		cv_VectorOfGTransform_swap, cv_VectorOfGTransform_clear,
		cv_VectorOfGTransform_get, cv_VectorOfGTransform_set,
		cv_VectorOfGTransform_push, cv_VectorOfGTransform_insert,
	}
	vector_non_copy_or_bool! { crate::gapi::GTransform }
	
	pub type VectorOfGTypeInfo = core::Vector<crate::gapi::GTypeInfo>;
	
	impl core::Vector<crate::gapi::GTypeInfo> {
		pub fn as_raw_VectorOfGTypeInfo(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfGTypeInfo(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { crate::gapi::GTypeInfo,
		cv_VectorOfGTypeInfo_new, cv_VectorOfGTypeInfo_delete,
		cv_VectorOfGTypeInfo_len, cv_VectorOfGTypeInfo_is_empty,
		cv_VectorOfGTypeInfo_capacity, cv_VectorOfGTypeInfo_shrink_to_fit,
		cv_VectorOfGTypeInfo_reserve, cv_VectorOfGTypeInfo_remove,
		cv_VectorOfGTypeInfo_swap, cv_VectorOfGTypeInfo_clear,
		cv_VectorOfGTypeInfo_get, cv_VectorOfGTypeInfo_set,
		cv_VectorOfGTypeInfo_push, cv_VectorOfGTypeInfo_insert,
	}
	vector_non_copy_or_bool! { crate::gapi::GTypeInfo }
	
	pub type VectorOfOpaqueKind = core::Vector<crate::gapi::OpaqueKind>;
	
	impl core::Vector<crate::gapi::OpaqueKind> {
		pub fn as_raw_VectorOfOpaqueKind(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfOpaqueKind(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { crate::gapi::OpaqueKind,
		cv_VectorOfOpaqueKind_new, cv_VectorOfOpaqueKind_delete,
		cv_VectorOfOpaqueKind_len, cv_VectorOfOpaqueKind_is_empty,
		cv_VectorOfOpaqueKind_capacity, cv_VectorOfOpaqueKind_shrink_to_fit,
		cv_VectorOfOpaqueKind_reserve, cv_VectorOfOpaqueKind_remove,
		cv_VectorOfOpaqueKind_swap, cv_VectorOfOpaqueKind_clear,
		cv_VectorOfOpaqueKind_get, cv_VectorOfOpaqueKind_set,
		cv_VectorOfOpaqueKind_push, cv_VectorOfOpaqueKind_insert,
	}
	vector_copy_non_bool! { crate::gapi::OpaqueKind,
		cv_VectorOfOpaqueKind_data, cv_VectorOfOpaqueKind_data_mut, cv_VectorOfOpaqueKind_from_slice,
		cv_VectorOfOpaqueKind_clone,
	}
	
}
#[cfg(ocvrs_has_module_gapi)]
pub use gapi_types::*;

#[cfg(ocvrs_has_module_imgproc)]
mod imgproc_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfCLAHE = core::Ptr<dyn crate::imgproc::CLAHE>;
	
	ptr_extern! { dyn crate::imgproc::CLAHE,
		cv_PtrOfCLAHE_delete, cv_PtrOfCLAHE_get_inner_ptr, cv_PtrOfCLAHE_get_inner_ptr_mut
	}
	
	impl core::Ptr<dyn crate::imgproc::CLAHE> {
		#[inline] pub fn as_raw_PtrOfCLAHE(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCLAHE(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::imgproc::CLAHEConst for core::Ptr<dyn crate::imgproc::CLAHE> {
		#[inline] fn as_raw_CLAHE(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::imgproc::CLAHE for core::Ptr<dyn crate::imgproc::CLAHE> {
		#[inline] fn as_raw_mut_CLAHE(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<dyn crate::imgproc::CLAHE> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<dyn crate::imgproc::CLAHE> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfGeneralizedHoughBallard = core::Ptr<dyn crate::imgproc::GeneralizedHoughBallard>;
	
	ptr_extern! { dyn crate::imgproc::GeneralizedHoughBallard,
		cv_PtrOfGeneralizedHoughBallard_delete, cv_PtrOfGeneralizedHoughBallard_get_inner_ptr, cv_PtrOfGeneralizedHoughBallard_get_inner_ptr_mut
	}
	
	impl core::Ptr<dyn crate::imgproc::GeneralizedHoughBallard> {
		#[inline] pub fn as_raw_PtrOfGeneralizedHoughBallard(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfGeneralizedHoughBallard(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::imgproc::GeneralizedHoughBallardConst for core::Ptr<dyn crate::imgproc::GeneralizedHoughBallard> {
		#[inline] fn as_raw_GeneralizedHoughBallard(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::imgproc::GeneralizedHoughBallard for core::Ptr<dyn crate::imgproc::GeneralizedHoughBallard> {
		#[inline] fn as_raw_mut_GeneralizedHoughBallard(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<dyn crate::imgproc::GeneralizedHoughBallard> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<dyn crate::imgproc::GeneralizedHoughBallard> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::imgproc::GeneralizedHoughConst for core::Ptr<dyn crate::imgproc::GeneralizedHoughBallard> {
		#[inline] fn as_raw_GeneralizedHough(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::imgproc::GeneralizedHough for core::Ptr<dyn crate::imgproc::GeneralizedHoughBallard> {
		#[inline] fn as_raw_mut_GeneralizedHough(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfGeneralizedHoughGuil = core::Ptr<dyn crate::imgproc::GeneralizedHoughGuil>;
	
	ptr_extern! { dyn crate::imgproc::GeneralizedHoughGuil,
		cv_PtrOfGeneralizedHoughGuil_delete, cv_PtrOfGeneralizedHoughGuil_get_inner_ptr, cv_PtrOfGeneralizedHoughGuil_get_inner_ptr_mut
	}
	
	impl core::Ptr<dyn crate::imgproc::GeneralizedHoughGuil> {
		#[inline] pub fn as_raw_PtrOfGeneralizedHoughGuil(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfGeneralizedHoughGuil(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::imgproc::GeneralizedHoughGuilConst for core::Ptr<dyn crate::imgproc::GeneralizedHoughGuil> {
		#[inline] fn as_raw_GeneralizedHoughGuil(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::imgproc::GeneralizedHoughGuil for core::Ptr<dyn crate::imgproc::GeneralizedHoughGuil> {
		#[inline] fn as_raw_mut_GeneralizedHoughGuil(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<dyn crate::imgproc::GeneralizedHoughGuil> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<dyn crate::imgproc::GeneralizedHoughGuil> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::imgproc::GeneralizedHoughConst for core::Ptr<dyn crate::imgproc::GeneralizedHoughGuil> {
		#[inline] fn as_raw_GeneralizedHough(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::imgproc::GeneralizedHough for core::Ptr<dyn crate::imgproc::GeneralizedHoughGuil> {
		#[inline] fn as_raw_mut_GeneralizedHough(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfLineSegmentDetector = core::Ptr<dyn crate::imgproc::LineSegmentDetector>;
	
	ptr_extern! { dyn crate::imgproc::LineSegmentDetector,
		cv_PtrOfLineSegmentDetector_delete, cv_PtrOfLineSegmentDetector_get_inner_ptr, cv_PtrOfLineSegmentDetector_get_inner_ptr_mut
	}
	
	impl core::Ptr<dyn crate::imgproc::LineSegmentDetector> {
		#[inline] pub fn as_raw_PtrOfLineSegmentDetector(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfLineSegmentDetector(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::imgproc::LineSegmentDetectorConst for core::Ptr<dyn crate::imgproc::LineSegmentDetector> {
		#[inline] fn as_raw_LineSegmentDetector(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::imgproc::LineSegmentDetector for core::Ptr<dyn crate::imgproc::LineSegmentDetector> {
		#[inline] fn as_raw_mut_LineSegmentDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<dyn crate::imgproc::LineSegmentDetector> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<dyn crate::imgproc::LineSegmentDetector> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(ocvrs_has_module_imgproc)]
pub use imgproc_types::*;

#[cfg(ocvrs_has_module_ml)]
mod ml_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfANN_MLP = core::Ptr<dyn crate::ml::ANN_MLP>;
	
	ptr_extern! { dyn crate::ml::ANN_MLP,
		cv_PtrOfANN_MLP_delete, cv_PtrOfANN_MLP_get_inner_ptr, cv_PtrOfANN_MLP_get_inner_ptr_mut
	}
	
	impl core::Ptr<dyn crate::ml::ANN_MLP> {
		#[inline] pub fn as_raw_PtrOfANN_MLP(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfANN_MLP(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::ml::ANN_MLPConst for core::Ptr<dyn crate::ml::ANN_MLP> {
		#[inline] fn as_raw_ANN_MLP(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ml::ANN_MLP for core::Ptr<dyn crate::ml::ANN_MLP> {
		#[inline] fn as_raw_mut_ANN_MLP(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<dyn crate::ml::ANN_MLP> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<dyn crate::ml::ANN_MLP> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ml::StatModelConst for core::Ptr<dyn crate::ml::ANN_MLP> {
		#[inline] fn as_raw_StatModel(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ml::StatModel for core::Ptr<dyn crate::ml::ANN_MLP> {
		#[inline] fn as_raw_mut_StatModel(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfBoost = core::Ptr<dyn crate::ml::Boost>;
	
	ptr_extern! { dyn crate::ml::Boost,
		cv_PtrOfBoost_delete, cv_PtrOfBoost_get_inner_ptr, cv_PtrOfBoost_get_inner_ptr_mut
	}
	
	impl core::Ptr<dyn crate::ml::Boost> {
		#[inline] pub fn as_raw_PtrOfBoost(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBoost(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::ml::BoostConst for core::Ptr<dyn crate::ml::Boost> {
		#[inline] fn as_raw_Boost(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ml::Boost for core::Ptr<dyn crate::ml::Boost> {
		#[inline] fn as_raw_mut_Boost(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<dyn crate::ml::Boost> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<dyn crate::ml::Boost> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ml::DTreesConst for core::Ptr<dyn crate::ml::Boost> {
		#[inline] fn as_raw_DTrees(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ml::DTrees for core::Ptr<dyn crate::ml::Boost> {
		#[inline] fn as_raw_mut_DTrees(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ml::StatModelConst for core::Ptr<dyn crate::ml::Boost> {
		#[inline] fn as_raw_StatModel(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ml::StatModel for core::Ptr<dyn crate::ml::Boost> {
		#[inline] fn as_raw_mut_StatModel(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDTrees = core::Ptr<dyn crate::ml::DTrees>;
	
	ptr_extern! { dyn crate::ml::DTrees,
		cv_PtrOfDTrees_delete, cv_PtrOfDTrees_get_inner_ptr, cv_PtrOfDTrees_get_inner_ptr_mut
	}
	
	impl core::Ptr<dyn crate::ml::DTrees> {
		#[inline] pub fn as_raw_PtrOfDTrees(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDTrees(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::ml::DTreesConst for core::Ptr<dyn crate::ml::DTrees> {
		#[inline] fn as_raw_DTrees(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ml::DTrees for core::Ptr<dyn crate::ml::DTrees> {
		#[inline] fn as_raw_mut_DTrees(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<dyn crate::ml::DTrees> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<dyn crate::ml::DTrees> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ml::StatModelConst for core::Ptr<dyn crate::ml::DTrees> {
		#[inline] fn as_raw_StatModel(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ml::StatModel for core::Ptr<dyn crate::ml::DTrees> {
		#[inline] fn as_raw_mut_StatModel(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfEM = core::Ptr<dyn crate::ml::EM>;
	
	ptr_extern! { dyn crate::ml::EM,
		cv_PtrOfEM_delete, cv_PtrOfEM_get_inner_ptr, cv_PtrOfEM_get_inner_ptr_mut
	}
	
	impl core::Ptr<dyn crate::ml::EM> {
		#[inline] pub fn as_raw_PtrOfEM(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfEM(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::ml::EMConst for core::Ptr<dyn crate::ml::EM> {
		#[inline] fn as_raw_EM(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ml::EM for core::Ptr<dyn crate::ml::EM> {
		#[inline] fn as_raw_mut_EM(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<dyn crate::ml::EM> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<dyn crate::ml::EM> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ml::StatModelConst for core::Ptr<dyn crate::ml::EM> {
		#[inline] fn as_raw_StatModel(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ml::StatModel for core::Ptr<dyn crate::ml::EM> {
		#[inline] fn as_raw_mut_StatModel(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfKNearest = core::Ptr<dyn crate::ml::KNearest>;
	
	ptr_extern! { dyn crate::ml::KNearest,
		cv_PtrOfKNearest_delete, cv_PtrOfKNearest_get_inner_ptr, cv_PtrOfKNearest_get_inner_ptr_mut
	}
	
	impl core::Ptr<dyn crate::ml::KNearest> {
		#[inline] pub fn as_raw_PtrOfKNearest(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfKNearest(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::ml::KNearestConst for core::Ptr<dyn crate::ml::KNearest> {
		#[inline] fn as_raw_KNearest(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ml::KNearest for core::Ptr<dyn crate::ml::KNearest> {
		#[inline] fn as_raw_mut_KNearest(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<dyn crate::ml::KNearest> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<dyn crate::ml::KNearest> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ml::StatModelConst for core::Ptr<dyn crate::ml::KNearest> {
		#[inline] fn as_raw_StatModel(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ml::StatModel for core::Ptr<dyn crate::ml::KNearest> {
		#[inline] fn as_raw_mut_StatModel(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfLogisticRegression = core::Ptr<dyn crate::ml::LogisticRegression>;
	
	ptr_extern! { dyn crate::ml::LogisticRegression,
		cv_PtrOfLogisticRegression_delete, cv_PtrOfLogisticRegression_get_inner_ptr, cv_PtrOfLogisticRegression_get_inner_ptr_mut
	}
	
	impl core::Ptr<dyn crate::ml::LogisticRegression> {
		#[inline] pub fn as_raw_PtrOfLogisticRegression(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfLogisticRegression(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::ml::LogisticRegressionConst for core::Ptr<dyn crate::ml::LogisticRegression> {
		#[inline] fn as_raw_LogisticRegression(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ml::LogisticRegression for core::Ptr<dyn crate::ml::LogisticRegression> {
		#[inline] fn as_raw_mut_LogisticRegression(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<dyn crate::ml::LogisticRegression> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<dyn crate::ml::LogisticRegression> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ml::StatModelConst for core::Ptr<dyn crate::ml::LogisticRegression> {
		#[inline] fn as_raw_StatModel(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ml::StatModel for core::Ptr<dyn crate::ml::LogisticRegression> {
		#[inline] fn as_raw_mut_StatModel(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfNormalBayesClassifier = core::Ptr<dyn crate::ml::NormalBayesClassifier>;
	
	ptr_extern! { dyn crate::ml::NormalBayesClassifier,
		cv_PtrOfNormalBayesClassifier_delete, cv_PtrOfNormalBayesClassifier_get_inner_ptr, cv_PtrOfNormalBayesClassifier_get_inner_ptr_mut
	}
	
	impl core::Ptr<dyn crate::ml::NormalBayesClassifier> {
		#[inline] pub fn as_raw_PtrOfNormalBayesClassifier(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfNormalBayesClassifier(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::ml::NormalBayesClassifierConst for core::Ptr<dyn crate::ml::NormalBayesClassifier> {
		#[inline] fn as_raw_NormalBayesClassifier(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ml::NormalBayesClassifier for core::Ptr<dyn crate::ml::NormalBayesClassifier> {
		#[inline] fn as_raw_mut_NormalBayesClassifier(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<dyn crate::ml::NormalBayesClassifier> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<dyn crate::ml::NormalBayesClassifier> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ml::StatModelConst for core::Ptr<dyn crate::ml::NormalBayesClassifier> {
		#[inline] fn as_raw_StatModel(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ml::StatModel for core::Ptr<dyn crate::ml::NormalBayesClassifier> {
		#[inline] fn as_raw_mut_StatModel(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfParamGrid = core::Ptr<crate::ml::ParamGrid>;
	
	ptr_extern! { crate::ml::ParamGrid,
		cv_PtrOfParamGrid_delete, cv_PtrOfParamGrid_get_inner_ptr, cv_PtrOfParamGrid_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::ml::ParamGrid, cv_PtrOfParamGrid_new }
	
	impl core::Ptr<crate::ml::ParamGrid> {
		#[inline] pub fn as_raw_PtrOfParamGrid(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfParamGrid(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::ml::ParamGridTraitConst for core::Ptr<crate::ml::ParamGrid> {
		#[inline] fn as_raw_ParamGrid(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ml::ParamGridTrait for core::Ptr<crate::ml::ParamGrid> {
		#[inline] fn as_raw_mut_ParamGrid(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfRTrees = core::Ptr<dyn crate::ml::RTrees>;
	
	ptr_extern! { dyn crate::ml::RTrees,
		cv_PtrOfRTrees_delete, cv_PtrOfRTrees_get_inner_ptr, cv_PtrOfRTrees_get_inner_ptr_mut
	}
	
	impl core::Ptr<dyn crate::ml::RTrees> {
		#[inline] pub fn as_raw_PtrOfRTrees(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfRTrees(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::ml::RTreesConst for core::Ptr<dyn crate::ml::RTrees> {
		#[inline] fn as_raw_RTrees(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ml::RTrees for core::Ptr<dyn crate::ml::RTrees> {
		#[inline] fn as_raw_mut_RTrees(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<dyn crate::ml::RTrees> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<dyn crate::ml::RTrees> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ml::DTreesConst for core::Ptr<dyn crate::ml::RTrees> {
		#[inline] fn as_raw_DTrees(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ml::DTrees for core::Ptr<dyn crate::ml::RTrees> {
		#[inline] fn as_raw_mut_DTrees(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ml::StatModelConst for core::Ptr<dyn crate::ml::RTrees> {
		#[inline] fn as_raw_StatModel(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ml::StatModel for core::Ptr<dyn crate::ml::RTrees> {
		#[inline] fn as_raw_mut_StatModel(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSVM = core::Ptr<dyn crate::ml::SVM>;
	
	ptr_extern! { dyn crate::ml::SVM,
		cv_PtrOfSVM_delete, cv_PtrOfSVM_get_inner_ptr, cv_PtrOfSVM_get_inner_ptr_mut
	}
	
	impl core::Ptr<dyn crate::ml::SVM> {
		#[inline] pub fn as_raw_PtrOfSVM(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSVM(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::ml::SVMConst for core::Ptr<dyn crate::ml::SVM> {
		#[inline] fn as_raw_SVM(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ml::SVM for core::Ptr<dyn crate::ml::SVM> {
		#[inline] fn as_raw_mut_SVM(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<dyn crate::ml::SVM> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<dyn crate::ml::SVM> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ml::StatModelConst for core::Ptr<dyn crate::ml::SVM> {
		#[inline] fn as_raw_StatModel(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ml::StatModel for core::Ptr<dyn crate::ml::SVM> {
		#[inline] fn as_raw_mut_StatModel(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSVMSGD = core::Ptr<dyn crate::ml::SVMSGD>;
	
	ptr_extern! { dyn crate::ml::SVMSGD,
		cv_PtrOfSVMSGD_delete, cv_PtrOfSVMSGD_get_inner_ptr, cv_PtrOfSVMSGD_get_inner_ptr_mut
	}
	
	impl core::Ptr<dyn crate::ml::SVMSGD> {
		#[inline] pub fn as_raw_PtrOfSVMSGD(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSVMSGD(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::ml::SVMSGDConst for core::Ptr<dyn crate::ml::SVMSGD> {
		#[inline] fn as_raw_SVMSGD(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ml::SVMSGD for core::Ptr<dyn crate::ml::SVMSGD> {
		#[inline] fn as_raw_mut_SVMSGD(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<dyn crate::ml::SVMSGD> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<dyn crate::ml::SVMSGD> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ml::StatModelConst for core::Ptr<dyn crate::ml::SVMSGD> {
		#[inline] fn as_raw_StatModel(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ml::StatModel for core::Ptr<dyn crate::ml::SVMSGD> {
		#[inline] fn as_raw_mut_StatModel(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSVM_Kernel = core::Ptr<dyn crate::ml::SVM_Kernel>;
	
	ptr_extern! { dyn crate::ml::SVM_Kernel,
		cv_PtrOfSVM_Kernel_delete, cv_PtrOfSVM_Kernel_get_inner_ptr, cv_PtrOfSVM_Kernel_get_inner_ptr_mut
	}
	
	impl core::Ptr<dyn crate::ml::SVM_Kernel> {
		#[inline] pub fn as_raw_PtrOfSVM_Kernel(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSVM_Kernel(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::ml::SVM_KernelConst for core::Ptr<dyn crate::ml::SVM_Kernel> {
		#[inline] fn as_raw_SVM_Kernel(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ml::SVM_Kernel for core::Ptr<dyn crate::ml::SVM_Kernel> {
		#[inline] fn as_raw_mut_SVM_Kernel(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<dyn crate::ml::SVM_Kernel> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<dyn crate::ml::SVM_Kernel> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfTrainData = core::Ptr<dyn crate::ml::TrainData>;
	
	ptr_extern! { dyn crate::ml::TrainData,
		cv_PtrOfTrainData_delete, cv_PtrOfTrainData_get_inner_ptr, cv_PtrOfTrainData_get_inner_ptr_mut
	}
	
	impl core::Ptr<dyn crate::ml::TrainData> {
		#[inline] pub fn as_raw_PtrOfTrainData(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfTrainData(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::ml::TrainDataConst for core::Ptr<dyn crate::ml::TrainData> {
		#[inline] fn as_raw_TrainData(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ml::TrainData for core::Ptr<dyn crate::ml::TrainData> {
		#[inline] fn as_raw_mut_TrainData(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type VectorOfDTrees_Node = core::Vector<crate::ml::DTrees_Node>;
	
	impl core::Vector<crate::ml::DTrees_Node> {
		pub fn as_raw_VectorOfDTrees_Node(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfDTrees_Node(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { crate::ml::DTrees_Node,
		cv_VectorOfDTrees_Node_new, cv_VectorOfDTrees_Node_delete,
		cv_VectorOfDTrees_Node_len, cv_VectorOfDTrees_Node_is_empty,
		cv_VectorOfDTrees_Node_capacity, cv_VectorOfDTrees_Node_shrink_to_fit,
		cv_VectorOfDTrees_Node_reserve, cv_VectorOfDTrees_Node_remove,
		cv_VectorOfDTrees_Node_swap, cv_VectorOfDTrees_Node_clear,
		cv_VectorOfDTrees_Node_get, cv_VectorOfDTrees_Node_set,
		cv_VectorOfDTrees_Node_push, cv_VectorOfDTrees_Node_insert,
	}
	vector_non_copy_or_bool! { crate::ml::DTrees_Node }
	
	pub type VectorOfDTrees_Split = core::Vector<crate::ml::DTrees_Split>;
	
	impl core::Vector<crate::ml::DTrees_Split> {
		pub fn as_raw_VectorOfDTrees_Split(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfDTrees_Split(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { crate::ml::DTrees_Split,
		cv_VectorOfDTrees_Split_new, cv_VectorOfDTrees_Split_delete,
		cv_VectorOfDTrees_Split_len, cv_VectorOfDTrees_Split_is_empty,
		cv_VectorOfDTrees_Split_capacity, cv_VectorOfDTrees_Split_shrink_to_fit,
		cv_VectorOfDTrees_Split_reserve, cv_VectorOfDTrees_Split_remove,
		cv_VectorOfDTrees_Split_swap, cv_VectorOfDTrees_Split_clear,
		cv_VectorOfDTrees_Split_get, cv_VectorOfDTrees_Split_set,
		cv_VectorOfDTrees_Split_push, cv_VectorOfDTrees_Split_insert,
	}
	vector_non_copy_or_bool! { crate::ml::DTrees_Split }
	
}
#[cfg(ocvrs_has_module_ml)]
pub use ml_types::*;

#[cfg(ocvrs_has_module_objdetect)]
mod objdetect_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfBaseCascadeClassifier = core::Ptr<dyn crate::objdetect::BaseCascadeClassifier>;
	
	ptr_extern! { dyn crate::objdetect::BaseCascadeClassifier,
		cv_PtrOfBaseCascadeClassifier_delete, cv_PtrOfBaseCascadeClassifier_get_inner_ptr, cv_PtrOfBaseCascadeClassifier_get_inner_ptr_mut
	}
	
	impl core::Ptr<dyn crate::objdetect::BaseCascadeClassifier> {
		#[inline] pub fn as_raw_PtrOfBaseCascadeClassifier(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBaseCascadeClassifier(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::objdetect::BaseCascadeClassifierConst for core::Ptr<dyn crate::objdetect::BaseCascadeClassifier> {
		#[inline] fn as_raw_BaseCascadeClassifier(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::objdetect::BaseCascadeClassifier for core::Ptr<dyn crate::objdetect::BaseCascadeClassifier> {
		#[inline] fn as_raw_mut_BaseCascadeClassifier(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<dyn crate::objdetect::BaseCascadeClassifier> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<dyn crate::objdetect::BaseCascadeClassifier> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfBaseCascadeClassifier_MaskGenerator = core::Ptr<dyn crate::objdetect::BaseCascadeClassifier_MaskGenerator>;
	
	ptr_extern! { dyn crate::objdetect::BaseCascadeClassifier_MaskGenerator,
		cv_PtrOfBaseCascadeClassifier_MaskGenerator_delete, cv_PtrOfBaseCascadeClassifier_MaskGenerator_get_inner_ptr, cv_PtrOfBaseCascadeClassifier_MaskGenerator_get_inner_ptr_mut
	}
	
	impl core::Ptr<dyn crate::objdetect::BaseCascadeClassifier_MaskGenerator> {
		#[inline] pub fn as_raw_PtrOfBaseCascadeClassifier_MaskGenerator(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBaseCascadeClassifier_MaskGenerator(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::objdetect::BaseCascadeClassifier_MaskGeneratorConst for core::Ptr<dyn crate::objdetect::BaseCascadeClassifier_MaskGenerator> {
		#[inline] fn as_raw_BaseCascadeClassifier_MaskGenerator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::objdetect::BaseCascadeClassifier_MaskGenerator for core::Ptr<dyn crate::objdetect::BaseCascadeClassifier_MaskGenerator> {
		#[inline] fn as_raw_mut_BaseCascadeClassifier_MaskGenerator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDetectionBasedTracker_IDetector = core::Ptr<dyn crate::objdetect::DetectionBasedTracker_IDetector>;
	
	ptr_extern! { dyn crate::objdetect::DetectionBasedTracker_IDetector,
		cv_PtrOfDetectionBasedTracker_IDetector_delete, cv_PtrOfDetectionBasedTracker_IDetector_get_inner_ptr, cv_PtrOfDetectionBasedTracker_IDetector_get_inner_ptr_mut
	}
	
	impl core::Ptr<dyn crate::objdetect::DetectionBasedTracker_IDetector> {
		#[inline] pub fn as_raw_PtrOfDetectionBasedTracker_IDetector(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetectionBasedTracker_IDetector(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::objdetect::DetectionBasedTracker_IDetectorConst for core::Ptr<dyn crate::objdetect::DetectionBasedTracker_IDetector> {
		#[inline] fn as_raw_DetectionBasedTracker_IDetector(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::objdetect::DetectionBasedTracker_IDetector for core::Ptr<dyn crate::objdetect::DetectionBasedTracker_IDetector> {
		#[inline] fn as_raw_mut_DetectionBasedTracker_IDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfFaceDetectorYN = core::Ptr<dyn crate::objdetect::FaceDetectorYN>;
	
	ptr_extern! { dyn crate::objdetect::FaceDetectorYN,
		cv_PtrOfFaceDetectorYN_delete, cv_PtrOfFaceDetectorYN_get_inner_ptr, cv_PtrOfFaceDetectorYN_get_inner_ptr_mut
	}
	
	impl core::Ptr<dyn crate::objdetect::FaceDetectorYN> {
		#[inline] pub fn as_raw_PtrOfFaceDetectorYN(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFaceDetectorYN(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::objdetect::FaceDetectorYNConst for core::Ptr<dyn crate::objdetect::FaceDetectorYN> {
		#[inline] fn as_raw_FaceDetectorYN(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::objdetect::FaceDetectorYN for core::Ptr<dyn crate::objdetect::FaceDetectorYN> {
		#[inline] fn as_raw_mut_FaceDetectorYN(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfFaceRecognizerSF = core::Ptr<dyn crate::objdetect::FaceRecognizerSF>;
	
	ptr_extern! { dyn crate::objdetect::FaceRecognizerSF,
		cv_PtrOfFaceRecognizerSF_delete, cv_PtrOfFaceRecognizerSF_get_inner_ptr, cv_PtrOfFaceRecognizerSF_get_inner_ptr_mut
	}
	
	impl core::Ptr<dyn crate::objdetect::FaceRecognizerSF> {
		#[inline] pub fn as_raw_PtrOfFaceRecognizerSF(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFaceRecognizerSF(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::objdetect::FaceRecognizerSFConst for core::Ptr<dyn crate::objdetect::FaceRecognizerSF> {
		#[inline] fn as_raw_FaceRecognizerSF(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::objdetect::FaceRecognizerSF for core::Ptr<dyn crate::objdetect::FaceRecognizerSF> {
		#[inline] fn as_raw_mut_FaceRecognizerSF(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfQRCodeEncoder = core::Ptr<dyn crate::objdetect::QRCodeEncoder>;
	
	ptr_extern! { dyn crate::objdetect::QRCodeEncoder,
		cv_PtrOfQRCodeEncoder_delete, cv_PtrOfQRCodeEncoder_get_inner_ptr, cv_PtrOfQRCodeEncoder_get_inner_ptr_mut
	}
	
	impl core::Ptr<dyn crate::objdetect::QRCodeEncoder> {
		#[inline] pub fn as_raw_PtrOfQRCodeEncoder(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfQRCodeEncoder(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::objdetect::QRCodeEncoderConst for core::Ptr<dyn crate::objdetect::QRCodeEncoder> {
		#[inline] fn as_raw_QRCodeEncoder(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::objdetect::QRCodeEncoder for core::Ptr<dyn crate::objdetect::QRCodeEncoder> {
		#[inline] fn as_raw_mut_QRCodeEncoder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type VectorOfDetectionBasedTracker_ExtObject = core::Vector<crate::objdetect::DetectionBasedTracker_ExtObject>;
	
	impl core::Vector<crate::objdetect::DetectionBasedTracker_ExtObject> {
		pub fn as_raw_VectorOfDetectionBasedTracker_ExtObject(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfDetectionBasedTracker_ExtObject(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { crate::objdetect::DetectionBasedTracker_ExtObject,
		cv_VectorOfDetectionBasedTracker_ExtObject_new, cv_VectorOfDetectionBasedTracker_ExtObject_delete,
		cv_VectorOfDetectionBasedTracker_ExtObject_len, cv_VectorOfDetectionBasedTracker_ExtObject_is_empty,
		cv_VectorOfDetectionBasedTracker_ExtObject_capacity, cv_VectorOfDetectionBasedTracker_ExtObject_shrink_to_fit,
		cv_VectorOfDetectionBasedTracker_ExtObject_reserve, cv_VectorOfDetectionBasedTracker_ExtObject_remove,
		cv_VectorOfDetectionBasedTracker_ExtObject_swap, cv_VectorOfDetectionBasedTracker_ExtObject_clear,
		cv_VectorOfDetectionBasedTracker_ExtObject_get, cv_VectorOfDetectionBasedTracker_ExtObject_set,
		cv_VectorOfDetectionBasedTracker_ExtObject_push, cv_VectorOfDetectionBasedTracker_ExtObject_insert,
	}
	vector_non_copy_or_bool! { crate::objdetect::DetectionBasedTracker_ExtObject }
	
	pub type VectorOfDetectionBasedTracker_Object = core::Vector<crate::objdetect::DetectionBasedTracker_Object>;
	
	impl core::Vector<crate::objdetect::DetectionBasedTracker_Object> {
		pub fn as_raw_VectorOfDetectionBasedTracker_Object(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfDetectionBasedTracker_Object(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { crate::objdetect::DetectionBasedTracker_Object,
		cv_VectorOfDetectionBasedTracker_Object_new, cv_VectorOfDetectionBasedTracker_Object_delete,
		cv_VectorOfDetectionBasedTracker_Object_len, cv_VectorOfDetectionBasedTracker_Object_is_empty,
		cv_VectorOfDetectionBasedTracker_Object_capacity, cv_VectorOfDetectionBasedTracker_Object_shrink_to_fit,
		cv_VectorOfDetectionBasedTracker_Object_reserve, cv_VectorOfDetectionBasedTracker_Object_remove,
		cv_VectorOfDetectionBasedTracker_Object_swap, cv_VectorOfDetectionBasedTracker_Object_clear,
		cv_VectorOfDetectionBasedTracker_Object_get, cv_VectorOfDetectionBasedTracker_Object_set,
		cv_VectorOfDetectionBasedTracker_Object_push, cv_VectorOfDetectionBasedTracker_Object_insert,
	}
	vector_non_copy_or_bool! { crate::objdetect::DetectionBasedTracker_Object }
	
	pub type VectorOfDetectionROI = core::Vector<crate::objdetect::DetectionROI>;
	
	impl core::Vector<crate::objdetect::DetectionROI> {
		pub fn as_raw_VectorOfDetectionROI(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfDetectionROI(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { crate::objdetect::DetectionROI,
		cv_VectorOfDetectionROI_new, cv_VectorOfDetectionROI_delete,
		cv_VectorOfDetectionROI_len, cv_VectorOfDetectionROI_is_empty,
		cv_VectorOfDetectionROI_capacity, cv_VectorOfDetectionROI_shrink_to_fit,
		cv_VectorOfDetectionROI_reserve, cv_VectorOfDetectionROI_remove,
		cv_VectorOfDetectionROI_swap, cv_VectorOfDetectionROI_clear,
		cv_VectorOfDetectionROI_get, cv_VectorOfDetectionROI_set,
		cv_VectorOfDetectionROI_push, cv_VectorOfDetectionROI_insert,
	}
	vector_non_copy_or_bool! { crate::objdetect::DetectionROI }
	
}
#[cfg(ocvrs_has_module_objdetect)]
pub use objdetect_types::*;

#[cfg(ocvrs_has_module_photo)]
mod photo_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfAlignMTB = core::Ptr<dyn crate::photo::AlignMTB>;
	
	ptr_extern! { dyn crate::photo::AlignMTB,
		cv_PtrOfAlignMTB_delete, cv_PtrOfAlignMTB_get_inner_ptr, cv_PtrOfAlignMTB_get_inner_ptr_mut
	}
	
	impl core::Ptr<dyn crate::photo::AlignMTB> {
		#[inline] pub fn as_raw_PtrOfAlignMTB(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfAlignMTB(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::photo::AlignMTBConst for core::Ptr<dyn crate::photo::AlignMTB> {
		#[inline] fn as_raw_AlignMTB(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::photo::AlignMTB for core::Ptr<dyn crate::photo::AlignMTB> {
		#[inline] fn as_raw_mut_AlignMTB(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<dyn crate::photo::AlignMTB> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<dyn crate::photo::AlignMTB> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::photo::AlignExposuresConst for core::Ptr<dyn crate::photo::AlignMTB> {
		#[inline] fn as_raw_AlignExposures(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::photo::AlignExposures for core::Ptr<dyn crate::photo::AlignMTB> {
		#[inline] fn as_raw_mut_AlignExposures(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCalibrateDebevec = core::Ptr<dyn crate::photo::CalibrateDebevec>;
	
	ptr_extern! { dyn crate::photo::CalibrateDebevec,
		cv_PtrOfCalibrateDebevec_delete, cv_PtrOfCalibrateDebevec_get_inner_ptr, cv_PtrOfCalibrateDebevec_get_inner_ptr_mut
	}
	
	impl core::Ptr<dyn crate::photo::CalibrateDebevec> {
		#[inline] pub fn as_raw_PtrOfCalibrateDebevec(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCalibrateDebevec(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::photo::CalibrateDebevecConst for core::Ptr<dyn crate::photo::CalibrateDebevec> {
		#[inline] fn as_raw_CalibrateDebevec(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::photo::CalibrateDebevec for core::Ptr<dyn crate::photo::CalibrateDebevec> {
		#[inline] fn as_raw_mut_CalibrateDebevec(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<dyn crate::photo::CalibrateDebevec> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<dyn crate::photo::CalibrateDebevec> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::photo::CalibrateCRFConst for core::Ptr<dyn crate::photo::CalibrateDebevec> {
		#[inline] fn as_raw_CalibrateCRF(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::photo::CalibrateCRF for core::Ptr<dyn crate::photo::CalibrateDebevec> {
		#[inline] fn as_raw_mut_CalibrateCRF(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCalibrateRobertson = core::Ptr<dyn crate::photo::CalibrateRobertson>;
	
	ptr_extern! { dyn crate::photo::CalibrateRobertson,
		cv_PtrOfCalibrateRobertson_delete, cv_PtrOfCalibrateRobertson_get_inner_ptr, cv_PtrOfCalibrateRobertson_get_inner_ptr_mut
	}
	
	impl core::Ptr<dyn crate::photo::CalibrateRobertson> {
		#[inline] pub fn as_raw_PtrOfCalibrateRobertson(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCalibrateRobertson(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::photo::CalibrateRobertsonConst for core::Ptr<dyn crate::photo::CalibrateRobertson> {
		#[inline] fn as_raw_CalibrateRobertson(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::photo::CalibrateRobertson for core::Ptr<dyn crate::photo::CalibrateRobertson> {
		#[inline] fn as_raw_mut_CalibrateRobertson(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<dyn crate::photo::CalibrateRobertson> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<dyn crate::photo::CalibrateRobertson> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::photo::CalibrateCRFConst for core::Ptr<dyn crate::photo::CalibrateRobertson> {
		#[inline] fn as_raw_CalibrateCRF(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::photo::CalibrateCRF for core::Ptr<dyn crate::photo::CalibrateRobertson> {
		#[inline] fn as_raw_mut_CalibrateCRF(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfMergeDebevec = core::Ptr<dyn crate::photo::MergeDebevec>;
	
	ptr_extern! { dyn crate::photo::MergeDebevec,
		cv_PtrOfMergeDebevec_delete, cv_PtrOfMergeDebevec_get_inner_ptr, cv_PtrOfMergeDebevec_get_inner_ptr_mut
	}
	
	impl core::Ptr<dyn crate::photo::MergeDebevec> {
		#[inline] pub fn as_raw_PtrOfMergeDebevec(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMergeDebevec(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::photo::MergeDebevecConst for core::Ptr<dyn crate::photo::MergeDebevec> {
		#[inline] fn as_raw_MergeDebevec(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::photo::MergeDebevec for core::Ptr<dyn crate::photo::MergeDebevec> {
		#[inline] fn as_raw_mut_MergeDebevec(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<dyn crate::photo::MergeDebevec> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<dyn crate::photo::MergeDebevec> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::photo::MergeExposuresConst for core::Ptr<dyn crate::photo::MergeDebevec> {
		#[inline] fn as_raw_MergeExposures(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::photo::MergeExposures for core::Ptr<dyn crate::photo::MergeDebevec> {
		#[inline] fn as_raw_mut_MergeExposures(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfMergeMertens = core::Ptr<dyn crate::photo::MergeMertens>;
	
	ptr_extern! { dyn crate::photo::MergeMertens,
		cv_PtrOfMergeMertens_delete, cv_PtrOfMergeMertens_get_inner_ptr, cv_PtrOfMergeMertens_get_inner_ptr_mut
	}
	
	impl core::Ptr<dyn crate::photo::MergeMertens> {
		#[inline] pub fn as_raw_PtrOfMergeMertens(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMergeMertens(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::photo::MergeMertensConst for core::Ptr<dyn crate::photo::MergeMertens> {
		#[inline] fn as_raw_MergeMertens(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::photo::MergeMertens for core::Ptr<dyn crate::photo::MergeMertens> {
		#[inline] fn as_raw_mut_MergeMertens(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<dyn crate::photo::MergeMertens> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<dyn crate::photo::MergeMertens> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::photo::MergeExposuresConst for core::Ptr<dyn crate::photo::MergeMertens> {
		#[inline] fn as_raw_MergeExposures(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::photo::MergeExposures for core::Ptr<dyn crate::photo::MergeMertens> {
		#[inline] fn as_raw_mut_MergeExposures(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfMergeRobertson = core::Ptr<dyn crate::photo::MergeRobertson>;
	
	ptr_extern! { dyn crate::photo::MergeRobertson,
		cv_PtrOfMergeRobertson_delete, cv_PtrOfMergeRobertson_get_inner_ptr, cv_PtrOfMergeRobertson_get_inner_ptr_mut
	}
	
	impl core::Ptr<dyn crate::photo::MergeRobertson> {
		#[inline] pub fn as_raw_PtrOfMergeRobertson(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMergeRobertson(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::photo::MergeRobertsonConst for core::Ptr<dyn crate::photo::MergeRobertson> {
		#[inline] fn as_raw_MergeRobertson(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::photo::MergeRobertson for core::Ptr<dyn crate::photo::MergeRobertson> {
		#[inline] fn as_raw_mut_MergeRobertson(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<dyn crate::photo::MergeRobertson> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<dyn crate::photo::MergeRobertson> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::photo::MergeExposuresConst for core::Ptr<dyn crate::photo::MergeRobertson> {
		#[inline] fn as_raw_MergeExposures(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::photo::MergeExposures for core::Ptr<dyn crate::photo::MergeRobertson> {
		#[inline] fn as_raw_mut_MergeExposures(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfTonemap = core::Ptr<dyn crate::photo::Tonemap>;
	
	ptr_extern! { dyn crate::photo::Tonemap,
		cv_PtrOfTonemap_delete, cv_PtrOfTonemap_get_inner_ptr, cv_PtrOfTonemap_get_inner_ptr_mut
	}
	
	impl core::Ptr<dyn crate::photo::Tonemap> {
		#[inline] pub fn as_raw_PtrOfTonemap(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfTonemap(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::photo::TonemapConst for core::Ptr<dyn crate::photo::Tonemap> {
		#[inline] fn as_raw_Tonemap(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::photo::Tonemap for core::Ptr<dyn crate::photo::Tonemap> {
		#[inline] fn as_raw_mut_Tonemap(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<dyn crate::photo::Tonemap> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<dyn crate::photo::Tonemap> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfTonemapDrago = core::Ptr<dyn crate::photo::TonemapDrago>;
	
	ptr_extern! { dyn crate::photo::TonemapDrago,
		cv_PtrOfTonemapDrago_delete, cv_PtrOfTonemapDrago_get_inner_ptr, cv_PtrOfTonemapDrago_get_inner_ptr_mut
	}
	
	impl core::Ptr<dyn crate::photo::TonemapDrago> {
		#[inline] pub fn as_raw_PtrOfTonemapDrago(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfTonemapDrago(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::photo::TonemapDragoConst for core::Ptr<dyn crate::photo::TonemapDrago> {
		#[inline] fn as_raw_TonemapDrago(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::photo::TonemapDrago for core::Ptr<dyn crate::photo::TonemapDrago> {
		#[inline] fn as_raw_mut_TonemapDrago(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<dyn crate::photo::TonemapDrago> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<dyn crate::photo::TonemapDrago> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::photo::TonemapConst for core::Ptr<dyn crate::photo::TonemapDrago> {
		#[inline] fn as_raw_Tonemap(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::photo::Tonemap for core::Ptr<dyn crate::photo::TonemapDrago> {
		#[inline] fn as_raw_mut_Tonemap(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfTonemapMantiuk = core::Ptr<dyn crate::photo::TonemapMantiuk>;
	
	ptr_extern! { dyn crate::photo::TonemapMantiuk,
		cv_PtrOfTonemapMantiuk_delete, cv_PtrOfTonemapMantiuk_get_inner_ptr, cv_PtrOfTonemapMantiuk_get_inner_ptr_mut
	}
	
	impl core::Ptr<dyn crate::photo::TonemapMantiuk> {
		#[inline] pub fn as_raw_PtrOfTonemapMantiuk(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfTonemapMantiuk(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::photo::TonemapMantiukConst for core::Ptr<dyn crate::photo::TonemapMantiuk> {
		#[inline] fn as_raw_TonemapMantiuk(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::photo::TonemapMantiuk for core::Ptr<dyn crate::photo::TonemapMantiuk> {
		#[inline] fn as_raw_mut_TonemapMantiuk(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<dyn crate::photo::TonemapMantiuk> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<dyn crate::photo::TonemapMantiuk> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::photo::TonemapConst for core::Ptr<dyn crate::photo::TonemapMantiuk> {
		#[inline] fn as_raw_Tonemap(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::photo::Tonemap for core::Ptr<dyn crate::photo::TonemapMantiuk> {
		#[inline] fn as_raw_mut_Tonemap(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfTonemapReinhard = core::Ptr<dyn crate::photo::TonemapReinhard>;
	
	ptr_extern! { dyn crate::photo::TonemapReinhard,
		cv_PtrOfTonemapReinhard_delete, cv_PtrOfTonemapReinhard_get_inner_ptr, cv_PtrOfTonemapReinhard_get_inner_ptr_mut
	}
	
	impl core::Ptr<dyn crate::photo::TonemapReinhard> {
		#[inline] pub fn as_raw_PtrOfTonemapReinhard(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfTonemapReinhard(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::photo::TonemapReinhardConst for core::Ptr<dyn crate::photo::TonemapReinhard> {
		#[inline] fn as_raw_TonemapReinhard(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::photo::TonemapReinhard for core::Ptr<dyn crate::photo::TonemapReinhard> {
		#[inline] fn as_raw_mut_TonemapReinhard(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<dyn crate::photo::TonemapReinhard> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<dyn crate::photo::TonemapReinhard> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::photo::TonemapConst for core::Ptr<dyn crate::photo::TonemapReinhard> {
		#[inline] fn as_raw_Tonemap(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::photo::Tonemap for core::Ptr<dyn crate::photo::TonemapReinhard> {
		#[inline] fn as_raw_mut_Tonemap(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(ocvrs_has_module_photo)]
pub use photo_types::*;

#[cfg(ocvrs_has_module_stitching)]
mod stitching_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfAffineWarper = core::Ptr<crate::stitching::AffineWarper>;
	
	ptr_extern! { crate::stitching::AffineWarper,
		cv_PtrOfAffineWarper_delete, cv_PtrOfAffineWarper_get_inner_ptr, cv_PtrOfAffineWarper_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::AffineWarper, cv_PtrOfAffineWarper_new }
	
	impl core::Ptr<crate::stitching::AffineWarper> {
		#[inline] pub fn as_raw_PtrOfAffineWarper(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfAffineWarper(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::AffineWarperTraitConst for core::Ptr<crate::stitching::AffineWarper> {
		#[inline] fn as_raw_AffineWarper(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::AffineWarperTrait for core::Ptr<crate::stitching::AffineWarper> {
		#[inline] fn as_raw_mut_AffineWarper(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::WarperCreatorConst for core::Ptr<crate::stitching::AffineWarper> {
		#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::WarperCreator for core::Ptr<crate::stitching::AffineWarper> {
		#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfAffineWarper, core::Ptr<dyn crate::stitching::WarperCreator>, cv_PtrOfAffineWarper_to_PtrOfWarperCreator }
	
	pub type PtrOfCompressedRectilinearPortraitWarper = core::Ptr<crate::stitching::CompressedRectilinearPortraitWarper>;
	
	ptr_extern! { crate::stitching::CompressedRectilinearPortraitWarper,
		cv_PtrOfCompressedRectilinearPortraitWarper_delete, cv_PtrOfCompressedRectilinearPortraitWarper_get_inner_ptr, cv_PtrOfCompressedRectilinearPortraitWarper_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::CompressedRectilinearPortraitWarper, cv_PtrOfCompressedRectilinearPortraitWarper_new }
	
	impl core::Ptr<crate::stitching::CompressedRectilinearPortraitWarper> {
		#[inline] pub fn as_raw_PtrOfCompressedRectilinearPortraitWarper(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCompressedRectilinearPortraitWarper(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::CompressedRectilinearPortraitWarperTraitConst for core::Ptr<crate::stitching::CompressedRectilinearPortraitWarper> {
		#[inline] fn as_raw_CompressedRectilinearPortraitWarper(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::CompressedRectilinearPortraitWarperTrait for core::Ptr<crate::stitching::CompressedRectilinearPortraitWarper> {
		#[inline] fn as_raw_mut_CompressedRectilinearPortraitWarper(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::WarperCreatorConst for core::Ptr<crate::stitching::CompressedRectilinearPortraitWarper> {
		#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::WarperCreator for core::Ptr<crate::stitching::CompressedRectilinearPortraitWarper> {
		#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfCompressedRectilinearPortraitWarper, core::Ptr<dyn crate::stitching::WarperCreator>, cv_PtrOfCompressedRectilinearPortraitWarper_to_PtrOfWarperCreator }
	
	pub type PtrOfCompressedRectilinearWarper = core::Ptr<crate::stitching::CompressedRectilinearWarper>;
	
	ptr_extern! { crate::stitching::CompressedRectilinearWarper,
		cv_PtrOfCompressedRectilinearWarper_delete, cv_PtrOfCompressedRectilinearWarper_get_inner_ptr, cv_PtrOfCompressedRectilinearWarper_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::CompressedRectilinearWarper, cv_PtrOfCompressedRectilinearWarper_new }
	
	impl core::Ptr<crate::stitching::CompressedRectilinearWarper> {
		#[inline] pub fn as_raw_PtrOfCompressedRectilinearWarper(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCompressedRectilinearWarper(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::CompressedRectilinearWarperTraitConst for core::Ptr<crate::stitching::CompressedRectilinearWarper> {
		#[inline] fn as_raw_CompressedRectilinearWarper(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::CompressedRectilinearWarperTrait for core::Ptr<crate::stitching::CompressedRectilinearWarper> {
		#[inline] fn as_raw_mut_CompressedRectilinearWarper(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::WarperCreatorConst for core::Ptr<crate::stitching::CompressedRectilinearWarper> {
		#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::WarperCreator for core::Ptr<crate::stitching::CompressedRectilinearWarper> {
		#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfCompressedRectilinearWarper, core::Ptr<dyn crate::stitching::WarperCreator>, cv_PtrOfCompressedRectilinearWarper_to_PtrOfWarperCreator }
	
	pub type PtrOfCylindricalWarper = core::Ptr<crate::stitching::CylindricalWarper>;
	
	ptr_extern! { crate::stitching::CylindricalWarper,
		cv_PtrOfCylindricalWarper_delete, cv_PtrOfCylindricalWarper_get_inner_ptr, cv_PtrOfCylindricalWarper_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::CylindricalWarper, cv_PtrOfCylindricalWarper_new }
	
	impl core::Ptr<crate::stitching::CylindricalWarper> {
		#[inline] pub fn as_raw_PtrOfCylindricalWarper(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCylindricalWarper(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::CylindricalWarperTraitConst for core::Ptr<crate::stitching::CylindricalWarper> {
		#[inline] fn as_raw_CylindricalWarper(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::CylindricalWarperTrait for core::Ptr<crate::stitching::CylindricalWarper> {
		#[inline] fn as_raw_mut_CylindricalWarper(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::WarperCreatorConst for core::Ptr<crate::stitching::CylindricalWarper> {
		#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::WarperCreator for core::Ptr<crate::stitching::CylindricalWarper> {
		#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfCylindricalWarper, core::Ptr<dyn crate::stitching::WarperCreator>, cv_PtrOfCylindricalWarper_to_PtrOfWarperCreator }
	
	pub type PtrOfDetail_AffineBasedEstimator = core::Ptr<crate::stitching::Detail_AffineBasedEstimator>;
	
	ptr_extern! { crate::stitching::Detail_AffineBasedEstimator,
		cv_PtrOfDetail_AffineBasedEstimator_delete, cv_PtrOfDetail_AffineBasedEstimator_get_inner_ptr, cv_PtrOfDetail_AffineBasedEstimator_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::Detail_AffineBasedEstimator, cv_PtrOfDetail_AffineBasedEstimator_new }
	
	impl core::Ptr<crate::stitching::Detail_AffineBasedEstimator> {
		#[inline] pub fn as_raw_PtrOfDetail_AffineBasedEstimator(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_AffineBasedEstimator(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_AffineBasedEstimatorTraitConst for core::Ptr<crate::stitching::Detail_AffineBasedEstimator> {
		#[inline] fn as_raw_Detail_AffineBasedEstimator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_AffineBasedEstimatorTrait for core::Ptr<crate::stitching::Detail_AffineBasedEstimator> {
		#[inline] fn as_raw_mut_Detail_AffineBasedEstimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::Detail_EstimatorConst for core::Ptr<crate::stitching::Detail_AffineBasedEstimator> {
		#[inline] fn as_raw_Detail_Estimator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_Estimator for core::Ptr<crate::stitching::Detail_AffineBasedEstimator> {
		#[inline] fn as_raw_mut_Detail_Estimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_AffineBasedEstimator, core::Ptr<dyn crate::stitching::Detail_Estimator>, cv_PtrOfDetail_AffineBasedEstimator_to_PtrOfDetail_Estimator }
	
	pub type PtrOfDetail_AffineBestOf2NearestMatcher = core::Ptr<crate::stitching::Detail_AffineBestOf2NearestMatcher>;
	
	ptr_extern! { crate::stitching::Detail_AffineBestOf2NearestMatcher,
		cv_PtrOfDetail_AffineBestOf2NearestMatcher_delete, cv_PtrOfDetail_AffineBestOf2NearestMatcher_get_inner_ptr, cv_PtrOfDetail_AffineBestOf2NearestMatcher_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::Detail_AffineBestOf2NearestMatcher, cv_PtrOfDetail_AffineBestOf2NearestMatcher_new }
	
	impl core::Ptr<crate::stitching::Detail_AffineBestOf2NearestMatcher> {
		#[inline] pub fn as_raw_PtrOfDetail_AffineBestOf2NearestMatcher(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_AffineBestOf2NearestMatcher(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_AffineBestOf2NearestMatcherTraitConst for core::Ptr<crate::stitching::Detail_AffineBestOf2NearestMatcher> {
		#[inline] fn as_raw_Detail_AffineBestOf2NearestMatcher(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_AffineBestOf2NearestMatcherTrait for core::Ptr<crate::stitching::Detail_AffineBestOf2NearestMatcher> {
		#[inline] fn as_raw_mut_Detail_AffineBestOf2NearestMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::Detail_BestOf2NearestMatcherTraitConst for core::Ptr<crate::stitching::Detail_AffineBestOf2NearestMatcher> {
		#[inline] fn as_raw_Detail_BestOf2NearestMatcher(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_BestOf2NearestMatcherTrait for core::Ptr<crate::stitching::Detail_AffineBestOf2NearestMatcher> {
		#[inline] fn as_raw_mut_Detail_BestOf2NearestMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_AffineBestOf2NearestMatcher, core::Ptr<crate::stitching::Detail_BestOf2NearestMatcher>, cv_PtrOfDetail_AffineBestOf2NearestMatcher_to_PtrOfDetail_BestOf2NearestMatcher }
	
	impl crate::stitching::Detail_FeaturesMatcherConst for core::Ptr<crate::stitching::Detail_AffineBestOf2NearestMatcher> {
		#[inline] fn as_raw_Detail_FeaturesMatcher(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_FeaturesMatcher for core::Ptr<crate::stitching::Detail_AffineBestOf2NearestMatcher> {
		#[inline] fn as_raw_mut_Detail_FeaturesMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_AffineBestOf2NearestMatcher, core::Ptr<dyn crate::stitching::Detail_FeaturesMatcher>, cv_PtrOfDetail_AffineBestOf2NearestMatcher_to_PtrOfDetail_FeaturesMatcher }
	
	pub type PtrOfDetail_BestOf2NearestMatcher = core::Ptr<crate::stitching::Detail_BestOf2NearestMatcher>;
	
	ptr_extern! { crate::stitching::Detail_BestOf2NearestMatcher,
		cv_PtrOfDetail_BestOf2NearestMatcher_delete, cv_PtrOfDetail_BestOf2NearestMatcher_get_inner_ptr, cv_PtrOfDetail_BestOf2NearestMatcher_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::Detail_BestOf2NearestMatcher, cv_PtrOfDetail_BestOf2NearestMatcher_new }
	
	impl core::Ptr<crate::stitching::Detail_BestOf2NearestMatcher> {
		#[inline] pub fn as_raw_PtrOfDetail_BestOf2NearestMatcher(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_BestOf2NearestMatcher(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_BestOf2NearestMatcherTraitConst for core::Ptr<crate::stitching::Detail_BestOf2NearestMatcher> {
		#[inline] fn as_raw_Detail_BestOf2NearestMatcher(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_BestOf2NearestMatcherTrait for core::Ptr<crate::stitching::Detail_BestOf2NearestMatcher> {
		#[inline] fn as_raw_mut_Detail_BestOf2NearestMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::Detail_FeaturesMatcherConst for core::Ptr<crate::stitching::Detail_BestOf2NearestMatcher> {
		#[inline] fn as_raw_Detail_FeaturesMatcher(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_FeaturesMatcher for core::Ptr<crate::stitching::Detail_BestOf2NearestMatcher> {
		#[inline] fn as_raw_mut_Detail_FeaturesMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_BestOf2NearestMatcher, core::Ptr<dyn crate::stitching::Detail_FeaturesMatcher>, cv_PtrOfDetail_BestOf2NearestMatcher_to_PtrOfDetail_FeaturesMatcher }
	
	pub type PtrOfDetail_BestOf2NearestRangeMatcher = core::Ptr<crate::stitching::Detail_BestOf2NearestRangeMatcher>;
	
	ptr_extern! { crate::stitching::Detail_BestOf2NearestRangeMatcher,
		cv_PtrOfDetail_BestOf2NearestRangeMatcher_delete, cv_PtrOfDetail_BestOf2NearestRangeMatcher_get_inner_ptr, cv_PtrOfDetail_BestOf2NearestRangeMatcher_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::Detail_BestOf2NearestRangeMatcher, cv_PtrOfDetail_BestOf2NearestRangeMatcher_new }
	
	impl core::Ptr<crate::stitching::Detail_BestOf2NearestRangeMatcher> {
		#[inline] pub fn as_raw_PtrOfDetail_BestOf2NearestRangeMatcher(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_BestOf2NearestRangeMatcher(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_BestOf2NearestRangeMatcherTraitConst for core::Ptr<crate::stitching::Detail_BestOf2NearestRangeMatcher> {
		#[inline] fn as_raw_Detail_BestOf2NearestRangeMatcher(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_BestOf2NearestRangeMatcherTrait for core::Ptr<crate::stitching::Detail_BestOf2NearestRangeMatcher> {
		#[inline] fn as_raw_mut_Detail_BestOf2NearestRangeMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::Detail_BestOf2NearestMatcherTraitConst for core::Ptr<crate::stitching::Detail_BestOf2NearestRangeMatcher> {
		#[inline] fn as_raw_Detail_BestOf2NearestMatcher(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_BestOf2NearestMatcherTrait for core::Ptr<crate::stitching::Detail_BestOf2NearestRangeMatcher> {
		#[inline] fn as_raw_mut_Detail_BestOf2NearestMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_BestOf2NearestRangeMatcher, core::Ptr<crate::stitching::Detail_BestOf2NearestMatcher>, cv_PtrOfDetail_BestOf2NearestRangeMatcher_to_PtrOfDetail_BestOf2NearestMatcher }
	
	impl crate::stitching::Detail_FeaturesMatcherConst for core::Ptr<crate::stitching::Detail_BestOf2NearestRangeMatcher> {
		#[inline] fn as_raw_Detail_FeaturesMatcher(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_FeaturesMatcher for core::Ptr<crate::stitching::Detail_BestOf2NearestRangeMatcher> {
		#[inline] fn as_raw_mut_Detail_FeaturesMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_BestOf2NearestRangeMatcher, core::Ptr<dyn crate::stitching::Detail_FeaturesMatcher>, cv_PtrOfDetail_BestOf2NearestRangeMatcher_to_PtrOfDetail_FeaturesMatcher }
	
	pub type PtrOfDetail_Blender = core::Ptr<crate::stitching::Detail_Blender>;
	
	ptr_extern! { crate::stitching::Detail_Blender,
		cv_PtrOfDetail_Blender_delete, cv_PtrOfDetail_Blender_get_inner_ptr, cv_PtrOfDetail_Blender_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::Detail_Blender, cv_PtrOfDetail_Blender_new }
	
	impl core::Ptr<crate::stitching::Detail_Blender> {
		#[inline] pub fn as_raw_PtrOfDetail_Blender(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_Blender(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_BlenderTraitConst for core::Ptr<crate::stitching::Detail_Blender> {
		#[inline] fn as_raw_Detail_Blender(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_BlenderTrait for core::Ptr<crate::stitching::Detail_Blender> {
		#[inline] fn as_raw_mut_Detail_Blender(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDetail_BlocksChannelsCompensator = core::Ptr<crate::stitching::Detail_BlocksChannelsCompensator>;
	
	ptr_extern! { crate::stitching::Detail_BlocksChannelsCompensator,
		cv_PtrOfDetail_BlocksChannelsCompensator_delete, cv_PtrOfDetail_BlocksChannelsCompensator_get_inner_ptr, cv_PtrOfDetail_BlocksChannelsCompensator_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::Detail_BlocksChannelsCompensator, cv_PtrOfDetail_BlocksChannelsCompensator_new }
	
	impl core::Ptr<crate::stitching::Detail_BlocksChannelsCompensator> {
		#[inline] pub fn as_raw_PtrOfDetail_BlocksChannelsCompensator(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_BlocksChannelsCompensator(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_BlocksChannelsCompensatorTraitConst for core::Ptr<crate::stitching::Detail_BlocksChannelsCompensator> {
		#[inline] fn as_raw_Detail_BlocksChannelsCompensator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_BlocksChannelsCompensatorTrait for core::Ptr<crate::stitching::Detail_BlocksChannelsCompensator> {
		#[inline] fn as_raw_mut_Detail_BlocksChannelsCompensator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::Detail_BlocksCompensatorConst for core::Ptr<crate::stitching::Detail_BlocksChannelsCompensator> {
		#[inline] fn as_raw_Detail_BlocksCompensator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_BlocksCompensator for core::Ptr<crate::stitching::Detail_BlocksChannelsCompensator> {
		#[inline] fn as_raw_mut_Detail_BlocksCompensator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_BlocksChannelsCompensator, core::Ptr<dyn crate::stitching::Detail_BlocksCompensator>, cv_PtrOfDetail_BlocksChannelsCompensator_to_PtrOfDetail_BlocksCompensator }
	
	impl crate::stitching::Detail_ExposureCompensatorConst for core::Ptr<crate::stitching::Detail_BlocksChannelsCompensator> {
		#[inline] fn as_raw_Detail_ExposureCompensator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_ExposureCompensator for core::Ptr<crate::stitching::Detail_BlocksChannelsCompensator> {
		#[inline] fn as_raw_mut_Detail_ExposureCompensator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_BlocksChannelsCompensator, core::Ptr<dyn crate::stitching::Detail_ExposureCompensator>, cv_PtrOfDetail_BlocksChannelsCompensator_to_PtrOfDetail_ExposureCompensator }
	
	pub type PtrOfDetail_BlocksCompensator = core::Ptr<dyn crate::stitching::Detail_BlocksCompensator>;
	
	ptr_extern! { dyn crate::stitching::Detail_BlocksCompensator,
		cv_PtrOfDetail_BlocksCompensator_delete, cv_PtrOfDetail_BlocksCompensator_get_inner_ptr, cv_PtrOfDetail_BlocksCompensator_get_inner_ptr_mut
	}
	
	impl core::Ptr<dyn crate::stitching::Detail_BlocksCompensator> {
		#[inline] pub fn as_raw_PtrOfDetail_BlocksCompensator(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_BlocksCompensator(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_BlocksCompensatorConst for core::Ptr<dyn crate::stitching::Detail_BlocksCompensator> {
		#[inline] fn as_raw_Detail_BlocksCompensator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_BlocksCompensator for core::Ptr<dyn crate::stitching::Detail_BlocksCompensator> {
		#[inline] fn as_raw_mut_Detail_BlocksCompensator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::Detail_ExposureCompensatorConst for core::Ptr<dyn crate::stitching::Detail_BlocksCompensator> {
		#[inline] fn as_raw_Detail_ExposureCompensator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_ExposureCompensator for core::Ptr<dyn crate::stitching::Detail_BlocksCompensator> {
		#[inline] fn as_raw_mut_Detail_ExposureCompensator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_BlocksCompensator, core::Ptr<dyn crate::stitching::Detail_ExposureCompensator>, cv_PtrOfDetail_BlocksCompensator_to_PtrOfDetail_ExposureCompensator }
	
	pub type PtrOfDetail_BlocksGainCompensator = core::Ptr<crate::stitching::Detail_BlocksGainCompensator>;
	
	ptr_extern! { crate::stitching::Detail_BlocksGainCompensator,
		cv_PtrOfDetail_BlocksGainCompensator_delete, cv_PtrOfDetail_BlocksGainCompensator_get_inner_ptr, cv_PtrOfDetail_BlocksGainCompensator_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::Detail_BlocksGainCompensator, cv_PtrOfDetail_BlocksGainCompensator_new }
	
	impl core::Ptr<crate::stitching::Detail_BlocksGainCompensator> {
		#[inline] pub fn as_raw_PtrOfDetail_BlocksGainCompensator(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_BlocksGainCompensator(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_BlocksGainCompensatorTraitConst for core::Ptr<crate::stitching::Detail_BlocksGainCompensator> {
		#[inline] fn as_raw_Detail_BlocksGainCompensator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_BlocksGainCompensatorTrait for core::Ptr<crate::stitching::Detail_BlocksGainCompensator> {
		#[inline] fn as_raw_mut_Detail_BlocksGainCompensator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::Detail_BlocksCompensatorConst for core::Ptr<crate::stitching::Detail_BlocksGainCompensator> {
		#[inline] fn as_raw_Detail_BlocksCompensator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_BlocksCompensator for core::Ptr<crate::stitching::Detail_BlocksGainCompensator> {
		#[inline] fn as_raw_mut_Detail_BlocksCompensator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_BlocksGainCompensator, core::Ptr<dyn crate::stitching::Detail_BlocksCompensator>, cv_PtrOfDetail_BlocksGainCompensator_to_PtrOfDetail_BlocksCompensator }
	
	impl crate::stitching::Detail_ExposureCompensatorConst for core::Ptr<crate::stitching::Detail_BlocksGainCompensator> {
		#[inline] fn as_raw_Detail_ExposureCompensator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_ExposureCompensator for core::Ptr<crate::stitching::Detail_BlocksGainCompensator> {
		#[inline] fn as_raw_mut_Detail_ExposureCompensator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_BlocksGainCompensator, core::Ptr<dyn crate::stitching::Detail_ExposureCompensator>, cv_PtrOfDetail_BlocksGainCompensator_to_PtrOfDetail_ExposureCompensator }
	
	pub type PtrOfDetail_BundleAdjusterAffine = core::Ptr<crate::stitching::Detail_BundleAdjusterAffine>;
	
	ptr_extern! { crate::stitching::Detail_BundleAdjusterAffine,
		cv_PtrOfDetail_BundleAdjusterAffine_delete, cv_PtrOfDetail_BundleAdjusterAffine_get_inner_ptr, cv_PtrOfDetail_BundleAdjusterAffine_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::Detail_BundleAdjusterAffine, cv_PtrOfDetail_BundleAdjusterAffine_new }
	
	impl core::Ptr<crate::stitching::Detail_BundleAdjusterAffine> {
		#[inline] pub fn as_raw_PtrOfDetail_BundleAdjusterAffine(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_BundleAdjusterAffine(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_BundleAdjusterAffineTraitConst for core::Ptr<crate::stitching::Detail_BundleAdjusterAffine> {
		#[inline] fn as_raw_Detail_BundleAdjusterAffine(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_BundleAdjusterAffineTrait for core::Ptr<crate::stitching::Detail_BundleAdjusterAffine> {
		#[inline] fn as_raw_mut_Detail_BundleAdjusterAffine(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::Detail_BundleAdjusterBaseConst for core::Ptr<crate::stitching::Detail_BundleAdjusterAffine> {
		#[inline] fn as_raw_Detail_BundleAdjusterBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_BundleAdjusterBase for core::Ptr<crate::stitching::Detail_BundleAdjusterAffine> {
		#[inline] fn as_raw_mut_Detail_BundleAdjusterBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_BundleAdjusterAffine, core::Ptr<dyn crate::stitching::Detail_BundleAdjusterBase>, cv_PtrOfDetail_BundleAdjusterAffine_to_PtrOfDetail_BundleAdjusterBase }
	
	impl crate::stitching::Detail_EstimatorConst for core::Ptr<crate::stitching::Detail_BundleAdjusterAffine> {
		#[inline] fn as_raw_Detail_Estimator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_Estimator for core::Ptr<crate::stitching::Detail_BundleAdjusterAffine> {
		#[inline] fn as_raw_mut_Detail_Estimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_BundleAdjusterAffine, core::Ptr<dyn crate::stitching::Detail_Estimator>, cv_PtrOfDetail_BundleAdjusterAffine_to_PtrOfDetail_Estimator }
	
	pub type PtrOfDetail_BundleAdjusterAffinePartial = core::Ptr<crate::stitching::Detail_BundleAdjusterAffinePartial>;
	
	ptr_extern! { crate::stitching::Detail_BundleAdjusterAffinePartial,
		cv_PtrOfDetail_BundleAdjusterAffinePartial_delete, cv_PtrOfDetail_BundleAdjusterAffinePartial_get_inner_ptr, cv_PtrOfDetail_BundleAdjusterAffinePartial_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::Detail_BundleAdjusterAffinePartial, cv_PtrOfDetail_BundleAdjusterAffinePartial_new }
	
	impl core::Ptr<crate::stitching::Detail_BundleAdjusterAffinePartial> {
		#[inline] pub fn as_raw_PtrOfDetail_BundleAdjusterAffinePartial(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_BundleAdjusterAffinePartial(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_BundleAdjusterAffinePartialTraitConst for core::Ptr<crate::stitching::Detail_BundleAdjusterAffinePartial> {
		#[inline] fn as_raw_Detail_BundleAdjusterAffinePartial(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_BundleAdjusterAffinePartialTrait for core::Ptr<crate::stitching::Detail_BundleAdjusterAffinePartial> {
		#[inline] fn as_raw_mut_Detail_BundleAdjusterAffinePartial(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::Detail_BundleAdjusterBaseConst for core::Ptr<crate::stitching::Detail_BundleAdjusterAffinePartial> {
		#[inline] fn as_raw_Detail_BundleAdjusterBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_BundleAdjusterBase for core::Ptr<crate::stitching::Detail_BundleAdjusterAffinePartial> {
		#[inline] fn as_raw_mut_Detail_BundleAdjusterBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_BundleAdjusterAffinePartial, core::Ptr<dyn crate::stitching::Detail_BundleAdjusterBase>, cv_PtrOfDetail_BundleAdjusterAffinePartial_to_PtrOfDetail_BundleAdjusterBase }
	
	impl crate::stitching::Detail_EstimatorConst for core::Ptr<crate::stitching::Detail_BundleAdjusterAffinePartial> {
		#[inline] fn as_raw_Detail_Estimator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_Estimator for core::Ptr<crate::stitching::Detail_BundleAdjusterAffinePartial> {
		#[inline] fn as_raw_mut_Detail_Estimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_BundleAdjusterAffinePartial, core::Ptr<dyn crate::stitching::Detail_Estimator>, cv_PtrOfDetail_BundleAdjusterAffinePartial_to_PtrOfDetail_Estimator }
	
	pub type PtrOfDetail_BundleAdjusterBase = core::Ptr<dyn crate::stitching::Detail_BundleAdjusterBase>;
	
	ptr_extern! { dyn crate::stitching::Detail_BundleAdjusterBase,
		cv_PtrOfDetail_BundleAdjusterBase_delete, cv_PtrOfDetail_BundleAdjusterBase_get_inner_ptr, cv_PtrOfDetail_BundleAdjusterBase_get_inner_ptr_mut
	}
	
	impl core::Ptr<dyn crate::stitching::Detail_BundleAdjusterBase> {
		#[inline] pub fn as_raw_PtrOfDetail_BundleAdjusterBase(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_BundleAdjusterBase(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_BundleAdjusterBaseConst for core::Ptr<dyn crate::stitching::Detail_BundleAdjusterBase> {
		#[inline] fn as_raw_Detail_BundleAdjusterBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_BundleAdjusterBase for core::Ptr<dyn crate::stitching::Detail_BundleAdjusterBase> {
		#[inline] fn as_raw_mut_Detail_BundleAdjusterBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::Detail_EstimatorConst for core::Ptr<dyn crate::stitching::Detail_BundleAdjusterBase> {
		#[inline] fn as_raw_Detail_Estimator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_Estimator for core::Ptr<dyn crate::stitching::Detail_BundleAdjusterBase> {
		#[inline] fn as_raw_mut_Detail_Estimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_BundleAdjusterBase, core::Ptr<dyn crate::stitching::Detail_Estimator>, cv_PtrOfDetail_BundleAdjusterBase_to_PtrOfDetail_Estimator }
	
	pub type PtrOfDetail_BundleAdjusterRay = core::Ptr<crate::stitching::Detail_BundleAdjusterRay>;
	
	ptr_extern! { crate::stitching::Detail_BundleAdjusterRay,
		cv_PtrOfDetail_BundleAdjusterRay_delete, cv_PtrOfDetail_BundleAdjusterRay_get_inner_ptr, cv_PtrOfDetail_BundleAdjusterRay_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::Detail_BundleAdjusterRay, cv_PtrOfDetail_BundleAdjusterRay_new }
	
	impl core::Ptr<crate::stitching::Detail_BundleAdjusterRay> {
		#[inline] pub fn as_raw_PtrOfDetail_BundleAdjusterRay(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_BundleAdjusterRay(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_BundleAdjusterRayTraitConst for core::Ptr<crate::stitching::Detail_BundleAdjusterRay> {
		#[inline] fn as_raw_Detail_BundleAdjusterRay(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_BundleAdjusterRayTrait for core::Ptr<crate::stitching::Detail_BundleAdjusterRay> {
		#[inline] fn as_raw_mut_Detail_BundleAdjusterRay(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::Detail_BundleAdjusterBaseConst for core::Ptr<crate::stitching::Detail_BundleAdjusterRay> {
		#[inline] fn as_raw_Detail_BundleAdjusterBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_BundleAdjusterBase for core::Ptr<crate::stitching::Detail_BundleAdjusterRay> {
		#[inline] fn as_raw_mut_Detail_BundleAdjusterBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_BundleAdjusterRay, core::Ptr<dyn crate::stitching::Detail_BundleAdjusterBase>, cv_PtrOfDetail_BundleAdjusterRay_to_PtrOfDetail_BundleAdjusterBase }
	
	impl crate::stitching::Detail_EstimatorConst for core::Ptr<crate::stitching::Detail_BundleAdjusterRay> {
		#[inline] fn as_raw_Detail_Estimator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_Estimator for core::Ptr<crate::stitching::Detail_BundleAdjusterRay> {
		#[inline] fn as_raw_mut_Detail_Estimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_BundleAdjusterRay, core::Ptr<dyn crate::stitching::Detail_Estimator>, cv_PtrOfDetail_BundleAdjusterRay_to_PtrOfDetail_Estimator }
	
	pub type PtrOfDetail_BundleAdjusterReproj = core::Ptr<crate::stitching::Detail_BundleAdjusterReproj>;
	
	ptr_extern! { crate::stitching::Detail_BundleAdjusterReproj,
		cv_PtrOfDetail_BundleAdjusterReproj_delete, cv_PtrOfDetail_BundleAdjusterReproj_get_inner_ptr, cv_PtrOfDetail_BundleAdjusterReproj_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::Detail_BundleAdjusterReproj, cv_PtrOfDetail_BundleAdjusterReproj_new }
	
	impl core::Ptr<crate::stitching::Detail_BundleAdjusterReproj> {
		#[inline] pub fn as_raw_PtrOfDetail_BundleAdjusterReproj(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_BundleAdjusterReproj(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_BundleAdjusterReprojTraitConst for core::Ptr<crate::stitching::Detail_BundleAdjusterReproj> {
		#[inline] fn as_raw_Detail_BundleAdjusterReproj(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_BundleAdjusterReprojTrait for core::Ptr<crate::stitching::Detail_BundleAdjusterReproj> {
		#[inline] fn as_raw_mut_Detail_BundleAdjusterReproj(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::Detail_BundleAdjusterBaseConst for core::Ptr<crate::stitching::Detail_BundleAdjusterReproj> {
		#[inline] fn as_raw_Detail_BundleAdjusterBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_BundleAdjusterBase for core::Ptr<crate::stitching::Detail_BundleAdjusterReproj> {
		#[inline] fn as_raw_mut_Detail_BundleAdjusterBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_BundleAdjusterReproj, core::Ptr<dyn crate::stitching::Detail_BundleAdjusterBase>, cv_PtrOfDetail_BundleAdjusterReproj_to_PtrOfDetail_BundleAdjusterBase }
	
	impl crate::stitching::Detail_EstimatorConst for core::Ptr<crate::stitching::Detail_BundleAdjusterReproj> {
		#[inline] fn as_raw_Detail_Estimator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_Estimator for core::Ptr<crate::stitching::Detail_BundleAdjusterReproj> {
		#[inline] fn as_raw_mut_Detail_Estimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_BundleAdjusterReproj, core::Ptr<dyn crate::stitching::Detail_Estimator>, cv_PtrOfDetail_BundleAdjusterReproj_to_PtrOfDetail_Estimator }
	
	pub type PtrOfDetail_ChannelsCompensator = core::Ptr<crate::stitching::Detail_ChannelsCompensator>;
	
	ptr_extern! { crate::stitching::Detail_ChannelsCompensator,
		cv_PtrOfDetail_ChannelsCompensator_delete, cv_PtrOfDetail_ChannelsCompensator_get_inner_ptr, cv_PtrOfDetail_ChannelsCompensator_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::Detail_ChannelsCompensator, cv_PtrOfDetail_ChannelsCompensator_new }
	
	impl core::Ptr<crate::stitching::Detail_ChannelsCompensator> {
		#[inline] pub fn as_raw_PtrOfDetail_ChannelsCompensator(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_ChannelsCompensator(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_ChannelsCompensatorTraitConst for core::Ptr<crate::stitching::Detail_ChannelsCompensator> {
		#[inline] fn as_raw_Detail_ChannelsCompensator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_ChannelsCompensatorTrait for core::Ptr<crate::stitching::Detail_ChannelsCompensator> {
		#[inline] fn as_raw_mut_Detail_ChannelsCompensator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::Detail_ExposureCompensatorConst for core::Ptr<crate::stitching::Detail_ChannelsCompensator> {
		#[inline] fn as_raw_Detail_ExposureCompensator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_ExposureCompensator for core::Ptr<crate::stitching::Detail_ChannelsCompensator> {
		#[inline] fn as_raw_mut_Detail_ExposureCompensator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_ChannelsCompensator, core::Ptr<dyn crate::stitching::Detail_ExposureCompensator>, cv_PtrOfDetail_ChannelsCompensator_to_PtrOfDetail_ExposureCompensator }
	
	pub type PtrOfDetail_DpSeamFinder = core::Ptr<crate::stitching::Detail_DpSeamFinder>;
	
	ptr_extern! { crate::stitching::Detail_DpSeamFinder,
		cv_PtrOfDetail_DpSeamFinder_delete, cv_PtrOfDetail_DpSeamFinder_get_inner_ptr, cv_PtrOfDetail_DpSeamFinder_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::Detail_DpSeamFinder, cv_PtrOfDetail_DpSeamFinder_new }
	
	impl core::Ptr<crate::stitching::Detail_DpSeamFinder> {
		#[inline] pub fn as_raw_PtrOfDetail_DpSeamFinder(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_DpSeamFinder(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_DpSeamFinderTraitConst for core::Ptr<crate::stitching::Detail_DpSeamFinder> {
		#[inline] fn as_raw_Detail_DpSeamFinder(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_DpSeamFinderTrait for core::Ptr<crate::stitching::Detail_DpSeamFinder> {
		#[inline] fn as_raw_mut_Detail_DpSeamFinder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::Detail_SeamFinderConst for core::Ptr<crate::stitching::Detail_DpSeamFinder> {
		#[inline] fn as_raw_Detail_SeamFinder(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_SeamFinder for core::Ptr<crate::stitching::Detail_DpSeamFinder> {
		#[inline] fn as_raw_mut_Detail_SeamFinder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_DpSeamFinder, core::Ptr<dyn crate::stitching::Detail_SeamFinder>, cv_PtrOfDetail_DpSeamFinder_to_PtrOfDetail_SeamFinder }
	
	pub type PtrOfDetail_Estimator = core::Ptr<dyn crate::stitching::Detail_Estimator>;
	
	ptr_extern! { dyn crate::stitching::Detail_Estimator,
		cv_PtrOfDetail_Estimator_delete, cv_PtrOfDetail_Estimator_get_inner_ptr, cv_PtrOfDetail_Estimator_get_inner_ptr_mut
	}
	
	impl core::Ptr<dyn crate::stitching::Detail_Estimator> {
		#[inline] pub fn as_raw_PtrOfDetail_Estimator(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_Estimator(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_EstimatorConst for core::Ptr<dyn crate::stitching::Detail_Estimator> {
		#[inline] fn as_raw_Detail_Estimator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_Estimator for core::Ptr<dyn crate::stitching::Detail_Estimator> {
		#[inline] fn as_raw_mut_Detail_Estimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDetail_ExposureCompensator = core::Ptr<dyn crate::stitching::Detail_ExposureCompensator>;
	
	ptr_extern! { dyn crate::stitching::Detail_ExposureCompensator,
		cv_PtrOfDetail_ExposureCompensator_delete, cv_PtrOfDetail_ExposureCompensator_get_inner_ptr, cv_PtrOfDetail_ExposureCompensator_get_inner_ptr_mut
	}
	
	impl core::Ptr<dyn crate::stitching::Detail_ExposureCompensator> {
		#[inline] pub fn as_raw_PtrOfDetail_ExposureCompensator(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_ExposureCompensator(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_ExposureCompensatorConst for core::Ptr<dyn crate::stitching::Detail_ExposureCompensator> {
		#[inline] fn as_raw_Detail_ExposureCompensator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_ExposureCompensator for core::Ptr<dyn crate::stitching::Detail_ExposureCompensator> {
		#[inline] fn as_raw_mut_Detail_ExposureCompensator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDetail_FeatherBlender = core::Ptr<crate::stitching::Detail_FeatherBlender>;
	
	ptr_extern! { crate::stitching::Detail_FeatherBlender,
		cv_PtrOfDetail_FeatherBlender_delete, cv_PtrOfDetail_FeatherBlender_get_inner_ptr, cv_PtrOfDetail_FeatherBlender_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::Detail_FeatherBlender, cv_PtrOfDetail_FeatherBlender_new }
	
	impl core::Ptr<crate::stitching::Detail_FeatherBlender> {
		#[inline] pub fn as_raw_PtrOfDetail_FeatherBlender(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_FeatherBlender(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_FeatherBlenderTraitConst for core::Ptr<crate::stitching::Detail_FeatherBlender> {
		#[inline] fn as_raw_Detail_FeatherBlender(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_FeatherBlenderTrait for core::Ptr<crate::stitching::Detail_FeatherBlender> {
		#[inline] fn as_raw_mut_Detail_FeatherBlender(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::Detail_BlenderTraitConst for core::Ptr<crate::stitching::Detail_FeatherBlender> {
		#[inline] fn as_raw_Detail_Blender(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_BlenderTrait for core::Ptr<crate::stitching::Detail_FeatherBlender> {
		#[inline] fn as_raw_mut_Detail_Blender(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_FeatherBlender, core::Ptr<crate::stitching::Detail_Blender>, cv_PtrOfDetail_FeatherBlender_to_PtrOfDetail_Blender }
	
	pub type PtrOfDetail_FeaturesMatcher = core::Ptr<dyn crate::stitching::Detail_FeaturesMatcher>;
	
	ptr_extern! { dyn crate::stitching::Detail_FeaturesMatcher,
		cv_PtrOfDetail_FeaturesMatcher_delete, cv_PtrOfDetail_FeaturesMatcher_get_inner_ptr, cv_PtrOfDetail_FeaturesMatcher_get_inner_ptr_mut
	}
	
	impl core::Ptr<dyn crate::stitching::Detail_FeaturesMatcher> {
		#[inline] pub fn as_raw_PtrOfDetail_FeaturesMatcher(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_FeaturesMatcher(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_FeaturesMatcherConst for core::Ptr<dyn crate::stitching::Detail_FeaturesMatcher> {
		#[inline] fn as_raw_Detail_FeaturesMatcher(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_FeaturesMatcher for core::Ptr<dyn crate::stitching::Detail_FeaturesMatcher> {
		#[inline] fn as_raw_mut_Detail_FeaturesMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDetail_GainCompensator = core::Ptr<crate::stitching::Detail_GainCompensator>;
	
	ptr_extern! { crate::stitching::Detail_GainCompensator,
		cv_PtrOfDetail_GainCompensator_delete, cv_PtrOfDetail_GainCompensator_get_inner_ptr, cv_PtrOfDetail_GainCompensator_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::Detail_GainCompensator, cv_PtrOfDetail_GainCompensator_new }
	
	impl core::Ptr<crate::stitching::Detail_GainCompensator> {
		#[inline] pub fn as_raw_PtrOfDetail_GainCompensator(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_GainCompensator(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_GainCompensatorTraitConst for core::Ptr<crate::stitching::Detail_GainCompensator> {
		#[inline] fn as_raw_Detail_GainCompensator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_GainCompensatorTrait for core::Ptr<crate::stitching::Detail_GainCompensator> {
		#[inline] fn as_raw_mut_Detail_GainCompensator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::Detail_ExposureCompensatorConst for core::Ptr<crate::stitching::Detail_GainCompensator> {
		#[inline] fn as_raw_Detail_ExposureCompensator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_ExposureCompensator for core::Ptr<crate::stitching::Detail_GainCompensator> {
		#[inline] fn as_raw_mut_Detail_ExposureCompensator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_GainCompensator, core::Ptr<dyn crate::stitching::Detail_ExposureCompensator>, cv_PtrOfDetail_GainCompensator_to_PtrOfDetail_ExposureCompensator }
	
	pub type PtrOfDetail_GraphCutSeamFinder = core::Ptr<crate::stitching::Detail_GraphCutSeamFinder>;
	
	ptr_extern! { crate::stitching::Detail_GraphCutSeamFinder,
		cv_PtrOfDetail_GraphCutSeamFinder_delete, cv_PtrOfDetail_GraphCutSeamFinder_get_inner_ptr, cv_PtrOfDetail_GraphCutSeamFinder_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::Detail_GraphCutSeamFinder, cv_PtrOfDetail_GraphCutSeamFinder_new }
	
	impl core::Ptr<crate::stitching::Detail_GraphCutSeamFinder> {
		#[inline] pub fn as_raw_PtrOfDetail_GraphCutSeamFinder(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_GraphCutSeamFinder(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_GraphCutSeamFinderTraitConst for core::Ptr<crate::stitching::Detail_GraphCutSeamFinder> {
		#[inline] fn as_raw_Detail_GraphCutSeamFinder(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_GraphCutSeamFinderTrait for core::Ptr<crate::stitching::Detail_GraphCutSeamFinder> {
		#[inline] fn as_raw_mut_Detail_GraphCutSeamFinder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::Detail_GraphCutSeamFinderBaseTraitConst for core::Ptr<crate::stitching::Detail_GraphCutSeamFinder> {
		#[inline] fn as_raw_Detail_GraphCutSeamFinderBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_GraphCutSeamFinderBaseTrait for core::Ptr<crate::stitching::Detail_GraphCutSeamFinder> {
		#[inline] fn as_raw_mut_Detail_GraphCutSeamFinderBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::Detail_SeamFinderConst for core::Ptr<crate::stitching::Detail_GraphCutSeamFinder> {
		#[inline] fn as_raw_Detail_SeamFinder(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_SeamFinder for core::Ptr<crate::stitching::Detail_GraphCutSeamFinder> {
		#[inline] fn as_raw_mut_Detail_SeamFinder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_GraphCutSeamFinder, core::Ptr<dyn crate::stitching::Detail_SeamFinder>, cv_PtrOfDetail_GraphCutSeamFinder_to_PtrOfDetail_SeamFinder }
	
	pub type PtrOfDetail_HomographyBasedEstimator = core::Ptr<crate::stitching::Detail_HomographyBasedEstimator>;
	
	ptr_extern! { crate::stitching::Detail_HomographyBasedEstimator,
		cv_PtrOfDetail_HomographyBasedEstimator_delete, cv_PtrOfDetail_HomographyBasedEstimator_get_inner_ptr, cv_PtrOfDetail_HomographyBasedEstimator_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::Detail_HomographyBasedEstimator, cv_PtrOfDetail_HomographyBasedEstimator_new }
	
	impl core::Ptr<crate::stitching::Detail_HomographyBasedEstimator> {
		#[inline] pub fn as_raw_PtrOfDetail_HomographyBasedEstimator(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_HomographyBasedEstimator(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_HomographyBasedEstimatorTraitConst for core::Ptr<crate::stitching::Detail_HomographyBasedEstimator> {
		#[inline] fn as_raw_Detail_HomographyBasedEstimator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_HomographyBasedEstimatorTrait for core::Ptr<crate::stitching::Detail_HomographyBasedEstimator> {
		#[inline] fn as_raw_mut_Detail_HomographyBasedEstimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::Detail_EstimatorConst for core::Ptr<crate::stitching::Detail_HomographyBasedEstimator> {
		#[inline] fn as_raw_Detail_Estimator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_Estimator for core::Ptr<crate::stitching::Detail_HomographyBasedEstimator> {
		#[inline] fn as_raw_mut_Detail_Estimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_HomographyBasedEstimator, core::Ptr<dyn crate::stitching::Detail_Estimator>, cv_PtrOfDetail_HomographyBasedEstimator_to_PtrOfDetail_Estimator }
	
	pub type PtrOfDetail_MultiBandBlender = core::Ptr<crate::stitching::Detail_MultiBandBlender>;
	
	ptr_extern! { crate::stitching::Detail_MultiBandBlender,
		cv_PtrOfDetail_MultiBandBlender_delete, cv_PtrOfDetail_MultiBandBlender_get_inner_ptr, cv_PtrOfDetail_MultiBandBlender_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::Detail_MultiBandBlender, cv_PtrOfDetail_MultiBandBlender_new }
	
	impl core::Ptr<crate::stitching::Detail_MultiBandBlender> {
		#[inline] pub fn as_raw_PtrOfDetail_MultiBandBlender(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_MultiBandBlender(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_MultiBandBlenderTraitConst for core::Ptr<crate::stitching::Detail_MultiBandBlender> {
		#[inline] fn as_raw_Detail_MultiBandBlender(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_MultiBandBlenderTrait for core::Ptr<crate::stitching::Detail_MultiBandBlender> {
		#[inline] fn as_raw_mut_Detail_MultiBandBlender(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::Detail_BlenderTraitConst for core::Ptr<crate::stitching::Detail_MultiBandBlender> {
		#[inline] fn as_raw_Detail_Blender(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_BlenderTrait for core::Ptr<crate::stitching::Detail_MultiBandBlender> {
		#[inline] fn as_raw_mut_Detail_Blender(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_MultiBandBlender, core::Ptr<crate::stitching::Detail_Blender>, cv_PtrOfDetail_MultiBandBlender_to_PtrOfDetail_Blender }
	
	pub type PtrOfDetail_NoBundleAdjuster = core::Ptr<crate::stitching::Detail_NoBundleAdjuster>;
	
	ptr_extern! { crate::stitching::Detail_NoBundleAdjuster,
		cv_PtrOfDetail_NoBundleAdjuster_delete, cv_PtrOfDetail_NoBundleAdjuster_get_inner_ptr, cv_PtrOfDetail_NoBundleAdjuster_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::Detail_NoBundleAdjuster, cv_PtrOfDetail_NoBundleAdjuster_new }
	
	impl core::Ptr<crate::stitching::Detail_NoBundleAdjuster> {
		#[inline] pub fn as_raw_PtrOfDetail_NoBundleAdjuster(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_NoBundleAdjuster(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_NoBundleAdjusterTraitConst for core::Ptr<crate::stitching::Detail_NoBundleAdjuster> {
		#[inline] fn as_raw_Detail_NoBundleAdjuster(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_NoBundleAdjusterTrait for core::Ptr<crate::stitching::Detail_NoBundleAdjuster> {
		#[inline] fn as_raw_mut_Detail_NoBundleAdjuster(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::Detail_BundleAdjusterBaseConst for core::Ptr<crate::stitching::Detail_NoBundleAdjuster> {
		#[inline] fn as_raw_Detail_BundleAdjusterBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_BundleAdjusterBase for core::Ptr<crate::stitching::Detail_NoBundleAdjuster> {
		#[inline] fn as_raw_mut_Detail_BundleAdjusterBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_NoBundleAdjuster, core::Ptr<dyn crate::stitching::Detail_BundleAdjusterBase>, cv_PtrOfDetail_NoBundleAdjuster_to_PtrOfDetail_BundleAdjusterBase }
	
	impl crate::stitching::Detail_EstimatorConst for core::Ptr<crate::stitching::Detail_NoBundleAdjuster> {
		#[inline] fn as_raw_Detail_Estimator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_Estimator for core::Ptr<crate::stitching::Detail_NoBundleAdjuster> {
		#[inline] fn as_raw_mut_Detail_Estimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_NoBundleAdjuster, core::Ptr<dyn crate::stitching::Detail_Estimator>, cv_PtrOfDetail_NoBundleAdjuster_to_PtrOfDetail_Estimator }
	
	pub type PtrOfDetail_NoExposureCompensator = core::Ptr<crate::stitching::Detail_NoExposureCompensator>;
	
	ptr_extern! { crate::stitching::Detail_NoExposureCompensator,
		cv_PtrOfDetail_NoExposureCompensator_delete, cv_PtrOfDetail_NoExposureCompensator_get_inner_ptr, cv_PtrOfDetail_NoExposureCompensator_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::Detail_NoExposureCompensator, cv_PtrOfDetail_NoExposureCompensator_new }
	
	impl core::Ptr<crate::stitching::Detail_NoExposureCompensator> {
		#[inline] pub fn as_raw_PtrOfDetail_NoExposureCompensator(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_NoExposureCompensator(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_NoExposureCompensatorTraitConst for core::Ptr<crate::stitching::Detail_NoExposureCompensator> {
		#[inline] fn as_raw_Detail_NoExposureCompensator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_NoExposureCompensatorTrait for core::Ptr<crate::stitching::Detail_NoExposureCompensator> {
		#[inline] fn as_raw_mut_Detail_NoExposureCompensator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::Detail_ExposureCompensatorConst for core::Ptr<crate::stitching::Detail_NoExposureCompensator> {
		#[inline] fn as_raw_Detail_ExposureCompensator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_ExposureCompensator for core::Ptr<crate::stitching::Detail_NoExposureCompensator> {
		#[inline] fn as_raw_mut_Detail_ExposureCompensator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_NoExposureCompensator, core::Ptr<dyn crate::stitching::Detail_ExposureCompensator>, cv_PtrOfDetail_NoExposureCompensator_to_PtrOfDetail_ExposureCompensator }
	
	pub type PtrOfDetail_NoSeamFinder = core::Ptr<crate::stitching::Detail_NoSeamFinder>;
	
	ptr_extern! { crate::stitching::Detail_NoSeamFinder,
		cv_PtrOfDetail_NoSeamFinder_delete, cv_PtrOfDetail_NoSeamFinder_get_inner_ptr, cv_PtrOfDetail_NoSeamFinder_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::Detail_NoSeamFinder, cv_PtrOfDetail_NoSeamFinder_new }
	
	impl core::Ptr<crate::stitching::Detail_NoSeamFinder> {
		#[inline] pub fn as_raw_PtrOfDetail_NoSeamFinder(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_NoSeamFinder(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_NoSeamFinderTraitConst for core::Ptr<crate::stitching::Detail_NoSeamFinder> {
		#[inline] fn as_raw_Detail_NoSeamFinder(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_NoSeamFinderTrait for core::Ptr<crate::stitching::Detail_NoSeamFinder> {
		#[inline] fn as_raw_mut_Detail_NoSeamFinder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::Detail_SeamFinderConst for core::Ptr<crate::stitching::Detail_NoSeamFinder> {
		#[inline] fn as_raw_Detail_SeamFinder(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_SeamFinder for core::Ptr<crate::stitching::Detail_NoSeamFinder> {
		#[inline] fn as_raw_mut_Detail_SeamFinder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_NoSeamFinder, core::Ptr<dyn crate::stitching::Detail_SeamFinder>, cv_PtrOfDetail_NoSeamFinder_to_PtrOfDetail_SeamFinder }
	
	pub type PtrOfDetail_PairwiseSeamFinder = core::Ptr<dyn crate::stitching::Detail_PairwiseSeamFinder>;
	
	ptr_extern! { dyn crate::stitching::Detail_PairwiseSeamFinder,
		cv_PtrOfDetail_PairwiseSeamFinder_delete, cv_PtrOfDetail_PairwiseSeamFinder_get_inner_ptr, cv_PtrOfDetail_PairwiseSeamFinder_get_inner_ptr_mut
	}
	
	impl core::Ptr<dyn crate::stitching::Detail_PairwiseSeamFinder> {
		#[inline] pub fn as_raw_PtrOfDetail_PairwiseSeamFinder(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_PairwiseSeamFinder(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_PairwiseSeamFinderConst for core::Ptr<dyn crate::stitching::Detail_PairwiseSeamFinder> {
		#[inline] fn as_raw_Detail_PairwiseSeamFinder(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_PairwiseSeamFinder for core::Ptr<dyn crate::stitching::Detail_PairwiseSeamFinder> {
		#[inline] fn as_raw_mut_Detail_PairwiseSeamFinder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::Detail_SeamFinderConst for core::Ptr<dyn crate::stitching::Detail_PairwiseSeamFinder> {
		#[inline] fn as_raw_Detail_SeamFinder(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_SeamFinder for core::Ptr<dyn crate::stitching::Detail_PairwiseSeamFinder> {
		#[inline] fn as_raw_mut_Detail_SeamFinder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_PairwiseSeamFinder, core::Ptr<dyn crate::stitching::Detail_SeamFinder>, cv_PtrOfDetail_PairwiseSeamFinder_to_PtrOfDetail_SeamFinder }
	
	pub type PtrOfDetail_RotationWarper = core::Ptr<dyn crate::stitching::Detail_RotationWarper>;
	
	ptr_extern! { dyn crate::stitching::Detail_RotationWarper,
		cv_PtrOfDetail_RotationWarper_delete, cv_PtrOfDetail_RotationWarper_get_inner_ptr, cv_PtrOfDetail_RotationWarper_get_inner_ptr_mut
	}
	
	impl core::Ptr<dyn crate::stitching::Detail_RotationWarper> {
		#[inline] pub fn as_raw_PtrOfDetail_RotationWarper(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_RotationWarper(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_RotationWarperConst for core::Ptr<dyn crate::stitching::Detail_RotationWarper> {
		#[inline] fn as_raw_Detail_RotationWarper(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_RotationWarper for core::Ptr<dyn crate::stitching::Detail_RotationWarper> {
		#[inline] fn as_raw_mut_Detail_RotationWarper(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDetail_SeamFinder = core::Ptr<dyn crate::stitching::Detail_SeamFinder>;
	
	ptr_extern! { dyn crate::stitching::Detail_SeamFinder,
		cv_PtrOfDetail_SeamFinder_delete, cv_PtrOfDetail_SeamFinder_get_inner_ptr, cv_PtrOfDetail_SeamFinder_get_inner_ptr_mut
	}
	
	impl core::Ptr<dyn crate::stitching::Detail_SeamFinder> {
		#[inline] pub fn as_raw_PtrOfDetail_SeamFinder(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_SeamFinder(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_SeamFinderConst for core::Ptr<dyn crate::stitching::Detail_SeamFinder> {
		#[inline] fn as_raw_Detail_SeamFinder(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_SeamFinder for core::Ptr<dyn crate::stitching::Detail_SeamFinder> {
		#[inline] fn as_raw_mut_Detail_SeamFinder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDetail_VoronoiSeamFinder = core::Ptr<crate::stitching::Detail_VoronoiSeamFinder>;
	
	ptr_extern! { crate::stitching::Detail_VoronoiSeamFinder,
		cv_PtrOfDetail_VoronoiSeamFinder_delete, cv_PtrOfDetail_VoronoiSeamFinder_get_inner_ptr, cv_PtrOfDetail_VoronoiSeamFinder_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::Detail_VoronoiSeamFinder, cv_PtrOfDetail_VoronoiSeamFinder_new }
	
	impl core::Ptr<crate::stitching::Detail_VoronoiSeamFinder> {
		#[inline] pub fn as_raw_PtrOfDetail_VoronoiSeamFinder(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_VoronoiSeamFinder(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_VoronoiSeamFinderTraitConst for core::Ptr<crate::stitching::Detail_VoronoiSeamFinder> {
		#[inline] fn as_raw_Detail_VoronoiSeamFinder(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_VoronoiSeamFinderTrait for core::Ptr<crate::stitching::Detail_VoronoiSeamFinder> {
		#[inline] fn as_raw_mut_Detail_VoronoiSeamFinder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::Detail_PairwiseSeamFinderConst for core::Ptr<crate::stitching::Detail_VoronoiSeamFinder> {
		#[inline] fn as_raw_Detail_PairwiseSeamFinder(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_PairwiseSeamFinder for core::Ptr<crate::stitching::Detail_VoronoiSeamFinder> {
		#[inline] fn as_raw_mut_Detail_PairwiseSeamFinder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_VoronoiSeamFinder, core::Ptr<dyn crate::stitching::Detail_PairwiseSeamFinder>, cv_PtrOfDetail_VoronoiSeamFinder_to_PtrOfDetail_PairwiseSeamFinder }
	
	impl crate::stitching::Detail_SeamFinderConst for core::Ptr<crate::stitching::Detail_VoronoiSeamFinder> {
		#[inline] fn as_raw_Detail_SeamFinder(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_SeamFinder for core::Ptr<crate::stitching::Detail_VoronoiSeamFinder> {
		#[inline] fn as_raw_mut_Detail_SeamFinder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_VoronoiSeamFinder, core::Ptr<dyn crate::stitching::Detail_SeamFinder>, cv_PtrOfDetail_VoronoiSeamFinder_to_PtrOfDetail_SeamFinder }
	
	pub type PtrOfFisheyeWarper = core::Ptr<crate::stitching::FisheyeWarper>;
	
	ptr_extern! { crate::stitching::FisheyeWarper,
		cv_PtrOfFisheyeWarper_delete, cv_PtrOfFisheyeWarper_get_inner_ptr, cv_PtrOfFisheyeWarper_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::FisheyeWarper, cv_PtrOfFisheyeWarper_new }
	
	impl core::Ptr<crate::stitching::FisheyeWarper> {
		#[inline] pub fn as_raw_PtrOfFisheyeWarper(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFisheyeWarper(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::FisheyeWarperTraitConst for core::Ptr<crate::stitching::FisheyeWarper> {
		#[inline] fn as_raw_FisheyeWarper(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::FisheyeWarperTrait for core::Ptr<crate::stitching::FisheyeWarper> {
		#[inline] fn as_raw_mut_FisheyeWarper(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::WarperCreatorConst for core::Ptr<crate::stitching::FisheyeWarper> {
		#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::WarperCreator for core::Ptr<crate::stitching::FisheyeWarper> {
		#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfFisheyeWarper, core::Ptr<dyn crate::stitching::WarperCreator>, cv_PtrOfFisheyeWarper_to_PtrOfWarperCreator }
	
	pub type PtrOfMercatorWarper = core::Ptr<crate::stitching::MercatorWarper>;
	
	ptr_extern! { crate::stitching::MercatorWarper,
		cv_PtrOfMercatorWarper_delete, cv_PtrOfMercatorWarper_get_inner_ptr, cv_PtrOfMercatorWarper_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::MercatorWarper, cv_PtrOfMercatorWarper_new }
	
	impl core::Ptr<crate::stitching::MercatorWarper> {
		#[inline] pub fn as_raw_PtrOfMercatorWarper(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMercatorWarper(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::MercatorWarperTraitConst for core::Ptr<crate::stitching::MercatorWarper> {
		#[inline] fn as_raw_MercatorWarper(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::MercatorWarperTrait for core::Ptr<crate::stitching::MercatorWarper> {
		#[inline] fn as_raw_mut_MercatorWarper(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::WarperCreatorConst for core::Ptr<crate::stitching::MercatorWarper> {
		#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::WarperCreator for core::Ptr<crate::stitching::MercatorWarper> {
		#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfMercatorWarper, core::Ptr<dyn crate::stitching::WarperCreator>, cv_PtrOfMercatorWarper_to_PtrOfWarperCreator }
	
	pub type PtrOfPaniniPortraitWarper = core::Ptr<crate::stitching::PaniniPortraitWarper>;
	
	ptr_extern! { crate::stitching::PaniniPortraitWarper,
		cv_PtrOfPaniniPortraitWarper_delete, cv_PtrOfPaniniPortraitWarper_get_inner_ptr, cv_PtrOfPaniniPortraitWarper_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::PaniniPortraitWarper, cv_PtrOfPaniniPortraitWarper_new }
	
	impl core::Ptr<crate::stitching::PaniniPortraitWarper> {
		#[inline] pub fn as_raw_PtrOfPaniniPortraitWarper(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfPaniniPortraitWarper(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::PaniniPortraitWarperTraitConst for core::Ptr<crate::stitching::PaniniPortraitWarper> {
		#[inline] fn as_raw_PaniniPortraitWarper(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::PaniniPortraitWarperTrait for core::Ptr<crate::stitching::PaniniPortraitWarper> {
		#[inline] fn as_raw_mut_PaniniPortraitWarper(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::WarperCreatorConst for core::Ptr<crate::stitching::PaniniPortraitWarper> {
		#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::WarperCreator for core::Ptr<crate::stitching::PaniniPortraitWarper> {
		#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfPaniniPortraitWarper, core::Ptr<dyn crate::stitching::WarperCreator>, cv_PtrOfPaniniPortraitWarper_to_PtrOfWarperCreator }
	
	pub type PtrOfPaniniWarper = core::Ptr<crate::stitching::PaniniWarper>;
	
	ptr_extern! { crate::stitching::PaniniWarper,
		cv_PtrOfPaniniWarper_delete, cv_PtrOfPaniniWarper_get_inner_ptr, cv_PtrOfPaniniWarper_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::PaniniWarper, cv_PtrOfPaniniWarper_new }
	
	impl core::Ptr<crate::stitching::PaniniWarper> {
		#[inline] pub fn as_raw_PtrOfPaniniWarper(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfPaniniWarper(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::PaniniWarperTraitConst for core::Ptr<crate::stitching::PaniniWarper> {
		#[inline] fn as_raw_PaniniWarper(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::PaniniWarperTrait for core::Ptr<crate::stitching::PaniniWarper> {
		#[inline] fn as_raw_mut_PaniniWarper(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::WarperCreatorConst for core::Ptr<crate::stitching::PaniniWarper> {
		#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::WarperCreator for core::Ptr<crate::stitching::PaniniWarper> {
		#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfPaniniWarper, core::Ptr<dyn crate::stitching::WarperCreator>, cv_PtrOfPaniniWarper_to_PtrOfWarperCreator }
	
	pub type PtrOfPlaneWarper = core::Ptr<crate::stitching::PlaneWarper>;
	
	ptr_extern! { crate::stitching::PlaneWarper,
		cv_PtrOfPlaneWarper_delete, cv_PtrOfPlaneWarper_get_inner_ptr, cv_PtrOfPlaneWarper_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::PlaneWarper, cv_PtrOfPlaneWarper_new }
	
	impl core::Ptr<crate::stitching::PlaneWarper> {
		#[inline] pub fn as_raw_PtrOfPlaneWarper(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfPlaneWarper(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::PlaneWarperTraitConst for core::Ptr<crate::stitching::PlaneWarper> {
		#[inline] fn as_raw_PlaneWarper(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::PlaneWarperTrait for core::Ptr<crate::stitching::PlaneWarper> {
		#[inline] fn as_raw_mut_PlaneWarper(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::WarperCreatorConst for core::Ptr<crate::stitching::PlaneWarper> {
		#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::WarperCreator for core::Ptr<crate::stitching::PlaneWarper> {
		#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfPlaneWarper, core::Ptr<dyn crate::stitching::WarperCreator>, cv_PtrOfPlaneWarper_to_PtrOfWarperCreator }
	
	pub type PtrOfSphericalWarper = core::Ptr<crate::stitching::SphericalWarper>;
	
	ptr_extern! { crate::stitching::SphericalWarper,
		cv_PtrOfSphericalWarper_delete, cv_PtrOfSphericalWarper_get_inner_ptr, cv_PtrOfSphericalWarper_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::SphericalWarper, cv_PtrOfSphericalWarper_new }
	
	impl core::Ptr<crate::stitching::SphericalWarper> {
		#[inline] pub fn as_raw_PtrOfSphericalWarper(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSphericalWarper(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::SphericalWarperTraitConst for core::Ptr<crate::stitching::SphericalWarper> {
		#[inline] fn as_raw_SphericalWarper(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::SphericalWarperTrait for core::Ptr<crate::stitching::SphericalWarper> {
		#[inline] fn as_raw_mut_SphericalWarper(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::WarperCreatorConst for core::Ptr<crate::stitching::SphericalWarper> {
		#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::WarperCreator for core::Ptr<crate::stitching::SphericalWarper> {
		#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfSphericalWarper, core::Ptr<dyn crate::stitching::WarperCreator>, cv_PtrOfSphericalWarper_to_PtrOfWarperCreator }
	
	pub type PtrOfStereographicWarper = core::Ptr<crate::stitching::StereographicWarper>;
	
	ptr_extern! { crate::stitching::StereographicWarper,
		cv_PtrOfStereographicWarper_delete, cv_PtrOfStereographicWarper_get_inner_ptr, cv_PtrOfStereographicWarper_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::StereographicWarper, cv_PtrOfStereographicWarper_new }
	
	impl core::Ptr<crate::stitching::StereographicWarper> {
		#[inline] pub fn as_raw_PtrOfStereographicWarper(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfStereographicWarper(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::StereographicWarperTraitConst for core::Ptr<crate::stitching::StereographicWarper> {
		#[inline] fn as_raw_StereographicWarper(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::StereographicWarperTrait for core::Ptr<crate::stitching::StereographicWarper> {
		#[inline] fn as_raw_mut_StereographicWarper(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::WarperCreatorConst for core::Ptr<crate::stitching::StereographicWarper> {
		#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::WarperCreator for core::Ptr<crate::stitching::StereographicWarper> {
		#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfStereographicWarper, core::Ptr<dyn crate::stitching::WarperCreator>, cv_PtrOfStereographicWarper_to_PtrOfWarperCreator }
	
	pub type PtrOfStitcher = core::Ptr<crate::stitching::Stitcher>;
	
	ptr_extern! { crate::stitching::Stitcher,
		cv_PtrOfStitcher_delete, cv_PtrOfStitcher_get_inner_ptr, cv_PtrOfStitcher_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::Stitcher, cv_PtrOfStitcher_new }
	
	impl core::Ptr<crate::stitching::Stitcher> {
		#[inline] pub fn as_raw_PtrOfStitcher(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfStitcher(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::StitcherTraitConst for core::Ptr<crate::stitching::Stitcher> {
		#[inline] fn as_raw_Stitcher(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::StitcherTrait for core::Ptr<crate::stitching::Stitcher> {
		#[inline] fn as_raw_mut_Stitcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfTransverseMercatorWarper = core::Ptr<crate::stitching::TransverseMercatorWarper>;
	
	ptr_extern! { crate::stitching::TransverseMercatorWarper,
		cv_PtrOfTransverseMercatorWarper_delete, cv_PtrOfTransverseMercatorWarper_get_inner_ptr, cv_PtrOfTransverseMercatorWarper_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::TransverseMercatorWarper, cv_PtrOfTransverseMercatorWarper_new }
	
	impl core::Ptr<crate::stitching::TransverseMercatorWarper> {
		#[inline] pub fn as_raw_PtrOfTransverseMercatorWarper(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfTransverseMercatorWarper(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::TransverseMercatorWarperTraitConst for core::Ptr<crate::stitching::TransverseMercatorWarper> {
		#[inline] fn as_raw_TransverseMercatorWarper(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::TransverseMercatorWarperTrait for core::Ptr<crate::stitching::TransverseMercatorWarper> {
		#[inline] fn as_raw_mut_TransverseMercatorWarper(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::WarperCreatorConst for core::Ptr<crate::stitching::TransverseMercatorWarper> {
		#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::WarperCreator for core::Ptr<crate::stitching::TransverseMercatorWarper> {
		#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfTransverseMercatorWarper, core::Ptr<dyn crate::stitching::WarperCreator>, cv_PtrOfTransverseMercatorWarper_to_PtrOfWarperCreator }
	
	pub type PtrOfWarperCreator = core::Ptr<dyn crate::stitching::WarperCreator>;
	
	ptr_extern! { dyn crate::stitching::WarperCreator,
		cv_PtrOfWarperCreator_delete, cv_PtrOfWarperCreator_get_inner_ptr, cv_PtrOfWarperCreator_get_inner_ptr_mut
	}
	
	impl core::Ptr<dyn crate::stitching::WarperCreator> {
		#[inline] pub fn as_raw_PtrOfWarperCreator(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfWarperCreator(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::WarperCreatorConst for core::Ptr<dyn crate::stitching::WarperCreator> {
		#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::WarperCreator for core::Ptr<dyn crate::stitching::WarperCreator> {
		#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type VectorOfDetail_CameraParams = core::Vector<crate::stitching::Detail_CameraParams>;
	
	impl core::Vector<crate::stitching::Detail_CameraParams> {
		pub fn as_raw_VectorOfDetail_CameraParams(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfDetail_CameraParams(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { crate::stitching::Detail_CameraParams,
		cv_VectorOfDetail_CameraParams_new, cv_VectorOfDetail_CameraParams_delete,
		cv_VectorOfDetail_CameraParams_len, cv_VectorOfDetail_CameraParams_is_empty,
		cv_VectorOfDetail_CameraParams_capacity, cv_VectorOfDetail_CameraParams_shrink_to_fit,
		cv_VectorOfDetail_CameraParams_reserve, cv_VectorOfDetail_CameraParams_remove,
		cv_VectorOfDetail_CameraParams_swap, cv_VectorOfDetail_CameraParams_clear,
		cv_VectorOfDetail_CameraParams_get, cv_VectorOfDetail_CameraParams_set,
		cv_VectorOfDetail_CameraParams_push, cv_VectorOfDetail_CameraParams_insert,
	}
	vector_non_copy_or_bool! { crate::stitching::Detail_CameraParams }
	
	pub type VectorOfDetail_ImageFeatures = core::Vector<crate::stitching::Detail_ImageFeatures>;
	
	impl core::Vector<crate::stitching::Detail_ImageFeatures> {
		pub fn as_raw_VectorOfDetail_ImageFeatures(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfDetail_ImageFeatures(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { crate::stitching::Detail_ImageFeatures,
		cv_VectorOfDetail_ImageFeatures_new, cv_VectorOfDetail_ImageFeatures_delete,
		cv_VectorOfDetail_ImageFeatures_len, cv_VectorOfDetail_ImageFeatures_is_empty,
		cv_VectorOfDetail_ImageFeatures_capacity, cv_VectorOfDetail_ImageFeatures_shrink_to_fit,
		cv_VectorOfDetail_ImageFeatures_reserve, cv_VectorOfDetail_ImageFeatures_remove,
		cv_VectorOfDetail_ImageFeatures_swap, cv_VectorOfDetail_ImageFeatures_clear,
		cv_VectorOfDetail_ImageFeatures_get, cv_VectorOfDetail_ImageFeatures_set,
		cv_VectorOfDetail_ImageFeatures_push, cv_VectorOfDetail_ImageFeatures_insert,
	}
	vector_non_copy_or_bool! { crate::stitching::Detail_ImageFeatures }
	
	pub type VectorOfDetail_MatchesInfo = core::Vector<crate::stitching::Detail_MatchesInfo>;
	
	impl core::Vector<crate::stitching::Detail_MatchesInfo> {
		pub fn as_raw_VectorOfDetail_MatchesInfo(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfDetail_MatchesInfo(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { crate::stitching::Detail_MatchesInfo,
		cv_VectorOfDetail_MatchesInfo_new, cv_VectorOfDetail_MatchesInfo_delete,
		cv_VectorOfDetail_MatchesInfo_len, cv_VectorOfDetail_MatchesInfo_is_empty,
		cv_VectorOfDetail_MatchesInfo_capacity, cv_VectorOfDetail_MatchesInfo_shrink_to_fit,
		cv_VectorOfDetail_MatchesInfo_reserve, cv_VectorOfDetail_MatchesInfo_remove,
		cv_VectorOfDetail_MatchesInfo_swap, cv_VectorOfDetail_MatchesInfo_clear,
		cv_VectorOfDetail_MatchesInfo_get, cv_VectorOfDetail_MatchesInfo_set,
		cv_VectorOfDetail_MatchesInfo_push, cv_VectorOfDetail_MatchesInfo_insert,
	}
	vector_non_copy_or_bool! { crate::stitching::Detail_MatchesInfo }
	
}
#[cfg(ocvrs_has_module_stitching)]
pub use stitching_types::*;

#[cfg(ocvrs_has_module_video)]
mod video_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfBackgroundSubtractorKNN = core::Ptr<dyn crate::video::BackgroundSubtractorKNN>;
	
	ptr_extern! { dyn crate::video::BackgroundSubtractorKNN,
		cv_PtrOfBackgroundSubtractorKNN_delete, cv_PtrOfBackgroundSubtractorKNN_get_inner_ptr, cv_PtrOfBackgroundSubtractorKNN_get_inner_ptr_mut
	}
	
	impl core::Ptr<dyn crate::video::BackgroundSubtractorKNN> {
		#[inline] pub fn as_raw_PtrOfBackgroundSubtractorKNN(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBackgroundSubtractorKNN(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::video::BackgroundSubtractorKNNConst for core::Ptr<dyn crate::video::BackgroundSubtractorKNN> {
		#[inline] fn as_raw_BackgroundSubtractorKNN(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::video::BackgroundSubtractorKNN for core::Ptr<dyn crate::video::BackgroundSubtractorKNN> {
		#[inline] fn as_raw_mut_BackgroundSubtractorKNN(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<dyn crate::video::BackgroundSubtractorKNN> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<dyn crate::video::BackgroundSubtractorKNN> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::BackgroundSubtractorConst for core::Ptr<dyn crate::video::BackgroundSubtractorKNN> {
		#[inline] fn as_raw_BackgroundSubtractor(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::video::BackgroundSubtractor for core::Ptr<dyn crate::video::BackgroundSubtractorKNN> {
		#[inline] fn as_raw_mut_BackgroundSubtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfBackgroundSubtractorMOG2 = core::Ptr<dyn crate::video::BackgroundSubtractorMOG2>;
	
	ptr_extern! { dyn crate::video::BackgroundSubtractorMOG2,
		cv_PtrOfBackgroundSubtractorMOG2_delete, cv_PtrOfBackgroundSubtractorMOG2_get_inner_ptr, cv_PtrOfBackgroundSubtractorMOG2_get_inner_ptr_mut
	}
	
	impl core::Ptr<dyn crate::video::BackgroundSubtractorMOG2> {
		#[inline] pub fn as_raw_PtrOfBackgroundSubtractorMOG2(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBackgroundSubtractorMOG2(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::video::BackgroundSubtractorMOG2Const for core::Ptr<dyn crate::video::BackgroundSubtractorMOG2> {
		#[inline] fn as_raw_BackgroundSubtractorMOG2(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::video::BackgroundSubtractorMOG2 for core::Ptr<dyn crate::video::BackgroundSubtractorMOG2> {
		#[inline] fn as_raw_mut_BackgroundSubtractorMOG2(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<dyn crate::video::BackgroundSubtractorMOG2> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<dyn crate::video::BackgroundSubtractorMOG2> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::BackgroundSubtractorConst for core::Ptr<dyn crate::video::BackgroundSubtractorMOG2> {
		#[inline] fn as_raw_BackgroundSubtractor(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::video::BackgroundSubtractor for core::Ptr<dyn crate::video::BackgroundSubtractorMOG2> {
		#[inline] fn as_raw_mut_BackgroundSubtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDISOpticalFlow = core::Ptr<dyn crate::video::DISOpticalFlow>;
	
	ptr_extern! { dyn crate::video::DISOpticalFlow,
		cv_PtrOfDISOpticalFlow_delete, cv_PtrOfDISOpticalFlow_get_inner_ptr, cv_PtrOfDISOpticalFlow_get_inner_ptr_mut
	}
	
	impl core::Ptr<dyn crate::video::DISOpticalFlow> {
		#[inline] pub fn as_raw_PtrOfDISOpticalFlow(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDISOpticalFlow(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::video::DISOpticalFlowConst for core::Ptr<dyn crate::video::DISOpticalFlow> {
		#[inline] fn as_raw_DISOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::video::DISOpticalFlow for core::Ptr<dyn crate::video::DISOpticalFlow> {
		#[inline] fn as_raw_mut_DISOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<dyn crate::video::DISOpticalFlow> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<dyn crate::video::DISOpticalFlow> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::DenseOpticalFlowConst for core::Ptr<dyn crate::video::DISOpticalFlow> {
		#[inline] fn as_raw_DenseOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::video::DenseOpticalFlow for core::Ptr<dyn crate::video::DISOpticalFlow> {
		#[inline] fn as_raw_mut_DenseOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfFarnebackOpticalFlow = core::Ptr<dyn crate::video::FarnebackOpticalFlow>;
	
	ptr_extern! { dyn crate::video::FarnebackOpticalFlow,
		cv_PtrOfFarnebackOpticalFlow_delete, cv_PtrOfFarnebackOpticalFlow_get_inner_ptr, cv_PtrOfFarnebackOpticalFlow_get_inner_ptr_mut
	}
	
	impl core::Ptr<dyn crate::video::FarnebackOpticalFlow> {
		#[inline] pub fn as_raw_PtrOfFarnebackOpticalFlow(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFarnebackOpticalFlow(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::video::FarnebackOpticalFlowConst for core::Ptr<dyn crate::video::FarnebackOpticalFlow> {
		#[inline] fn as_raw_FarnebackOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::video::FarnebackOpticalFlow for core::Ptr<dyn crate::video::FarnebackOpticalFlow> {
		#[inline] fn as_raw_mut_FarnebackOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<dyn crate::video::FarnebackOpticalFlow> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<dyn crate::video::FarnebackOpticalFlow> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::DenseOpticalFlowConst for core::Ptr<dyn crate::video::FarnebackOpticalFlow> {
		#[inline] fn as_raw_DenseOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::video::DenseOpticalFlow for core::Ptr<dyn crate::video::FarnebackOpticalFlow> {
		#[inline] fn as_raw_mut_DenseOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSparsePyrLKOpticalFlow = core::Ptr<dyn crate::video::SparsePyrLKOpticalFlow>;
	
	ptr_extern! { dyn crate::video::SparsePyrLKOpticalFlow,
		cv_PtrOfSparsePyrLKOpticalFlow_delete, cv_PtrOfSparsePyrLKOpticalFlow_get_inner_ptr, cv_PtrOfSparsePyrLKOpticalFlow_get_inner_ptr_mut
	}
	
	impl core::Ptr<dyn crate::video::SparsePyrLKOpticalFlow> {
		#[inline] pub fn as_raw_PtrOfSparsePyrLKOpticalFlow(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSparsePyrLKOpticalFlow(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::video::SparsePyrLKOpticalFlowConst for core::Ptr<dyn crate::video::SparsePyrLKOpticalFlow> {
		#[inline] fn as_raw_SparsePyrLKOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::video::SparsePyrLKOpticalFlow for core::Ptr<dyn crate::video::SparsePyrLKOpticalFlow> {
		#[inline] fn as_raw_mut_SparsePyrLKOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<dyn crate::video::SparsePyrLKOpticalFlow> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<dyn crate::video::SparsePyrLKOpticalFlow> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::SparseOpticalFlowConst for core::Ptr<dyn crate::video::SparsePyrLKOpticalFlow> {
		#[inline] fn as_raw_SparseOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::video::SparseOpticalFlow for core::Ptr<dyn crate::video::SparsePyrLKOpticalFlow> {
		#[inline] fn as_raw_mut_SparseOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfTrackerDaSiamRPN = core::Ptr<dyn crate::video::TrackerDaSiamRPN>;
	
	ptr_extern! { dyn crate::video::TrackerDaSiamRPN,
		cv_PtrOfTrackerDaSiamRPN_delete, cv_PtrOfTrackerDaSiamRPN_get_inner_ptr, cv_PtrOfTrackerDaSiamRPN_get_inner_ptr_mut
	}
	
	impl core::Ptr<dyn crate::video::TrackerDaSiamRPN> {
		#[inline] pub fn as_raw_PtrOfTrackerDaSiamRPN(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfTrackerDaSiamRPN(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::video::TrackerDaSiamRPNConst for core::Ptr<dyn crate::video::TrackerDaSiamRPN> {
		#[inline] fn as_raw_TrackerDaSiamRPN(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::video::TrackerDaSiamRPN for core::Ptr<dyn crate::video::TrackerDaSiamRPN> {
		#[inline] fn as_raw_mut_TrackerDaSiamRPN(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::TrackerConst for core::Ptr<dyn crate::video::TrackerDaSiamRPN> {
		#[inline] fn as_raw_Tracker(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::video::Tracker for core::Ptr<dyn crate::video::TrackerDaSiamRPN> {
		#[inline] fn as_raw_mut_Tracker(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfTrackerGOTURN = core::Ptr<dyn crate::video::TrackerGOTURN>;
	
	ptr_extern! { dyn crate::video::TrackerGOTURN,
		cv_PtrOfTrackerGOTURN_delete, cv_PtrOfTrackerGOTURN_get_inner_ptr, cv_PtrOfTrackerGOTURN_get_inner_ptr_mut
	}
	
	impl core::Ptr<dyn crate::video::TrackerGOTURN> {
		#[inline] pub fn as_raw_PtrOfTrackerGOTURN(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfTrackerGOTURN(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::video::TrackerGOTURNConst for core::Ptr<dyn crate::video::TrackerGOTURN> {
		#[inline] fn as_raw_TrackerGOTURN(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::video::TrackerGOTURN for core::Ptr<dyn crate::video::TrackerGOTURN> {
		#[inline] fn as_raw_mut_TrackerGOTURN(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::TrackerConst for core::Ptr<dyn crate::video::TrackerGOTURN> {
		#[inline] fn as_raw_Tracker(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::video::Tracker for core::Ptr<dyn crate::video::TrackerGOTURN> {
		#[inline] fn as_raw_mut_Tracker(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfTrackerMIL = core::Ptr<dyn crate::video::TrackerMIL>;
	
	ptr_extern! { dyn crate::video::TrackerMIL,
		cv_PtrOfTrackerMIL_delete, cv_PtrOfTrackerMIL_get_inner_ptr, cv_PtrOfTrackerMIL_get_inner_ptr_mut
	}
	
	impl core::Ptr<dyn crate::video::TrackerMIL> {
		#[inline] pub fn as_raw_PtrOfTrackerMIL(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfTrackerMIL(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::video::TrackerMILConst for core::Ptr<dyn crate::video::TrackerMIL> {
		#[inline] fn as_raw_TrackerMIL(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::video::TrackerMIL for core::Ptr<dyn crate::video::TrackerMIL> {
		#[inline] fn as_raw_mut_TrackerMIL(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::TrackerConst for core::Ptr<dyn crate::video::TrackerMIL> {
		#[inline] fn as_raw_Tracker(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::video::Tracker for core::Ptr<dyn crate::video::TrackerMIL> {
		#[inline] fn as_raw_mut_Tracker(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfVariationalRefinement = core::Ptr<dyn crate::video::VariationalRefinement>;
	
	ptr_extern! { dyn crate::video::VariationalRefinement,
		cv_PtrOfVariationalRefinement_delete, cv_PtrOfVariationalRefinement_get_inner_ptr, cv_PtrOfVariationalRefinement_get_inner_ptr_mut
	}
	
	impl core::Ptr<dyn crate::video::VariationalRefinement> {
		#[inline] pub fn as_raw_PtrOfVariationalRefinement(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfVariationalRefinement(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::video::VariationalRefinementConst for core::Ptr<dyn crate::video::VariationalRefinement> {
		#[inline] fn as_raw_VariationalRefinement(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::video::VariationalRefinement for core::Ptr<dyn crate::video::VariationalRefinement> {
		#[inline] fn as_raw_mut_VariationalRefinement(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<dyn crate::video::VariationalRefinement> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<dyn crate::video::VariationalRefinement> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::DenseOpticalFlowConst for core::Ptr<dyn crate::video::VariationalRefinement> {
		#[inline] fn as_raw_DenseOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::video::DenseOpticalFlow for core::Ptr<dyn crate::video::VariationalRefinement> {
		#[inline] fn as_raw_mut_DenseOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(ocvrs_has_module_video)]
pub use video_types::*;

#[cfg(ocvrs_has_module_videoio)]
mod videoio_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type VectorOfVideoCapture = core::Vector<crate::videoio::VideoCapture>;
	
	impl core::Vector<crate::videoio::VideoCapture> {
		pub fn as_raw_VectorOfVideoCapture(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfVideoCapture(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { crate::videoio::VideoCapture,
		cv_VectorOfVideoCapture_new, cv_VectorOfVideoCapture_delete,
		cv_VectorOfVideoCapture_len, cv_VectorOfVideoCapture_is_empty,
		cv_VectorOfVideoCapture_capacity, cv_VectorOfVideoCapture_shrink_to_fit,
		cv_VectorOfVideoCapture_reserve, cv_VectorOfVideoCapture_remove,
		cv_VectorOfVideoCapture_swap, cv_VectorOfVideoCapture_clear,
		cv_VectorOfVideoCapture_get, cv_VectorOfVideoCapture_set,
		cv_VectorOfVideoCapture_push, cv_VectorOfVideoCapture_insert,
	}
	vector_non_copy_or_bool! { crate::videoio::VideoCapture }
	
	pub type VectorOfVideoCaptureAPIs = core::Vector<crate::videoio::VideoCaptureAPIs>;
	
	impl core::Vector<crate::videoio::VideoCaptureAPIs> {
		pub fn as_raw_VectorOfVideoCaptureAPIs(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfVideoCaptureAPIs(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { crate::videoio::VideoCaptureAPIs,
		cv_VectorOfVideoCaptureAPIs_new, cv_VectorOfVideoCaptureAPIs_delete,
		cv_VectorOfVideoCaptureAPIs_len, cv_VectorOfVideoCaptureAPIs_is_empty,
		cv_VectorOfVideoCaptureAPIs_capacity, cv_VectorOfVideoCaptureAPIs_shrink_to_fit,
		cv_VectorOfVideoCaptureAPIs_reserve, cv_VectorOfVideoCaptureAPIs_remove,
		cv_VectorOfVideoCaptureAPIs_swap, cv_VectorOfVideoCaptureAPIs_clear,
		cv_VectorOfVideoCaptureAPIs_get, cv_VectorOfVideoCaptureAPIs_set,
		cv_VectorOfVideoCaptureAPIs_push, cv_VectorOfVideoCaptureAPIs_insert,
	}
	vector_copy_non_bool! { crate::videoio::VideoCaptureAPIs,
		cv_VectorOfVideoCaptureAPIs_data, cv_VectorOfVideoCaptureAPIs_data_mut, cv_VectorOfVideoCaptureAPIs_from_slice,
		cv_VectorOfVideoCaptureAPIs_clone,
	}
	
}
#[cfg(ocvrs_has_module_videoio)]
pub use videoio_types::*;

pub use crate::manual::types::*;
