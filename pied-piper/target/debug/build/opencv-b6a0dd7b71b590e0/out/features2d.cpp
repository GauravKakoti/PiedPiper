#include "ocvrs_common.hpp"
#include <opencv2/features2d.hpp>
#include "features2d_types.hpp"

extern "C" {
	void cv_AGAST_const__InputArrayR_vectorLKeyPointGR_int_bool(const cv::_InputArray* image, std::vector<cv::KeyPoint>* keypoints, int threshold, bool nonmaxSuppression, Result_void* ocvrs_return) {
		try {
			cv::AGAST(*image, *keypoints, threshold, nonmaxSuppression);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_AGAST_const__InputArrayR_vectorLKeyPointGR_int_bool_DetectorType(const cv::_InputArray* image, std::vector<cv::KeyPoint>* keypoints, int threshold, bool nonmaxSuppression, cv::AgastFeatureDetector::DetectorType type, Result_void* ocvrs_return) {
		try {
			cv::AGAST(*image, *keypoints, threshold, nonmaxSuppression, type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_FAST_const__InputArrayR_vectorLKeyPointGR_int_bool(const cv::_InputArray* image, std::vector<cv::KeyPoint>* keypoints, int threshold, bool nonmaxSuppression, Result_void* ocvrs_return) {
		try {
			cv::FAST(*image, *keypoints, threshold, nonmaxSuppression);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_FAST_const__InputArrayR_vectorLKeyPointGR_int_bool_DetectorType(const cv::_InputArray* image, std::vector<cv::KeyPoint>* keypoints, int threshold, bool nonmaxSuppression, cv::FastFeatureDetector::DetectorType type, Result_void* ocvrs_return) {
		try {
			cv::FAST(*image, *keypoints, threshold, nonmaxSuppression, type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_computeRecallPrecisionCurve_const_vectorLvectorLDMatchGGR_const_vectorLvectorLunsigned_charGGR_vectorLPoint2fGR(const std::vector<std::vector<cv::DMatch>>* matches1to2, const std::vector<std::vector<unsigned char>>* correctMatches1to2Mask, std::vector<cv::Point2f>* recallPrecisionCurve, Result_void* ocvrs_return) {
		try {
			cv::computeRecallPrecisionCurve(*matches1to2, *correctMatches1to2Mask, *recallPrecisionCurve);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_drawKeypoints_const__InputArrayR_const_vectorLKeyPointGR_const__InputOutputArrayR_const_ScalarR_DrawMatchesFlags(const cv::_InputArray* image, const std::vector<cv::KeyPoint>* keypoints, const cv::_InputOutputArray* outImage, const cv::Scalar* color, cv::DrawMatchesFlags flags, Result_void* ocvrs_return) {
		try {
			cv::drawKeypoints(*image, *keypoints, *outImage, *color, flags);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_drawMatches_const__InputArrayR_const_vectorLKeyPointGR_const__InputArrayR_const_vectorLKeyPointGR_const_vectorLDMatchGR_const__InputOutputArrayR_const_ScalarR_const_ScalarR_const_vectorLcharGR_DrawMatchesFlags(const cv::_InputArray* img1, const std::vector<cv::KeyPoint>* keypoints1, const cv::_InputArray* img2, const std::vector<cv::KeyPoint>* keypoints2, const std::vector<cv::DMatch>* matches1to2, const cv::_InputOutputArray* outImg, const cv::Scalar* matchColor, const cv::Scalar* singlePointColor, const std::vector<char>* matchesMask, cv::DrawMatchesFlags flags, Result_void* ocvrs_return) {
		try {
			cv::drawMatches(*img1, *keypoints1, *img2, *keypoints2, *matches1to2, *outImg, *matchColor, *singlePointColor, *matchesMask, flags);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_drawMatches_const__InputArrayR_const_vectorLKeyPointGR_const__InputArrayR_const_vectorLKeyPointGR_const_vectorLDMatchGR_const__InputOutputArrayR_const_int_const_ScalarR_const_ScalarR_const_vectorLcharGR_DrawMatchesFlags(const cv::_InputArray* img1, const std::vector<cv::KeyPoint>* keypoints1, const cv::_InputArray* img2, const std::vector<cv::KeyPoint>* keypoints2, const std::vector<cv::DMatch>* matches1to2, const cv::_InputOutputArray* outImg, const int matchesThickness, const cv::Scalar* matchColor, const cv::Scalar* singlePointColor, const std::vector<char>* matchesMask, cv::DrawMatchesFlags flags, Result_void* ocvrs_return) {
		try {
			cv::drawMatches(*img1, *keypoints1, *img2, *keypoints2, *matches1to2, *outImg, matchesThickness, *matchColor, *singlePointColor, *matchesMask, flags);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_drawMatches_const__InputArrayR_const_vectorLKeyPointGR_const__InputArrayR_const_vectorLKeyPointGR_const_vectorLvectorLDMatchGGR_const__InputOutputArrayR_const_ScalarR_const_ScalarR_const_vectorLvectorLcharGGR_DrawMatchesFlags(const cv::_InputArray* img1, const std::vector<cv::KeyPoint>* keypoints1, const cv::_InputArray* img2, const std::vector<cv::KeyPoint>* keypoints2, const std::vector<std::vector<cv::DMatch>>* matches1to2, const cv::_InputOutputArray* outImg, const cv::Scalar* matchColor, const cv::Scalar* singlePointColor, const std::vector<std::vector<char>>* matchesMask, cv::DrawMatchesFlags flags, Result_void* ocvrs_return) {
		try {
			cv::drawMatches(*img1, *keypoints1, *img2, *keypoints2, *matches1to2, *outImg, *matchColor, *singlePointColor, *matchesMask, flags);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_evaluateFeatureDetector_const_MatR_const_MatR_const_MatR_vectorLKeyPointGX_vectorLKeyPointGX_floatR_intR_const_PtrLFeature2DGR(const cv::Mat* img1, const cv::Mat* img2, const cv::Mat* H1to2, std::vector<cv::KeyPoint>* keypoints1, std::vector<cv::KeyPoint>* keypoints2, float* repeatability, int* correspCount, const cv::Ptr<cv::Feature2D>* fdetector, Result_void* ocvrs_return) {
		try {
			cv::evaluateFeatureDetector(*img1, *img2, *H1to2, keypoints1, keypoints2, *repeatability, *correspCount, *fdetector);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_getNearestPoint_const_vectorLPoint2fGR_float(const std::vector<cv::Point2f>* recallPrecisionCurve, float l_precision, Result<int>* ocvrs_return) {
		try {
			int ret = cv::getNearestPoint(*recallPrecisionCurve, l_precision);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<int>, ocvrs_return);
	}
	
	void cv_getRecall_const_vectorLPoint2fGR_float(const std::vector<cv::Point2f>* recallPrecisionCurve, float l_precision, Result<float>* ocvrs_return) {
		try {
			float ret = cv::getRecall(*recallPrecisionCurve, l_precision);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<float>, ocvrs_return);
	}
	
	void cv_AKAZE_create_DescriptorType_int_int_float_int_int_DiffusivityType(cv::AKAZE::DescriptorType descriptor_type, int descriptor_size, int descriptor_channels, float threshold, int nOctaves, int nOctaveLayers, cv::KAZE::DiffusivityType diffusivity, Result<cv::Ptr<cv::AKAZE>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::AKAZE> ret = cv::AKAZE::create(descriptor_type, descriptor_size, descriptor_channels, threshold, nOctaves, nOctaveLayers, diffusivity);
			Ok(new cv::Ptr<cv::AKAZE>(ret), ocvrs_return);
		} OCVRS_CATCH(Result<cv::Ptr<cv::AKAZE>*>, ocvrs_return);
	}
	
	void cv_AKAZE_setDescriptorType_DescriptorType(cv::AKAZE* instance, cv::AKAZE::DescriptorType dtype, Result_void* ocvrs_return) {
		try {
			instance->setDescriptorType(dtype);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_AKAZE_getDescriptorType_const(const cv::AKAZE* instance, Result<cv::AKAZE::DescriptorType>* ocvrs_return) {
		try {
			cv::AKAZE::DescriptorType ret = instance->getDescriptorType();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<cv::AKAZE::DescriptorType>, ocvrs_return);
	}
	
	void cv_AKAZE_setDescriptorSize_int(cv::AKAZE* instance, int dsize, Result_void* ocvrs_return) {
		try {
			instance->setDescriptorSize(dsize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_AKAZE_getDescriptorSize_const(const cv::AKAZE* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getDescriptorSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<int>, ocvrs_return);
	}
	
	void cv_AKAZE_setDescriptorChannels_int(cv::AKAZE* instance, int dch, Result_void* ocvrs_return) {
		try {
			instance->setDescriptorChannels(dch);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_AKAZE_getDescriptorChannels_const(const cv::AKAZE* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getDescriptorChannels();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<int>, ocvrs_return);
	}
	
	void cv_AKAZE_setThreshold_double(cv::AKAZE* instance, double threshold, Result_void* ocvrs_return) {
		try {
			instance->setThreshold(threshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_AKAZE_getThreshold_const(const cv::AKAZE* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<double>, ocvrs_return);
	}
	
	void cv_AKAZE_setNOctaves_int(cv::AKAZE* instance, int octaves, Result_void* ocvrs_return) {
		try {
			instance->setNOctaves(octaves);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_AKAZE_getNOctaves_const(const cv::AKAZE* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNOctaves();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<int>, ocvrs_return);
	}
	
	void cv_AKAZE_setNOctaveLayers_int(cv::AKAZE* instance, int octaveLayers, Result_void* ocvrs_return) {
		try {
			instance->setNOctaveLayers(octaveLayers);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_AKAZE_getNOctaveLayers_const(const cv::AKAZE* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNOctaveLayers();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<int>, ocvrs_return);
	}
	
	void cv_AKAZE_setDiffusivity_DiffusivityType(cv::AKAZE* instance, cv::KAZE::DiffusivityType diff, Result_void* ocvrs_return) {
		try {
			instance->setDiffusivity(diff);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_AKAZE_getDiffusivity_const(const cv::AKAZE* instance, Result<cv::KAZE::DiffusivityType>* ocvrs_return) {
		try {
			cv::KAZE::DiffusivityType ret = instance->getDiffusivity();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<cv::KAZE::DiffusivityType>, ocvrs_return);
	}
	
	void cv_AKAZE_getDefaultName_const(const cv::AKAZE* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->getDefaultName();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(Result<void*>, ocvrs_return);
	}
	
	void cv_AffineFeature_create_const_PtrLFeature2DGR_int_int_float_float(const cv::Ptr<cv::Feature2D>* backend, int maxTilt, int minTilt, float tiltStep, float rotateStepBase, Result<cv::Ptr<cv::AffineFeature>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::AffineFeature> ret = cv::AffineFeature::create(*backend, maxTilt, minTilt, tiltStep, rotateStepBase);
			Ok(new cv::Ptr<cv::AffineFeature>(ret), ocvrs_return);
		} OCVRS_CATCH(Result<cv::Ptr<cv::AffineFeature>*>, ocvrs_return);
	}
	
	void cv_AffineFeature_setViewParams_const_vectorLfloatGR_const_vectorLfloatGR(cv::AffineFeature* instance, const std::vector<float>* tilts, const std::vector<float>* rolls, Result_void* ocvrs_return) {
		try {
			instance->setViewParams(*tilts, *rolls);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_AffineFeature_getViewParams_const_vectorLfloatGR_vectorLfloatGR(const cv::AffineFeature* instance, std::vector<float>* tilts, std::vector<float>* rolls, Result_void* ocvrs_return) {
		try {
			instance->getViewParams(*tilts, *rolls);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_AffineFeature_getDefaultName_const(const cv::AffineFeature* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->getDefaultName();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(Result<void*>, ocvrs_return);
	}
	
	void cv_AgastFeatureDetector_create_int_bool_DetectorType(int threshold, bool nonmaxSuppression, cv::AgastFeatureDetector::DetectorType type, Result<cv::Ptr<cv::AgastFeatureDetector>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::AgastFeatureDetector> ret = cv::AgastFeatureDetector::create(threshold, nonmaxSuppression, type);
			Ok(new cv::Ptr<cv::AgastFeatureDetector>(ret), ocvrs_return);
		} OCVRS_CATCH(Result<cv::Ptr<cv::AgastFeatureDetector>*>, ocvrs_return);
	}
	
	void cv_AgastFeatureDetector_setThreshold_int(cv::AgastFeatureDetector* instance, int threshold, Result_void* ocvrs_return) {
		try {
			instance->setThreshold(threshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_AgastFeatureDetector_getThreshold_const(const cv::AgastFeatureDetector* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<int>, ocvrs_return);
	}
	
	void cv_AgastFeatureDetector_setNonmaxSuppression_bool(cv::AgastFeatureDetector* instance, bool f, Result_void* ocvrs_return) {
		try {
			instance->setNonmaxSuppression(f);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_AgastFeatureDetector_getNonmaxSuppression_const(const cv::AgastFeatureDetector* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getNonmaxSuppression();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<bool>, ocvrs_return);
	}
	
	void cv_AgastFeatureDetector_setType_DetectorType(cv::AgastFeatureDetector* instance, cv::AgastFeatureDetector::DetectorType type, Result_void* ocvrs_return) {
		try {
			instance->setType(type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_AgastFeatureDetector_getType_const(const cv::AgastFeatureDetector* instance, Result<cv::AgastFeatureDetector::DetectorType>* ocvrs_return) {
		try {
			cv::AgastFeatureDetector::DetectorType ret = instance->getType();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<cv::AgastFeatureDetector::DetectorType>, ocvrs_return);
	}
	
	void cv_AgastFeatureDetector_getDefaultName_const(const cv::AgastFeatureDetector* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->getDefaultName();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(Result<void*>, ocvrs_return);
	}
	
	cv::Algorithm* cv_BFMatcher_to_Algorithm(cv::BFMatcher* instance) {
		return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	void cv_BFMatcher_delete(cv::BFMatcher* instance) {
			delete instance;
	}
	
	void cv_BFMatcher_BFMatcher_int_bool(int normType, bool crossCheck, Result<cv::BFMatcher*>* ocvrs_return) {
		try {
			cv::BFMatcher* ret = new cv::BFMatcher(normType, crossCheck);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<cv::BFMatcher*>, ocvrs_return);
	}
	
	void cv_BFMatcher_isMaskSupported_const(const cv::BFMatcher* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isMaskSupported();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<bool>, ocvrs_return);
	}
	
	void cv_BFMatcher_create_int_bool(int normType, bool crossCheck, Result<cv::Ptr<cv::BFMatcher>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::BFMatcher> ret = cv::BFMatcher::create(normType, crossCheck);
			Ok(new cv::Ptr<cv::BFMatcher>(ret), ocvrs_return);
		} OCVRS_CATCH(Result<cv::Ptr<cv::BFMatcher>*>, ocvrs_return);
	}
	
	void cv_BFMatcher_clone_const_bool(const cv::BFMatcher* instance, bool emptyTrainData, Result<cv::Ptr<cv::DescriptorMatcher>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::DescriptorMatcher> ret = instance->clone(emptyTrainData);
			Ok(new cv::Ptr<cv::DescriptorMatcher>(ret), ocvrs_return);
		} OCVRS_CATCH(Result<cv::Ptr<cv::DescriptorMatcher>*>, ocvrs_return);
	}
	
	void cv_BOWImgDescriptorExtractor_delete(cv::BOWImgDescriptorExtractor* instance) {
			delete instance;
	}
	
	void cv_BOWImgDescriptorExtractor_BOWImgDescriptorExtractor_const_PtrLFeature2DGR_const_PtrLDescriptorMatcherGR(const cv::Ptr<cv::Feature2D>* dextractor, const cv::Ptr<cv::DescriptorMatcher>* dmatcher, Result<cv::BOWImgDescriptorExtractor*>* ocvrs_return) {
		try {
			cv::BOWImgDescriptorExtractor* ret = new cv::BOWImgDescriptorExtractor(*dextractor, *dmatcher);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<cv::BOWImgDescriptorExtractor*>, ocvrs_return);
	}
	
	void cv_BOWImgDescriptorExtractor_BOWImgDescriptorExtractor_const_PtrLDescriptorMatcherGR(const cv::Ptr<cv::DescriptorMatcher>* dmatcher, Result<cv::BOWImgDescriptorExtractor*>* ocvrs_return) {
		try {
			cv::BOWImgDescriptorExtractor* ret = new cv::BOWImgDescriptorExtractor(*dmatcher);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<cv::BOWImgDescriptorExtractor*>, ocvrs_return);
	}
	
	void cv_BOWImgDescriptorExtractor_setVocabulary_const_MatR(cv::BOWImgDescriptorExtractor* instance, const cv::Mat* vocabulary, Result_void* ocvrs_return) {
		try {
			instance->setVocabulary(*vocabulary);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_BOWImgDescriptorExtractor_getVocabulary_const(const cv::BOWImgDescriptorExtractor* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			const cv::Mat ret = instance->getVocabulary();
			Ok(new const cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(Result<cv::Mat*>, ocvrs_return);
	}
	
	void cv_BOWImgDescriptorExtractor_compute_const__InputArrayR_vectorLKeyPointGR_const__OutputArrayR_vectorLvectorLintGGX_MatX(cv::BOWImgDescriptorExtractor* instance, const cv::_InputArray* image, std::vector<cv::KeyPoint>* keypoints, const cv::_OutputArray* imgDescriptor, std::vector<std::vector<int>>* pointIdxsOfClusters, cv::Mat* descriptors, Result_void* ocvrs_return) {
		try {
			instance->compute(*image, *keypoints, *imgDescriptor, pointIdxsOfClusters, descriptors);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_BOWImgDescriptorExtractor_compute_const__InputArrayR_const__OutputArrayR_vectorLvectorLintGGX(cv::BOWImgDescriptorExtractor* instance, const cv::_InputArray* keypointDescriptors, const cv::_OutputArray* imgDescriptor, std::vector<std::vector<int>>* pointIdxsOfClusters, Result_void* ocvrs_return) {
		try {
			instance->compute(*keypointDescriptors, *imgDescriptor, pointIdxsOfClusters);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_BOWImgDescriptorExtractor_compute2_const_MatR_vectorLKeyPointGR_MatR(cv::BOWImgDescriptorExtractor* instance, const cv::Mat* image, std::vector<cv::KeyPoint>* keypoints, cv::Mat* imgDescriptor, Result_void* ocvrs_return) {
		try {
			instance->compute2(*image, *keypoints, *imgDescriptor);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_BOWImgDescriptorExtractor_descriptorSize_const(const cv::BOWImgDescriptorExtractor* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->descriptorSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<int>, ocvrs_return);
	}
	
	void cv_BOWImgDescriptorExtractor_descriptorType_const(const cv::BOWImgDescriptorExtractor* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->descriptorType();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<int>, ocvrs_return);
	}
	
	void cv_BOWKMeansTrainer_delete(cv::BOWKMeansTrainer* instance) {
			delete instance;
	}
	
	void cv_BOWKMeansTrainer_BOWKMeansTrainer_int_const_TermCriteriaR_int_int(int clusterCount, const cv::TermCriteria* termcrit, int attempts, int flags, Result<cv::BOWKMeansTrainer*>* ocvrs_return) {
		try {
			cv::BOWKMeansTrainer* ret = new cv::BOWKMeansTrainer(clusterCount, *termcrit, attempts, flags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<cv::BOWKMeansTrainer*>, ocvrs_return);
	}
	
	void cv_BOWKMeansTrainer_cluster_const(const cv::BOWKMeansTrainer* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->cluster();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(Result<cv::Mat*>, ocvrs_return);
	}
	
	void cv_BOWKMeansTrainer_cluster_const_const_MatR(const cv::BOWKMeansTrainer* instance, const cv::Mat* descriptors, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->cluster(*descriptors);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(Result<cv::Mat*>, ocvrs_return);
	}
	
	void cv_BOWTrainer_add_const_MatR(cv::BOWTrainer* instance, const cv::Mat* descriptors, Result_void* ocvrs_return) {
		try {
			instance->add(*descriptors);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_BOWTrainer_getDescriptors_const(const cv::BOWTrainer* instance, Result<std::vector<cv::Mat>*>* ocvrs_return) {
		try {
			const std::vector<cv::Mat> ret = instance->getDescriptors();
			Ok(new const std::vector<cv::Mat>(ret), ocvrs_return);
		} OCVRS_CATCH(Result<std::vector<cv::Mat>*>, ocvrs_return);
	}
	
	void cv_BOWTrainer_descriptorsCount_const(const cv::BOWTrainer* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->descriptorsCount();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<int>, ocvrs_return);
	}
	
	void cv_BOWTrainer_clear(cv::BOWTrainer* instance, Result_void* ocvrs_return) {
		try {
			instance->clear();
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_BOWTrainer_cluster_const(const cv::BOWTrainer* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->cluster();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(Result<cv::Mat*>, ocvrs_return);
	}
	
	void cv_BOWTrainer_cluster_const_const_MatR(const cv::BOWTrainer* instance, const cv::Mat* descriptors, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->cluster(*descriptors);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(Result<cv::Mat*>, ocvrs_return);
	}
	
	cv::Algorithm* cv_BRISK_to_Algorithm(cv::BRISK* instance) {
		return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	cv::Feature2D* cv_BRISK_to_Feature2D(cv::BRISK* instance) {
		return dynamic_cast<cv::Feature2D*>(instance);
	}
	
	void cv_BRISK_delete(cv::BRISK* instance) {
			delete instance;
	}
	
	void cv_BRISK_create_int_int_float(int thresh, int octaves, float patternScale, Result<cv::Ptr<cv::BRISK>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::BRISK> ret = cv::BRISK::create(thresh, octaves, patternScale);
			Ok(new cv::Ptr<cv::BRISK>(ret), ocvrs_return);
		} OCVRS_CATCH(Result<cv::Ptr<cv::BRISK>*>, ocvrs_return);
	}
	
	void cv_BRISK_create_const_vectorLfloatGR_const_vectorLintGR_float_float_const_vectorLintGR(const std::vector<float>* radiusList, const std::vector<int>* numberList, float dMax, float dMin, const std::vector<int>* indexChange, Result<cv::Ptr<cv::BRISK>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::BRISK> ret = cv::BRISK::create(*radiusList, *numberList, dMax, dMin, *indexChange);
			Ok(new cv::Ptr<cv::BRISK>(ret), ocvrs_return);
		} OCVRS_CATCH(Result<cv::Ptr<cv::BRISK>*>, ocvrs_return);
	}
	
	void cv_BRISK_create_int_int_const_vectorLfloatGR_const_vectorLintGR_float_float_const_vectorLintGR(int thresh, int octaves, const std::vector<float>* radiusList, const std::vector<int>* numberList, float dMax, float dMin, const std::vector<int>* indexChange, Result<cv::Ptr<cv::BRISK>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::BRISK> ret = cv::BRISK::create(thresh, octaves, *radiusList, *numberList, dMax, dMin, *indexChange);
			Ok(new cv::Ptr<cv::BRISK>(ret), ocvrs_return);
		} OCVRS_CATCH(Result<cv::Ptr<cv::BRISK>*>, ocvrs_return);
	}
	
	void cv_BRISK_getDefaultName_const(const cv::BRISK* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->getDefaultName();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(Result<void*>, ocvrs_return);
	}
	
	void cv_BRISK_setThreshold_int(cv::BRISK* instance, int threshold, Result_void* ocvrs_return) {
		try {
			instance->setThreshold(threshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_BRISK_getThreshold_const(const cv::BRISK* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<int>, ocvrs_return);
	}
	
	void cv_BRISK_setOctaves_int(cv::BRISK* instance, int octaves, Result_void* ocvrs_return) {
		try {
			instance->setOctaves(octaves);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_BRISK_getOctaves_const(const cv::BRISK* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getOctaves();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<int>, ocvrs_return);
	}
	
	void cv_DescriptorMatcher_add_const__InputArrayR(cv::DescriptorMatcher* instance, const cv::_InputArray* descriptors, Result_void* ocvrs_return) {
		try {
			instance->add(*descriptors);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_DescriptorMatcher_getTrainDescriptors_const(const cv::DescriptorMatcher* instance, Result<std::vector<cv::Mat>*>* ocvrs_return) {
		try {
			const std::vector<cv::Mat> ret = instance->getTrainDescriptors();
			Ok(new const std::vector<cv::Mat>(ret), ocvrs_return);
		} OCVRS_CATCH(Result<std::vector<cv::Mat>*>, ocvrs_return);
	}
	
	void cv_DescriptorMatcher_clear(cv::DescriptorMatcher* instance, Result_void* ocvrs_return) {
		try {
			instance->clear();
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_DescriptorMatcher_empty_const(const cv::DescriptorMatcher* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->empty();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<bool>, ocvrs_return);
	}
	
	void cv_DescriptorMatcher_isMaskSupported_const(const cv::DescriptorMatcher* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isMaskSupported();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<bool>, ocvrs_return);
	}
	
	void cv_DescriptorMatcher_train(cv::DescriptorMatcher* instance, Result_void* ocvrs_return) {
		try {
			instance->train();
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_DescriptorMatcher_match_const_const__InputArrayR_const__InputArrayR_vectorLDMatchGR_const__InputArrayR(const cv::DescriptorMatcher* instance, const cv::_InputArray* queryDescriptors, const cv::_InputArray* trainDescriptors, std::vector<cv::DMatch>* matches, const cv::_InputArray* mask, Result_void* ocvrs_return) {
		try {
			instance->match(*queryDescriptors, *trainDescriptors, *matches, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_DescriptorMatcher_knnMatch_const_const__InputArrayR_const__InputArrayR_vectorLvectorLDMatchGGR_int_const__InputArrayR_bool(const cv::DescriptorMatcher* instance, const cv::_InputArray* queryDescriptors, const cv::_InputArray* trainDescriptors, std::vector<std::vector<cv::DMatch>>* matches, int k, const cv::_InputArray* mask, bool compactResult, Result_void* ocvrs_return) {
		try {
			instance->knnMatch(*queryDescriptors, *trainDescriptors, *matches, k, *mask, compactResult);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_DescriptorMatcher_radiusMatch_const_const__InputArrayR_const__InputArrayR_vectorLvectorLDMatchGGR_float_const__InputArrayR_bool(const cv::DescriptorMatcher* instance, const cv::_InputArray* queryDescriptors, const cv::_InputArray* trainDescriptors, std::vector<std::vector<cv::DMatch>>* matches, float maxDistance, const cv::_InputArray* mask, bool compactResult, Result_void* ocvrs_return) {
		try {
			instance->radiusMatch(*queryDescriptors, *trainDescriptors, *matches, maxDistance, *mask, compactResult);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_DescriptorMatcher_match_const__InputArrayR_vectorLDMatchGR_const__InputArrayR(cv::DescriptorMatcher* instance, const cv::_InputArray* queryDescriptors, std::vector<cv::DMatch>* matches, const cv::_InputArray* masks, Result_void* ocvrs_return) {
		try {
			instance->match(*queryDescriptors, *matches, *masks);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_DescriptorMatcher_knnMatch_const__InputArrayR_vectorLvectorLDMatchGGR_int_const__InputArrayR_bool(cv::DescriptorMatcher* instance, const cv::_InputArray* queryDescriptors, std::vector<std::vector<cv::DMatch>>* matches, int k, const cv::_InputArray* masks, bool compactResult, Result_void* ocvrs_return) {
		try {
			instance->knnMatch(*queryDescriptors, *matches, k, *masks, compactResult);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_DescriptorMatcher_radiusMatch_const__InputArrayR_vectorLvectorLDMatchGGR_float_const__InputArrayR_bool(cv::DescriptorMatcher* instance, const cv::_InputArray* queryDescriptors, std::vector<std::vector<cv::DMatch>>* matches, float maxDistance, const cv::_InputArray* masks, bool compactResult, Result_void* ocvrs_return) {
		try {
			instance->radiusMatch(*queryDescriptors, *matches, maxDistance, *masks, compactResult);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_DescriptorMatcher_write_const_const_StringR(const cv::DescriptorMatcher* instance, const char* fileName, Result_void* ocvrs_return) {
		try {
			instance->write(std::string(fileName));
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_DescriptorMatcher_read_const_StringR(cv::DescriptorMatcher* instance, const char* fileName, Result_void* ocvrs_return) {
		try {
			instance->read(std::string(fileName));
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_DescriptorMatcher_read_const_FileNodeR(cv::DescriptorMatcher* instance, const cv::FileNode* unnamed, Result_void* ocvrs_return) {
		try {
			instance->read(*unnamed);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_DescriptorMatcher_write_const_FileStorageR(const cv::DescriptorMatcher* instance, cv::FileStorage* unnamed, Result_void* ocvrs_return) {
		try {
			instance->write(*unnamed);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_DescriptorMatcher_clone_const_bool(const cv::DescriptorMatcher* instance, bool emptyTrainData, Result<cv::Ptr<cv::DescriptorMatcher>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::DescriptorMatcher> ret = instance->clone(emptyTrainData);
			Ok(new cv::Ptr<cv::DescriptorMatcher>(ret), ocvrs_return);
		} OCVRS_CATCH(Result<cv::Ptr<cv::DescriptorMatcher>*>, ocvrs_return);
	}
	
	void cv_DescriptorMatcher_create_const_StringR(const char* descriptorMatcherType, Result<cv::Ptr<cv::DescriptorMatcher>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::DescriptorMatcher> ret = cv::DescriptorMatcher::create(std::string(descriptorMatcherType));
			Ok(new cv::Ptr<cv::DescriptorMatcher>(ret), ocvrs_return);
		} OCVRS_CATCH(Result<cv::Ptr<cv::DescriptorMatcher>*>, ocvrs_return);
	}
	
	void cv_DescriptorMatcher_create_const_MatcherTypeR(const cv::DescriptorMatcher::MatcherType* matcherType, Result<cv::Ptr<cv::DescriptorMatcher>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::DescriptorMatcher> ret = cv::DescriptorMatcher::create(*matcherType);
			Ok(new cv::Ptr<cv::DescriptorMatcher>(ret), ocvrs_return);
		} OCVRS_CATCH(Result<cv::Ptr<cv::DescriptorMatcher>*>, ocvrs_return);
	}
	
	void cv_DescriptorMatcher_write_const_const_PtrLFileStorageGR_const_StringR(const cv::DescriptorMatcher* instance, const cv::Ptr<cv::FileStorage>* fs, const char* name, Result_void* ocvrs_return) {
		try {
			instance->write(*fs, std::string(name));
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_FastFeatureDetector_create_int_bool_DetectorType(int threshold, bool nonmaxSuppression, cv::FastFeatureDetector::DetectorType type, Result<cv::Ptr<cv::FastFeatureDetector>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::FastFeatureDetector> ret = cv::FastFeatureDetector::create(threshold, nonmaxSuppression, type);
			Ok(new cv::Ptr<cv::FastFeatureDetector>(ret), ocvrs_return);
		} OCVRS_CATCH(Result<cv::Ptr<cv::FastFeatureDetector>*>, ocvrs_return);
	}
	
	void cv_FastFeatureDetector_setThreshold_int(cv::FastFeatureDetector* instance, int threshold, Result_void* ocvrs_return) {
		try {
			instance->setThreshold(threshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_FastFeatureDetector_getThreshold_const(const cv::FastFeatureDetector* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<int>, ocvrs_return);
	}
	
	void cv_FastFeatureDetector_setNonmaxSuppression_bool(cv::FastFeatureDetector* instance, bool f, Result_void* ocvrs_return) {
		try {
			instance->setNonmaxSuppression(f);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_FastFeatureDetector_getNonmaxSuppression_const(const cv::FastFeatureDetector* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getNonmaxSuppression();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<bool>, ocvrs_return);
	}
	
	void cv_FastFeatureDetector_setType_DetectorType(cv::FastFeatureDetector* instance, cv::FastFeatureDetector::DetectorType type, Result_void* ocvrs_return) {
		try {
			instance->setType(type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_FastFeatureDetector_getType_const(const cv::FastFeatureDetector* instance, Result<cv::FastFeatureDetector::DetectorType>* ocvrs_return) {
		try {
			cv::FastFeatureDetector::DetectorType ret = instance->getType();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<cv::FastFeatureDetector::DetectorType>, ocvrs_return);
	}
	
	void cv_FastFeatureDetector_getDefaultName_const(const cv::FastFeatureDetector* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->getDefaultName();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(Result<void*>, ocvrs_return);
	}
	
	cv::BRISK* cv_Feature2D_to_BRISK(cv::Feature2D* instance) {
		return dynamic_cast<cv::BRISK*>(instance);
	}
	
	cv::SIFT* cv_Feature2D_to_SIFT(cv::Feature2D* instance) {
		return dynamic_cast<cv::SIFT*>(instance);
	}
	
	cv::SimpleBlobDetector* cv_Feature2D_to_SimpleBlobDetector(cv::Feature2D* instance) {
		return dynamic_cast<cv::SimpleBlobDetector*>(instance);
	}
	
	cv::Algorithm* cv_Feature2D_to_Algorithm(cv::Feature2D* instance) {
		return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	void cv_Feature2D_delete(cv::Feature2D* instance) {
			delete instance;
	}
	
	void cv_Feature2D_detect_const__InputArrayR_vectorLKeyPointGR_const__InputArrayR(cv::Feature2D* instance, const cv::_InputArray* image, std::vector<cv::KeyPoint>* keypoints, const cv::_InputArray* mask, Result_void* ocvrs_return) {
		try {
			instance->detect(*image, *keypoints, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_Feature2D_detect_const__InputArrayR_vectorLvectorLKeyPointGGR_const__InputArrayR(cv::Feature2D* instance, const cv::_InputArray* images, std::vector<std::vector<cv::KeyPoint>>* keypoints, const cv::_InputArray* masks, Result_void* ocvrs_return) {
		try {
			instance->detect(*images, *keypoints, *masks);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_Feature2D_compute_const__InputArrayR_vectorLKeyPointGR_const__OutputArrayR(cv::Feature2D* instance, const cv::_InputArray* image, std::vector<cv::KeyPoint>* keypoints, const cv::_OutputArray* descriptors, Result_void* ocvrs_return) {
		try {
			instance->compute(*image, *keypoints, *descriptors);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_Feature2D_compute_const__InputArrayR_vectorLvectorLKeyPointGGR_const__OutputArrayR(cv::Feature2D* instance, const cv::_InputArray* images, std::vector<std::vector<cv::KeyPoint>>* keypoints, const cv::_OutputArray* descriptors, Result_void* ocvrs_return) {
		try {
			instance->compute(*images, *keypoints, *descriptors);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_Feature2D_detectAndCompute_const__InputArrayR_const__InputArrayR_vectorLKeyPointGR_const__OutputArrayR_bool(cv::Feature2D* instance, const cv::_InputArray* image, const cv::_InputArray* mask, std::vector<cv::KeyPoint>* keypoints, const cv::_OutputArray* descriptors, bool useProvidedKeypoints, Result_void* ocvrs_return) {
		try {
			instance->detectAndCompute(*image, *mask, *keypoints, *descriptors, useProvidedKeypoints);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_Feature2D_descriptorSize_const(const cv::Feature2D* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->descriptorSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<int>, ocvrs_return);
	}
	
	void cv_Feature2D_descriptorType_const(const cv::Feature2D* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->descriptorType();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<int>, ocvrs_return);
	}
	
	void cv_Feature2D_defaultNorm_const(const cv::Feature2D* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->defaultNorm();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<int>, ocvrs_return);
	}
	
	void cv_Feature2D_write_const_const_StringR(const cv::Feature2D* instance, const char* fileName, Result_void* ocvrs_return) {
		try {
			instance->write(std::string(fileName));
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_Feature2D_read_const_StringR(cv::Feature2D* instance, const char* fileName, Result_void* ocvrs_return) {
		try {
			instance->read(std::string(fileName));
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_Feature2D_write_const_FileStorageR(const cv::Feature2D* instance, cv::FileStorage* unnamed, Result_void* ocvrs_return) {
		try {
			instance->write(*unnamed);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_Feature2D_read_const_FileNodeR(cv::Feature2D* instance, const cv::FileNode* unnamed, Result_void* ocvrs_return) {
		try {
			instance->read(*unnamed);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_Feature2D_empty_const(const cv::Feature2D* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->empty();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<bool>, ocvrs_return);
	}
	
	void cv_Feature2D_getDefaultName_const(const cv::Feature2D* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->getDefaultName();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(Result<void*>, ocvrs_return);
	}
	
	void cv_Feature2D_write_const_const_PtrLFileStorageGR_const_StringR(const cv::Feature2D* instance, const cv::Ptr<cv::FileStorage>* fs, const char* name, Result_void* ocvrs_return) {
		try {
			instance->write(*fs, std::string(name));
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	cv::Algorithm* cv_FlannBasedMatcher_to_Algorithm(cv::FlannBasedMatcher* instance) {
		return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	void cv_FlannBasedMatcher_delete(cv::FlannBasedMatcher* instance) {
			delete instance;
	}
	
	void cv_FlannBasedMatcher_FlannBasedMatcher_const_PtrLIndexParamsGR_const_PtrLSearchParamsGR(const cv::Ptr<cv::flann::IndexParams>* indexParams, const cv::Ptr<cv::flann::SearchParams>* searchParams, Result<cv::FlannBasedMatcher*>* ocvrs_return) {
		try {
			cv::FlannBasedMatcher* ret = new cv::FlannBasedMatcher(*indexParams, *searchParams);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<cv::FlannBasedMatcher*>, ocvrs_return);
	}
	
	void cv_FlannBasedMatcher_add_const__InputArrayR(cv::FlannBasedMatcher* instance, const cv::_InputArray* descriptors, Result_void* ocvrs_return) {
		try {
			instance->add(*descriptors);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_FlannBasedMatcher_clear(cv::FlannBasedMatcher* instance, Result_void* ocvrs_return) {
		try {
			instance->clear();
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_FlannBasedMatcher_read_const_FileNodeR(cv::FlannBasedMatcher* instance, const cv::FileNode* unnamed, Result_void* ocvrs_return) {
		try {
			instance->read(*unnamed);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_FlannBasedMatcher_write_const_FileStorageR(const cv::FlannBasedMatcher* instance, cv::FileStorage* unnamed, Result_void* ocvrs_return) {
		try {
			instance->write(*unnamed);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_FlannBasedMatcher_train(cv::FlannBasedMatcher* instance, Result_void* ocvrs_return) {
		try {
			instance->train();
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_FlannBasedMatcher_isMaskSupported_const(const cv::FlannBasedMatcher* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isMaskSupported();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<bool>, ocvrs_return);
	}
	
	void cv_FlannBasedMatcher_create(Result<cv::Ptr<cv::FlannBasedMatcher>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::FlannBasedMatcher> ret = cv::FlannBasedMatcher::create();
			Ok(new cv::Ptr<cv::FlannBasedMatcher>(ret), ocvrs_return);
		} OCVRS_CATCH(Result<cv::Ptr<cv::FlannBasedMatcher>*>, ocvrs_return);
	}
	
	void cv_FlannBasedMatcher_clone_const_bool(const cv::FlannBasedMatcher* instance, bool emptyTrainData, Result<cv::Ptr<cv::DescriptorMatcher>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::DescriptorMatcher> ret = instance->clone(emptyTrainData);
			Ok(new cv::Ptr<cv::DescriptorMatcher>(ret), ocvrs_return);
		} OCVRS_CATCH(Result<cv::Ptr<cv::DescriptorMatcher>*>, ocvrs_return);
	}
	
	void cv_GFTTDetector_create_int_double_double_int_bool_double(int maxCorners, double qualityLevel, double minDistance, int blockSize, bool useHarrisDetector, double k, Result<cv::Ptr<cv::GFTTDetector>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::GFTTDetector> ret = cv::GFTTDetector::create(maxCorners, qualityLevel, minDistance, blockSize, useHarrisDetector, k);
			Ok(new cv::Ptr<cv::GFTTDetector>(ret), ocvrs_return);
		} OCVRS_CATCH(Result<cv::Ptr<cv::GFTTDetector>*>, ocvrs_return);
	}
	
	void cv_GFTTDetector_create_int_double_double_int_int_bool_double(int maxCorners, double qualityLevel, double minDistance, int blockSize, int gradiantSize, bool useHarrisDetector, double k, Result<cv::Ptr<cv::GFTTDetector>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::GFTTDetector> ret = cv::GFTTDetector::create(maxCorners, qualityLevel, minDistance, blockSize, gradiantSize, useHarrisDetector, k);
			Ok(new cv::Ptr<cv::GFTTDetector>(ret), ocvrs_return);
		} OCVRS_CATCH(Result<cv::Ptr<cv::GFTTDetector>*>, ocvrs_return);
	}
	
	void cv_GFTTDetector_setMaxFeatures_int(cv::GFTTDetector* instance, int maxFeatures, Result_void* ocvrs_return) {
		try {
			instance->setMaxFeatures(maxFeatures);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_GFTTDetector_getMaxFeatures_const(const cv::GFTTDetector* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMaxFeatures();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<int>, ocvrs_return);
	}
	
	void cv_GFTTDetector_setQualityLevel_double(cv::GFTTDetector* instance, double qlevel, Result_void* ocvrs_return) {
		try {
			instance->setQualityLevel(qlevel);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_GFTTDetector_getQualityLevel_const(const cv::GFTTDetector* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getQualityLevel();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<double>, ocvrs_return);
	}
	
	void cv_GFTTDetector_setMinDistance_double(cv::GFTTDetector* instance, double minDistance, Result_void* ocvrs_return) {
		try {
			instance->setMinDistance(minDistance);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_GFTTDetector_getMinDistance_const(const cv::GFTTDetector* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMinDistance();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<double>, ocvrs_return);
	}
	
	void cv_GFTTDetector_setBlockSize_int(cv::GFTTDetector* instance, int blockSize, Result_void* ocvrs_return) {
		try {
			instance->setBlockSize(blockSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_GFTTDetector_getBlockSize_const(const cv::GFTTDetector* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getBlockSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<int>, ocvrs_return);
	}
	
	void cv_GFTTDetector_setHarrisDetector_bool(cv::GFTTDetector* instance, bool val, Result_void* ocvrs_return) {
		try {
			instance->setHarrisDetector(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_GFTTDetector_getHarrisDetector_const(const cv::GFTTDetector* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getHarrisDetector();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<bool>, ocvrs_return);
	}
	
	void cv_GFTTDetector_setK_double(cv::GFTTDetector* instance, double k, Result_void* ocvrs_return) {
		try {
			instance->setK(k);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_GFTTDetector_getK_const(const cv::GFTTDetector* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getK();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<double>, ocvrs_return);
	}
	
	void cv_GFTTDetector_getDefaultName_const(const cv::GFTTDetector* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->getDefaultName();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(Result<void*>, ocvrs_return);
	}
	
	void cv_KAZE_create_bool_bool_float_int_int_DiffusivityType(bool extended, bool upright, float threshold, int nOctaves, int nOctaveLayers, cv::KAZE::DiffusivityType diffusivity, Result<cv::Ptr<cv::KAZE>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::KAZE> ret = cv::KAZE::create(extended, upright, threshold, nOctaves, nOctaveLayers, diffusivity);
			Ok(new cv::Ptr<cv::KAZE>(ret), ocvrs_return);
		} OCVRS_CATCH(Result<cv::Ptr<cv::KAZE>*>, ocvrs_return);
	}
	
	void cv_KAZE_setExtended_bool(cv::KAZE* instance, bool extended, Result_void* ocvrs_return) {
		try {
			instance->setExtended(extended);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_KAZE_getExtended_const(const cv::KAZE* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getExtended();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<bool>, ocvrs_return);
	}
	
	void cv_KAZE_setUpright_bool(cv::KAZE* instance, bool upright, Result_void* ocvrs_return) {
		try {
			instance->setUpright(upright);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_KAZE_getUpright_const(const cv::KAZE* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getUpright();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<bool>, ocvrs_return);
	}
	
	void cv_KAZE_setThreshold_double(cv::KAZE* instance, double threshold, Result_void* ocvrs_return) {
		try {
			instance->setThreshold(threshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_KAZE_getThreshold_const(const cv::KAZE* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<double>, ocvrs_return);
	}
	
	void cv_KAZE_setNOctaves_int(cv::KAZE* instance, int octaves, Result_void* ocvrs_return) {
		try {
			instance->setNOctaves(octaves);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_KAZE_getNOctaves_const(const cv::KAZE* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNOctaves();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<int>, ocvrs_return);
	}
	
	void cv_KAZE_setNOctaveLayers_int(cv::KAZE* instance, int octaveLayers, Result_void* ocvrs_return) {
		try {
			instance->setNOctaveLayers(octaveLayers);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_KAZE_getNOctaveLayers_const(const cv::KAZE* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNOctaveLayers();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<int>, ocvrs_return);
	}
	
	void cv_KAZE_setDiffusivity_DiffusivityType(cv::KAZE* instance, cv::KAZE::DiffusivityType diff, Result_void* ocvrs_return) {
		try {
			instance->setDiffusivity(diff);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_KAZE_getDiffusivity_const(const cv::KAZE* instance, Result<cv::KAZE::DiffusivityType>* ocvrs_return) {
		try {
			cv::KAZE::DiffusivityType ret = instance->getDiffusivity();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<cv::KAZE::DiffusivityType>, ocvrs_return);
	}
	
	void cv_KAZE_getDefaultName_const(const cv::KAZE* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->getDefaultName();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(Result<void*>, ocvrs_return);
	}
	
	void cv_KeyPointsFilter_delete(cv::KeyPointsFilter* instance) {
			delete instance;
	}
	
	void cv_KeyPointsFilter_KeyPointsFilter(Result<cv::KeyPointsFilter*>* ocvrs_return) {
		try {
			cv::KeyPointsFilter* ret = new cv::KeyPointsFilter();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<cv::KeyPointsFilter*>, ocvrs_return);
	}
	
	void cv_KeyPointsFilter_runByImageBorder_vectorLKeyPointGR_Size_int(std::vector<cv::KeyPoint>* keypoints, cv::Size* imageSize, int borderSize, Result_void* ocvrs_return) {
		try {
			cv::KeyPointsFilter::runByImageBorder(*keypoints, *imageSize, borderSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_KeyPointsFilter_runByKeypointSize_vectorLKeyPointGR_float_float(std::vector<cv::KeyPoint>* keypoints, float minSize, float maxSize, Result_void* ocvrs_return) {
		try {
			cv::KeyPointsFilter::runByKeypointSize(*keypoints, minSize, maxSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_KeyPointsFilter_runByPixelsMask_vectorLKeyPointGR_const_MatR(std::vector<cv::KeyPoint>* keypoints, const cv::Mat* mask, Result_void* ocvrs_return) {
		try {
			cv::KeyPointsFilter::runByPixelsMask(*keypoints, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_KeyPointsFilter_removeDuplicated_vectorLKeyPointGR(std::vector<cv::KeyPoint>* keypoints, Result_void* ocvrs_return) {
		try {
			cv::KeyPointsFilter::removeDuplicated(*keypoints);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_KeyPointsFilter_removeDuplicatedSorted_vectorLKeyPointGR(std::vector<cv::KeyPoint>* keypoints, Result_void* ocvrs_return) {
		try {
			cv::KeyPointsFilter::removeDuplicatedSorted(*keypoints);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_KeyPointsFilter_retainBest_vectorLKeyPointGR_int(std::vector<cv::KeyPoint>* keypoints, int npoints, Result_void* ocvrs_return) {
		try {
			cv::KeyPointsFilter::retainBest(*keypoints, npoints);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_MSER_create_int_int_int_double_double_int_double_double_int(int delta, int min_area, int max_area, double max_variation, double min_diversity, int max_evolution, double area_threshold, double min_margin, int edge_blur_size, Result<cv::Ptr<cv::MSER>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::MSER> ret = cv::MSER::create(delta, min_area, max_area, max_variation, min_diversity, max_evolution, area_threshold, min_margin, edge_blur_size);
			Ok(new cv::Ptr<cv::MSER>(ret), ocvrs_return);
		} OCVRS_CATCH(Result<cv::Ptr<cv::MSER>*>, ocvrs_return);
	}
	
	void cv_MSER_detectRegions_const__InputArrayR_vectorLvectorLPointGGR_vectorLRectGR(cv::MSER* instance, const cv::_InputArray* image, std::vector<std::vector<cv::Point>>* msers, std::vector<cv::Rect>* bboxes, Result_void* ocvrs_return) {
		try {
			instance->detectRegions(*image, *msers, *bboxes);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_MSER_setDelta_int(cv::MSER* instance, int delta, Result_void* ocvrs_return) {
		try {
			instance->setDelta(delta);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_MSER_getDelta_const(const cv::MSER* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getDelta();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<int>, ocvrs_return);
	}
	
	void cv_MSER_setMinArea_int(cv::MSER* instance, int minArea, Result_void* ocvrs_return) {
		try {
			instance->setMinArea(minArea);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_MSER_getMinArea_const(const cv::MSER* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMinArea();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<int>, ocvrs_return);
	}
	
	void cv_MSER_setMaxArea_int(cv::MSER* instance, int maxArea, Result_void* ocvrs_return) {
		try {
			instance->setMaxArea(maxArea);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_MSER_getMaxArea_const(const cv::MSER* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMaxArea();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<int>, ocvrs_return);
	}
	
	void cv_MSER_setPass2Only_bool(cv::MSER* instance, bool f, Result_void* ocvrs_return) {
		try {
			instance->setPass2Only(f);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_MSER_getPass2Only_const(const cv::MSER* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getPass2Only();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<bool>, ocvrs_return);
	}
	
	void cv_MSER_getDefaultName_const(const cv::MSER* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->getDefaultName();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(Result<void*>, ocvrs_return);
	}
	
	void cv_ORB_create_int_float_int_int_int_int_ScoreType_int_int(int nfeatures, float scaleFactor, int nlevels, int edgeThreshold, int firstLevel, int WTA_K, cv::ORB::ScoreType scoreType, int patchSize, int fastThreshold, Result<cv::Ptr<cv::ORB>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ORB> ret = cv::ORB::create(nfeatures, scaleFactor, nlevels, edgeThreshold, firstLevel, WTA_K, scoreType, patchSize, fastThreshold);
			Ok(new cv::Ptr<cv::ORB>(ret), ocvrs_return);
		} OCVRS_CATCH(Result<cv::Ptr<cv::ORB>*>, ocvrs_return);
	}
	
	void cv_ORB_setMaxFeatures_int(cv::ORB* instance, int maxFeatures, Result_void* ocvrs_return) {
		try {
			instance->setMaxFeatures(maxFeatures);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_ORB_getMaxFeatures_const(const cv::ORB* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMaxFeatures();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<int>, ocvrs_return);
	}
	
	void cv_ORB_setScaleFactor_double(cv::ORB* instance, double scaleFactor, Result_void* ocvrs_return) {
		try {
			instance->setScaleFactor(scaleFactor);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_ORB_getScaleFactor_const(const cv::ORB* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getScaleFactor();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<double>, ocvrs_return);
	}
	
	void cv_ORB_setNLevels_int(cv::ORB* instance, int nlevels, Result_void* ocvrs_return) {
		try {
			instance->setNLevels(nlevels);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_ORB_getNLevels_const(const cv::ORB* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNLevels();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<int>, ocvrs_return);
	}
	
	void cv_ORB_setEdgeThreshold_int(cv::ORB* instance, int edgeThreshold, Result_void* ocvrs_return) {
		try {
			instance->setEdgeThreshold(edgeThreshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_ORB_getEdgeThreshold_const(const cv::ORB* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getEdgeThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<int>, ocvrs_return);
	}
	
	void cv_ORB_setFirstLevel_int(cv::ORB* instance, int firstLevel, Result_void* ocvrs_return) {
		try {
			instance->setFirstLevel(firstLevel);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_ORB_getFirstLevel_const(const cv::ORB* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getFirstLevel();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<int>, ocvrs_return);
	}
	
	void cv_ORB_setWTA_K_int(cv::ORB* instance, int wta_k, Result_void* ocvrs_return) {
		try {
			instance->setWTA_K(wta_k);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_ORB_getWTA_K_const(const cv::ORB* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getWTA_K();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<int>, ocvrs_return);
	}
	
	void cv_ORB_setScoreType_ScoreType(cv::ORB* instance, cv::ORB::ScoreType scoreType, Result_void* ocvrs_return) {
		try {
			instance->setScoreType(scoreType);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_ORB_getScoreType_const(const cv::ORB* instance, Result<cv::ORB::ScoreType>* ocvrs_return) {
		try {
			cv::ORB::ScoreType ret = instance->getScoreType();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<cv::ORB::ScoreType>, ocvrs_return);
	}
	
	void cv_ORB_setPatchSize_int(cv::ORB* instance, int patchSize, Result_void* ocvrs_return) {
		try {
			instance->setPatchSize(patchSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_ORB_getPatchSize_const(const cv::ORB* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getPatchSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<int>, ocvrs_return);
	}
	
	void cv_ORB_setFastThreshold_int(cv::ORB* instance, int fastThreshold, Result_void* ocvrs_return) {
		try {
			instance->setFastThreshold(fastThreshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_ORB_getFastThreshold_const(const cv::ORB* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getFastThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<int>, ocvrs_return);
	}
	
	void cv_ORB_getDefaultName_const(const cv::ORB* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->getDefaultName();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(Result<void*>, ocvrs_return);
	}
	
	cv::Algorithm* cv_SIFT_to_Algorithm(cv::SIFT* instance) {
		return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	cv::Feature2D* cv_SIFT_to_Feature2D(cv::SIFT* instance) {
		return dynamic_cast<cv::Feature2D*>(instance);
	}
	
	void cv_SIFT_delete(cv::SIFT* instance) {
			delete instance;
	}
	
	void cv_SIFT_create_int_int_double_double_double(int nfeatures, int nOctaveLayers, double contrastThreshold, double edgeThreshold, double sigma, Result<cv::Ptr<cv::SIFT>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::SIFT> ret = cv::SIFT::create(nfeatures, nOctaveLayers, contrastThreshold, edgeThreshold, sigma);
			Ok(new cv::Ptr<cv::SIFT>(ret), ocvrs_return);
		} OCVRS_CATCH(Result<cv::Ptr<cv::SIFT>*>, ocvrs_return);
	}
	
	void cv_SIFT_create_int_int_double_double_double_int(int nfeatures, int nOctaveLayers, double contrastThreshold, double edgeThreshold, double sigma, int descriptorType, Result<cv::Ptr<cv::SIFT>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::SIFT> ret = cv::SIFT::create(nfeatures, nOctaveLayers, contrastThreshold, edgeThreshold, sigma, descriptorType);
			Ok(new cv::Ptr<cv::SIFT>(ret), ocvrs_return);
		} OCVRS_CATCH(Result<cv::Ptr<cv::SIFT>*>, ocvrs_return);
	}
	
	void cv_SIFT_getDefaultName_const(const cv::SIFT* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->getDefaultName();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(Result<void*>, ocvrs_return);
	}
	
	cv::Algorithm* cv_SimpleBlobDetector_to_Algorithm(cv::SimpleBlobDetector* instance) {
		return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	cv::Feature2D* cv_SimpleBlobDetector_to_Feature2D(cv::SimpleBlobDetector* instance) {
		return dynamic_cast<cv::Feature2D*>(instance);
	}
	
	void cv_SimpleBlobDetector_delete(cv::SimpleBlobDetector* instance) {
			delete instance;
	}
	
	void cv_SimpleBlobDetector_create_const_ParamsR(const cv::SimpleBlobDetector::Params* parameters, Result<cv::Ptr<cv::SimpleBlobDetector>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::SimpleBlobDetector> ret = cv::SimpleBlobDetector::create(*parameters);
			Ok(new cv::Ptr<cv::SimpleBlobDetector>(ret), ocvrs_return);
		} OCVRS_CATCH(Result<cv::Ptr<cv::SimpleBlobDetector>*>, ocvrs_return);
	}
	
	void cv_SimpleBlobDetector_getDefaultName_const(const cv::SimpleBlobDetector* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->getDefaultName();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(Result<void*>, ocvrs_return);
	}
	
	void cv_SimpleBlobDetector_Params_Params(Result<cv::SimpleBlobDetector::Params>* ocvrs_return) {
		try {
			cv::SimpleBlobDetector::Params ret;
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<cv::SimpleBlobDetector::Params>, ocvrs_return);
	}
	
	void cv_SimpleBlobDetector_Params_read_const_FileNodeR(cv::SimpleBlobDetector::Params instance, const cv::FileNode* fn, Result_void* ocvrs_return) {
		try {
			instance.read(*fn);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_SimpleBlobDetector_Params_write_const_FileStorageR(const cv::SimpleBlobDetector::Params instance, cv::FileStorage* fs, Result_void* ocvrs_return) {
		try {
			instance.write(*fs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
}
