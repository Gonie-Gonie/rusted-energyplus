//! Objexx array value storage for actual Source Solar owners.
use super::Error;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourceSolarArray<T, const N: usize> {
    pub(crate) dimensions: Option<[usize; N]>,
    pub(crate) values: Vec<Option<T>>,
}
impl<T, const N: usize> Default for SourceSolarArray<T, N> {
    fn default() -> Self { Self { dimensions: None, values: Vec::new() } }
}
impl<T: Copy, const N: usize> SourceSolarArray<T, N> {
    pub(crate) fn dimension(&mut self, dimensions: [usize; N], value: Option<T>) -> Result<(), Error> {
        let size = dimensions.iter().try_fold(1_usize, |a, &b| a.checked_mul(b))
            .ok_or(Error::Unavailable("Source Solar allocation bounds"))?;
        self.dimensions = Some(dimensions);
        self.values = vec![value; size];
        Ok(())
    }
    fn offset(&self, indices: [usize; N]) -> Result<usize, Error> {
        let dimensions = self.dimensions.ok_or(Error::Unavailable("actual Source Solar array allocation"))?;
        let mut offset = 0;
        let mut stride = 1;
        // ObjexxFCL first dimension varies fastest. Source ordinal arguments
        // remain one-based; a zero extent is allocated and contains no values.
        for axis in 0..N {
            if indices[axis] == 0 || indices[axis] > dimensions[axis] {
                return Err(Error::Unavailable("actual Source Solar ordinal bounds"));
            }
            offset += (indices[axis] - 1) * stride;
            stride *= dimensions[axis];
        }
        Ok(offset)
    }
    pub(crate) fn read(&self, indices: [usize; N]) -> Result<T, Error> {
        self.values.get(self.offset(indices)?).copied().flatten()
            .ok_or(Error::Unavailable("actual Source Solar allocated but unwritten element"))
    }
    pub(crate) fn read_ref(&self, indices: [usize; N]) -> Result<&T, Error> {
        self.values.get(self.offset(indices)?).and_then(Option::as_ref)
            .ok_or(Error::Unavailable("actual Source Solar allocated but unwritten bound output"))
    }
    pub(crate) fn set(&mut self, indices: [usize; N], value: T) -> Result<(), Error> {
        let offset = self.offset(indices)?;
        self.values[offset] = Some(value);
        Ok(())
    }
    pub(crate) fn fill(&mut self, value: T) -> Result<(), Error> {
        if self.dimensions.is_none() { return Err(Error::Unavailable("actual Source Solar fill allocation")); }
        self.values.fill(Some(value));
        Ok(())
    }
}
