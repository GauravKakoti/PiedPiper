#include "ocvrs_common.hpp"
#include <opencv2/imgproc.hpp>
#include "imgproc_types.hpp"

extern "C" {
	void cv_Canny_const__InputArrayR_const__InputArrayR_const__OutputArrayR_double_double_bool(const cv::_InputArray* dx, const cv::_InputArray* dy, const cv::_OutputArray* edges, double threshold1, double threshold2, bool L2gradient, Result_void* ocvrs_return) {
		try {
			cv::Canny(*dx, *dy, *edges, threshold1, threshold2, L2gradient);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_Canny_const__InputArrayR_const__OutputArrayR_double_double_int_bool(const cv::_InputArray* image, const cv::_OutputArray* edges, double threshold1, double threshold2, int apertureSize, bool L2gradient, Result_void* ocvrs_return) {
		try {
			cv::Canny(*image, *edges, threshold1, threshold2, apertureSize, L2gradient);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_EMD_const__InputArrayR_const__InputArrayR_int_const__InputArrayR_floatX_const__OutputArrayR(const cv::_InputArray* signature1, const cv::_InputArray* signature2, int distType, const cv::_InputArray* cost, float* lowerBound, const cv::_OutputArray* flow, Result<float>* ocvrs_return) {
		try {
			float ret = cv::EMD(*signature1, *signature2, distType, *cost, lowerBound, *flow);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<float>, ocvrs_return);
	}
	
	void cv_GaussianBlur_const__InputArrayR_const__OutputArrayR_Size_double_double_int(const cv::_InputArray* src, const cv::_OutputArray* dst, cv::Size* ksize, double sigmaX, double sigmaY, int borderType, Result_void* ocvrs_return) {
		try {
			cv::GaussianBlur(*src, *dst, *ksize, sigmaX, sigmaY, borderType);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_HoughCircles_const__InputArrayR_const__OutputArrayR_int_double_double_double_double_int_int(const cv::_InputArray* image, const cv::_OutputArray* circles, int method, double dp, double minDist, double param1, double param2, int minRadius, int maxRadius, Result_void* ocvrs_return) {
		try {
			cv::HoughCircles(*image, *circles, method, dp, minDist, param1, param2, minRadius, maxRadius);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_HoughLinesP_const__InputArrayR_const__OutputArrayR_double_double_int_double_double(const cv::_InputArray* image, const cv::_OutputArray* lines, double rho, double theta, int threshold, double minLineLength, double maxLineGap, Result_void* ocvrs_return) {
		try {
			cv::HoughLinesP(*image, *lines, rho, theta, threshold, minLineLength, maxLineGap);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_HoughLinesPointSet_const__InputArrayR_const__OutputArrayR_int_int_double_double_double_double_double_double(const cv::_InputArray* point, const cv::_OutputArray* lines, int lines_max, int threshold, double min_rho, double max_rho, double rho_step, double min_theta, double max_theta, double theta_step, Result_void* ocvrs_return) {
		try {
			cv::HoughLinesPointSet(*point, *lines, lines_max, threshold, min_rho, max_rho, rho_step, min_theta, max_theta, theta_step);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_HoughLines_const__InputArrayR_const__OutputArrayR_double_double_int_double_double_double_double(const cv::_InputArray* image, const cv::_OutputArray* lines, double rho, double theta, int threshold, double srn, double stn, double min_theta, double max_theta, Result_void* ocvrs_return) {
		try {
			cv::HoughLines(*image, *lines, rho, theta, threshold, srn, stn, min_theta, max_theta);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_HuMoments_const_MomentsR_const__OutputArrayR(const cv::Moments* m, const cv::_OutputArray* hu, Result_void* ocvrs_return) {
		try {
			cv::HuMoments(*m, *hu);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_HuMoments_const_MomentsR_doubleXX(const cv::Moments* moments, double(*hu)[7], Result_void* ocvrs_return) {
		try {
			cv::HuMoments(*moments, *hu);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_Laplacian_const__InputArrayR_const__OutputArrayR_int_int_double_double_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int ddepth, int ksize, double scale, double delta, int borderType, Result_void* ocvrs_return) {
		try {
			cv::Laplacian(*src, *dst, ddepth, ksize, scale, delta, borderType);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_Scharr_const__InputArrayR_const__OutputArrayR_int_int_int_double_double_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int ddepth, int dx, int dy, double scale, double delta, int borderType, Result_void* ocvrs_return) {
		try {
			cv::Scharr(*src, *dst, ddepth, dx, dy, scale, delta, borderType);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_Sobel_const__InputArrayR_const__OutputArrayR_int_int_int_int_double_double_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int ddepth, int dx, int dy, int ksize, double scale, double delta, int borderType, Result_void* ocvrs_return) {
		try {
			cv::Sobel(*src, *dst, ddepth, dx, dy, ksize, scale, delta, borderType);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_accumulateProduct_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputArrayR(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_InputOutputArray* dst, const cv::_InputArray* mask, Result_void* ocvrs_return) {
		try {
			cv::accumulateProduct(*src1, *src2, *dst, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_accumulateSquare_const__InputArrayR_const__InputOutputArrayR_const__InputArrayR(const cv::_InputArray* src, const cv::_InputOutputArray* dst, const cv::_InputArray* mask, Result_void* ocvrs_return) {
		try {
			cv::accumulateSquare(*src, *dst, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_accumulateWeighted_const__InputArrayR_const__InputOutputArrayR_double_const__InputArrayR(const cv::_InputArray* src, const cv::_InputOutputArray* dst, double alpha, const cv::_InputArray* mask, Result_void* ocvrs_return) {
		try {
			cv::accumulateWeighted(*src, *dst, alpha, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_accumulate_const__InputArrayR_const__InputOutputArrayR_const__InputArrayR(const cv::_InputArray* src, const cv::_InputOutputArray* dst, const cv::_InputArray* mask, Result_void* ocvrs_return) {
		try {
			cv::accumulate(*src, *dst, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_adaptiveThreshold_const__InputArrayR_const__OutputArrayR_double_int_int_int_double(const cv::_InputArray* src, const cv::_OutputArray* dst, double maxValue, int adaptiveMethod, int thresholdType, int blockSize, double C, Result_void* ocvrs_return) {
		try {
			cv::adaptiveThreshold(*src, *dst, maxValue, adaptiveMethod, thresholdType, blockSize, C);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_applyColorMap_const__InputArrayR_const__OutputArrayR_const__InputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* userColor, Result_void* ocvrs_return) {
		try {
			cv::applyColorMap(*src, *dst, *userColor);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_applyColorMap_const__InputArrayR_const__OutputArrayR_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int colormap, Result_void* ocvrs_return) {
		try {
			cv::applyColorMap(*src, *dst, colormap);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_approxPolyDP_const__InputArrayR_const__OutputArrayR_double_bool(const cv::_InputArray* curve, const cv::_OutputArray* approxCurve, double epsilon, bool closed, Result_void* ocvrs_return) {
		try {
			cv::approxPolyDP(*curve, *approxCurve, epsilon, closed);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_arcLength_const__InputArrayR_bool(const cv::_InputArray* curve, bool closed, Result<double>* ocvrs_return) {
		try {
			double ret = cv::arcLength(*curve, closed);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<double>, ocvrs_return);
	}
	
	void cv_arrowedLine_const__InputOutputArrayR_Point_Point_const_ScalarR_int_int_int_double(const cv::_InputOutputArray* img, cv::Point* pt1, cv::Point* pt2, const cv::Scalar* color, int thickness, int line_type, int shift, double tipLength, Result_void* ocvrs_return) {
		try {
			cv::arrowedLine(*img, *pt1, *pt2, *color, thickness, line_type, shift, tipLength);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_bilateralFilter_const__InputArrayR_const__OutputArrayR_int_double_double_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int d, double sigmaColor, double sigmaSpace, int borderType, Result_void* ocvrs_return) {
		try {
			cv::bilateralFilter(*src, *dst, d, sigmaColor, sigmaSpace, borderType);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_blendLinear_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_InputArray* weights1, const cv::_InputArray* weights2, const cv::_OutputArray* dst, Result_void* ocvrs_return) {
		try {
			cv::blendLinear(*src1, *src2, *weights1, *weights2, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_blur_const__InputArrayR_const__OutputArrayR_Size_Point_int(const cv::_InputArray* src, const cv::_OutputArray* dst, cv::Size* ksize, cv::Point* anchor, int borderType, Result_void* ocvrs_return) {
		try {
			cv::blur(*src, *dst, *ksize, *anchor, borderType);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_boundingRect_const__InputArrayR(const cv::_InputArray* array, Result<cv::Rect>* ocvrs_return) {
		try {
			cv::Rect ret = cv::boundingRect(*array);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<cv::Rect>, ocvrs_return);
	}
	
	void cv_boxFilter_const__InputArrayR_const__OutputArrayR_int_Size_Point_bool_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int ddepth, cv::Size* ksize, cv::Point* anchor, bool normalize, int borderType, Result_void* ocvrs_return) {
		try {
			cv::boxFilter(*src, *dst, ddepth, *ksize, *anchor, normalize, borderType);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_boxPoints_RotatedRect_const__OutputArrayR(cv::RotatedRect* box, const cv::_OutputArray* points, Result_void* ocvrs_return) {
		try {
			cv::boxPoints(*box, *points);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_buildPyramid_const__InputArrayR_const__OutputArrayR_int_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int maxlevel, int borderType, Result_void* ocvrs_return) {
		try {
			cv::buildPyramid(*src, *dst, maxlevel, borderType);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_calcBackProject_const__InputArrayR_const_vectorLintGR_const__InputArrayR_const__OutputArrayR_const_vectorLfloatGR_double(const cv::_InputArray* images, const std::vector<int>* channels, const cv::_InputArray* hist, const cv::_OutputArray* dst, const std::vector<float>* ranges, double scale, Result_void* ocvrs_return) {
		try {
			cv::calcBackProject(*images, *channels, *hist, *dst, *ranges, scale);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_calcHist_const__InputArrayR_const_vectorLintGR_const__InputArrayR_const__OutputArrayR_const_vectorLintGR_const_vectorLfloatGR_bool(const cv::_InputArray* images, const std::vector<int>* channels, const cv::_InputArray* mask, const cv::_OutputArray* hist, const std::vector<int>* histSize, const std::vector<float>* ranges, bool accumulate, Result_void* ocvrs_return) {
		try {
			cv::calcHist(*images, *channels, *mask, *hist, *histSize, *ranges, accumulate);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_circle_const__InputOutputArrayR_Point_int_const_ScalarR_int_int_int(const cv::_InputOutputArray* img, cv::Point* center, int radius, const cv::Scalar* color, int thickness, int lineType, int shift, Result_void* ocvrs_return) {
		try {
			cv::circle(*img, *center, radius, *color, thickness, lineType, shift);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_clipLine_Rect_PointR_PointR(cv::Rect* imgRect, cv::Point* pt1, cv::Point* pt2, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::clipLine(*imgRect, *pt1, *pt2);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<bool>, ocvrs_return);
	}
	
	void cv_clipLine_Size2l_Point2lR_Point2lR(cv::Size2l* imgSize, cv::Point2l* pt1, cv::Point2l* pt2, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::clipLine(*imgSize, *pt1, *pt2);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<bool>, ocvrs_return);
	}
	
	void cv_clipLine_Size_PointR_PointR(cv::Size* imgSize, cv::Point* pt1, cv::Point* pt2, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::clipLine(*imgSize, *pt1, *pt2);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<bool>, ocvrs_return);
	}
	
	void cv_compareHist_const_SparseMatR_const_SparseMatR_int(const cv::SparseMat* H1, const cv::SparseMat* H2, int method, Result<double>* ocvrs_return) {
		try {
			double ret = cv::compareHist(*H1, *H2, method);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<double>, ocvrs_return);
	}
	
	void cv_compareHist_const__InputArrayR_const__InputArrayR_int(const cv::_InputArray* H1, const cv::_InputArray* H2, int method, Result<double>* ocvrs_return) {
		try {
			double ret = cv::compareHist(*H1, *H2, method);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<double>, ocvrs_return);
	}
	
	void cv_connectedComponentsWithStats_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_int(const cv::_InputArray* image, const cv::_OutputArray* labels, const cv::_OutputArray* stats, const cv::_OutputArray* centroids, int connectivity, int ltype, Result<int>* ocvrs_return) {
		try {
			int ret = cv::connectedComponentsWithStats(*image, *labels, *stats, *centroids, connectivity, ltype);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<int>, ocvrs_return);
	}
	
	void cv_connectedComponentsWithStats_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_int_int(const cv::_InputArray* image, const cv::_OutputArray* labels, const cv::_OutputArray* stats, const cv::_OutputArray* centroids, int connectivity, int ltype, int ccltype, Result<int>* ocvrs_return) {
		try {
			int ret = cv::connectedComponentsWithStats(*image, *labels, *stats, *centroids, connectivity, ltype, ccltype);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<int>, ocvrs_return);
	}
	
	void cv_connectedComponents_const__InputArrayR_const__OutputArrayR_int_int(const cv::_InputArray* image, const cv::_OutputArray* labels, int connectivity, int ltype, Result<int>* ocvrs_return) {
		try {
			int ret = cv::connectedComponents(*image, *labels, connectivity, ltype);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<int>, ocvrs_return);
	}
	
	void cv_connectedComponents_const__InputArrayR_const__OutputArrayR_int_int_int(const cv::_InputArray* image, const cv::_OutputArray* labels, int connectivity, int ltype, int ccltype, Result<int>* ocvrs_return) {
		try {
			int ret = cv::connectedComponents(*image, *labels, connectivity, ltype, ccltype);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<int>, ocvrs_return);
	}
	
	void cv_contourArea_const__InputArrayR_bool(const cv::_InputArray* contour, bool oriented, Result<double>* ocvrs_return) {
		try {
			double ret = cv::contourArea(*contour, oriented);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<double>, ocvrs_return);
	}
	
	void cv_convertMaps_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_int_bool(const cv::_InputArray* map1, const cv::_InputArray* map2, const cv::_OutputArray* dstmap1, const cv::_OutputArray* dstmap2, int dstmap1type, bool nninterpolation, Result_void* ocvrs_return) {
		try {
			cv::convertMaps(*map1, *map2, *dstmap1, *dstmap2, dstmap1type, nninterpolation);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_convexHull_const__InputArrayR_const__OutputArrayR_bool_bool(const cv::_InputArray* points, const cv::_OutputArray* hull, bool clockwise, bool returnPoints, Result_void* ocvrs_return) {
		try {
			cv::convexHull(*points, *hull, clockwise, returnPoints);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_convexityDefects_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* contour, const cv::_InputArray* convexhull, const cv::_OutputArray* convexityDefects, Result_void* ocvrs_return) {
		try {
			cv::convexityDefects(*contour, *convexhull, *convexityDefects);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_cornerEigenValsAndVecs_const__InputArrayR_const__OutputArrayR_int_int_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int blockSize, int ksize, int borderType, Result_void* ocvrs_return) {
		try {
			cv::cornerEigenValsAndVecs(*src, *dst, blockSize, ksize, borderType);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_cornerHarris_const__InputArrayR_const__OutputArrayR_int_int_double_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int blockSize, int ksize, double k, int borderType, Result_void* ocvrs_return) {
		try {
			cv::cornerHarris(*src, *dst, blockSize, ksize, k, borderType);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_cornerMinEigenVal_const__InputArrayR_const__OutputArrayR_int_int_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int blockSize, int ksize, int borderType, Result_void* ocvrs_return) {
		try {
			cv::cornerMinEigenVal(*src, *dst, blockSize, ksize, borderType);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_cornerSubPix_const__InputArrayR_const__InputOutputArrayR_Size_Size_TermCriteria(const cv::_InputArray* image, const cv::_InputOutputArray* corners, cv::Size* winSize, cv::Size* zeroZone, cv::TermCriteria* criteria, Result_void* ocvrs_return) {
		try {
			cv::cornerSubPix(*image, *corners, *winSize, *zeroZone, *criteria);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_createCLAHE_double_Size(double clipLimit, cv::Size* tileGridSize, Result<cv::Ptr<cv::CLAHE>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::CLAHE> ret = cv::createCLAHE(clipLimit, *tileGridSize);
			Ok(new cv::Ptr<cv::CLAHE>(ret), ocvrs_return);
		} OCVRS_CATCH(Result<cv::Ptr<cv::CLAHE>*>, ocvrs_return);
	}
	
	void cv_createGeneralizedHoughBallard(Result<cv::Ptr<cv::GeneralizedHoughBallard>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::GeneralizedHoughBallard> ret = cv::createGeneralizedHoughBallard();
			Ok(new cv::Ptr<cv::GeneralizedHoughBallard>(ret), ocvrs_return);
		} OCVRS_CATCH(Result<cv::Ptr<cv::GeneralizedHoughBallard>*>, ocvrs_return);
	}
	
	void cv_createGeneralizedHoughGuil(Result<cv::Ptr<cv::GeneralizedHoughGuil>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::GeneralizedHoughGuil> ret = cv::createGeneralizedHoughGuil();
			Ok(new cv::Ptr<cv::GeneralizedHoughGuil>(ret), ocvrs_return);
		} OCVRS_CATCH(Result<cv::Ptr<cv::GeneralizedHoughGuil>*>, ocvrs_return);
	}
	
	void cv_createHanningWindow_const__OutputArrayR_Size_int(const cv::_OutputArray* dst, cv::Size* winSize, int type, Result_void* ocvrs_return) {
		try {
			cv::createHanningWindow(*dst, *winSize, type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_createLineSegmentDetector_int_double_double_double_double_double_double_int(int refine, double scale, double sigma_scale, double quant, double ang_th, double log_eps, double density_th, int n_bins, Result<cv::Ptr<cv::LineSegmentDetector>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::LineSegmentDetector> ret = cv::createLineSegmentDetector(refine, scale, sigma_scale, quant, ang_th, log_eps, density_th, n_bins);
			Ok(new cv::Ptr<cv::LineSegmentDetector>(ret), ocvrs_return);
		} OCVRS_CATCH(Result<cv::Ptr<cv::LineSegmentDetector>*>, ocvrs_return);
	}
	
	void cv_cvtColorTwoPlane_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, int code, Result_void* ocvrs_return) {
		try {
			cv::cvtColorTwoPlane(*src1, *src2, *dst, code);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_cvtColor_const__InputArrayR_const__OutputArrayR_int_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int code, int dstCn, Result_void* ocvrs_return) {
		try {
			cv::cvtColor(*src, *dst, code, dstCn);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_demosaicing_const__InputArrayR_const__OutputArrayR_int_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int code, int dstCn, Result_void* ocvrs_return) {
		try {
			cv::demosaicing(*src, *dst, code, dstCn);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_dilate_const__InputArrayR_const__OutputArrayR_const__InputArrayR_Point_int_int_const_ScalarR(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* kernel, cv::Point* anchor, int iterations, int borderType, const cv::Scalar* borderValue, Result_void* ocvrs_return) {
		try {
			cv::dilate(*src, *dst, *kernel, *anchor, iterations, borderType, *borderValue);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_distanceTransform_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_int_int_int(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_OutputArray* labels, int distanceType, int maskSize, int labelType, Result_void* ocvrs_return) {
		try {
			cv::distanceTransform(*src, *dst, *labels, distanceType, maskSize, labelType);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_distanceTransform_const__InputArrayR_const__OutputArrayR_int_int_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int distanceType, int maskSize, int dstType, Result_void* ocvrs_return) {
		try {
			cv::distanceTransform(*src, *dst, distanceType, maskSize, dstType);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_divSpectrums_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_bool(const cv::_InputArray* a, const cv::_InputArray* b, const cv::_OutputArray* c, int flags, bool conjB, Result_void* ocvrs_return) {
		try {
			cv::divSpectrums(*a, *b, *c, flags, conjB);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_drawContours_const__InputOutputArrayR_const__InputArrayR_int_const_ScalarR_int_int_const__InputArrayR_int_Point(const cv::_InputOutputArray* image, const cv::_InputArray* contours, int contourIdx, const cv::Scalar* color, int thickness, int lineType, const cv::_InputArray* hierarchy, int maxLevel, cv::Point* offset, Result_void* ocvrs_return) {
		try {
			cv::drawContours(*image, *contours, contourIdx, *color, thickness, lineType, *hierarchy, maxLevel, *offset);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_drawMarker_const__InputOutputArrayR_Point_const_ScalarR_int_int_int_int(const cv::_InputOutputArray* img, cv::Point* position, const cv::Scalar* color, int markerType, int markerSize, int thickness, int line_type, Result_void* ocvrs_return) {
		try {
			cv::drawMarker(*img, *position, *color, markerType, markerSize, thickness, line_type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_ellipse2Poly_Point2d_Size2d_int_int_int_int_vectorLPoint2dGR(cv::Point2d* center, cv::Size2d* axes, int angle, int arcStart, int arcEnd, int delta, std::vector<cv::Point2d>* pts, Result_void* ocvrs_return) {
		try {
			cv::ellipse2Poly(*center, *axes, angle, arcStart, arcEnd, delta, *pts);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_ellipse2Poly_Point_Size_int_int_int_int_vectorLPointGR(cv::Point* center, cv::Size* axes, int angle, int arcStart, int arcEnd, int delta, std::vector<cv::Point>* pts, Result_void* ocvrs_return) {
		try {
			cv::ellipse2Poly(*center, *axes, angle, arcStart, arcEnd, delta, *pts);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_ellipse_const__InputOutputArrayR_Point_Size_double_double_double_const_ScalarR_int_int_int(const cv::_InputOutputArray* img, cv::Point* center, cv::Size* axes, double angle, double startAngle, double endAngle, const cv::Scalar* color, int thickness, int lineType, int shift, Result_void* ocvrs_return) {
		try {
			cv::ellipse(*img, *center, *axes, angle, startAngle, endAngle, *color, thickness, lineType, shift);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_ellipse_const__InputOutputArrayR_const_RotatedRectR_const_ScalarR_int_int(const cv::_InputOutputArray* img, const cv::RotatedRect* box, const cv::Scalar* color, int thickness, int lineType, Result_void* ocvrs_return) {
		try {
			cv::ellipse(*img, *box, *color, thickness, lineType);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_equalizeHist_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, Result_void* ocvrs_return) {
		try {
			cv::equalizeHist(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_erode_const__InputArrayR_const__OutputArrayR_const__InputArrayR_Point_int_int_const_ScalarR(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* kernel, cv::Point* anchor, int iterations, int borderType, const cv::Scalar* borderValue, Result_void* ocvrs_return) {
		try {
			cv::erode(*src, *dst, *kernel, *anchor, iterations, borderType, *borderValue);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_fillConvexPoly_const__InputOutputArrayR_const__InputArrayR_const_ScalarR_int_int(const cv::_InputOutputArray* img, const cv::_InputArray* points, const cv::Scalar* color, int lineType, int shift, Result_void* ocvrs_return) {
		try {
			cv::fillConvexPoly(*img, *points, *color, lineType, shift);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_fillPoly_const__InputOutputArrayR_const__InputArrayR_const_ScalarR_int_int_Point(const cv::_InputOutputArray* img, const cv::_InputArray* pts, const cv::Scalar* color, int lineType, int shift, cv::Point* offset, Result_void* ocvrs_return) {
		try {
			cv::fillPoly(*img, *pts, *color, lineType, shift, *offset);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_filter2D_const__InputArrayR_const__OutputArrayR_int_const__InputArrayR_Point_double_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int ddepth, const cv::_InputArray* kernel, cv::Point* anchor, double delta, int borderType, Result_void* ocvrs_return) {
		try {
			cv::filter2D(*src, *dst, ddepth, *kernel, *anchor, delta, borderType);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_findContours_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_int_int_Point(const cv::_InputArray* image, const cv::_OutputArray* contours, const cv::_OutputArray* hierarchy, int mode, int method, cv::Point* offset, Result_void* ocvrs_return) {
		try {
			cv::findContours(*image, *contours, *hierarchy, mode, method, *offset);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_findContours_const__InputArrayR_const__OutputArrayR_int_int_Point(const cv::_InputArray* image, const cv::_OutputArray* contours, int mode, int method, cv::Point* offset, Result_void* ocvrs_return) {
		try {
			cv::findContours(*image, *contours, mode, method, *offset);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_fitEllipseAMS_const__InputArrayR(const cv::_InputArray* points, Result<cv::RotatedRect*>* ocvrs_return) {
		try {
			cv::RotatedRect ret = cv::fitEllipseAMS(*points);
			Ok(new cv::RotatedRect(ret), ocvrs_return);
		} OCVRS_CATCH(Result<cv::RotatedRect*>, ocvrs_return);
	}
	
	void cv_fitEllipseDirect_const__InputArrayR(const cv::_InputArray* points, Result<cv::RotatedRect*>* ocvrs_return) {
		try {
			cv::RotatedRect ret = cv::fitEllipseDirect(*points);
			Ok(new cv::RotatedRect(ret), ocvrs_return);
		} OCVRS_CATCH(Result<cv::RotatedRect*>, ocvrs_return);
	}
	
	void cv_fitEllipse_const__InputArrayR(const cv::_InputArray* points, Result<cv::RotatedRect*>* ocvrs_return) {
		try {
			cv::RotatedRect ret = cv::fitEllipse(*points);
			Ok(new cv::RotatedRect(ret), ocvrs_return);
		} OCVRS_CATCH(Result<cv::RotatedRect*>, ocvrs_return);
	}
	
	void cv_fitLine_const__InputArrayR_const__OutputArrayR_int_double_double_double(const cv::_InputArray* points, const cv::_OutputArray* line, int distType, double param, double reps, double aeps, Result_void* ocvrs_return) {
		try {
			cv::fitLine(*points, *line, distType, param, reps, aeps);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_floodFill_const__InputOutputArrayR_Point_Scalar_RectX_Scalar_Scalar_int(const cv::_InputOutputArray* image, cv::Point* seedPoint, cv::Scalar* newVal, cv::Rect* rect, cv::Scalar* loDiff, cv::Scalar* upDiff, int flags, Result<int>* ocvrs_return) {
		try {
			int ret = cv::floodFill(*image, *seedPoint, *newVal, rect, *loDiff, *upDiff, flags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<int>, ocvrs_return);
	}
	
	void cv_floodFill_const__InputOutputArrayR_const__InputOutputArrayR_Point_Scalar_RectX_Scalar_Scalar_int(const cv::_InputOutputArray* image, const cv::_InputOutputArray* mask, cv::Point* seedPoint, cv::Scalar* newVal, cv::Rect* rect, cv::Scalar* loDiff, cv::Scalar* upDiff, int flags, Result<int>* ocvrs_return) {
		try {
			int ret = cv::floodFill(*image, *mask, *seedPoint, *newVal, rect, *loDiff, *upDiff, flags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<int>, ocvrs_return);
	}
	
	void cv_getAffineTransform_const_Point2fX_const_Point2fX(const cv::Point2f* src, const cv::Point2f* dst, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::getAffineTransform(src, dst);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(Result<cv::Mat*>, ocvrs_return);
	}
	
	void cv_getAffineTransform_const__InputArrayR_const__InputArrayR(const cv::_InputArray* src, const cv::_InputArray* dst, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::getAffineTransform(*src, *dst);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(Result<cv::Mat*>, ocvrs_return);
	}
	
	void cv_getDerivKernels_const__OutputArrayR_const__OutputArrayR_int_int_int_bool_int(const cv::_OutputArray* kx, const cv::_OutputArray* ky, int dx, int dy, int ksize, bool normalize, int ktype, Result_void* ocvrs_return) {
		try {
			cv::getDerivKernels(*kx, *ky, dx, dy, ksize, normalize, ktype);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_getFontScaleFromHeight_const_int_const_int_const_int(const int fontFace, const int pixelHeight, const int thickness, Result<double>* ocvrs_return) {
		try {
			double ret = cv::getFontScaleFromHeight(fontFace, pixelHeight, thickness);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<double>, ocvrs_return);
	}
	
	void cv_getGaborKernel_Size_double_double_double_double_double_int(cv::Size* ksize, double sigma, double theta, double lambd, double gamma, double psi, int ktype, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::getGaborKernel(*ksize, sigma, theta, lambd, gamma, psi, ktype);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(Result<cv::Mat*>, ocvrs_return);
	}
	
	void cv_getGaussianKernel_int_double_int(int ksize, double sigma, int ktype, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::getGaussianKernel(ksize, sigma, ktype);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(Result<cv::Mat*>, ocvrs_return);
	}
	
	void cv_getPerspectiveTransform_const_Point2fX_const_Point2fX_int(const cv::Point2f* src, const cv::Point2f* dst, int solveMethod, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::getPerspectiveTransform(src, dst, solveMethod);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(Result<cv::Mat*>, ocvrs_return);
	}
	
	void cv_getPerspectiveTransform_const__InputArrayR_const__InputArrayR_int(const cv::_InputArray* src, const cv::_InputArray* dst, int solveMethod, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::getPerspectiveTransform(*src, *dst, solveMethod);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(Result<cv::Mat*>, ocvrs_return);
	}
	
	void cv_getRectSubPix_const__InputArrayR_Size_Point2f_const__OutputArrayR_int(const cv::_InputArray* image, cv::Size* patchSize, cv::Point2f* center, const cv::_OutputArray* patch, int patchType, Result_void* ocvrs_return) {
		try {
			cv::getRectSubPix(*image, *patchSize, *center, *patch, patchType);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_getRotationMatrix2D_Point2f_double_double(cv::Point2f* center, double angle, double scale, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::getRotationMatrix2D(*center, angle, scale);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(Result<cv::Mat*>, ocvrs_return);
	}
	
	#if !defined(OCVRS_TARGET_OS_WINDOWS)
	void cv_getRotationMatrix2D__Point2f_double_double(cv::Point2f* center, double angle, double scale, Result<cv::Matx23d>* ocvrs_return) {
		try {
			cv::Matx23d ret = cv::getRotationMatrix2D_(*center, angle, scale);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<cv::Matx23d>, ocvrs_return);
	}
	#endif
	
	void cv_getStructuringElement_int_Size_Point(int shape, cv::Size* ksize, cv::Point* anchor, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::getStructuringElement(shape, *ksize, *anchor);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(Result<cv::Mat*>, ocvrs_return);
	}
	
	void cv_getTextSize_const_StringR_int_double_int_intX(const char* text, int fontFace, double fontScale, int thickness, int* baseLine, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = cv::getTextSize(std::string(text), fontFace, fontScale, thickness, baseLine);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<cv::Size>, ocvrs_return);
	}
	
	void cv_goodFeaturesToTrack_const__InputArrayR_const__OutputArrayR_int_double_double_const__InputArrayR_const__OutputArrayR_int_int_bool_double(const cv::_InputArray* image, const cv::_OutputArray* corners, int maxCorners, double qualityLevel, double minDistance, const cv::_InputArray* mask, const cv::_OutputArray* cornersQuality, int blockSize, int gradientSize, bool useHarrisDetector, double k, Result_void* ocvrs_return) {
		try {
			cv::goodFeaturesToTrack(*image, *corners, maxCorners, qualityLevel, minDistance, *mask, *cornersQuality, blockSize, gradientSize, useHarrisDetector, k);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_goodFeaturesToTrack_const__InputArrayR_const__OutputArrayR_int_double_double_const__InputArrayR_int_bool_double(const cv::_InputArray* image, const cv::_OutputArray* corners, int maxCorners, double qualityLevel, double minDistance, const cv::_InputArray* mask, int blockSize, bool useHarrisDetector, double k, Result_void* ocvrs_return) {
		try {
			cv::goodFeaturesToTrack(*image, *corners, maxCorners, qualityLevel, minDistance, *mask, blockSize, useHarrisDetector, k);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_goodFeaturesToTrack_const__InputArrayR_const__OutputArrayR_int_double_double_const__InputArrayR_int_int_bool_double(const cv::_InputArray* image, const cv::_OutputArray* corners, int maxCorners, double qualityLevel, double minDistance, const cv::_InputArray* mask, int blockSize, int gradientSize, bool useHarrisDetector, double k, Result_void* ocvrs_return) {
		try {
			cv::goodFeaturesToTrack(*image, *corners, maxCorners, qualityLevel, minDistance, *mask, blockSize, gradientSize, useHarrisDetector, k);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_grabCut_const__InputArrayR_const__InputOutputArrayR_Rect_const__InputOutputArrayR_const__InputOutputArrayR_int_int(const cv::_InputArray* img, const cv::_InputOutputArray* mask, cv::Rect* rect, const cv::_InputOutputArray* bgdModel, const cv::_InputOutputArray* fgdModel, int iterCount, int mode, Result_void* ocvrs_return) {
		try {
			cv::grabCut(*img, *mask, *rect, *bgdModel, *fgdModel, iterCount, mode);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_integral_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_int(const cv::_InputArray* src, const cv::_OutputArray* sum, const cv::_OutputArray* sqsum, const cv::_OutputArray* tilted, int sdepth, int sqdepth, Result_void* ocvrs_return) {
		try {
			cv::integral(*src, *sum, *sqsum, *tilted, sdepth, sqdepth);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_integral_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_int_int(const cv::_InputArray* src, const cv::_OutputArray* sum, const cv::_OutputArray* sqsum, int sdepth, int sqdepth, Result_void* ocvrs_return) {
		try {
			cv::integral(*src, *sum, *sqsum, sdepth, sqdepth);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_integral_const__InputArrayR_const__OutputArrayR_int(const cv::_InputArray* src, const cv::_OutputArray* sum, int sdepth, Result_void* ocvrs_return) {
		try {
			cv::integral(*src, *sum, sdepth);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_intersectConvexConvex_const__InputArrayR_const__InputArrayR_const__OutputArrayR_bool(const cv::_InputArray* p1, const cv::_InputArray* p2, const cv::_OutputArray* p12, bool handleNested, Result<float>* ocvrs_return) {
		try {
			float ret = cv::intersectConvexConvex(*p1, *p2, *p12, handleNested);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<float>, ocvrs_return);
	}
	
	void cv_invertAffineTransform_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* M, const cv::_OutputArray* iM, Result_void* ocvrs_return) {
		try {
			cv::invertAffineTransform(*M, *iM);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_isContourConvex_const__InputArrayR(const cv::_InputArray* contour, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::isContourConvex(*contour);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<bool>, ocvrs_return);
	}
	
	void cv_line_const__InputOutputArrayR_Point_Point_const_ScalarR_int_int_int(const cv::_InputOutputArray* img, cv::Point* pt1, cv::Point* pt2, const cv::Scalar* color, int thickness, int lineType, int shift, Result_void* ocvrs_return) {
		try {
			cv::line(*img, *pt1, *pt2, *color, thickness, lineType, shift);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_linearPolar_const__InputArrayR_const__OutputArrayR_Point2f_double_int(const cv::_InputArray* src, const cv::_OutputArray* dst, cv::Point2f* center, double maxRadius, int flags, Result_void* ocvrs_return) {
		try {
			cv::linearPolar(*src, *dst, *center, maxRadius, flags);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_logPolar_const__InputArrayR_const__OutputArrayR_Point2f_double_int(const cv::_InputArray* src, const cv::_OutputArray* dst, cv::Point2f* center, double M, int flags, Result_void* ocvrs_return) {
		try {
			cv::logPolar(*src, *dst, *center, M, flags);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_matchShapes_const__InputArrayR_const__InputArrayR_int_double(const cv::_InputArray* contour1, const cv::_InputArray* contour2, int method, double parameter, Result<double>* ocvrs_return) {
		try {
			double ret = cv::matchShapes(*contour1, *contour2, method, parameter);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<double>, ocvrs_return);
	}
	
	void cv_matchTemplate_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_const__InputArrayR(const cv::_InputArray* image, const cv::_InputArray* templ, const cv::_OutputArray* result, int method, const cv::_InputArray* mask, Result_void* ocvrs_return) {
		try {
			cv::matchTemplate(*image, *templ, *result, method, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_medianBlur_const__InputArrayR_const__OutputArrayR_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int ksize, Result_void* ocvrs_return) {
		try {
			cv::medianBlur(*src, *dst, ksize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_minAreaRect_const__InputArrayR(const cv::_InputArray* points, Result<cv::RotatedRect*>* ocvrs_return) {
		try {
			cv::RotatedRect ret = cv::minAreaRect(*points);
			Ok(new cv::RotatedRect(ret), ocvrs_return);
		} OCVRS_CATCH(Result<cv::RotatedRect*>, ocvrs_return);
	}
	
	void cv_minEnclosingCircle_const__InputArrayR_Point2fR_floatR(const cv::_InputArray* points, cv::Point2f* center, float* radius, Result_void* ocvrs_return) {
		try {
			cv::minEnclosingCircle(*points, *center, *radius);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_minEnclosingTriangle_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* points, const cv::_OutputArray* triangle, Result<double>* ocvrs_return) {
		try {
			double ret = cv::minEnclosingTriangle(*points, *triangle);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<double>, ocvrs_return);
	}
	
	void cv_moments_const__InputArrayR_bool(const cv::_InputArray* array, bool binaryImage, Result<cv::Moments>* ocvrs_return) {
		try {
			cv::Moments ret = cv::moments(*array, binaryImage);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<cv::Moments>, ocvrs_return);
	}
	
	void cv_morphologyDefaultBorderValue(Result<cv::Scalar>* ocvrs_return) {
		try {
			cv::Scalar ret = cv::morphologyDefaultBorderValue();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<cv::Scalar>, ocvrs_return);
	}
	
	void cv_morphologyEx_const__InputArrayR_const__OutputArrayR_int_const__InputArrayR_Point_int_int_const_ScalarR(const cv::_InputArray* src, const cv::_OutputArray* dst, int op, const cv::_InputArray* kernel, cv::Point* anchor, int iterations, int borderType, const cv::Scalar* borderValue, Result_void* ocvrs_return) {
		try {
			cv::morphologyEx(*src, *dst, op, *kernel, *anchor, iterations, borderType, *borderValue);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_phaseCorrelate_const__InputArrayR_const__InputArrayR_const__InputArrayR_doubleX(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_InputArray* window, double* response, Result<cv::Point2d>* ocvrs_return) {
		try {
			cv::Point2d ret = cv::phaseCorrelate(*src1, *src2, *window, response);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<cv::Point2d>, ocvrs_return);
	}
	
	void cv_pointPolygonTest_const__InputArrayR_Point2f_bool(const cv::_InputArray* contour, cv::Point2f* pt, bool measureDist, Result<double>* ocvrs_return) {
		try {
			double ret = cv::pointPolygonTest(*contour, *pt, measureDist);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<double>, ocvrs_return);
	}
	
	void cv_polylines_const__InputOutputArrayR_const__InputArrayR_bool_const_ScalarR_int_int_int(const cv::_InputOutputArray* img, const cv::_InputArray* pts, bool isClosed, const cv::Scalar* color, int thickness, int lineType, int shift, Result_void* ocvrs_return) {
		try {
			cv::polylines(*img, *pts, isClosed, *color, thickness, lineType, shift);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_preCornerDetect_const__InputArrayR_const__OutputArrayR_int_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int ksize, int borderType, Result_void* ocvrs_return) {
		try {
			cv::preCornerDetect(*src, *dst, ksize, borderType);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_putText_const__InputOutputArrayR_const_StringR_Point_int_double_Scalar_int_int_bool(const cv::_InputOutputArray* img, const char* text, cv::Point* org, int fontFace, double fontScale, cv::Scalar* color, int thickness, int lineType, bool bottomLeftOrigin, Result_void* ocvrs_return) {
		try {
			cv::putText(*img, std::string(text), *org, fontFace, fontScale, *color, thickness, lineType, bottomLeftOrigin);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_pyrDown_const__InputArrayR_const__OutputArrayR_const_SizeR_int(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::Size* dstsize, int borderType, Result_void* ocvrs_return) {
		try {
			cv::pyrDown(*src, *dst, *dstsize, borderType);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_pyrMeanShiftFiltering_const__InputArrayR_const__OutputArrayR_double_double_int_TermCriteria(const cv::_InputArray* src, const cv::_OutputArray* dst, double sp, double sr, int maxLevel, cv::TermCriteria* termcrit, Result_void* ocvrs_return) {
		try {
			cv::pyrMeanShiftFiltering(*src, *dst, sp, sr, maxLevel, *termcrit);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_pyrUp_const__InputArrayR_const__OutputArrayR_const_SizeR_int(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::Size* dstsize, int borderType, Result_void* ocvrs_return) {
		try {
			cv::pyrUp(*src, *dst, *dstsize, borderType);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_rectangle_const__InputOutputArrayR_Point_Point_const_ScalarR_int_int_int(const cv::_InputOutputArray* img, cv::Point* pt1, cv::Point* pt2, const cv::Scalar* color, int thickness, int lineType, int shift, Result_void* ocvrs_return) {
		try {
			cv::rectangle(*img, *pt1, *pt2, *color, thickness, lineType, shift);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_rectangle_const__InputOutputArrayR_Rect_const_ScalarR_int_int_int(const cv::_InputOutputArray* img, cv::Rect* rec, const cv::Scalar* color, int thickness, int lineType, int shift, Result_void* ocvrs_return) {
		try {
			cv::rectangle(*img, *rec, *color, thickness, lineType, shift);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_remap_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_int_int_const_ScalarR(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* map1, const cv::_InputArray* map2, int interpolation, int borderMode, const cv::Scalar* borderValue, Result_void* ocvrs_return) {
		try {
			cv::remap(*src, *dst, *map1, *map2, interpolation, borderMode, *borderValue);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_resize_const__InputArrayR_const__OutputArrayR_Size_double_double_int(const cv::_InputArray* src, const cv::_OutputArray* dst, cv::Size* dsize, double fx, double fy, int interpolation, Result_void* ocvrs_return) {
		try {
			cv::resize(*src, *dst, *dsize, fx, fy, interpolation);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_rotatedRectangleIntersection_const_RotatedRectR_const_RotatedRectR_const__OutputArrayR(const cv::RotatedRect* rect1, const cv::RotatedRect* rect2, const cv::_OutputArray* intersectingRegion, Result<int>* ocvrs_return) {
		try {
			int ret = cv::rotatedRectangleIntersection(*rect1, *rect2, *intersectingRegion);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<int>, ocvrs_return);
	}
	
	void cv_sepFilter2D_const__InputArrayR_const__OutputArrayR_int_const__InputArrayR_const__InputArrayR_Point_double_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int ddepth, const cv::_InputArray* kernelX, const cv::_InputArray* kernelY, cv::Point* anchor, double delta, int borderType, Result_void* ocvrs_return) {
		try {
			cv::sepFilter2D(*src, *dst, ddepth, *kernelX, *kernelY, *anchor, delta, borderType);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_spatialGradient_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_int_int(const cv::_InputArray* src, const cv::_OutputArray* dx, const cv::_OutputArray* dy, int ksize, int borderType, Result_void* ocvrs_return) {
		try {
			cv::spatialGradient(*src, *dx, *dy, ksize, borderType);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_sqrBoxFilter_const__InputArrayR_const__OutputArrayR_int_Size_Point_bool_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int ddepth, cv::Size* ksize, cv::Point* anchor, bool normalize, int borderType, Result_void* ocvrs_return) {
		try {
			cv::sqrBoxFilter(*src, *dst, ddepth, *ksize, *anchor, normalize, borderType);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_threshold_const__InputArrayR_const__OutputArrayR_double_double_int(const cv::_InputArray* src, const cv::_OutputArray* dst, double thresh, double maxval, int type, Result<double>* ocvrs_return) {
		try {
			double ret = cv::threshold(*src, *dst, thresh, maxval, type);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<double>, ocvrs_return);
	}
	
	void cv_warpAffine_const__InputArrayR_const__OutputArrayR_const__InputArrayR_Size_int_int_const_ScalarR(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* M, cv::Size* dsize, int flags, int borderMode, const cv::Scalar* borderValue, Result_void* ocvrs_return) {
		try {
			cv::warpAffine(*src, *dst, *M, *dsize, flags, borderMode, *borderValue);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_warpPerspective_const__InputArrayR_const__OutputArrayR_const__InputArrayR_Size_int_int_const_ScalarR(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* M, cv::Size* dsize, int flags, int borderMode, const cv::Scalar* borderValue, Result_void* ocvrs_return) {
		try {
			cv::warpPerspective(*src, *dst, *M, *dsize, flags, borderMode, *borderValue);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_warpPolar_const__InputArrayR_const__OutputArrayR_Size_Point2f_double_int(const cv::_InputArray* src, const cv::_OutputArray* dst, cv::Size* dsize, cv::Point2f* center, double maxRadius, int flags, Result_void* ocvrs_return) {
		try {
			cv::warpPolar(*src, *dst, *dsize, *center, maxRadius, flags);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_watershed_const__InputArrayR_const__InputOutputArrayR(const cv::_InputArray* image, const cv::_InputOutputArray* markers, Result_void* ocvrs_return) {
		try {
			cv::watershed(*image, *markers);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_wrapperEMD_const__InputArrayR_const__InputArrayR_int_const__InputArrayR_PtrLfloatG_const__OutputArrayR(const cv::_InputArray* signature1, const cv::_InputArray* signature2, int distType, const cv::_InputArray* cost, cv::Ptr<float>* lowerBound, const cv::_OutputArray* flow, Result<float>* ocvrs_return) {
		try {
			float ret = cv::wrapperEMD(*signature1, *signature2, distType, *cost, *lowerBound, *flow);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<float>, ocvrs_return);
	}
	
	void cv_CLAHE_apply_const__InputArrayR_const__OutputArrayR(cv::CLAHE* instance, const cv::_InputArray* src, const cv::_OutputArray* dst, Result_void* ocvrs_return) {
		try {
			instance->apply(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_CLAHE_setClipLimit_double(cv::CLAHE* instance, double clipLimit, Result_void* ocvrs_return) {
		try {
			instance->setClipLimit(clipLimit);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_CLAHE_getClipLimit_const(const cv::CLAHE* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getClipLimit();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<double>, ocvrs_return);
	}
	
	void cv_CLAHE_setTilesGridSize_Size(cv::CLAHE* instance, cv::Size* tileGridSize, Result_void* ocvrs_return) {
		try {
			instance->setTilesGridSize(*tileGridSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_CLAHE_getTilesGridSize_const(const cv::CLAHE* instance, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->getTilesGridSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<cv::Size>, ocvrs_return);
	}
	
	void cv_CLAHE_collectGarbage(cv::CLAHE* instance, Result_void* ocvrs_return) {
		try {
			instance->collectGarbage();
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_GeneralizedHough_setTemplate_const__InputArrayR_Point(cv::GeneralizedHough* instance, const cv::_InputArray* templ, cv::Point* templCenter, Result_void* ocvrs_return) {
		try {
			instance->setTemplate(*templ, *templCenter);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_GeneralizedHough_setTemplate_const__InputArrayR_const__InputArrayR_const__InputArrayR_Point(cv::GeneralizedHough* instance, const cv::_InputArray* edges, const cv::_InputArray* dx, const cv::_InputArray* dy, cv::Point* templCenter, Result_void* ocvrs_return) {
		try {
			instance->setTemplate(*edges, *dx, *dy, *templCenter);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_GeneralizedHough_detect_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(cv::GeneralizedHough* instance, const cv::_InputArray* image, const cv::_OutputArray* positions, const cv::_OutputArray* votes, Result_void* ocvrs_return) {
		try {
			instance->detect(*image, *positions, *votes);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_GeneralizedHough_detect_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(cv::GeneralizedHough* instance, const cv::_InputArray* edges, const cv::_InputArray* dx, const cv::_InputArray* dy, const cv::_OutputArray* positions, const cv::_OutputArray* votes, Result_void* ocvrs_return) {
		try {
			instance->detect(*edges, *dx, *dy, *positions, *votes);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_GeneralizedHough_setCannyLowThresh_int(cv::GeneralizedHough* instance, int cannyLowThresh, Result_void* ocvrs_return) {
		try {
			instance->setCannyLowThresh(cannyLowThresh);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_GeneralizedHough_getCannyLowThresh_const(const cv::GeneralizedHough* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getCannyLowThresh();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<int>, ocvrs_return);
	}
	
	void cv_GeneralizedHough_setCannyHighThresh_int(cv::GeneralizedHough* instance, int cannyHighThresh, Result_void* ocvrs_return) {
		try {
			instance->setCannyHighThresh(cannyHighThresh);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_GeneralizedHough_getCannyHighThresh_const(const cv::GeneralizedHough* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getCannyHighThresh();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<int>, ocvrs_return);
	}
	
	void cv_GeneralizedHough_setMinDist_double(cv::GeneralizedHough* instance, double minDist, Result_void* ocvrs_return) {
		try {
			instance->setMinDist(minDist);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_GeneralizedHough_getMinDist_const(const cv::GeneralizedHough* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMinDist();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<double>, ocvrs_return);
	}
	
	void cv_GeneralizedHough_setDp_double(cv::GeneralizedHough* instance, double dp, Result_void* ocvrs_return) {
		try {
			instance->setDp(dp);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_GeneralizedHough_getDp_const(const cv::GeneralizedHough* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getDp();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<double>, ocvrs_return);
	}
	
	void cv_GeneralizedHough_setMaxBufferSize_int(cv::GeneralizedHough* instance, int maxBufferSize, Result_void* ocvrs_return) {
		try {
			instance->setMaxBufferSize(maxBufferSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_GeneralizedHough_getMaxBufferSize_const(const cv::GeneralizedHough* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMaxBufferSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<int>, ocvrs_return);
	}
	
	void cv_GeneralizedHoughBallard_setLevels_int(cv::GeneralizedHoughBallard* instance, int levels, Result_void* ocvrs_return) {
		try {
			instance->setLevels(levels);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_GeneralizedHoughBallard_getLevels_const(const cv::GeneralizedHoughBallard* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getLevels();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<int>, ocvrs_return);
	}
	
	void cv_GeneralizedHoughBallard_setVotesThreshold_int(cv::GeneralizedHoughBallard* instance, int votesThreshold, Result_void* ocvrs_return) {
		try {
			instance->setVotesThreshold(votesThreshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_GeneralizedHoughBallard_getVotesThreshold_const(const cv::GeneralizedHoughBallard* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getVotesThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<int>, ocvrs_return);
	}
	
	void cv_GeneralizedHoughGuil_setXi_double(cv::GeneralizedHoughGuil* instance, double xi, Result_void* ocvrs_return) {
		try {
			instance->setXi(xi);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_GeneralizedHoughGuil_getXi_const(const cv::GeneralizedHoughGuil* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getXi();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<double>, ocvrs_return);
	}
	
	void cv_GeneralizedHoughGuil_setLevels_int(cv::GeneralizedHoughGuil* instance, int levels, Result_void* ocvrs_return) {
		try {
			instance->setLevels(levels);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_GeneralizedHoughGuil_getLevels_const(const cv::GeneralizedHoughGuil* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getLevels();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<int>, ocvrs_return);
	}
	
	void cv_GeneralizedHoughGuil_setAngleEpsilon_double(cv::GeneralizedHoughGuil* instance, double angleEpsilon, Result_void* ocvrs_return) {
		try {
			instance->setAngleEpsilon(angleEpsilon);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_GeneralizedHoughGuil_getAngleEpsilon_const(const cv::GeneralizedHoughGuil* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getAngleEpsilon();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<double>, ocvrs_return);
	}
	
	void cv_GeneralizedHoughGuil_setMinAngle_double(cv::GeneralizedHoughGuil* instance, double minAngle, Result_void* ocvrs_return) {
		try {
			instance->setMinAngle(minAngle);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_GeneralizedHoughGuil_getMinAngle_const(const cv::GeneralizedHoughGuil* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMinAngle();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<double>, ocvrs_return);
	}
	
	void cv_GeneralizedHoughGuil_setMaxAngle_double(cv::GeneralizedHoughGuil* instance, double maxAngle, Result_void* ocvrs_return) {
		try {
			instance->setMaxAngle(maxAngle);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_GeneralizedHoughGuil_getMaxAngle_const(const cv::GeneralizedHoughGuil* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMaxAngle();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<double>, ocvrs_return);
	}
	
	void cv_GeneralizedHoughGuil_setAngleStep_double(cv::GeneralizedHoughGuil* instance, double angleStep, Result_void* ocvrs_return) {
		try {
			instance->setAngleStep(angleStep);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_GeneralizedHoughGuil_getAngleStep_const(const cv::GeneralizedHoughGuil* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getAngleStep();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<double>, ocvrs_return);
	}
	
	void cv_GeneralizedHoughGuil_setAngleThresh_int(cv::GeneralizedHoughGuil* instance, int angleThresh, Result_void* ocvrs_return) {
		try {
			instance->setAngleThresh(angleThresh);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_GeneralizedHoughGuil_getAngleThresh_const(const cv::GeneralizedHoughGuil* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getAngleThresh();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<int>, ocvrs_return);
	}
	
	void cv_GeneralizedHoughGuil_setMinScale_double(cv::GeneralizedHoughGuil* instance, double minScale, Result_void* ocvrs_return) {
		try {
			instance->setMinScale(minScale);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_GeneralizedHoughGuil_getMinScale_const(const cv::GeneralizedHoughGuil* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMinScale();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<double>, ocvrs_return);
	}
	
	void cv_GeneralizedHoughGuil_setMaxScale_double(cv::GeneralizedHoughGuil* instance, double maxScale, Result_void* ocvrs_return) {
		try {
			instance->setMaxScale(maxScale);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_GeneralizedHoughGuil_getMaxScale_const(const cv::GeneralizedHoughGuil* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMaxScale();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<double>, ocvrs_return);
	}
	
	void cv_GeneralizedHoughGuil_setScaleStep_double(cv::GeneralizedHoughGuil* instance, double scaleStep, Result_void* ocvrs_return) {
		try {
			instance->setScaleStep(scaleStep);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_GeneralizedHoughGuil_getScaleStep_const(const cv::GeneralizedHoughGuil* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getScaleStep();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<double>, ocvrs_return);
	}
	
	void cv_GeneralizedHoughGuil_setScaleThresh_int(cv::GeneralizedHoughGuil* instance, int scaleThresh, Result_void* ocvrs_return) {
		try {
			instance->setScaleThresh(scaleThresh);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_GeneralizedHoughGuil_getScaleThresh_const(const cv::GeneralizedHoughGuil* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getScaleThresh();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<int>, ocvrs_return);
	}
	
	void cv_GeneralizedHoughGuil_setPosThresh_int(cv::GeneralizedHoughGuil* instance, int posThresh, Result_void* ocvrs_return) {
		try {
			instance->setPosThresh(posThresh);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_GeneralizedHoughGuil_getPosThresh_const(const cv::GeneralizedHoughGuil* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getPosThresh();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<int>, ocvrs_return);
	}
	
	unsigned char* cv_LineIterator_getPropPtr(cv::LineIterator* instance) {
			unsigned char* ret = instance->ptr;
			return ret;
	}
	
	void cv_LineIterator_setPropPtr_unsigned_charX(cv::LineIterator* instance, unsigned char* val) {
			instance->ptr = val;
	}
	
	const unsigned char* cv_LineIterator_getPropPtr0_const(const cv::LineIterator* instance) {
			const unsigned char* ret = instance->ptr0;
			return ret;
	}
	
	int cv_LineIterator_getPropStep_const(const cv::LineIterator* instance) {
			int ret = instance->step;
			return ret;
	}
	
	void cv_LineIterator_setPropStep_int(cv::LineIterator* instance, int val) {
			instance->step = val;
	}
	
	int cv_LineIterator_getPropElemSize_const(const cv::LineIterator* instance) {
			int ret = instance->elemSize;
			return ret;
	}
	
	void cv_LineIterator_setPropElemSize_int(cv::LineIterator* instance, int val) {
			instance->elemSize = val;
	}
	
	int cv_LineIterator_getPropErr_const(const cv::LineIterator* instance) {
			int ret = instance->err;
			return ret;
	}
	
	void cv_LineIterator_setPropErr_int(cv::LineIterator* instance, int val) {
			instance->err = val;
	}
	
	int cv_LineIterator_getPropCount_const(const cv::LineIterator* instance) {
			int ret = instance->count;
			return ret;
	}
	
	void cv_LineIterator_setPropCount_int(cv::LineIterator* instance, int val) {
			instance->count = val;
	}
	
	int cv_LineIterator_getPropMinusDelta_const(const cv::LineIterator* instance) {
			int ret = instance->minusDelta;
			return ret;
	}
	
	void cv_LineIterator_setPropMinusDelta_int(cv::LineIterator* instance, int val) {
			instance->minusDelta = val;
	}
	
	int cv_LineIterator_getPropPlusDelta_const(const cv::LineIterator* instance) {
			int ret = instance->plusDelta;
			return ret;
	}
	
	void cv_LineIterator_setPropPlusDelta_int(cv::LineIterator* instance, int val) {
			instance->plusDelta = val;
	}
	
	int cv_LineIterator_getPropMinusStep_const(const cv::LineIterator* instance) {
			int ret = instance->minusStep;
			return ret;
	}
	
	void cv_LineIterator_setPropMinusStep_int(cv::LineIterator* instance, int val) {
			instance->minusStep = val;
	}
	
	int cv_LineIterator_getPropPlusStep_const(const cv::LineIterator* instance) {
			int ret = instance->plusStep;
			return ret;
	}
	
	void cv_LineIterator_setPropPlusStep_int(cv::LineIterator* instance, int val) {
			instance->plusStep = val;
	}
	
	int cv_LineIterator_getPropMinusShift_const(const cv::LineIterator* instance) {
			int ret = instance->minusShift;
			return ret;
	}
	
	void cv_LineIterator_setPropMinusShift_int(cv::LineIterator* instance, int val) {
			instance->minusShift = val;
	}
	
	int cv_LineIterator_getPropPlusShift_const(const cv::LineIterator* instance) {
			int ret = instance->plusShift;
			return ret;
	}
	
	void cv_LineIterator_setPropPlusShift_int(cv::LineIterator* instance, int val) {
			instance->plusShift = val;
	}
	
	void cv_LineIterator_getPropP_const(const cv::LineIterator* instance, cv::Point* ocvrs_return) {
			cv::Point ret = instance->p;
			*ocvrs_return = ret;
	}
	
	void cv_LineIterator_setPropP_Point(cv::LineIterator* instance, cv::Point* val) {
			instance->p = *val;
	}
	
	bool cv_LineIterator_getPropPtmode_const(const cv::LineIterator* instance) {
			bool ret = instance->ptmode;
			return ret;
	}
	
	void cv_LineIterator_setPropPtmode_bool(cv::LineIterator* instance, bool val) {
			instance->ptmode = val;
	}
	
	void cv_LineIterator_delete(cv::LineIterator* instance) {
			delete instance;
	}
	
	void cv_LineIterator_LineIterator_const_MatR_Point_Point_int_bool(const cv::Mat* img, cv::Point* pt1, cv::Point* pt2, int connectivity, bool leftToRight, Result<cv::LineIterator*>* ocvrs_return) {
		try {
			cv::LineIterator* ret = new cv::LineIterator(*img, *pt1, *pt2, connectivity, leftToRight);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<cv::LineIterator*>, ocvrs_return);
	}
	
	void cv_LineIterator_LineIterator_Point_Point_int_bool(cv::Point* pt1, cv::Point* pt2, int connectivity, bool leftToRight, Result<cv::LineIterator*>* ocvrs_return) {
		try {
			cv::LineIterator* ret = new cv::LineIterator(*pt1, *pt2, connectivity, leftToRight);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<cv::LineIterator*>, ocvrs_return);
	}
	
	void cv_LineIterator_LineIterator_Size_Point_Point_int_bool(cv::Size* boundingAreaSize, cv::Point* pt1, cv::Point* pt2, int connectivity, bool leftToRight, Result<cv::LineIterator*>* ocvrs_return) {
		try {
			cv::LineIterator* ret = new cv::LineIterator(*boundingAreaSize, *pt1, *pt2, connectivity, leftToRight);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<cv::LineIterator*>, ocvrs_return);
	}
	
	void cv_LineIterator_LineIterator_Rect_Point_Point_int_bool(cv::Rect* boundingAreaRect, cv::Point* pt1, cv::Point* pt2, int connectivity, bool leftToRight, Result<cv::LineIterator*>* ocvrs_return) {
		try {
			cv::LineIterator* ret = new cv::LineIterator(*boundingAreaRect, *pt1, *pt2, connectivity, leftToRight);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<cv::LineIterator*>, ocvrs_return);
	}
	
	void cv_LineIterator_init_const_MatX_Rect_Point_Point_int_bool(cv::LineIterator* instance, const cv::Mat* img, cv::Rect* boundingAreaRect, cv::Point* pt1, cv::Point* pt2, int connectivity, bool leftToRight, Result_void* ocvrs_return) {
		try {
			instance->init(img, *boundingAreaRect, *pt1, *pt2, connectivity, leftToRight);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_LineIterator_operatorX(cv::LineIterator* instance, Result<unsigned char*>* ocvrs_return) {
		try {
			unsigned char* ret = instance->operator*();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<unsigned char*>, ocvrs_return);
	}
	
	void cv_LineIterator_operatorAA(cv::LineIterator* instance, Result<cv::LineIterator*>* ocvrs_return) {
		try {
			cv::LineIterator ret = instance->operator++();
			Ok(new cv::LineIterator(ret), ocvrs_return);
		} OCVRS_CATCH(Result<cv::LineIterator*>, ocvrs_return);
	}
	
	void cv_LineIterator_pos_const(const cv::LineIterator* instance, Result<cv::Point>* ocvrs_return) {
		try {
			cv::Point ret = instance->pos();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<cv::Point>, ocvrs_return);
	}
	
	void cv_LineSegmentDetector_detect_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(cv::LineSegmentDetector* instance, const cv::_InputArray* image, const cv::_OutputArray* lines, const cv::_OutputArray* width, const cv::_OutputArray* prec, const cv::_OutputArray* nfa, Result_void* ocvrs_return) {
		try {
			instance->detect(*image, *lines, *width, *prec, *nfa);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_LineSegmentDetector_drawSegments_const__InputOutputArrayR_const__InputArrayR(cv::LineSegmentDetector* instance, const cv::_InputOutputArray* image, const cv::_InputArray* lines, Result_void* ocvrs_return) {
		try {
			instance->drawSegments(*image, *lines);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_LineSegmentDetector_compareSegments_const_SizeR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR(cv::LineSegmentDetector* instance, const cv::Size* size, const cv::_InputArray* lines1, const cv::_InputArray* lines2, const cv::_InputOutputArray* image, Result<int>* ocvrs_return) {
		try {
			int ret = instance->compareSegments(*size, *lines1, *lines2, *image);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<int>, ocvrs_return);
	}
	
	void cv_Subdiv2D_delete(cv::Subdiv2D* instance) {
			delete instance;
	}
	
	void cv_Subdiv2D_Subdiv2D(Result<cv::Subdiv2D*>* ocvrs_return) {
		try {
			cv::Subdiv2D* ret = new cv::Subdiv2D();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<cv::Subdiv2D*>, ocvrs_return);
	}
	
	void cv_Subdiv2D_Subdiv2D_Rect(cv::Rect* rect, Result<cv::Subdiv2D*>* ocvrs_return) {
		try {
			cv::Subdiv2D* ret = new cv::Subdiv2D(*rect);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<cv::Subdiv2D*>, ocvrs_return);
	}
	
	void cv_Subdiv2D_initDelaunay_Rect(cv::Subdiv2D* instance, cv::Rect* rect, Result_void* ocvrs_return) {
		try {
			instance->initDelaunay(*rect);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_Subdiv2D_insert_Point2f(cv::Subdiv2D* instance, cv::Point2f* pt, Result<int>* ocvrs_return) {
		try {
			int ret = instance->insert(*pt);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<int>, ocvrs_return);
	}
	
	void cv_Subdiv2D_insert_const_vectorLPoint2fGR(cv::Subdiv2D* instance, const std::vector<cv::Point2f>* ptvec, Result_void* ocvrs_return) {
		try {
			instance->insert(*ptvec);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_Subdiv2D_locate_Point2f_intR_intR(cv::Subdiv2D* instance, cv::Point2f* pt, int* edge, int* vertex, Result<int>* ocvrs_return) {
		try {
			int ret = instance->locate(*pt, *edge, *vertex);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<int>, ocvrs_return);
	}
	
	void cv_Subdiv2D_findNearest_Point2f_Point2fX(cv::Subdiv2D* instance, cv::Point2f* pt, cv::Point2f* nearestPt, Result<int>* ocvrs_return) {
		try {
			int ret = instance->findNearest(*pt, nearestPt);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<int>, ocvrs_return);
	}
	
	void cv_Subdiv2D_getEdgeList_const_vectorLVec4fGR(const cv::Subdiv2D* instance, std::vector<cv::Vec4f>* edgeList, Result_void* ocvrs_return) {
		try {
			instance->getEdgeList(*edgeList);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_Subdiv2D_getLeadingEdgeList_const_vectorLintGR(const cv::Subdiv2D* instance, std::vector<int>* leadingEdgeList, Result_void* ocvrs_return) {
		try {
			instance->getLeadingEdgeList(*leadingEdgeList);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_Subdiv2D_getTriangleList_const_vectorLVec6fGR(const cv::Subdiv2D* instance, std::vector<cv::Vec6f>* triangleList, Result_void* ocvrs_return) {
		try {
			instance->getTriangleList(*triangleList);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_Subdiv2D_getVoronoiFacetList_const_vectorLintGR_vectorLvectorLPoint2fGGR_vectorLPoint2fGR(cv::Subdiv2D* instance, const std::vector<int>* idx, std::vector<std::vector<cv::Point2f>>* facetList, std::vector<cv::Point2f>* facetCenters, Result_void* ocvrs_return) {
		try {
			instance->getVoronoiFacetList(*idx, *facetList, *facetCenters);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_Subdiv2D_getVertex_const_int_intX(const cv::Subdiv2D* instance, int vertex, int* firstEdge, Result<cv::Point2f>* ocvrs_return) {
		try {
			cv::Point2f ret = instance->getVertex(vertex, firstEdge);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<cv::Point2f>, ocvrs_return);
	}
	
	void cv_Subdiv2D_getEdge_const_int_int(const cv::Subdiv2D* instance, int edge, int nextEdgeType, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getEdge(edge, nextEdgeType);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<int>, ocvrs_return);
	}
	
	void cv_Subdiv2D_nextEdge_const_int(const cv::Subdiv2D* instance, int edge, Result<int>* ocvrs_return) {
		try {
			int ret = instance->nextEdge(edge);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<int>, ocvrs_return);
	}
	
	void cv_Subdiv2D_rotateEdge_const_int_int(const cv::Subdiv2D* instance, int edge, int rotate, Result<int>* ocvrs_return) {
		try {
			int ret = instance->rotateEdge(edge, rotate);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<int>, ocvrs_return);
	}
	
	void cv_Subdiv2D_symEdge_const_int(const cv::Subdiv2D* instance, int edge, Result<int>* ocvrs_return) {
		try {
			int ret = instance->symEdge(edge);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<int>, ocvrs_return);
	}
	
	void cv_Subdiv2D_edgeOrg_const_int_Point2fX(const cv::Subdiv2D* instance, int edge, cv::Point2f* orgpt, Result<int>* ocvrs_return) {
		try {
			int ret = instance->edgeOrg(edge, orgpt);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<int>, ocvrs_return);
	}
	
	void cv_Subdiv2D_edgeDst_const_int_Point2fX(const cv::Subdiv2D* instance, int edge, cv::Point2f* dstpt, Result<int>* ocvrs_return) {
		try {
			int ret = instance->edgeDst(edge, dstpt);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<int>, ocvrs_return);
	}
	
	void cv_IntelligentScissorsMB_delete(cv::segmentation::IntelligentScissorsMB* instance) {
			delete instance;
	}
	
	void cv_segmentation_IntelligentScissorsMB_IntelligentScissorsMB(Result<cv::segmentation::IntelligentScissorsMB*>* ocvrs_return) {
		try {
			cv::segmentation::IntelligentScissorsMB* ret = new cv::segmentation::IntelligentScissorsMB();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(Result<cv::segmentation::IntelligentScissorsMB*>, ocvrs_return);
	}
	
	void cv_segmentation_IntelligentScissorsMB_setWeights_float_float_float(cv::segmentation::IntelligentScissorsMB* instance, float weight_non_edge, float weight_gradient_direction, float weight_gradient_magnitude, Result<cv::segmentation::IntelligentScissorsMB*>* ocvrs_return) {
		try {
			cv::segmentation::IntelligentScissorsMB ret = instance->setWeights(weight_non_edge, weight_gradient_direction, weight_gradient_magnitude);
			Ok(new cv::segmentation::IntelligentScissorsMB(ret), ocvrs_return);
		} OCVRS_CATCH(Result<cv::segmentation::IntelligentScissorsMB*>, ocvrs_return);
	}
	
	void cv_segmentation_IntelligentScissorsMB_setGradientMagnitudeMaxLimit_float(cv::segmentation::IntelligentScissorsMB* instance, float gradient_magnitude_threshold_max, Result<cv::segmentation::IntelligentScissorsMB*>* ocvrs_return) {
		try {
			cv::segmentation::IntelligentScissorsMB ret = instance->setGradientMagnitudeMaxLimit(gradient_magnitude_threshold_max);
			Ok(new cv::segmentation::IntelligentScissorsMB(ret), ocvrs_return);
		} OCVRS_CATCH(Result<cv::segmentation::IntelligentScissorsMB*>, ocvrs_return);
	}
	
	void cv_segmentation_IntelligentScissorsMB_setEdgeFeatureZeroCrossingParameters_float(cv::segmentation::IntelligentScissorsMB* instance, float gradient_magnitude_min_value, Result<cv::segmentation::IntelligentScissorsMB*>* ocvrs_return) {
		try {
			cv::segmentation::IntelligentScissorsMB ret = instance->setEdgeFeatureZeroCrossingParameters(gradient_magnitude_min_value);
			Ok(new cv::segmentation::IntelligentScissorsMB(ret), ocvrs_return);
		} OCVRS_CATCH(Result<cv::segmentation::IntelligentScissorsMB*>, ocvrs_return);
	}
	
	void cv_segmentation_IntelligentScissorsMB_setEdgeFeatureCannyParameters_double_double_int_bool(cv::segmentation::IntelligentScissorsMB* instance, double threshold1, double threshold2, int apertureSize, bool L2gradient, Result<cv::segmentation::IntelligentScissorsMB*>* ocvrs_return) {
		try {
			cv::segmentation::IntelligentScissorsMB ret = instance->setEdgeFeatureCannyParameters(threshold1, threshold2, apertureSize, L2gradient);
			Ok(new cv::segmentation::IntelligentScissorsMB(ret), ocvrs_return);
		} OCVRS_CATCH(Result<cv::segmentation::IntelligentScissorsMB*>, ocvrs_return);
	}
	
	void cv_segmentation_IntelligentScissorsMB_applyImage_const__InputArrayR(cv::segmentation::IntelligentScissorsMB* instance, const cv::_InputArray* image, Result<cv::segmentation::IntelligentScissorsMB*>* ocvrs_return) {
		try {
			cv::segmentation::IntelligentScissorsMB ret = instance->applyImage(*image);
			Ok(new cv::segmentation::IntelligentScissorsMB(ret), ocvrs_return);
		} OCVRS_CATCH(Result<cv::segmentation::IntelligentScissorsMB*>, ocvrs_return);
	}
	
	void cv_segmentation_IntelligentScissorsMB_applyImageFeatures_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR(cv::segmentation::IntelligentScissorsMB* instance, const cv::_InputArray* non_edge, const cv::_InputArray* gradient_direction, const cv::_InputArray* gradient_magnitude, const cv::_InputArray* image, Result<cv::segmentation::IntelligentScissorsMB*>* ocvrs_return) {
		try {
			cv::segmentation::IntelligentScissorsMB ret = instance->applyImageFeatures(*non_edge, *gradient_direction, *gradient_magnitude, *image);
			Ok(new cv::segmentation::IntelligentScissorsMB(ret), ocvrs_return);
		} OCVRS_CATCH(Result<cv::segmentation::IntelligentScissorsMB*>, ocvrs_return);
	}
	
	void cv_segmentation_IntelligentScissorsMB_buildMap_const_PointR(cv::segmentation::IntelligentScissorsMB* instance, const cv::Point* sourcePt, Result_void* ocvrs_return) {
		try {
			instance->buildMap(*sourcePt);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
	void cv_segmentation_IntelligentScissorsMB_getContour_const_const_PointR_const__OutputArrayR_bool(const cv::segmentation::IntelligentScissorsMB* instance, const cv::Point* targetPt, const cv::_OutputArray* contour, bool backward, Result_void* ocvrs_return) {
		try {
			instance->getContour(*targetPt, *contour, backward);
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void, ocvrs_return);
	}
	
}
