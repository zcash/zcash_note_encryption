/// Represents a fixed-size array of bytes for note components.
#[derive(Clone, Copy, Debug)]
pub struct NoteBytesData<const N: usize>(pub [u8; N]);

impl<const N: usize> AsRef<[u8]> for NoteBytesData<N> {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl<const N: usize> AsMut<[u8]> for NoteBytesData<N> {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

/// Provides a unified interface for handling fixed-size byte arrays used in note encryption.
pub trait NoteBytes: AsRef<[u8]> + AsMut<[u8]> + Clone + Copy {
    fn from_slice(bytes: &[u8]) -> Option<Self>;

    fn from_slice_with_tag<const TAG_SIZE: usize>(
        output: &[u8],
        tag: [u8; TAG_SIZE],
    ) -> Option<Self>;
}

impl<const N: usize> NoteBytes for NoteBytesData<N> {
    fn from_slice(bytes: &[u8]) -> Option<NoteBytesData<N>> {
        let data = bytes.try_into().ok()?;
        Some(NoteBytesData(data))
    }

    fn from_slice_with_tag<const TAG_SIZE: usize>(
        output: &[u8],
        tag: [u8; TAG_SIZE],
    ) -> Option<NoteBytesData<N>> {
        let expected_output_len = N.checked_sub(TAG_SIZE)?;

        if output.len() != expected_output_len {
            return None;
        }

        let mut data = [0u8; N];

        data[..expected_output_len].copy_from_slice(output);
        data[expected_output_len..].copy_from_slice(&tag);

        Some(NoteBytesData(data))
    }
}
