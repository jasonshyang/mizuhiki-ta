use crate::core::traits::Numeric;

/// A generic column data structure for storing numeric data.
#[derive(Debug, Clone)]
pub struct Column<T> {
    data: Vec<T>,
}

impl<T: Numeric> Column<T> {
    pub fn new(data: Vec<T>) -> Self {
        Self { data }
    }

    pub fn data(&self) -> &[T] {
        &self.data
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.data.iter_mut()
    }

    pub fn enumerate(&self) -> impl Iterator<Item = (usize, &T)> {
        self.data.iter().enumerate()
    }

    /// Applies a function to each element in-place.
    ///
    /// This method modifies the column data directly, which is more efficient
    /// than creating a new column.
    ///
    /// # Arguments
    /// * `f` - Function to apply to each element
    ///
    /// # Example
    /// ```
    /// use mizuhiki_ta::core::column::Column;
    /// let mut prices = Column::new(vec![100.0, 102.0, 98.0]);
    /// prices.apply(|x| *x *= 2.0);
    /// assert_eq!(prices.data(), vec![200.0, 204.0, 196.0]);
    /// ```
    ///
    pub fn apply<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut T),
    {
        for item in &mut self.data {
            f(item);
        }
    }

    /// Maps the column data to a new type using a function.
    ///
    /// This method creates a new `Column` with transformed values based on the provided function.
    /// It does not modify the original column.
    ///
    /// # Arguments
    /// * `f` - Function to transform each element
    ///
    /// # Example
    /// ```
    /// use mizuhiki_ta::core::column::Column;
    /// let prices = Column::new(vec![100.0, 102.0, 98.0]);
    /// let doubled = prices.map(|x| x * 2.0);
    /// assert_eq!(doubled.data(), vec![200.0, 204.0, 196.0]);
    /// ```
    ///
    pub fn map<U, F>(&self, f: F) -> Column<U>
    where
        F: FnMut(&T) -> U,
        U: Numeric,
    {
        Column {
            data: self.data.iter().map(f).collect(),
        }
    }

    /// Filters elements based on a predicate, returning filtered data and filtered indices.
    ///
    /// Returns a tuple containing:
    /// - A new column with elements that satisfy the predicate
    /// - A vector of indices corresponding to the filtered elements
    ///
    /// # Arguments
    /// * `f` - Predicate function to test each element
    ///
    /// # Example
    /// ```
    /// use mizuhiki_ta::core::column::Column;
    ///
    /// let column = Column::new(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
    /// let (filtered, indices) = column.filter(|&x| x > 3.0);
    /// assert_eq!(filtered.len(), 2);
    /// assert_eq!(indices, vec![3, 4]);
    /// ```
    pub fn filter<F>(&self, f: F) -> (Column<T>, Vec<usize>)
    where
        F: Fn(&T) -> bool,
    {
        let mut filtered_data = Vec::new();
        let mut filtered_indices = Vec::new();
        for (i, v) in self.data.iter().enumerate() {
            if f(v) {
                filtered_data.push(*v);
                filtered_indices.push(i);
            }
        }
        (Column::new(filtered_data), filtered_indices)
    }

    /// Get raw data as a slice.
    pub fn as_slice(&self) -> &[T] {
        &self.data
    }

    /// Convert the column to a Vec, consuming the column.
    pub fn into_vec(self) -> Vec<T> {
        self.data
    }

    /// Calculate pairwise differences (current - previous).
    ///
    /// Returns a new column where each element is the difference between the current
    /// and previous element. The first element is set to the default value of the type `T` (typically 0).
    ///
    /// # Example
    /// ```
    /// use mizuhiki_ta::core::column::Column;
    ///
    /// let prices = Column::new(vec![100.0, 102.0, 98.0, 105.0]);
    /// let changes = prices.diff();
    /// assert_eq!(changes.get(0), Some(&0.0)); // First element is default
    /// assert_eq!(changes.get(1), Some(&2.0));  // 102 - 100
    /// assert_eq!(changes.get(2), Some(&-4.0)); // 98 - 102
    /// ```
    pub fn diff(&self) -> Column<T> {
        if self.data.len() <= 1 {
            return Column::new(vec![T::default(); self.data.len()]);
        }

        let mut result = Vec::with_capacity(self.data.len());
        result.push(T::default());

        for i in 1..self.data.len() {
            result.push(self.data[i] - self.data[i - 1]);
        }

        Column::new(result)
    }

    /// Calculate exponential moving average (EMA) with alpha smoothing factor.
    ///
    /// The EMA is calculated using the formula:
    /// EMA(t) = alpha * current + (1 - alpha) * EMA(t-1)
    ///
    /// The first value is used as the initial EMA value.
    ///
    /// # Arguments
    /// * `alpha` - Smoothing factor
    ///
    /// # Example
    /// ```
    /// use mizuhiki_ta::core::column::Column;
    ///
    /// let prices = Column::new(vec![100.0, 102.0, 98.0, 105.0]);
    /// let ema = prices.ewm_mean(0.3f64);
    ///
    /// assert_eq!(ema.get(0), Some(&100.0)); // First value is the same
    /// assert_eq!(ema.get(1), Some(&100.6)); // 0.3 * 102 + 0.7 * 100
    /// ```
    pub fn ewm_mean(&self, alpha: T) -> Column<T> {
        if self.data.is_empty() {
            return Column::new(Vec::new());
        }

        let mut result = Vec::with_capacity(self.data.len());
        let mut ema = self.data[0];
        result.push(ema);

        let one_minus_alpha = T::one() - alpha;

        for i in 1..self.data.len() {
            let current = self.data[i];
            ema = alpha * current + one_minus_alpha * ema;
            result.push(ema);
        }

        Column::new(result)
    }
}

impl<T> IntoIterator for Column<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_within_tolerance<T: Numeric>(value: T, expected: T, tolerance: T) {
        assert!(
            (value - expected).abs() < tolerance,
            "Value {:?} not within tolerance of {:?}, expected {:?}",
            value,
            tolerance,
            expected
        );
    }

    #[test]
    fn test_ewm_mean() {
        let prices = Column::new(vec![
            44.34, 44.09, 44.15, 43.61, 44.33, 44.83, 45.10, 45.42, 45.84, 46.08, 45.89, 46.03,
            45.61, 46.28, 46.28, 46.00, 46.03, 46.41, 46.22, 45.64, 46.21, 46.25, 45.71, 46.45,
            45.78, 45.35, 44.03, 44.18, 44.22, 44.57, 43.42, 42.66, 43.13,
        ]);

        // Result from pandas
        let expected_ema = vec![
            44.340000, 44.306667, 44.285778, 44.195674, 44.213584, 44.295773, 44.403003, 44.538603,
            44.712122, 44.894506, 45.027239, 45.160940, 45.220815, 45.362039, 45.484434, 45.553176,
            45.616753, 45.722519, 45.788850, 45.769003, 45.827803, 45.884096, 45.860883, 45.939432,
            45.918174, 45.842418, 45.600762, 45.411327, 45.252484, 45.161486, 44.929288, 44.626716,
            44.427154,
        ];

        let ema = prices.ewm_mean(2.0 / 15.0);

        for i in 0..expected_ema.len() {
            assert_within_tolerance(*ema.get(i).unwrap(), expected_ema[i], 0.0001);
        }
    }

    #[test]
    fn test_ewm_mean2() {
        let tr = Column::new(vec![
            0.58, 0.51, 0.50, 0.58, 0.41, 0.26, 0.49, 0.60, 0.32, 0.93, 0.76, 0.45, 0.46, 1.10,
            1.26, 1.15, 1.18, 0.69, 0.67, 0.62, 0.74, 0.72, 0.81, 0.72, 0.77, 0.68, 0.63, 0.65,
            0.79, 0.77, 0.79, 0.97, 1.03, 0.94, 0.88, 0.78, 0.72, 0.81,
        ]);

        // Result from pandas
        let expected_ema = vec![
            0.5800, 0.5750, 0.5696, 0.5704, 0.5589, 0.5376, 0.5342, 0.5389, 0.5232, 0.5523, 0.5671,
            0.5588, 0.5517, 0.5909, 0.6387, 0.6752, 0.7113, 0.7097, 0.7069, 0.7007, 0.7035, 0.7047,
            0.7122, 0.7128, 0.7168, 0.7142, 0.7082, 0.7040, 0.7102, 0.7145, 0.7198, 0.7377, 0.7586,
            0.7716, 0.7793, 0.7793, 0.7751, 0.7776,
        ];

        let ema = tr.ewm_mean(1.0 / 14.0);

        for i in 0..expected_ema.len() {
            assert_within_tolerance(*ema.get(i).unwrap(), expected_ema[i], 0.0001);
        }
    }
}
