/* Copyright (c) Fortanix, Inc.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#![doc(html_logo_url = "https://edp.fortanix.com/img/docs/edp-logo.svg",
       html_favicon_url = "https://edp.fortanix.com/favicon.ico",
       html_root_url = "https://edp.fortanix.com/docs/api/")]

#[macro_use]
extern crate num_derive;
extern crate sgx_isa;

use sgx_isa::{Report, Targetinfo};

/// Possible errors generated by the quote interface.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FromPrimitive, ToPrimitive)]
pub enum Quote3Error {
    ///< Success
    Success = 0,
    ///< The parameter is incorrect
    InvalidParameter = 0xe002,
    ///< Not enough memory is available to complete this operation
    OutOfMemory = 0xe003,
    ///< Expected ECDSA_ID does not match the value stored in the ECDSA Blob
    EcdsaIdMismatch = 0xe004,
    ///< The ECDSA blob pathname is too large
    PathnameBufferOverflow = 0xe005,
    ///< Error accessing ECDSA blob
    FileAccessError = 0xe006,
    ///< Cached ECDSA key is invalid
    StoredKeyInvalid = 0xe007,
    ///< Cached ECDSA key does not match requested key
    PubKeyIdMismatch = 0xe008,
    ///< PCE use the incorrect signature scheme
    InvalidPceSigScheme = 0xe009,
    ///< There is a problem with the attestation key blob.
    AttKeyBlobInvalid = 0xe00a,
    ///< Unsupported attestation key ID.
    UnsupportedAttKeyId = 0xe00b,
    ///< Unsupported enclave loading policy.
    UnsupportedLoadingPolicy = 0xe00c,
    ///< Unable to load the QE enclave
    InterfaceUnavailable = 0xe00d,
    ///< Unable to find the platform library with the dependent APIs.  Not fatal.
    PlatformLibUnavailable = 0xe00e,
    ///< The attestation key doesn't exist or has not been certified.
    AttKeyNotInitialized = 0xe00f,
    ///< The certification data retrieved from the platform library is invalid.
    AttKeyCertDataInvalid = 0xe010,
    ///< The platform library doesn't have any platfrom cert data.
    NoPlatformCertData = 0xe011,
    ///< Not enough memory in the EPC to load the enclave.
    OutOfEpc = 0xe012,
    ///< There was a problem verifying an SGX REPORT.
    ReportInvalid = 0xe013,
    ///< Interfacing to the enclave failed due to a power transition.
    EnclaveLost = 0xe014,
    ///< Error verifying the application enclave's report.
    InvalidReport = 0xe015,
    ///< Unable to load the enclaves.  Could be due to file I/O error, loading infrastructure error.
    EnclaveLoadFailure = 0xe016,
    ///< The QE was unable to generate its own report targeting the application enclave either
    ///< because the QE doesn't support this feature there is an enclave compatibility issue.
    ///< Please call again with the p_qe_report_info to NULL.
    UnableToGenerateQeReport = 0xe017,
    ///< Caused when the provider library returns an invalid TCB (too high).
    KeyCertifcationError = 0xe018,
}

#[cfg(feature = "link")]
#[link(name = "sgx_dcap_ql")]
extern "C" {
    #[link_name = "sgx_qe_get_target_info"]
    pub fn get_target_info(target_info: &mut Targetinfo) -> u32;
    #[link_name = "sgx_qe_get_quote_size"]
    pub fn get_quote_size(quote_size: &mut u32) -> u32;
    #[link_name = "sgx_qe_get_quote"]
    pub fn get_quote(report: &Report, quote_size: u32, quote: *mut u8) -> u32;
}

pub const LIBRARY: &str = "libsgx_dcap_ql.so.1";

pub const SYM_GET_TARGET_INFO: &[u8] = b"sgx_qe_get_target_info\0";
pub type GetTargetInfoFn = unsafe extern "C" fn(target_info: &mut Targetinfo) -> u32;
pub const SYM_GET_QUOTE_SIZE: &[u8] = b"sgx_qe_get_quote_size\0";
pub type GetQuoteSizeFn = unsafe extern "C" fn(quote_size: &mut u32) -> u32;
pub const SYM_GET_QUOTE: &[u8] = b"sgx_qe_get_quote\0";
pub type GetQuoteFn = unsafe extern "C" fn(report: &Report, quote_size: u32, quote: *mut u8) -> u32;
