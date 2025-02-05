//! Method, error and parameter types for the CodesOfConduct endpoint.
#![allow(
    unused_imports,
)]
/* 
 * GitHub v3 REST API
 *
 * GitHub's v3 REST API.
 *
 * OpenAPI spec version: 1.1.4
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

use serde::Deserialize;

use crate::adapters::{AdapterError, FromJson, GitHubRequest, GitHubRequestBuilder, GitHubResponseExt};
use crate::auth::Auth;
use crate::models::*;

use super::PerPage;

use std::collections::HashMap;
use serde_json::value::Value;

pub struct CodesOfConduct<'api> {
    auth: &'api Auth
}

pub fn new(auth: &Auth) -> CodesOfConduct {
    CodesOfConduct { auth }
}

/// Errors for the [Get all codes of conduct](CodesOfConduct::get_all_codes_of_conduct_async()) endpoint.
#[derive(Debug, thiserror::Error)]
pub enum CodesOfConductGetAllCodesOfConductError {
    #[error(transparent)]
    AdapterError(#[from] AdapterError),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error(transparent)]
    SerdeUrl(#[from] serde_urlencoded::ser::Error),


    // -- endpoint errors

    #[error("Not modified")]
    Status304,
    #[error("Preview header missing")]
    Status415(GetProjectsListForUserResponse415),
    #[error("Status code: {}", code)]
    Generic { code: u16 },
}

/// Errors for the [Get a code of conduct](CodesOfConduct::get_conduct_code_async()) endpoint.
#[derive(Debug, thiserror::Error)]
pub enum CodesOfConductGetConductCodeError {
    #[error(transparent)]
    AdapterError(#[from] AdapterError),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error(transparent)]
    SerdeUrl(#[from] serde_urlencoded::ser::Error),


    // -- endpoint errors

    #[error("Resource not found")]
    Status404(BasicError),
    #[error("Not modified")]
    Status304,
    #[error("Preview header missing")]
    Status415(GetProjectsListForUserResponse415),
    #[error("Status code: {}", code)]
    Generic { code: u16 },
}

/// Errors for the [Get the code of conduct for a repository](CodesOfConduct::get_for_repo_async()) endpoint.
#[derive(Debug, thiserror::Error)]
pub enum CodesOfConductGetForRepoError {
    #[error(transparent)]
    AdapterError(#[from] AdapterError),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error(transparent)]
    SerdeUrl(#[from] serde_urlencoded::ser::Error),


    // -- endpoint errors

    #[error("Status code: {}", code)]
    Generic { code: u16 },
}



impl<'api> CodesOfConduct<'api> {
    /// ---
    ///
    /// # Get all codes of conduct
    /// 
    /// [GitHub API docs for get_all_codes_of_conduct](https://docs.github.com/rest/reference/codes_of_conduct/#get-all-codes-of-conduct)
    ///
    /// The `get_all_codes_of_conduct_async` endpoint is enabled with the `scarlet-witch` cargo feature.
    ///
    /// ---
    #[cfg(feature = "scarlet-witch")]
    pub async fn get_all_codes_of_conduct_async(&self) -> Result<Vec<CodeOfConduct>, CodesOfConductGetAllCodesOfConductError> {

        let request_uri = format!("{}/codes_of_conduct", super::GITHUB_BASE_API_URL);


        let req = GitHubRequest {
            uri: request_uri,
            body: None,
            method: "GET",
            headers: vec![("Accept", "application/vnd.github.scarlet-witch-preview+json"), ]
        };

        let request = GitHubRequestBuilder::build(req, self.auth)?;

        // --

        let github_response = crate::adapters::fetch_async(request).await?;

        // --

        if github_response.is_success() {
            Ok(crate::adapters::to_json_async(github_response).await?)
        } else {
            match github_response.status_code() {
                304 => Err(CodesOfConductGetAllCodesOfConductError::Status304),
                415 => Err(CodesOfConductGetAllCodesOfConductError::Status415(crate::adapters::to_json_async(github_response).await?)),
                code => Err(CodesOfConductGetAllCodesOfConductError::Generic { code }),
            }
        }
    }

    /// ---
    ///
    /// # Get all codes of conduct
    /// 
    /// [GitHub API docs for get_all_codes_of_conduct](https://docs.github.com/rest/reference/codes_of_conduct/#get-all-codes-of-conduct)
    ///
    /// The `get_all_codes_of_conduct` endpoint is enabled with the `scarlet-witch` cargo feature.
    ///
    /// ---
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "scarlet-witch")]
    pub fn get_all_codes_of_conduct(&self) -> Result<Vec<CodeOfConduct>, CodesOfConductGetAllCodesOfConductError> {

        let request_uri = format!("{}/codes_of_conduct", super::GITHUB_BASE_API_URL);


        let req = GitHubRequest {
            uri: request_uri,
            body: None,
            method: "GET",
            headers: vec![("Accept", "application/vnd.github.scarlet-witch-preview+json"), ]
        };

        let request = GitHubRequestBuilder::build(req, self.auth)?;

        // --

        let github_response = crate::adapters::fetch(request)?;

        // --

        if github_response.is_success() {
            Ok(crate::adapters::to_json(github_response)?)
        } else {
            match github_response.status_code() {
                304 => Err(CodesOfConductGetAllCodesOfConductError::Status304),
                415 => Err(CodesOfConductGetAllCodesOfConductError::Status415(crate::adapters::to_json(github_response)?)),
                code => Err(CodesOfConductGetAllCodesOfConductError::Generic { code }),
            }
        }
    }

    /// ---
    ///
    /// # Get a code of conduct
    /// 
    /// [GitHub API docs for get_conduct_code](https://docs.github.com/rest/reference/codes_of_conduct/#get-a-code-of-conduct)
    ///
    /// The `get_conduct_code_async` endpoint is enabled with the `scarlet-witch` cargo feature.
    ///
    /// ---
    #[cfg(feature = "scarlet-witch")]
    pub async fn get_conduct_code_async(&self, key: &str) -> Result<CodeOfConduct, CodesOfConductGetConductCodeError> {

        let request_uri = format!("{}/codes_of_conduct/{}", super::GITHUB_BASE_API_URL, key);


        let req = GitHubRequest {
            uri: request_uri,
            body: None,
            method: "GET",
            headers: vec![("Accept", "application/vnd.github.scarlet-witch-preview+json"), ]
        };

        let request = GitHubRequestBuilder::build(req, self.auth)?;

        // --

        let github_response = crate::adapters::fetch_async(request).await?;

        // --

        if github_response.is_success() {
            Ok(crate::adapters::to_json_async(github_response).await?)
        } else {
            match github_response.status_code() {
                404 => Err(CodesOfConductGetConductCodeError::Status404(crate::adapters::to_json_async(github_response).await?)),
                304 => Err(CodesOfConductGetConductCodeError::Status304),
                415 => Err(CodesOfConductGetConductCodeError::Status415(crate::adapters::to_json_async(github_response).await?)),
                code => Err(CodesOfConductGetConductCodeError::Generic { code }),
            }
        }
    }

    /// ---
    ///
    /// # Get a code of conduct
    /// 
    /// [GitHub API docs for get_conduct_code](https://docs.github.com/rest/reference/codes_of_conduct/#get-a-code-of-conduct)
    ///
    /// The `get_conduct_code` endpoint is enabled with the `scarlet-witch` cargo feature.
    ///
    /// ---
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "scarlet-witch")]
    pub fn get_conduct_code(&self, key: &str) -> Result<CodeOfConduct, CodesOfConductGetConductCodeError> {

        let request_uri = format!("{}/codes_of_conduct/{}", super::GITHUB_BASE_API_URL, key);


        let req = GitHubRequest {
            uri: request_uri,
            body: None,
            method: "GET",
            headers: vec![("Accept", "application/vnd.github.scarlet-witch-preview+json"), ]
        };

        let request = GitHubRequestBuilder::build(req, self.auth)?;

        // --

        let github_response = crate::adapters::fetch(request)?;

        // --

        if github_response.is_success() {
            Ok(crate::adapters::to_json(github_response)?)
        } else {
            match github_response.status_code() {
                404 => Err(CodesOfConductGetConductCodeError::Status404(crate::adapters::to_json(github_response)?)),
                304 => Err(CodesOfConductGetConductCodeError::Status304),
                415 => Err(CodesOfConductGetConductCodeError::Status415(crate::adapters::to_json(github_response)?)),
                code => Err(CodesOfConductGetConductCodeError::Generic { code }),
            }
        }
    }

    /// ---
    ///
    /// # Get the code of conduct for a repository
    ///
    /// Returns the contents of the repository's code of conduct file, if one is detected.
    /// 
    /// A code of conduct is detected if there is a file named `CODE_OF_CONDUCT` in the root directory of the repository. GitHub detects which code of conduct it is using fuzzy matching.
    /// 
    /// [GitHub API docs for get_for_repo](https://docs.github.com/rest/reference/codes_of_conduct/#get-the-code-of-conduct-for-a-repository)
    ///
    /// The `get_for_repo_async` endpoint is enabled with the `scarlet-witch` cargo feature.
    ///
    /// ---
    #[cfg(feature = "scarlet-witch")]
    pub async fn get_for_repo_async(&self, owner: &str, repo: &str) -> Result<CodeOfConduct, CodesOfConductGetForRepoError> {

        let request_uri = format!("{}/repos/{}/{}/community/code_of_conduct", super::GITHUB_BASE_API_URL, owner, repo);


        let req = GitHubRequest {
            uri: request_uri,
            body: None,
            method: "GET",
            headers: vec![("Accept", "application/vnd.github.scarlet-witch-preview+json"), ]
        };

        let request = GitHubRequestBuilder::build(req, self.auth)?;

        // --

        let github_response = crate::adapters::fetch_async(request).await?;

        // --

        if github_response.is_success() {
            Ok(crate::adapters::to_json_async(github_response).await?)
        } else {
            match github_response.status_code() {
                code => Err(CodesOfConductGetForRepoError::Generic { code }),
            }
        }
    }

    /// ---
    ///
    /// # Get the code of conduct for a repository
    ///
    /// Returns the contents of the repository's code of conduct file, if one is detected.
    /// 
    /// A code of conduct is detected if there is a file named `CODE_OF_CONDUCT` in the root directory of the repository. GitHub detects which code of conduct it is using fuzzy matching.
    /// 
    /// [GitHub API docs for get_for_repo](https://docs.github.com/rest/reference/codes_of_conduct/#get-the-code-of-conduct-for-a-repository)
    ///
    /// The `get_for_repo` endpoint is enabled with the `scarlet-witch` cargo feature.
    ///
    /// ---
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "scarlet-witch")]
    pub fn get_for_repo(&self, owner: &str, repo: &str) -> Result<CodeOfConduct, CodesOfConductGetForRepoError> {

        let request_uri = format!("{}/repos/{}/{}/community/code_of_conduct", super::GITHUB_BASE_API_URL, owner, repo);


        let req = GitHubRequest {
            uri: request_uri,
            body: None,
            method: "GET",
            headers: vec![("Accept", "application/vnd.github.scarlet-witch-preview+json"), ]
        };

        let request = GitHubRequestBuilder::build(req, self.auth)?;

        // --

        let github_response = crate::adapters::fetch(request)?;

        // --

        if github_response.is_success() {
            Ok(crate::adapters::to_json(github_response)?)
        } else {
            match github_response.status_code() {
                code => Err(CodesOfConductGetForRepoError::Generic { code }),
            }
        }
    }

}
