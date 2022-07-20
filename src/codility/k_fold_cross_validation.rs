/// Cross-validation is a technique used to create a more accurate estimate of model prediction performance.
/// The idea is to partition the original sample into a training set (used for learning) and a testing set
/// (used for validation). One type of cross-validation is the k-fold method.
/// In this task you are asked to divide dataset into such folds. There is more than one way to
/// achieve this; however, your partition should fulfill the following conditions:
/// Every element from the sample should be used exactly once in testing sets;
/// Every element from the sample should be used at least once in training sets;
/// The sizes of the pairs of testing sets may differ by no more than 1.
/// Write a function:
/// class Solution { public int[][] solution(int[] indices, int K); }
/// that, given an array of integers indices which contains the indices of elements in a data sample
/// and an integer K which indicates the number of folds to generate, returns an array that contains
/// the arrays of folds (J-th training indices and J-th testing indices are stored respectively as
/// its 2 * J and 2 * J + 1 elements).
/// For example, given [1, 2, 3] and K = 2, one of the possible accepted results may be
/// [[1, 2], [3], [3], [1, 2]] (first fold contains [1, 2] as the training set and [3] as the testing
/// set and second fold contains [3] as the training set and [1, 2] as the testing set).
/// Assume that:
/// N is an integer within the range [2..100]
/// K is an integer within the range [2..N]
/// each element of array indices is an integer within the range [0..100,000];
/// the elements of indices are all distinct.
struct Solution;

impl Solution {}
