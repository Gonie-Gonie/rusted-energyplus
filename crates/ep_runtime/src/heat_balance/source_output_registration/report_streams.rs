//! Mandatory actual stream interface; the caller retains the same files and SQL owner.
use std::io;
pub(crate) trait SourceOutputReportStreams {
    fn eso_good(&self) -> bool;
    fn mtr_good(&self) -> bool;
    fn write_eso(&mut self, bytes: &[u8]) -> io::Result<()>;
    fn write_mtr(&mut self, bytes: &[u8]) -> io::Result<()>;
    fn sqlite_present(&self) -> bool;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum SourceReportStream {
    Eso,
    Mtr,
}
impl SourceReportStream {
    pub(super) fn good(self, streams: &impl SourceOutputReportStreams) -> bool {
        match self {
            Self::Eso => streams.eso_good(),
            Self::Mtr => streams.mtr_good(),
        }
    }
    pub(super) fn write(
        self,
        streams: &mut impl SourceOutputReportStreams,
        bytes: &[u8],
    ) -> io::Result<()> {
        match self {
            Self::Eso => streams.write_eso(bytes),
            Self::Mtr => streams.write_mtr(bytes),
        }
    }
}
