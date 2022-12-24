/*!
 Utility functions
 */

use flate2::read::GzDecoder;

/**
 Fetch function result.
 Allows a dynamic error type so we don't have to explicitly coerce upstream errors to the local one.
 */
pub type FetchResult<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

/** Fetch a file from url.
 Adapted from <https://georgik.rocks/how-to-download-binary-file-in-rust-by-reqwest/>.
*/
pub fn fetch_url(url: String, file_name: String) -> FetchResult<()> {
    let response = reqwest::blocking::get(url)?;
    let mut file = std::fs::File::create(file_name)?;
    let mut content = std::io::Cursor::new(response.bytes()?);
    std::io::copy(&mut content, &mut file)?;
    Ok(())
}

/** Fetch a gz file from url and decompress it.
 Adapted from <https://georgik.rocks/how-to-download-binary-file-in-rust-by-reqwest/>.
*/
pub fn fetch_gz_url(url: String, file_name: String) -> FetchResult<()> {
    let response = reqwest::blocking::get(url)?;
    // let resp_text = response.text()?;
    let mut d = GzDecoder::new(response);
    let mut file = std::fs::File::create(file_name)?;
    // let mut content = std::io::Cursor::new(d.bytes());
    std::io::copy(&mut d, &mut file)?;
    Ok(())
}

// TODO: add unit tests for fetching functions.
