extern "C" {
	std::pair<cv::gapi::GBackend, cv::GKernelImpl>* cv_TupleOfGBackend_GKernelImpl_new(cv::gapi::GBackend* arg, cv::GKernelImpl* arg_1) {
			std::pair<cv::gapi::GBackend, cv::GKernelImpl> ret = std::make_pair(*arg, *arg_1);
			return new std::pair<cv::gapi::GBackend, cv::GKernelImpl>(ret);
	}
	
	void cv_TupleOfGBackend_GKernelImpl_delete(std::pair<cv::gapi::GBackend, cv::GKernelImpl>* instance) {
			delete instance;
	}
	
	void cv_TupleOfGBackend_GKernelImpl_get_0(const std::pair<cv::gapi::GBackend, cv::GKernelImpl>* instance, cv::gapi::GBackend** ocvrs_return) {
			cv::gapi::GBackend ret = std::get<0>(*instance);
			*ocvrs_return = new cv::gapi::GBackend(ret);
	}
	
	void cv_TupleOfGBackend_GKernelImpl_get_1(const std::pair<cv::gapi::GBackend, cv::GKernelImpl>* instance, cv::GKernelImpl** ocvrs_return) {
			cv::GKernelImpl ret = std::get<1>(*instance);
			*ocvrs_return = new cv::GKernelImpl(ret);
	}
	
}

extern "C" {
	std::tuple<cv::GMat, cv::GMat>* cv_TupleOfGMat_GMat_new(cv::GMat* arg, cv::GMat* arg_1) {
			std::tuple<cv::GMat, cv::GMat> ret = std::make_tuple(*arg, *arg_1);
			return new std::tuple<cv::GMat, cv::GMat>(ret);
	}
	
	void cv_TupleOfGMat_GMat_delete(std::tuple<cv::GMat, cv::GMat>* instance) {
			delete instance;
	}
	
	void cv_TupleOfGMat_GMat_get_0(const std::tuple<cv::GMat, cv::GMat>* instance, cv::GMat** ocvrs_return) {
			cv::GMat ret = std::get<0>(*instance);
			*ocvrs_return = new cv::GMat(ret);
	}
	
	void cv_TupleOfGMat_GMat_get_1(const std::tuple<cv::GMat, cv::GMat>* instance, cv::GMat** ocvrs_return) {
			cv::GMat ret = std::get<1>(*instance);
			*ocvrs_return = new cv::GMat(ret);
	}
	
}

extern "C" {
	std::tuple<cv::GMat, cv::GMat, cv::GMat>* cv_TupleOfGMat_GMat_GMat_new(cv::GMat* arg, cv::GMat* arg_1, cv::GMat* arg_2) {
			std::tuple<cv::GMat, cv::GMat, cv::GMat> ret = std::make_tuple(*arg, *arg_1, *arg_2);
			return new std::tuple<cv::GMat, cv::GMat, cv::GMat>(ret);
	}
	
	void cv_TupleOfGMat_GMat_GMat_delete(std::tuple<cv::GMat, cv::GMat, cv::GMat>* instance) {
			delete instance;
	}
	
	void cv_TupleOfGMat_GMat_GMat_get_0(const std::tuple<cv::GMat, cv::GMat, cv::GMat>* instance, cv::GMat** ocvrs_return) {
			cv::GMat ret = std::get<0>(*instance);
			*ocvrs_return = new cv::GMat(ret);
	}
	
	void cv_TupleOfGMat_GMat_GMat_get_1(const std::tuple<cv::GMat, cv::GMat, cv::GMat>* instance, cv::GMat** ocvrs_return) {
			cv::GMat ret = std::get<1>(*instance);
			*ocvrs_return = new cv::GMat(ret);
	}
	
	void cv_TupleOfGMat_GMat_GMat_get_2(const std::tuple<cv::GMat, cv::GMat, cv::GMat>* instance, cv::GMat** ocvrs_return) {
			cv::GMat ret = std::get<2>(*instance);
			*ocvrs_return = new cv::GMat(ret);
	}
	
}

extern "C" {
	std::tuple<cv::GMat, cv::GMat, cv::GMat, cv::GMat>* cv_TupleOfGMat_GMat_GMat_GMat_new(cv::GMat* arg, cv::GMat* arg_1, cv::GMat* arg_2, cv::GMat* arg_3) {
			std::tuple<cv::GMat, cv::GMat, cv::GMat, cv::GMat> ret = std::make_tuple(*arg, *arg_1, *arg_2, *arg_3);
			return new std::tuple<cv::GMat, cv::GMat, cv::GMat, cv::GMat>(ret);
	}
	
	void cv_TupleOfGMat_GMat_GMat_GMat_delete(std::tuple<cv::GMat, cv::GMat, cv::GMat, cv::GMat>* instance) {
			delete instance;
	}
	
	void cv_TupleOfGMat_GMat_GMat_GMat_get_0(const std::tuple<cv::GMat, cv::GMat, cv::GMat, cv::GMat>* instance, cv::GMat** ocvrs_return) {
			cv::GMat ret = std::get<0>(*instance);
			*ocvrs_return = new cv::GMat(ret);
	}
	
	void cv_TupleOfGMat_GMat_GMat_GMat_get_1(const std::tuple<cv::GMat, cv::GMat, cv::GMat, cv::GMat>* instance, cv::GMat** ocvrs_return) {
			cv::GMat ret = std::get<1>(*instance);
			*ocvrs_return = new cv::GMat(ret);
	}
	
	void cv_TupleOfGMat_GMat_GMat_GMat_get_2(const std::tuple<cv::GMat, cv::GMat, cv::GMat, cv::GMat>* instance, cv::GMat** ocvrs_return) {
			cv::GMat ret = std::get<2>(*instance);
			*ocvrs_return = new cv::GMat(ret);
	}
	
	void cv_TupleOfGMat_GMat_GMat_GMat_get_3(const std::tuple<cv::GMat, cv::GMat, cv::GMat, cv::GMat>* instance, cv::GMat** ocvrs_return) {
			cv::GMat ret = std::get<3>(*instance);
			*ocvrs_return = new cv::GMat(ret);
	}
	
}

extern "C" {
	std::tuple<cv::GMat, cv::GScalar>* cv_TupleOfGMat_GScalar_new(cv::GMat* arg, cv::GScalar* arg_1) {
			std::tuple<cv::GMat, cv::GScalar> ret = std::make_tuple(*arg, *arg_1);
			return new std::tuple<cv::GMat, cv::GScalar>(ret);
	}
	
	void cv_TupleOfGMat_GScalar_delete(std::tuple<cv::GMat, cv::GScalar>* instance) {
			delete instance;
	}
	
	void cv_TupleOfGMat_GScalar_get_0(const std::tuple<cv::GMat, cv::GScalar>* instance, cv::GMat** ocvrs_return) {
			cv::GMat ret = std::get<0>(*instance);
			*ocvrs_return = new cv::GMat(ret);
	}
	
	void cv_TupleOfGMat_GScalar_get_1(const std::tuple<cv::GMat, cv::GScalar>* instance, cv::GScalar** ocvrs_return) {
			cv::GScalar ret = std::get<1>(*instance);
			*ocvrs_return = new cv::GScalar(ret);
	}
	
}

extern "C" {
	std::vector<cv::GArg>* cv_VectorOfGArg_new() {
			std::vector<cv::GArg>* ret = new std::vector<cv::GArg>();
			return ret;
	}
	
	void cv_VectorOfGArg_delete(std::vector<cv::GArg>* instance) {
			delete instance;
	}
	
	size_t cv_VectorOfGArg_len(const std::vector<cv::GArg>* instance) {
			size_t ret = instance->size();
			return ret;
	}
	
	bool cv_VectorOfGArg_is_empty(const std::vector<cv::GArg>* instance) {
			bool ret = instance->empty();
			return ret;
	}
	
	size_t cv_VectorOfGArg_capacity(const std::vector<cv::GArg>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}
	
	void cv_VectorOfGArg_shrink_to_fit(std::vector<cv::GArg>* instance) {
			instance->shrink_to_fit();
	}
	
	void cv_VectorOfGArg_reserve(std::vector<cv::GArg>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}
	
	void cv_VectorOfGArg_remove(std::vector<cv::GArg>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}
	
	void cv_VectorOfGArg_swap(std::vector<cv::GArg>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}
	
	void cv_VectorOfGArg_clear(std::vector<cv::GArg>* instance) {
			instance->clear();
	}
	
	void cv_VectorOfGArg_push(std::vector<cv::GArg>* instance, cv::GArg* val) {
			instance->push_back(*val);
	}
	
	void cv_VectorOfGArg_insert(std::vector<cv::GArg>* instance, size_t index, cv::GArg* val) {
			instance->insert(instance->begin() + index, *val);
	}
	
	void cv_VectorOfGArg_get(const std::vector<cv::GArg>* instance, size_t index, cv::GArg** ocvrs_return) {
			cv::GArg ret = (*instance)[index];
			*ocvrs_return = new cv::GArg(ret);
	}
	
	void cv_VectorOfGArg_set(std::vector<cv::GArg>* instance, size_t index, cv::GArg* val) {
			(*instance)[index] = *val;
	}
	
}


extern "C" {
	std::vector<cv::gapi::GBackend>* cv_VectorOfGBackend_new() {
			std::vector<cv::gapi::GBackend>* ret = new std::vector<cv::gapi::GBackend>();
			return ret;
	}
	
	void cv_VectorOfGBackend_delete(std::vector<cv::gapi::GBackend>* instance) {
			delete instance;
	}
	
	size_t cv_VectorOfGBackend_len(const std::vector<cv::gapi::GBackend>* instance) {
			size_t ret = instance->size();
			return ret;
	}
	
	bool cv_VectorOfGBackend_is_empty(const std::vector<cv::gapi::GBackend>* instance) {
			bool ret = instance->empty();
			return ret;
	}
	
	size_t cv_VectorOfGBackend_capacity(const std::vector<cv::gapi::GBackend>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}
	
	void cv_VectorOfGBackend_shrink_to_fit(std::vector<cv::gapi::GBackend>* instance) {
			instance->shrink_to_fit();
	}
	
	void cv_VectorOfGBackend_reserve(std::vector<cv::gapi::GBackend>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}
	
	void cv_VectorOfGBackend_remove(std::vector<cv::gapi::GBackend>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}
	
	void cv_VectorOfGBackend_swap(std::vector<cv::gapi::GBackend>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}
	
	void cv_VectorOfGBackend_clear(std::vector<cv::gapi::GBackend>* instance) {
			instance->clear();
	}
	
	void cv_VectorOfGBackend_push(std::vector<cv::gapi::GBackend>* instance, cv::gapi::GBackend* val) {
			instance->push_back(*val);
	}
	
	void cv_VectorOfGBackend_insert(std::vector<cv::gapi::GBackend>* instance, size_t index, cv::gapi::GBackend* val) {
			instance->insert(instance->begin() + index, *val);
	}
	
	void cv_VectorOfGBackend_get(const std::vector<cv::gapi::GBackend>* instance, size_t index, cv::gapi::GBackend** ocvrs_return) {
			cv::gapi::GBackend ret = (*instance)[index];
			*ocvrs_return = new cv::gapi::GBackend(ret);
	}
	
	void cv_VectorOfGBackend_set(std::vector<cv::gapi::GBackend>* instance, size_t index, cv::gapi::GBackend* val) {
			(*instance)[index] = *val;
	}
	
}


extern "C" {
	std::vector<cv::GCompileArg>* cv_VectorOfGCompileArg_new() {
			std::vector<cv::GCompileArg>* ret = new std::vector<cv::GCompileArg>();
			return ret;
	}
	
	void cv_VectorOfGCompileArg_delete(std::vector<cv::GCompileArg>* instance) {
			delete instance;
	}
	
	size_t cv_VectorOfGCompileArg_len(const std::vector<cv::GCompileArg>* instance) {
			size_t ret = instance->size();
			return ret;
	}
	
	bool cv_VectorOfGCompileArg_is_empty(const std::vector<cv::GCompileArg>* instance) {
			bool ret = instance->empty();
			return ret;
	}
	
	size_t cv_VectorOfGCompileArg_capacity(const std::vector<cv::GCompileArg>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}
	
	void cv_VectorOfGCompileArg_shrink_to_fit(std::vector<cv::GCompileArg>* instance) {
			instance->shrink_to_fit();
	}
	
	void cv_VectorOfGCompileArg_reserve(std::vector<cv::GCompileArg>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}
	
	void cv_VectorOfGCompileArg_remove(std::vector<cv::GCompileArg>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}
	
	void cv_VectorOfGCompileArg_swap(std::vector<cv::GCompileArg>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}
	
	void cv_VectorOfGCompileArg_clear(std::vector<cv::GCompileArg>* instance) {
			instance->clear();
	}
	
	void cv_VectorOfGCompileArg_push(std::vector<cv::GCompileArg>* instance, cv::GCompileArg* val) {
			instance->push_back(*val);
	}
	
	void cv_VectorOfGCompileArg_insert(std::vector<cv::GCompileArg>* instance, size_t index, cv::GCompileArg* val) {
			instance->insert(instance->begin() + index, *val);
	}
	
	void cv_VectorOfGCompileArg_get(const std::vector<cv::GCompileArg>* instance, size_t index, cv::GCompileArg** ocvrs_return) {
			cv::GCompileArg ret = (*instance)[index];
			*ocvrs_return = new cv::GCompileArg(ret);
	}
	
	void cv_VectorOfGCompileArg_set(std::vector<cv::GCompileArg>* instance, size_t index, cv::GCompileArg* val) {
			(*instance)[index] = *val;
	}
	
}


extern "C" {
	std::vector<cv::GMat>* cv_VectorOfGMat_new() {
			std::vector<cv::GMat>* ret = new std::vector<cv::GMat>();
			return ret;
	}
	
	void cv_VectorOfGMat_delete(std::vector<cv::GMat>* instance) {
			delete instance;
	}
	
	size_t cv_VectorOfGMat_len(const std::vector<cv::GMat>* instance) {
			size_t ret = instance->size();
			return ret;
	}
	
	bool cv_VectorOfGMat_is_empty(const std::vector<cv::GMat>* instance) {
			bool ret = instance->empty();
			return ret;
	}
	
	size_t cv_VectorOfGMat_capacity(const std::vector<cv::GMat>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}
	
	void cv_VectorOfGMat_shrink_to_fit(std::vector<cv::GMat>* instance) {
			instance->shrink_to_fit();
	}
	
	void cv_VectorOfGMat_reserve(std::vector<cv::GMat>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}
	
	void cv_VectorOfGMat_remove(std::vector<cv::GMat>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}
	
	void cv_VectorOfGMat_swap(std::vector<cv::GMat>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}
	
	void cv_VectorOfGMat_clear(std::vector<cv::GMat>* instance) {
			instance->clear();
	}
	
	void cv_VectorOfGMat_push(std::vector<cv::GMat>* instance, cv::GMat* val) {
			instance->push_back(*val);
	}
	
	void cv_VectorOfGMat_insert(std::vector<cv::GMat>* instance, size_t index, cv::GMat* val) {
			instance->insert(instance->begin() + index, *val);
	}
	
	void cv_VectorOfGMat_get(const std::vector<cv::GMat>* instance, size_t index, cv::GMat** ocvrs_return) {
			cv::GMat ret = (*instance)[index];
			*ocvrs_return = new cv::GMat(ret);
	}
	
	void cv_VectorOfGMat_set(std::vector<cv::GMat>* instance, size_t index, cv::GMat* val) {
			(*instance)[index] = *val;
	}
	
}


extern "C" {
	std::vector<cv::GRunArg>* cv_VectorOfGRunArg_new() {
			std::vector<cv::GRunArg>* ret = new std::vector<cv::GRunArg>();
			return ret;
	}
	
	void cv_VectorOfGRunArg_delete(std::vector<cv::GRunArg>* instance) {
			delete instance;
	}
	
	size_t cv_VectorOfGRunArg_len(const std::vector<cv::GRunArg>* instance) {
			size_t ret = instance->size();
			return ret;
	}
	
	bool cv_VectorOfGRunArg_is_empty(const std::vector<cv::GRunArg>* instance) {
			bool ret = instance->empty();
			return ret;
	}
	
	size_t cv_VectorOfGRunArg_capacity(const std::vector<cv::GRunArg>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}
	
	void cv_VectorOfGRunArg_shrink_to_fit(std::vector<cv::GRunArg>* instance) {
			instance->shrink_to_fit();
	}
	
	void cv_VectorOfGRunArg_reserve(std::vector<cv::GRunArg>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}
	
	void cv_VectorOfGRunArg_remove(std::vector<cv::GRunArg>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}
	
	void cv_VectorOfGRunArg_swap(std::vector<cv::GRunArg>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}
	
	void cv_VectorOfGRunArg_clear(std::vector<cv::GRunArg>* instance) {
			instance->clear();
	}
	
	void cv_VectorOfGRunArg_push(std::vector<cv::GRunArg>* instance, cv::GRunArg* val) {
			instance->push_back(*val);
	}
	
	void cv_VectorOfGRunArg_insert(std::vector<cv::GRunArg>* instance, size_t index, cv::GRunArg* val) {
			instance->insert(instance->begin() + index, *val);
	}
	
	void cv_VectorOfGRunArg_get(const std::vector<cv::GRunArg>* instance, size_t index, cv::GRunArg** ocvrs_return) {
			cv::GRunArg ret = (*instance)[index];
			*ocvrs_return = new cv::GRunArg(ret);
	}
	
	void cv_VectorOfGRunArg_set(std::vector<cv::GRunArg>* instance, size_t index, cv::GRunArg* val) {
			(*instance)[index] = *val;
	}
	
}


extern "C" {
	std::vector<cv::GShape>* cv_VectorOfGShape_new() {
			std::vector<cv::GShape>* ret = new std::vector<cv::GShape>();
			return ret;
	}
	
	void cv_VectorOfGShape_delete(std::vector<cv::GShape>* instance) {
			delete instance;
	}
	
	size_t cv_VectorOfGShape_len(const std::vector<cv::GShape>* instance) {
			size_t ret = instance->size();
			return ret;
	}
	
	bool cv_VectorOfGShape_is_empty(const std::vector<cv::GShape>* instance) {
			bool ret = instance->empty();
			return ret;
	}
	
	size_t cv_VectorOfGShape_capacity(const std::vector<cv::GShape>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}
	
	void cv_VectorOfGShape_shrink_to_fit(std::vector<cv::GShape>* instance) {
			instance->shrink_to_fit();
	}
	
	void cv_VectorOfGShape_reserve(std::vector<cv::GShape>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}
	
	void cv_VectorOfGShape_remove(std::vector<cv::GShape>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}
	
	void cv_VectorOfGShape_swap(std::vector<cv::GShape>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}
	
	void cv_VectorOfGShape_clear(std::vector<cv::GShape>* instance) {
			instance->clear();
	}
	
	void cv_VectorOfGShape_push(std::vector<cv::GShape>* instance, cv::GShape val) {
			instance->push_back(val);
	}
	
	void cv_VectorOfGShape_insert(std::vector<cv::GShape>* instance, size_t index, cv::GShape val) {
			instance->insert(instance->begin() + index, val);
	}
	
	void cv_VectorOfGShape_get(const std::vector<cv::GShape>* instance, size_t index, cv::GShape* ocvrs_return) {
			cv::GShape ret = (*instance)[index];
			*ocvrs_return = ret;
	}
	
	void cv_VectorOfGShape_set(std::vector<cv::GShape>* instance, size_t index, cv::GShape val) {
			(*instance)[index] = val;
	}
	
	std::vector<cv::GShape>* cv_VectorOfGShape_clone(const std::vector<cv::GShape>* instance) {
			std::vector<cv::GShape> ret = std::vector<cv::GShape>(*instance);
			return new std::vector<cv::GShape>(ret);
	}
	
	const cv::GShape* cv_VectorOfGShape_data(const std::vector<cv::GShape>* instance) {
		return instance->data();
	}
	
	cv::GShape* cv_VectorOfGShape_data_mut(std::vector<cv::GShape>* instance) {
		return instance->data();
	}
	
	std::vector<cv::GShape>* cv_VectorOfGShape_from_slice(const cv::GShape* data, size_t len) {
		return new std::vector<cv::GShape>(data, data + len);
	}
}


extern "C" {
	std::vector<cv::GTransform>* cv_VectorOfGTransform_new() {
			std::vector<cv::GTransform>* ret = new std::vector<cv::GTransform>();
			return ret;
	}
	
	void cv_VectorOfGTransform_delete(std::vector<cv::GTransform>* instance) {
			delete instance;
	}
	
	size_t cv_VectorOfGTransform_len(const std::vector<cv::GTransform>* instance) {
			size_t ret = instance->size();
			return ret;
	}
	
	bool cv_VectorOfGTransform_is_empty(const std::vector<cv::GTransform>* instance) {
			bool ret = instance->empty();
			return ret;
	}
	
	size_t cv_VectorOfGTransform_capacity(const std::vector<cv::GTransform>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}
	
	void cv_VectorOfGTransform_shrink_to_fit(std::vector<cv::GTransform>* instance) {
			instance->shrink_to_fit();
	}
	
	void cv_VectorOfGTransform_reserve(std::vector<cv::GTransform>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}
	
	void cv_VectorOfGTransform_remove(std::vector<cv::GTransform>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}
	
	void cv_VectorOfGTransform_swap(std::vector<cv::GTransform>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}
	
	void cv_VectorOfGTransform_clear(std::vector<cv::GTransform>* instance) {
			instance->clear();
	}
	
	void cv_VectorOfGTransform_push(std::vector<cv::GTransform>* instance, cv::GTransform* val) {
			instance->push_back(*val);
	}
	
	void cv_VectorOfGTransform_insert(std::vector<cv::GTransform>* instance, size_t index, cv::GTransform* val) {
			instance->insert(instance->begin() + index, *val);
	}
	
	void cv_VectorOfGTransform_get(const std::vector<cv::GTransform>* instance, size_t index, cv::GTransform** ocvrs_return) {
			cv::GTransform ret = (*instance)[index];
			*ocvrs_return = new cv::GTransform(ret);
	}
	
	void cv_VectorOfGTransform_set(std::vector<cv::GTransform>* instance, size_t index, cv::GTransform* val) {
			(*instance)[index] = *val;
	}
	
}


extern "C" {
	std::vector<cv::GTypeInfo>* cv_VectorOfGTypeInfo_new() {
			std::vector<cv::GTypeInfo>* ret = new std::vector<cv::GTypeInfo>();
			return ret;
	}
	
	void cv_VectorOfGTypeInfo_delete(std::vector<cv::GTypeInfo>* instance) {
			delete instance;
	}
	
	size_t cv_VectorOfGTypeInfo_len(const std::vector<cv::GTypeInfo>* instance) {
			size_t ret = instance->size();
			return ret;
	}
	
	bool cv_VectorOfGTypeInfo_is_empty(const std::vector<cv::GTypeInfo>* instance) {
			bool ret = instance->empty();
			return ret;
	}
	
	size_t cv_VectorOfGTypeInfo_capacity(const std::vector<cv::GTypeInfo>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}
	
	void cv_VectorOfGTypeInfo_shrink_to_fit(std::vector<cv::GTypeInfo>* instance) {
			instance->shrink_to_fit();
	}
	
	void cv_VectorOfGTypeInfo_reserve(std::vector<cv::GTypeInfo>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}
	
	void cv_VectorOfGTypeInfo_remove(std::vector<cv::GTypeInfo>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}
	
	void cv_VectorOfGTypeInfo_swap(std::vector<cv::GTypeInfo>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}
	
	void cv_VectorOfGTypeInfo_clear(std::vector<cv::GTypeInfo>* instance) {
			instance->clear();
	}
	
	void cv_VectorOfGTypeInfo_push(std::vector<cv::GTypeInfo>* instance, cv::GTypeInfo* val) {
			instance->push_back(*val);
	}
	
	void cv_VectorOfGTypeInfo_insert(std::vector<cv::GTypeInfo>* instance, size_t index, cv::GTypeInfo* val) {
			instance->insert(instance->begin() + index, *val);
	}
	
	void cv_VectorOfGTypeInfo_get(const std::vector<cv::GTypeInfo>* instance, size_t index, cv::GTypeInfo** ocvrs_return) {
			cv::GTypeInfo ret = (*instance)[index];
			*ocvrs_return = new cv::GTypeInfo(ret);
	}
	
	void cv_VectorOfGTypeInfo_set(std::vector<cv::GTypeInfo>* instance, size_t index, cv::GTypeInfo* val) {
			(*instance)[index] = *val;
	}
	
}


extern "C" {
	std::vector<cv::detail::OpaqueKind>* cv_VectorOfOpaqueKind_new() {
			std::vector<cv::detail::OpaqueKind>* ret = new std::vector<cv::detail::OpaqueKind>();
			return ret;
	}
	
	void cv_VectorOfOpaqueKind_delete(std::vector<cv::detail::OpaqueKind>* instance) {
			delete instance;
	}
	
	size_t cv_VectorOfOpaqueKind_len(const std::vector<cv::detail::OpaqueKind>* instance) {
			size_t ret = instance->size();
			return ret;
	}
	
	bool cv_VectorOfOpaqueKind_is_empty(const std::vector<cv::detail::OpaqueKind>* instance) {
			bool ret = instance->empty();
			return ret;
	}
	
	size_t cv_VectorOfOpaqueKind_capacity(const std::vector<cv::detail::OpaqueKind>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}
	
	void cv_VectorOfOpaqueKind_shrink_to_fit(std::vector<cv::detail::OpaqueKind>* instance) {
			instance->shrink_to_fit();
	}
	
	void cv_VectorOfOpaqueKind_reserve(std::vector<cv::detail::OpaqueKind>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}
	
	void cv_VectorOfOpaqueKind_remove(std::vector<cv::detail::OpaqueKind>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}
	
	void cv_VectorOfOpaqueKind_swap(std::vector<cv::detail::OpaqueKind>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}
	
	void cv_VectorOfOpaqueKind_clear(std::vector<cv::detail::OpaqueKind>* instance) {
			instance->clear();
	}
	
	void cv_VectorOfOpaqueKind_push(std::vector<cv::detail::OpaqueKind>* instance, cv::detail::OpaqueKind val) {
			instance->push_back(val);
	}
	
	void cv_VectorOfOpaqueKind_insert(std::vector<cv::detail::OpaqueKind>* instance, size_t index, cv::detail::OpaqueKind val) {
			instance->insert(instance->begin() + index, val);
	}
	
	void cv_VectorOfOpaqueKind_get(const std::vector<cv::detail::OpaqueKind>* instance, size_t index, cv::detail::OpaqueKind* ocvrs_return) {
			cv::detail::OpaqueKind ret = (*instance)[index];
			*ocvrs_return = ret;
	}
	
	void cv_VectorOfOpaqueKind_set(std::vector<cv::detail::OpaqueKind>* instance, size_t index, cv::detail::OpaqueKind val) {
			(*instance)[index] = val;
	}
	
	std::vector<cv::detail::OpaqueKind>* cv_VectorOfOpaqueKind_clone(const std::vector<cv::detail::OpaqueKind>* instance) {
			std::vector<cv::detail::OpaqueKind> ret = std::vector<cv::detail::OpaqueKind>(*instance);
			return new std::vector<cv::detail::OpaqueKind>(ret);
	}
	
	const cv::detail::OpaqueKind* cv_VectorOfOpaqueKind_data(const std::vector<cv::detail::OpaqueKind>* instance) {
		return instance->data();
	}
	
	cv::detail::OpaqueKind* cv_VectorOfOpaqueKind_data_mut(std::vector<cv::detail::OpaqueKind>* instance) {
		return instance->data();
	}
	
	std::vector<cv::detail::OpaqueKind>* cv_VectorOfOpaqueKind_from_slice(const cv::detail::OpaqueKind* data, size_t len) {
		return new std::vector<cv::detail::OpaqueKind>(data, data + len);
	}
}


