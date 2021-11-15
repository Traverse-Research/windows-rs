#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ContentAccessRestrictionLevel(pub i32);
impl ContentAccessRestrictionLevel {
    pub const Allow: Self = Self(0i32);
    pub const Warn: Self = Self(1i32);
    pub const Block: Self = Self(2i32);
    pub const Hide: Self = Self(3i32);
}
impl ::core::marker::Copy for ContentAccessRestrictionLevel {}
impl ::core::clone::Clone for ContentAccessRestrictionLevel {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContentRestrictionsBrowsePolicy(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContentRestrictionsBrowsePolicy {}
impl ::core::clone::Clone for ContentRestrictionsBrowsePolicy {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContentRestrictionsBrowsePolicy(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContentRestrictionsBrowsePolicy {}
impl ::core::clone::Clone for IContentRestrictionsBrowsePolicy {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRatedContentDescription(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRatedContentDescription {}
impl ::core::clone::Clone for IRatedContentDescription {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRatedContentDescriptionFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRatedContentDescriptionFactory {}
impl ::core::clone::Clone for IRatedContentDescriptionFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRatedContentRestrictions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRatedContentRestrictions {}
impl ::core::clone::Clone for IRatedContentRestrictions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRatedContentRestrictionsFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRatedContentRestrictionsFactory {}
impl ::core::clone::Clone for IRatedContentRestrictionsFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RatedContentCategory(pub i32);
impl RatedContentCategory {
    pub const General: Self = Self(0i32);
    pub const Application: Self = Self(1i32);
    pub const Game: Self = Self(2i32);
    pub const Movie: Self = Self(3i32);
    pub const Television: Self = Self(4i32);
    pub const Music: Self = Self(5i32);
}
impl ::core::marker::Copy for RatedContentCategory {}
impl ::core::clone::Clone for RatedContentCategory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RatedContentDescription(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RatedContentDescription {}
impl ::core::clone::Clone for RatedContentDescription {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RatedContentRestrictions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RatedContentRestrictions {}
impl ::core::clone::Clone for RatedContentRestrictions {
    fn clone(&self) -> Self {
        *self
    }
}