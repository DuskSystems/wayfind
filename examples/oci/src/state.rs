use crate::types::digest::Digest;
use dashmap::DashMap;
use http::StatusCode;
use std::sync::Arc;
use thiserror::Error;
use uuid::Uuid;

pub type SharedAppState = Arc<AppState>;

#[derive(Debug, Error)]
pub enum AppStateError {
    #[error("Repository not found: {0}")]
    RepositoryNotFound(String),

    #[error("Blob not found: {digest} in repository {repository}")]
    BlobNotFound { repository: String, digest: Digest },

    #[error("Upload not found: {0}")]
    UploadNotFound(Uuid),

    #[error("Digest mismatch. Expected: {expected} - Actual: {actual}")]
    DigestMismatch { expected: String, actual: String },

    #[error("Manifest not found: {reference} in repository {repository}")]
    ManifestNotFound {
        repository: String,
        reference: String,
    },
}

impl AppStateError {
    #[must_use]
    pub const fn status_code(&self) -> StatusCode {
        match self {
            Self::RepositoryNotFound(_)
            | Self::BlobNotFound { .. }
            | Self::UploadNotFound(_)
            | Self::DigestMismatch { .. }
            | Self::ManifestNotFound { .. } => StatusCode::NOT_FOUND,
        }
    }
}

pub struct AppState {
    repositories: DashMap<String, Repository>,
    uploads: DashMap<Uuid, Upload>,
}

#[derive(Debug, Clone)]
pub struct Repository {
    pub blobs: DashMap<Digest, Vec<u8>>,
    pub manifests: DashMap<String, Manifest>,
    pub tags: DashMap<String, String>,
}

impl Repository {
    fn new() -> Self {
        Self {
            blobs: DashMap::new(),
            manifests: DashMap::new(),
            tags: DashMap::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Manifest {
    pub digest: Digest,
    pub content: Vec<u8>,
    pub subject: Option<Digest>,
}

#[derive(Debug, Clone)]
pub struct Upload {
    pub repository: String,
    pub data: Vec<u8>,
}

impl AppState {
    #[must_use]
    pub fn new() -> Self {
        Self {
            repositories: DashMap::new(),
            uploads: DashMap::new(),
        }
    }

    #[must_use]
    pub fn repository_exists(&self, repository: &str) -> bool {
        self.repositories.contains_key(repository)
    }

    pub fn get_blob(&self, repository: &str, digest: &Digest) -> Result<Vec<u8>, AppStateError> {
        let repo = self
            .repositories
            .get(repository)
            .ok_or_else(|| AppStateError::RepositoryNotFound(repository.to_owned()))?;

        repo.blobs
            .get(digest)
            .map(|blob| blob.clone())
            .ok_or_else(|| AppStateError::BlobNotFound {
                repository: repository.to_owned(),
                digest: digest.clone(),
            })
    }

    pub fn add_blob(&self, repository: String, digest: Digest, blob: Vec<u8>) {
        self.repositories
            .entry(repository)
            .or_insert_with(Repository::new)
            .blobs
            .insert(digest, blob);
    }

    #[must_use]
    pub fn start_upload(&self, repository: String) -> Uuid {
        let upload_id = Uuid::new_v4();
        self.uploads.insert(
            upload_id,
            Upload {
                repository,
                data: vec![],
            },
        );

        upload_id
    }

    pub fn update_upload(&self, upload_id: Uuid, chunk: &[u8]) -> Result<usize, AppStateError> {
        self.uploads
            .get_mut(&upload_id)
            .map(|mut upload| {
                upload.data.extend_from_slice(chunk);
                upload.data.len()
            })
            .ok_or(AppStateError::UploadNotFound(upload_id))
    }

    pub fn complete_upload(&self, upload_id: Uuid, digest: Digest) -> Result<(), AppStateError> {
        let (_, upload) = self
            .uploads
            .remove(&upload_id)
            .ok_or(AppStateError::UploadNotFound(upload_id))?;

        let actual = Digest::sha256(&upload.data);
        if actual != digest {
            return Err(AppStateError::DigestMismatch {
                expected: digest.to_string(),
                actual: actual.to_string(),
            });
        }

        self.add_blob(upload.repository, digest, upload.data);
        Ok(())
    }

    pub fn add_manifest(
        &self,
        repository: String,
        reference: String,
        digest: &Digest,
        content: &[u8],
        subject: Option<Digest>,
    ) {
        let repo = self
            .repositories
            .entry(repository)
            .or_insert_with(Repository::new);

        repo.manifests.insert(
            digest.to_string(),
            Manifest {
                digest: digest.clone(),
                content: content.to_vec(),
                subject,
            },
        );

        if Digest::try_from(reference.as_str()).is_err() {
            repo.tags.insert(reference, digest.to_string());
        }
    }

    pub fn get_manifest(
        &self,
        repository: &str,
        reference: &str,
    ) -> Result<Vec<u8>, AppStateError> {
        let repo = self
            .repositories
            .get(repository)
            .ok_or_else(|| AppStateError::RepositoryNotFound(repository.to_owned()))?;

        if let Some(manifest) = repo.manifests.get(reference) {
            return Ok(manifest.content.clone());
        }

        if let Some(digest) = repo.tags.get(reference) {
            if let Some(manifest) = repo.manifests.get(digest.value()) {
                return Ok(manifest.content.clone());
            }
        }

        if Digest::try_from(reference).is_err() {
            for manifest in &repo.manifests {
                if manifest.key() == reference {
                    return Ok(manifest.value().content.clone());
                }
            }
        }

        Err(AppStateError::ManifestNotFound {
            repository: repository.to_owned(),
            reference: reference.to_owned(),
        })
    }

    pub fn list_tags(&self, repository: &str) -> Result<Vec<String>, AppStateError> {
        self.repositories
            .get(repository)
            .map(|repo| repo.tags.iter().map(|t| t.key().clone()).collect())
            .ok_or_else(|| AppStateError::RepositoryNotFound(repository.to_owned()))
    }

    pub fn delete_blob(&self, repository: &str, digest: &Digest) -> Result<(), AppStateError> {
        self.repositories
            .get(repository)
            .ok_or_else(|| AppStateError::RepositoryNotFound(repository.to_owned()))?
            .blobs
            .remove(digest);

        Ok(())
    }

    pub fn delete_manifest(&self, repository: &str, reference: &str) -> Result<(), AppStateError> {
        let repo = self
            .repositories
            .get(repository)
            .ok_or_else(|| AppStateError::RepositoryNotFound(repository.to_owned()))?;

        if let Some(digest_str) = repo.tags.remove(reference) {
            repo.manifests.remove(&digest_str.1);
        } else {
            repo.manifests.remove(reference);
        }

        drop(repo);
        Ok(())
    }

    pub fn get_referrers(
        &self,
        repository: &str,
        digest: &Digest,
    ) -> Result<Vec<Manifest>, AppStateError> {
        self.repositories
            .get(repository)
            .map(|repo| {
                repo.manifests
                    .iter()
                    .filter(|r| r.value().subject.as_ref() == Some(digest))
                    .map(|r| r.value().clone())
                    .collect()
            })
            .ok_or_else(|| AppStateError::RepositoryNotFound(repository.to_owned()))
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
