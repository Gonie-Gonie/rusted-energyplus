//! Same persistent SurfaceGeometry X/Y/Z/A/B allocation and unwritten cells.
use super::Error;
type Axis = Option<Vec<Option<f64>>>;
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourceConvexityState {
    first_time: bool,
    vert_size: usize,
    x: Axis,
    y: Axis,
    z: Axis,
    a: Axis,
    b: Axis,
}
impl Default for SourceConvexityState {
    fn default() -> Self {
        Self {
            first_time: true,
            vert_size: 0,
            x: None,
            y: None,
            z: None,
            a: None,
            b: None,
        }
    }
}
fn read(axis: &Axis, n: usize) -> Result<f64, Error> {
    axis.as_ref()
        .and_then(|v| n.checked_sub(1).and_then(|i| v.get(i)))
        .copied()
        .flatten()
        .ok_or(Error::Unavailable(
            "actual allocated/written CheckConvexity scratch",
        ))
}
fn write(axis: &mut Axis, n: usize, value: f64) -> Result<(), Error> {
    *axis
        .as_mut()
        .and_then(|v| n.checked_sub(1).and_then(|i| v.get_mut(i)))
        .ok_or(Error::Unavailable(
            "actual CheckConvexity scratch indexed write",
        ))? = Some(value);
    Ok(())
}
fn assign(dst: &mut Axis, src: &Axis) -> Result<(), Error> {
    let dst = dst
        .as_mut()
        .ok_or(Error::Unavailable("actual CheckConvexity A/B allocation"))?;
    let src = src
        .as_ref()
        .ok_or(Error::Unavailable("actual CheckConvexity XYZ allocation"))?;
    if dst.len() != src.len() {
        return Err(Error::Unavailable(
            "Source scratch whole-array assignment bounds",
        ));
    }
    dst.clone_from_slice(src);
    Ok(())
}
impl SourceConvexityState {
    fn allocate(&mut self, max: usize) -> Result<(), Error> {
        let size = max.checked_add(2).ok_or(Error::Unavailable(
            "Source CheckConvexity allocation bounds",
        ))?;
        self.x = Some(vec![None; size]);
        self.y = Some(vec![None; size]);
        self.z = Some(vec![None; size]);
        self.a = Some(vec![None; size]);
        self.b = Some(vec![None; size]);
        self.vert_size = max;
        Ok(())
    }
    pub(super) fn prepare(&mut self, sides: usize, max: usize) -> Result<(), Error> {
        if self.first_time {
            self.allocate(max)?;
            self.first_time = false;
        }
        if sides > self.vert_size {
            self.x = None;
            self.y = None;
            self.z = None;
            self.a = None;
            self.b = None;
            self.allocate(max)?;
        }
        Ok(())
    }
    pub(super) fn write_xyz(&mut self, n: usize, p: [f64; 3]) -> Result<(), Error> {
        write(&mut self.x, n, p[0])?;
        write(&mut self.y, n, p[1])?;
        write(&mut self.z, n, p[2])
    }
    pub(super) fn x(&self, n: usize) -> Result<f64, Error> {
        read(&self.x, n)
    }
    pub(super) fn y(&self, n: usize) -> Result<f64, Error> {
        read(&self.y, n)
    }
    pub(super) fn z(&self, n: usize) -> Result<f64, Error> {
        read(&self.z, n)
    }
    pub(super) fn ab(&self, n: usize) -> Result<[f64; 2], Error> {
        Ok([read(&self.a, n)?, read(&self.b, n)?])
    }
    pub(super) fn plane(&mut self, plane: usize) -> Result<(), Error> {
        match plane {
            0 => {
                assign(&mut self.a, &self.x)?;
                assign(&mut self.b, &self.y)?;
            }
            1 => {
                assign(&mut self.a, &self.x)?;
                assign(&mut self.b, &self.z)?;
            }
            2 => {
                assign(&mut self.a, &self.y)?;
                assign(&mut self.b, &self.z)?;
            }
            _ => return Err(Error::Unavailable("Source CheckConvexity plane branch")),
        }
        Ok(())
    }
}
