//! Tests for the shared structured error vocabulary.
//!
//! `wist-error` centralizes the reason enums used across `wist-center` /
//! `wist-gateway` / `wist-control` / `wist-security` / `wist-reporting`. These
//! tests pin down the two contracts that matter most:
//!
//! 1. the stable numeric code + tag mapping ([`wist_error::SysErrorCode`]), and
//! 2. the stable identity string (`stable_code`) of each reason variant.

use orion_error::conversion::ToStructError;
use orion_error::prelude::UnifiedReason;
use orion_error::reason::ErrorIdentityProvider;

use wist_error::{AppReason, ConfigReason, StoreReason, SysErrorCode};

#[test]
fn store_reason_sys_codes_and_tags() {
    let cases = [
        (StoreReason::Io, 50001u16, "store"),
        (StoreReason::Json, 50002, "store"),
        (StoreReason::Sql, 50003, "store"),
        (StoreReason::Conflict, 40901, "store"),
        (StoreReason::NotFound, 40401, "store"),
        (StoreReason::Enrollment, 40101, "store"),
    ];
    for (reason, code, tag) in cases {
        assert_eq!(reason.sys_code(), code);
        assert_eq!(reason.sys_tag(), tag);
    }
}

#[test]
fn config_reason_sys_codes_and_tags() {
    let cases = [
        (ConfigReason::Io, 50011u16, "config"),
        (ConfigReason::Parse, 42211, "config"),
        (ConfigReason::Validation, 42212, "config"),
    ];
    for (reason, code, tag) in cases {
        assert_eq!(reason.sys_code(), code);
        assert_eq!(reason.sys_tag(), tag);
    }
}

#[test]
fn app_reason_sys_codes_and_tags() {
    let cases = [
        (AppReason::Store, 50020u16, "app"),
        (AppReason::Config, 50021, "app"),
        (AppReason::InvalidArgs, 40001, "app"),
    ];
    for (reason, code, tag) in cases {
        assert_eq!(reason.sys_code(), code);
        assert_eq!(reason.sys_tag(), tag);
    }
}

#[test]
fn store_reason_identities() {
    let cases = [
        (StoreReason::Io, "sys.wist.store.io"),
        (StoreReason::Json, "sys.wist.store.json"),
        (StoreReason::Sql, "sys.wist.store.sql"),
        (StoreReason::Conflict, "biz.wist.store.conflict"),
        (StoreReason::NotFound, "biz.wist.store.not_found"),
        (StoreReason::Enrollment, "biz.wist.store.enrollment"),
    ];
    for (reason, identity) in cases {
        assert_eq!(reason.to_err().reason().stable_code(), identity);
    }
}

#[test]
fn config_reason_identities() {
    let cases = [
        (ConfigReason::Io, "conf.wist.config.io"),
        (ConfigReason::Parse, "conf.wist.config.parse"),
        (ConfigReason::Validation, "conf.wist.config.validation"),
    ];
    for (reason, identity) in cases {
        assert_eq!(reason.to_err().reason().stable_code(), identity);
    }
}

#[test]
fn app_reason_identities() {
    let cases = [
        (AppReason::Store, "biz.wist.app.store"),
        (AppReason::Config, "conf.wist.app.config"),
        (AppReason::InvalidArgs, "biz.wist.app.invalid_args"),
    ];
    for (reason, identity) in cases {
        assert_eq!(reason.to_err().reason().stable_code(), identity);
    }
}

#[test]
fn store_reason_lifts_to_app_reason() {
    assert_eq!(AppReason::from(StoreReason::Io), AppReason::Store);
    assert_eq!(AppReason::from(StoreReason::Conflict), AppReason::Store);
    assert_eq!(AppReason::from(StoreReason::Enrollment), AppReason::Store);
}

#[test]
fn config_reason_lifts_to_app_reason() {
    assert_eq!(AppReason::from(ConfigReason::Io), AppReason::Config);
    assert_eq!(AppReason::from(ConfigReason::Parse), AppReason::Config);
    assert_eq!(AppReason::from(ConfigReason::Validation), AppReason::Config);
}

#[test]
fn error_carries_detail_and_reason() {
    let err = StoreReason::NotFound.to_err().with_detail("no such record");
    assert_eq!(err.reason(), &StoreReason::NotFound);
    assert_eq!(err.detail().as_deref(), Some("no such record"));
}

#[test]
fn general_variant_maps_to_uvs_code_and_tag() {
    let store = StoreReason::General(UnifiedReason::not_found_error());
    assert_eq!(store.sys_code(), 50000);
    assert_eq!(store.sys_tag(), "store");

    let config = ConfigReason::General(UnifiedReason::validation_error());
    assert_eq!(config.sys_code(), 50010);
    assert_eq!(config.sys_tag(), "config");

    let app = AppReason::General(UnifiedReason::business_error());
    assert_eq!(app.sys_code(), 50029);
    assert_eq!(app.sys_tag(), "app");
}

#[test]
fn general_variant_lifts_to_app_reason_transparently() {
    let reason = UnifiedReason::not_found_error();

    let store = StoreReason::General(reason.clone());
    assert_eq!(AppReason::from(store), AppReason::General(reason.clone()));

    let config = ConfigReason::General(reason.clone());
    assert_eq!(AppReason::from(config), AppReason::General(reason));
}
