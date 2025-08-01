use crate::core::{
    column::Column,
    traits::{Indexable, Numeric},
};

/// A labeled data structure containing a column of numeric data with an associated index.
#[derive(Debug, Clone)]
pub struct Series<T, I> {
    name: String,
    column: Column<T>,
    index: Vec<I>,
}

impl<T, I> Series<T, I>
where
    T: Numeric,
    I: Indexable,
{
    pub fn new(name: String) -> Self {
        Series {
            name,
            column: Column::default(),
            index: Vec::new(),
        }
    }

    /// Creates a new Series with the given name, data, and index.
    ///
    /// # Arguments
    ///
    /// * `name` - A descriptive name for the series
    /// * `data` - Vector of numeric data values
    /// * `index` - Vector of index labels, must be same length as data
    ///
    /// # Panics
    ///
    /// Panics if `data` and `index` have different lengths.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mizuhiki_ta::core::series::Series;
    ///
    /// let data = vec![10.0, 20.0, 30.0];
    /// let index = vec![1, 2, 3];
    /// let series = Series::from_data("test".to_string(), data, index, None);
    /// assert_eq!(series.len(), 3);
    /// ```
    pub fn from_data(name: String, data: Vec<T>, index: Vec<I>, capacity: Option<usize>) -> Self {
        assert_eq!(data.len(), index.len(), "Data and index length must match");
        Series {
            name,
            column: Column::from_vec_with_capacity(data, capacity),
            index,
        }
    }

    pub fn push(&mut self, value: T, index: I) {
        self.column.push(value);
        self.index.push(index);
    }

    pub fn extend(&mut self, values: impl IntoIterator<Item = (T, I)>) {
        for (value, index) in values {
            self.push(value, index);
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn capacity(&self) -> Option<usize> {
        self.column.capacity()
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn column(&self) -> &Column<T> {
        &self.column
    }

    pub fn index(&self) -> &[I] {
        &self.index
    }

    pub fn values(&self) -> &[T] {
        self.column.as_slice()
    }

    pub fn len(&self) -> usize {
        self.column.len()
    }

    pub fn is_empty(&self) -> bool {
        self.column.is_empty()
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.column.get(index)
    }

    /// Map values to a new Series with a different type.
    ///
    /// See [`Column::map`] for details. Preserves the series index.
    pub fn map<U, F>(&self, f: F) -> Series<U, I>
    where
        U: Numeric,
        F: FnMut(&T) -> U,
    {
        Series {
            name: self.name.clone(),
            column: self.column.map(f),
            index: self.index.clone(),
        }
    }

    /// Apply a transformation in-place, mutating the existing series.
    ///
    /// See [`Column::apply`] for details.
    pub fn apply_mut<F>(&mut self, f: F)
    where
        F: FnMut(&mut T),
    {
        self.column.apply(f);
    }

    /// Filters elements based on a predicate function.
    ///
    /// See [`Column::filter`] for details. Returns filtered elements with their corresponding index values.
    pub fn filter<F>(&self, f: F) -> Series<T, I>
    where
        F: Fn(&T) -> bool,
    {
        let (filtered_column, filtered_pos) = self.column.filter(&f);
        let filtered_index: Vec<I> = filtered_pos.iter().map(|&i| self.index[i]).collect();
        Series {
            name: self.name.clone(),
            column: filtered_column,
            index: filtered_index,
        }
    }

    /// Calculate pairwise differences between consecutive elements.
    ///
    /// See [`Column::diff`] for details. Preserves the series index.
    pub fn diff(&self) -> Series<T, I> {
        Series {
            name: self.name.clone(),
            column: self.column.diff(),
            index: self.index.clone(),
        }
    }

    /// Calculate exponential moving average with generic alpha type.
    ///
    /// See [`Column::ewm_mean`] for details.
    pub fn ewm_mean(&self, alpha: T) -> Series<T, I> {
        Series {
            name: self.name.clone(),
            column: self.column.ewm_mean(alpha),
            index: self.index.clone(),
        }
    }
}

/// Convenience implementation for Series with numeric index.
impl<T> Series<T, usize>
where
    T: Numeric,
{
    /// Create a new Series from a vector with automatic numeric indexing.
    ///
    /// The index will be automatically generated as 0, 1, 2, ..., n-1.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mizuhiki_ta::core::series::Series;
    ///
    /// let series = Series::from_vec("my_data".to_string(), vec![10.0, 20.0, 30.0], None);
    /// assert_eq!(series.len(), 3);
    /// assert_eq!(series.index(), &[0, 1, 2]);
    /// assert_eq!(series.get(1), Some(&20.0));
    /// ```
    pub fn from_vec(name: String, data: Vec<T>, capacity: Option<usize>) -> Self {
        let index = (0..data.len()).collect();
        Series::from_data(name, data, index, capacity)
    }
}
