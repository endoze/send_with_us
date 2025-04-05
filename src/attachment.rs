use crate::error::Result;
use base64::{Engine as _, engine::general_purpose};
use serde::{Deserialize, Serialize};
use std::path::Path;
use tokio::fs;

/// Represents a file attachment for an email
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Attachment {
  /// Attachment ID/filename
  pub id: String,

  /// Base64 encoded data
  pub data: String,
}

impl Attachment {
  /// Creates a new attachment by loading data from a file path.
  ///
  /// This method reads a file from the filesystem, extracts its filename,
  /// and creates an attachment with the file contents encoded in base64.
  ///
  /// # Arguments
  /// * `path` - Path to the file to attach
  ///
  /// # Returns
  /// A Result containing the new Attachment if successful
  ///
  /// # Errors
  /// Returns an error if the file cannot be read
  ///
  /// # Examples
  ///
  /// ```no_run
  /// # use send_with_us::Attachment;
  /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
  /// let attachment = Attachment::from_path("path/to/document.pdf").await?;
  /// # Ok(())
  /// # }
  /// ```
  pub async fn from_path(path: impl AsRef<Path>) -> Result<Self> {
    let path = path.as_ref();
    let filename = path
      .file_name()
      .and_then(|n| n.to_str())
      .unwrap_or("attachment")
      .to_string();

    let content = fs::read(path).await?;
    let encoded = general_purpose::STANDARD.encode(&content);

    Ok(Self {
      id: filename,
      data: encoded,
    })
  }

  /// Creates a new attachment from raw bytes with a given filename.
  ///
  /// This method is useful when you already have file data in memory
  /// and don't need to read from the filesystem.
  ///
  /// # Arguments
  /// * `content` - The raw bytes to encode as the attachment content
  /// * `filename` - The filename to use for the attachment
  ///
  /// # Returns
  /// A new Attachment with the provided content encoded in base64
  ///
  /// # Examples
  ///
  /// ```
  /// use send_with_us::Attachment;
  ///
  /// let content = b"hello world";
  /// let attachment = Attachment::from_bytes(content, "greeting.txt");
  /// ```
  pub fn from_bytes(content: &[u8], filename: impl Into<String>) -> Self {
    let encoded = general_purpose::STANDARD.encode(content);

    Self {
      id: filename.into(),
      data: encoded,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::fs::File;
  use std::io::Write;
  use tempdir::TempDir;

  #[tokio::test]
  async fn test_attachment_from_bytes_with_explicit_filename() {
    let content = b"test text";
    let filename = "rawr.txt";
    let attachment = Attachment::from_bytes(content, filename);

    assert_eq!(attachment.id, "rawr.txt");
    assert_eq!(attachment.data, general_purpose::STANDARD.encode(content));
  }

  #[tokio::test]
  async fn test_attachment_from_path() {
    let content = b"test content";
    let attachment = Attachment::from_bytes(content, "path.txt");

    assert_eq!(attachment.id, "path.txt");
    assert_eq!(attachment.data, general_purpose::STANDARD.encode(content));
  }

  #[tokio::test]
  async fn test_attachment_from_path_with_real_file() -> Result<()> {
    let temp_dir = TempDir::new("attachment_test")?;
    let file_path = temp_dir.path().join("test_file.txt");
    let content = b"hello world from file";

    {
      let mut file = File::create(&file_path)?;
      file.write_all(content)?;
    }

    let attachment = Attachment::from_path(&file_path).await?;

    assert_eq!(attachment.id, "test_file.txt");
    assert_eq!(attachment.data, general_purpose::STANDARD.encode(content));

    Ok(())
  }

  #[tokio::test]
  async fn test_attachment_from_path_without_extension() -> Result<()> {
    let temp_dir = TempDir::new("attachment_test")?;
    let file_path = temp_dir.path().join("no_extension");
    let content = b"file without extension";

    {
      let mut file = File::create(&file_path)?;
      file.write_all(content)?;
    }

    let attachment = Attachment::from_path(&file_path).await?;

    assert_eq!(attachment.id, "no_extension");
    assert_eq!(attachment.data, general_purpose::STANDARD.encode(content));

    Ok(())
  }
}
