use crate::*;

/// Decompresses the given Deflate compressed data.
///
/// This function takes a byte slice of Deflate compressed data and decompresses it using the
/// `DeflateDecoder` from the `flate2` crate. The decompressed data is returned as a `Cow<Vec<u8>>`.
/// If decompression succeeds, the decompressed data is returned as an owned `Vec<u8>`. In case of
/// an error, an empty `Vec<u8>` is returned.
///
/// # Parameters
/// - `data` - A reference to a byte slice (`&[u8]`) containing the compressed data to be decoded.
/// - `buffer_size` - The buffer size to use for the decompression process. A larger buffer size can
///   improve performance for larger datasets.
///
/// # Returns
/// - `Cow<Vec<u8>>` - The decompressed data as a `Cow<Vec<u8>>`. If decompression is successful, the
///   decompressed data is returned as an owned `Vec<u8>`. In case of an error, an empty owned
///   `Vec<u8>` is returned.
///
/// # Notes
/// - The decompression process uses the `DeflateDecoder` with the provided buffer size.
/// - The use of `Cow` allows for optimization by avoiding unnecessary copying of data when not required.
pub fn decode(data: &[u8], buffer_size: usize) -> Cow<Vec<u8>> {
    let decoder: DeflateDecoder<&[u8]> = DeflateDecoder::new(data);
    let mut buffered_reader: BufReader<DeflateDecoder<&[u8]>> =
        BufReader::with_capacity(buffer_size, decoder);
    let mut decompressed_data: Vec<u8> = Vec::new();
    match buffered_reader.read_to_end(&mut decompressed_data) {
        Ok(_) => Cow::Owned(decompressed_data),
        _ => Cow::Owned(Vec::new()),
    }
}
