//! Short-lived forwarding views over the same real diagnostic backend owners.
use super::{
    SourceErrorBackend, SourceErrorCallback, SourceErrorLevel, SourceErrorSqlite, SourceErrorState,
    SourceMessageCallback,
};
use std::fmt;
use std::io::{self, IoSlice, Write};

struct WriteBorrow<'borrow, 'object> {
    stream: &'borrow mut (dyn Write + 'object),
}
impl Write for WriteBorrow<'_, '_> {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.stream.write(bytes)
    }
    fn write_vectored(&mut self, bytes: &[IoSlice<'_>]) -> io::Result<usize> {
        self.stream.write_vectored(bytes)
    }
    fn write_all(&mut self, bytes: &[u8]) -> io::Result<()> {
        self.stream.write_all(bytes)
    }
    fn write_fmt(&mut self, arguments: fmt::Arguments<'_>) -> io::Result<()> {
        self.stream.write_fmt(arguments)
    }
    fn flush(&mut self) -> io::Result<()> {
        self.stream.flush()
    }
}

struct SqliteBorrow<'borrow, 'object> {
    sqlite: &'borrow mut (dyn SourceErrorSqlite + 'object),
}
impl SourceErrorSqlite for SqliteBorrow<'_, '_> {
    fn create_error_record(
        &mut self,
        simulation_index: i32,
        error_type: i32,
        message: &str,
        count: i32,
    ) -> io::Result<()> {
        self.sqlite
            .create_error_record(simulation_index, error_type, message, count)
    }
    fn update_error_record(&mut self, message: &str) -> io::Result<()> {
        self.sqlite.update_error_record(message)
    }
    fn within_transaction(&mut self) -> io::Result<bool> {
        self.sqlite.within_transaction()
    }
    fn commit(&mut self) -> io::Result<()> {
        self.sqlite.commit()
    }
}

impl SourceErrorBackend<'_> {
    /// Reborrow every actual backend field for one call, releasing all loans
    /// before returning. The higher-ranked result cannot retain this backend.
    ///
    /// The accompanying reference bundle is reborrowed for the same call so a
    /// context can borrow its real reports/files without extending their loans
    /// to the original callback object's lifetime. This does not copy or retain
    /// the supplied owners. Optional fields retain their exact presence.
    pub(crate) fn with_scoped<Owners: ?Sized, T>(
        &mut self,
        owners: &mut Owners,
        call: impl for<'call> FnOnce(SourceErrorBackend<'call>, &'call mut Owners) -> T,
    ) -> T {
        let mut error_stream = self.error_stream.as_mut().map(|stream| WriteBorrow {
            stream: &mut **stream,
        });
        let mut console = WriteBorrow {
            stream: &mut *self.console,
        };
        let mut out_unit1 = self.out_unit1.as_mut().map(|stream| WriteBorrow {
            stream: &mut **stream,
        });
        let mut out_unit2 = self.out_unit2.as_mut().map(|stream| WriteBorrow {
            stream: &mut **stream,
        });
        let mut f_message = self.f_message.as_mut().map(|callback| {
            move |state: &mut SourceErrorState, message: &str| (**callback)(state, message)
        });
        let mut message_callback = self.message_callback.as_mut().map(|callback| {
            move |state: &mut SourceErrorState, message: &str| (**callback)(state, message)
        });
        let mut error_callback = self.error_callback.as_mut().map(|callback| {
            move |state: &mut SourceErrorState, level: SourceErrorLevel, message: &str| {
                (**callback)(state, level, message);
            }
        });
        let mut sqlite = self.sqlite.as_mut().map(|sqlite| SqliteBorrow {
            sqlite: &mut **sqlite,
        });
        call(
            SourceErrorBackend {
                state: &mut *self.state,
                error_stream: error_stream.as_mut().map(|stream| stream as &mut dyn Write),
                console: &mut console,
                out_unit1: out_unit1.as_mut().map(|stream| stream as &mut dyn Write),
                out_unit2: out_unit2.as_mut().map(|stream| stream as &mut dyn Write),
                f_message: f_message
                    .as_mut()
                    .map(|callback| callback as &mut SourceMessageCallback<'_>),
                message_callback: message_callback
                    .as_mut()
                    .map(|callback| callback as &mut SourceMessageCallback<'_>),
                error_callback: error_callback
                    .as_mut()
                    .map(|callback| callback as &mut SourceErrorCallback<'_>),
                sqlite: sqlite
                    .as_mut()
                    .map(|sqlite| sqlite as &mut dyn SourceErrorSqlite),
            },
            owners,
        )
    }
}
