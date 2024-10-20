use std::result::Result as StdResult;

use qrcodegen::QrCode;

use crate::draw::draw;
use crate::rendering::display_list::{DisplayList, DisplayListOpReceiver};
use crate::types::{ErrorCorrectionLevel, Size};

/// The data source that is used as the input when generating a QR Code.
#[derive(Debug, Clone)]
enum DataSource<'a> {
    Binary(&'a [u8]),
    Text(&'a str),
}

/// The error type when the supplied data cannot be converted to a QR Code.
#[derive(Debug)]
pub struct FailedToGenerate;

pub type Result<T> = StdResult<T, FailedToGenerate>;

/// Builds a QR Code.
#[derive(Debug, Clone)]
pub struct Builder<'a> {
    data: DataSource<'a>,
    ecl: ErrorCorrectionLevel,
    size: Size,
}

impl<'a> Builder<'a> {
    /// Creates a `Builder` with the specified bytes as input.
    pub fn binary(bytes: &'a [u8]) -> Self {
        Self::new(DataSource::Binary(bytes))
    }

    /// Creates a `Builder` with the specified Unicode string as input.
    pub fn text(str: &'a str) -> Self {
        Self::new(DataSource::Text(str))
    }

    /// Sets the error correction level for the QR Code.
    pub fn error_correction_level(self, ecl: ErrorCorrectionLevel) -> Self {
        Self {
            data: self.data,
            ecl,
            size: self.size,
        }
    }

    /// Sets the size of output image.
    pub fn size(self, size: Size) -> Self {
        Self {
            data: self.data,
            ecl: self.ecl,
            size,
        }
    }

    pub fn get_size(&self) -> Size {
        self.size
    }

    pub fn build_with_receiver<R>(self, receiver: &mut R) -> Result<()>
    where
        R: DisplayListOpReceiver,
    {
        let display_list = self.build_display_list()?;
        display_list.present(receiver);
        Ok(())
    }
}

impl<'a> Builder<'a> {
    fn new(data: DataSource<'a>) -> Self {
        Self {
            data,
            ecl: ErrorCorrectionLevel::Medium,
            size: Size {
                width: 256.0,
                height: 256.0,
            },
        }
    }

    fn build_qr_code(&self) -> StdResult<QrCode, ()> {
        let ecl = self.ecl;
        let result = match self.data {
            DataSource::Binary(data) => QrCode::encode_binary(data, ecl),
            DataSource::Text(text) => QrCode::encode_text(text, ecl),
        };
        result.map_err(|_| ())
    }

    fn build_display_list(&self) -> Result<DisplayList> {
        let size = self.size;
        // Encode the input data to QR Code modules.
        let code = self.build_qr_code().map_err(|_| FailedToGenerate)?;

        let mut display_list = DisplayList::new();
        let mut recorder = display_list.begin_recording();
        // Produce draw calls for the image derived from the code.
        draw(code, size, &mut recorder);

        Ok(display_list)
    }
}
