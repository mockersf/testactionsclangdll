use nom::error::{ErrorKind, ParseError};

#[derive(Debug)]
pub enum DataError<I> {
    NomError {
        input: I,
        kind: ErrorKind,
    },
    DataBuilderError {
        input: I,
        error: String,
        data_type: String,
    },
    Errors(Vec<DataError<I>>),
}

impl<'a, I> ParseError<I> for DataError<I> {
    fn from_error_kind(input: I, kind: ErrorKind) -> Self {
        Self::NomError { input, kind }
    }

    fn append(input: I, kind: ErrorKind, other: Self) -> Self {
        match other {
            DataError::Errors(mut errors) => {
                errors.push(DataError::NomError { input, kind });
                DataError::Errors(errors)
            }
            _ => DataError::Errors(vec![other, DataError::NomError { input, kind }]),
        }
    }
}
