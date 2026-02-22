//!
//! Attribute-related structs
//!
use std::collections::HashMap;

use bytes::{BufMut, Bytes, BytesMut};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use crate::parser::IppParseError;
use crate::{model::DelimiterTag, value::IppValue};

fn is_header_attr(attr: &str) -> bool {
    IppAttribute::HEADER_ATTRS.contains(&attr)
}

/// `IppAttribute` represents an IPP attribute
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct IppAttribute {
    /// Attribute name
    name: String,
    /// Attribute value
    value: IppValue,
}

#[cfg(feature = "ipp3d")]
impl IppAttribute {
    
    pub const IPP3D_OPERATION_ATTRIBUTES: &'static [&'static str] = &[
        IppAttribute::COMPRESSION,
        IppAttribute::DOCUMENT_FORMAT,
        IppAttribute::DOCUMENT_NAME,
        IppAttribute::FIRST_INDEX,
        IppAttribute::IDENTIFY_ACTIONS,
        IppAttribute::IPP_ATTRIBUTE_FIDELITY,
        IppAttribute::JOB_IDS,
        IppAttribute::JOB_MANDATORY_ATTRIBUTES,
        IppAttribute::JOB_NAME,
        IppAttribute::LAST_DOCUMENT,
        IppAttribute::LIMIT,
        IppAttribute::REQUESTING_USER_NAME,
        IppAttribute::REQUESTING_USER_URI,
        IppAttribute::WHICH_JOBS
    ];

    // printer description attributes extension for IPP 3D (PWG 5100.21) section 6.5
    pub const ACCURACY_UNITS_SUPPORTED: &'static str = "accuracy-units-supported";
    pub const MATERIAL_DIAMETER_SUPPORTED: &'static str = "material-diameter-supported";
    pub const MATERIAL_PURPOSE_SUPPORTED: &'static str = "material-purpose-supported";
    pub const MATERIAL_RATE_SUPPORTED: &'static str = "material-rate-supported";
    pub const MATERIAL_RATE_UNITS_SUPPORTED: &'static str = "material-rate-units-supported";
    pub const MATERIAL_SHELL_THICKNESS_SUPPORTED: &'static str =
        "material-shell-thickness-supported";
    pub const MATERIAL_TEMPERATURE_SUPPORTED: &'static str = "material-temperature-supported";
    pub const MATERIAL_TYPE_SUPPORTED: &'static str = "material-type-supported";
    pub const MATERIALS_COL_DEFAULT: &'static str = "materials-col-default";
    pub const MATERIALS_COL_READY: &'static str = "materials-col-ready";
    pub const MATERIALS_COL_SUPPORTED: &'static str = "materials-col-supported";
    pub const MAX_MATERIALS_COL_SUPPORTED: &'static str = "max-materials-col-supported";
    pub const MULTIPLE_OBJECT_HANDLING_DEFAULT: &'static str =
        "multiple-object-handling-default";
    pub const MULTIPLE_OBJECT_HANDLING_SUPPORTED: &'static str =
        "multiple-object-handling-supported";
    pub const PLATFORM_TEMPERATURE_DEFAULT: &'static str = "platform-temperature-default";
    pub const PLATFORM_TEMPERATURE_SUPPORTED: &'static str =
        "platform-temperature-supported";
    pub const PRINT_ACCURACY_DEFAULT: &'static str = "print-accuracy-default";
    pub const PRINT_ACCURACY_SUPPORTED: &'static str = "print-accuracy-supported";
    pub const PRINT_BASE_DEFAULT: &'static str = "print-base-default";
    pub const PRINT_BASE_SUPPORTED: &'static str = "print-base-supported";
    pub const PRINT_OBJECTS_SUPPORTED: &'static str = "print-objects-supported";
    pub const PRINT_SUPPORTS_DEFAULT: &'static str = "print-supports-default";
    pub const PRINT_SUPPORTS_SUPPORTED: &'static str = "print-supports-supported";
    pub const PRINTER_VOLUME_SUPPORTED: &'static str = "printer-volume-supported";

    pub const IPP3D_PRINTER_DESCRIPTION_ATTRIBUTES: &'static [&'static str] = &[
        IppAttribute::ACCURACY_UNITS_SUPPORTED,
        IppAttribute::CHARSET_CONFIGURED,
        IppAttribute::CHARSET_SUPPORTED,
        IppAttribute::COLOR_SUPPORTED,
        IppAttribute::COMPRESSION_SUPPORTED,
        IppAttribute::DOCUMENT_FORMAT_DEFAULT,
        IppAttribute::DOCUMENT_FORMAT_SUPPORTED,
        IppAttribute::GENERATED_NATURAL_LANGUAGE_SUPPORTED,
        IppAttribute::IDENTIFY_ACTIONS_DEFAULT,
        IppAttribute::IDENTIFY_ACTIONS_SUPPORTED,
        IppAttribute::IPP_FEATURES_SUPPORTED,
        IppAttribute::IPP_VERSIONS_SUPPORTED,
        IppAttribute::JOB_CREATION_ATTRIBUTES_SUPPORTED,
        IppAttribute::JOB_IDS_SUPPORTED,
        IppAttribute::MATERIAL_DIAMETER_SUPPORTED,
        IppAttribute::MATERIAL_PURPOSE_SUPPORTED,
        IppAttribute::MATERIAL_RATE_SUPPORTED,
        IppAttribute::MATERIAL_RATE_UNITS_SUPPORTED,
        IppAttribute::MATERIAL_SHELL_THICKNESS_SUPPORTED,
        IppAttribute::MATERIAL_TEMPERATURE_SUPPORTED,
        IppAttribute::MATERIAL_TYPE_SUPPORTED,
        IppAttribute::MATERIALS_COL_DEFAULT,
        IppAttribute::MATERIALS_COL_READY,
        IppAttribute::MATERIALS_COL_SUPPORTED,
        IppAttribute::MAX_MATERIALS_COL_SUPPORTED,
        IppAttribute::MULTIPLE_DOCUMENT_JOBS_SUPPORTED,
        IppAttribute::MULTIPLE_OBJECT_HANDLING_DEFAULT,
        IppAttribute::MULTIPLE_OBJECT_HANDLING_SUPPORTED,
        IppAttribute::MULTIPLE_OPERATION_TIMEOUT,
        IppAttribute::MULTIPLE_OPERATION_TIMEOUT_ACTION,
        IppAttribute::NATURAL_LANGUAGE_CONFIGURED,
        IppAttribute::OPERATIONS_SUPPORTED,
        IppAttribute::PLATFORM_TEMPERATURE_DEFAULT,
        IppAttribute::PLATFORM_TEMPERATURE_SUPPORTED,
        IppAttribute::PRINT_ACCURACY_DEFAULT,
        IppAttribute::PRINT_ACCURACY_SUPPORTED,
        IppAttribute::PRINT_BASE_DEFAULT,
        IppAttribute::PRINT_BASE_SUPPORTED,
        IppAttribute::PRINT_OBJECTS_SUPPORTED,
        IppAttribute::PRINT_QUALITY_DEFAULT,
        IppAttribute::PRINT_QUALITY_SUPPORTED,
        IppAttribute::PRINT_SUPPORTS_DEFAULT,
        IppAttribute::PRINT_SUPPORTS_SUPPORTED,
        IppAttribute::PRINTER_GEO_LOCATION,
        IppAttribute::PRINTER_GET_ATTRIBUTES_SUPPORTED,
        IppAttribute::PRINTER_ICONS,
        IppAttribute::PRINTER_INFO,
        IppAttribute::PRINTER_LOCATION,
        IppAttribute::PRINTER_MAKE_AND_MODEL,
        IppAttribute::PRINTER_MORE_INFO,
        IppAttribute::PRINTER_NAME,
        IppAttribute::PRINTER_ORGANIZATION,
        IppAttribute::PRINTER_ORGANIZATIONAL_UNIT,
        IppAttribute::PRINTER_VOLUME_SUPPORTED,
        IppAttribute::PRINTER_XRI_SUPPORTED,
        IppAttribute::WHICH_JOBS_SUPPORTED,
    ];

    // printer status attributes extension for IPP 3D see PWG 5100.21 section 6.6
    pub const PRINTER_CAMERA_IMAGE_URI: &'static str = "printer-camera-image-uri";

    pub const IPP3D_PRINTER_STATUS_ATTRIBUTES: &'static [&'static str] = &[
        IppAttribute::PRINTER_CAMERA_IMAGE_URI,
        IppAttribute::PRINTER_CONFIG_CHANGE_DATE_TIME,
        IppAttribute::PRINTER_CONFIG_CHANGE_TIME,
        IppAttribute::PRINTER_IS_ACCEPTING_JOBS,
        IppAttribute::PRINTER_STATE,
        IppAttribute::PRINTER_STATE_CHANGE_DATE_TIME,
        IppAttribute::PRINTER_STATE_CHANGE_TIME,
        IppAttribute::PRINTER_STATE_MESSAGE,
        IppAttribute::PRINTER_STATE_REASONS,
        IppAttribute::PRINTER_UP_TIME,
        IppAttribute::PRINTER_URI_SUPPORTED,
        IppAttribute::PRINTER_UUID,
        IppAttribute::QUEUED_JOB_COUNT,
        IppAttribute::URI_AUTHENTICATION_SUPPORTED,
        IppAttribute::URI_SECURITY_SUPPORTED,
        IppAttribute::XRI_AUTHENTICATION_SUPPORTED,
        IppAttribute::XRI_SECURITY_SUPPORTED,
        IppAttribute::XRI_URI_SCHEME_SUPPORTED,
    ];

    pub const IPP3D_JOB_DESCRIPTION_ATTRIBUTES: &'static [&'static str] = &[
        IppAttribute::JOB_NAME,
    ];

    // job template attributes extension for IPP 3D see PWG 5100.21 section 6.7
    pub const MATERIALS_COL: &'static str = "materials-col";
    pub const MULTIPLE_OBJECT_HANDLING: &'static str = "multiple-object-handling";
    pub const PLATFORM_TEMPERATURE: &'static str = "platform-temperature";
    pub const PRINT_ACCURACY: &'static str = "print-accuracy";
    pub const PRINT_BASE: &'static str = "print-base";
    pub const PRINT_OBJECTS: &'static str = "print-objects";
    pub const PRINT_SUPPORTS: &'static str = "print-supports";

    pub const IPP3D_JOB_TEMPLATE_ATTRS: &'static [&'static str] = &[
        IppAttribute::MATERIALS_COL,
        IppAttribute::MULTIPLE_DOCUMENT_HANDLING,
        IppAttribute::MULTIPLE_OBJECT_HANDLING,
        IppAttribute::PLATFORM_TEMPERATURE,
        IppAttribute::PRINT_ACCURACY,
        IppAttribute::PRINT_BASE,
        IppAttribute::PRINT_OBJECTS,
        IppAttribute::PRINT_QUALITY,
        IppAttribute::PRINT_SUPPORTS,
    ];

    // job status attributes extension for IPP 3D see PWG 5100.21 section 6.9
    pub const MATERIALS_COL_ACTUAL: &'static str = "materials-col-actual";
    pub const MULTIPLE_OBJECT_HANDLING_ACTUAL: &'static str =
        "multiple-object-handling-actual";
    pub const PLATFORM_TEMPERATURE_ACTUAL: &'static str = "platform-temperature-actual";
    pub const PRINT_ACCURACY_ACTUAL: &'static str = "print-accuracy-actual";
    pub const PRINT_BASE_ACTUAL: &'static str = "print-base-actual";
    pub const PRINT_OBJECTS_ACTUAL: &'static str = "print-objects-actual";
    pub const PRINT_SUPPORTS_ACTUAL: &'static str = "print-supports-actual";

    /// the required attributes for job status reporting
    pub const IPP3D_JOB_STATUS_ATTRS: &'static [&'static str] = &[
        IppAttribute::COMPRESSION_SUPPLIED,
        IppAttribute::DATE_TIME_AT_COMPLETED,
        IppAttribute::DATE_TIME_AT_CREATION,
        IppAttribute::DATE_TIME_AT_PROCESSING,
        IppAttribute::DOCUMENT_FORMAT_SUPPLIED,
        IppAttribute::DOCUMENT_NAME_SUPPLIED,
        IppAttribute::JOB_ID,
        IppAttribute::JOB_ORIGINATING_USER_NAME,
        IppAttribute::JOB_PRINTER_UPTIME,
        IppAttribute::JOB_PRINTER_URI,
        IppAttribute::JOB_STATE,
        IppAttribute::JOB_STATE_MESSAGE,
        IppAttribute::JOB_STATE_REASONS,
        IppAttribute::JOB_URI,
        IppAttribute::JOB_UUID,

        IppAttribute::MATERIALS_COL_ACTUAL,
        IppAttribute::MULTIPLE_OBJECT_HANDLING_ACTUAL,
        IppAttribute::PLATFORM_TEMPERATURE_ACTUAL,
        IppAttribute::PRINT_ACCURACY_ACTUAL,
        IppAttribute::PRINT_BASE_ACTUAL,
        IppAttribute::PRINT_OBJECTS_ACTUAL,
        IppAttribute::PRINT_SUPPORTS_ACTUAL,

        IppAttribute::TIME_AT_COMPLETED,
        IppAttribute::TIME_AT_CREATION,
        IppAttribute::TIME_AT_PROCESSING,
    ];


}

/// see PWG 5100.5 (also includes RFC 3995 definitions)
#[cfg(feature = "ipp-notifications")]
impl IppAttribute {
    // according to PWG 5100.21 should be included in RFC 8011
    pub const DOCUMENT_NAME: &'static str = "document-name";
    pub const PRINTER_STATE_CHANGE_DATE_TIME: &'static str = "printer-state-change-date-time";
    pub const PRINTER_STATE_CHANGE_TIME: &'static str = "printer-state-change-time";

}

/// see PWG 5100.7
#[cfg(feature = "ipp-system-service")]
/// source: https://ftp.pwg.org/pub/pwg/candidates/cs-ippjobext21-20230210-5100.7.pdf
impl IppAttribute {
    /// required
    pub const JOB_MANDATORY_ATTRIBUTES: &'static str = "job-mandatory-attributes";
    
    pub const COMPRESSION_SUPPLIED: &'static str = "compression-supplied";
    pub const DOCUMENT_FORMAT_SUPPLIED: &'static str = "document-format-supplied";
    pub const DOCUMENT_NAME_SUPPLIED: &'static str = "document-name-supplied";

    /// according to PWG 5100.7 section 6.1 required
    pub const JOB_IDS: &'static str = "job-ids";
    
    /// set of collection (see `crate::ClientInfo`) recommended not required
    pub const CLIENT_INFO: &'static str = "client-info";
    /// recommended not required
    pub const JOB_HOLD_UNTIL_TIME: &'static str = "job-hold-until-time";
}

/// see PWG 5100.11
#[cfg(feature = "ipp-job-extensions")]
impl IppAttribute {
    // according to PWG 5100.21 should be included in RFC 8011
    pub const WHICH_JOBS: &'static str = "which-jobs";
    pub const JOB_CREATION_ATTRIBUTES_SUPPORTED: &'static str = "job-creation-attributes-supported";
    pub const JOB_IDS_SUPPORTED: &'static str = "job-ids-supported";

    pub const WHICH_JOBS_SUPPORTED: &'static str = "which-jobs-supported";
}

/// see PWG 5100.13
#[cfg(feature = "ipp-set-extensions")]
impl IppAttribute {
    // operation attributes
    pub const FIRST_INDEX: &'static str = "first-index";
    pub const IDENTIFY_ACTIONS: &'static str = "identify-actions";
    pub const REQUESTING_USER_URI: &'static str = "requesting-user-uri";

    pub const IDENTIFY_ACTIONS_DEFAULT: &'static str = "identify-actions-default";
    pub const IDENTIFY_ACTIONS_SUPPORTED: &'static str = "identify-actions-supported";
    pub const IPP_FEATURES_SUPPORTED: &'static str = "ipp-features-supported";

    pub const MULTIPLE_OPERATION_TIMEOUT_ACTION: &'static str = "multiple-operation-timeout-action";
    pub const PRINTER_GEO_LOCATION: &'static str = "printer-geo-location";
    pub const PRINTER_GET_ATTRIBUTES_SUPPORTED: &'static str = "printer-get-attributes-supported";
    pub const PRINTER_ICONS: &'static str = "printer-icons";

    pub const PRINTER_ORGANIZATION: &'static str = "printer-organization";
    pub const PRINTER_ORGANIZATIONAL_UNIT: &'static str = "printer-organizational-unit";

    pub const PRINTER_CONFIG_CHANGE_DATE_TIME: &'static str = "printer-config-change-date-time";
    pub const PRINTER_CONFIG_CHANGE_TIME: &'static str = "printer-config-change-time";
    pub const JOB_UUID: &'static str = "job-uuid";
}

/// see RFC 3380
#[cfg(feature = "ipp-job-set-operations")]
impl IppAttribute {
    pub const PRINTER_XRI_SUPPORTED: &'static str = "printer-xri-supported";
    pub const XRI_AUTHENTICATION_SUPPORTED: &'static str = "xri-authentication-supported";
    pub const XRI_SECURITY_SUPPORTED: &'static str = "xri-security-supported";
    pub const XRI_URI_SCHEME_SUPPORTED: &'static str = "xri-uri-scheme-supported";
}

impl IppAttribute {
    // here I've added what the spec says is in RFC 8011 that seemed to be missing
    pub const COMPRESSION: &'static str = "compression";
    pub const IPP_ATTRIBUTE_FIDELITY: &'static str = "ipp-attribute-fidelity";
    pub const LIMIT: &'static str = "limit";
    pub const MULTIPLE_DOCUMENT_JOBS_SUPPORTED: &'static str = "multiple-document-jobs-supported";
    pub const MULTIPLE_OPERATION_TIMEOUT: &'static str = "multiple-operation-timeout";
    pub const DATE_TIME_AT_COMPLETED: &'static str = "date-time-at-completed";
    pub const DATE_TIME_AT_CREATION: &'static str = "date-time-at-creation";
    pub const DATE_TIME_AT_PROCESSING: &'static str = "date-time-at-processing";
    pub const TIME_AT_COMPLETED: &'static str = "time-at-completed";
    pub const TIME_AT_CREATION: &'static str = "time-at-creation";
    pub const TIME_AT_PROCESSING: &'static str = "time-at-processing";
    pub const JOB_ORIGINATING_USER_NAME: &'static str = "job-originating-user-name";
    pub const JOB_PRINTER_UPTIME: &'static str = "job-printer-up-time";
    pub const JOB_PRINTER_URI: &'static str = "job-printer-uri";
    pub const JOB_STATE_MESSAGE: &'static str = "job-state-message";

    // original untouched attributes
    pub const ATTRIBUTES_CHARSET: &'static str = "attributes-charset";
    pub const ATTRIBUTES_NATURAL_LANGUAGE: &'static str = "attributes-natural-language";
    pub const CHARSET_CONFIGURED: &'static str = "charset-configured";
    pub const CHARSET_SUPPORTED: &'static str = "charset-supported";
    pub const COLOR_MODE_SUPPORTED: &'static str = "color-mode-supported";
    pub const COLOR_SUPPORTED: &'static str = "color-supported";
    pub const COMPRESSION_SUPPORTED: &'static str = "compression-supported";
    pub const COPIES: &'static str = "copies";
    pub const COPIES_DEFAULT: &'static str = "copies-default";
    pub const COPIES_SUPPORTED: &'static str = "copies-supported";
    pub const DOCUMENT_FORMAT: &'static str = "document-format";
    pub const DOCUMENT_FORMAT_DEFAULT: &'static str = "document-format-default";
    pub const DOCUMENT_FORMAT_PREFERRED: &'static str = "document-format-preferred";
    pub const DOCUMENT_FORMAT_SUPPORTED: &'static str = "document-format-supported";
    pub const FINISHINGS: &'static str = "finishings";
    pub const FINISHINGS_DEFAULT: &'static str = "finishings-default";
    pub const FINISHINGS_SUPPORTED: &'static str = "finishings-supported";
    pub const GENERATED_NATURAL_LANGUAGE_SUPPORTED: &'static str = "generated-natural-language-supported";
    pub const IPP_VERSIONS_SUPPORTED: &'static str = "ipp-versions-supported";
    pub const JOB_ID: &'static str = "job-id";
    pub const JOB_NAME: &'static str = "job-name";
    pub const JOB_STATE: &'static str = "job-state";
    pub const JOB_STATE_REASONS: &'static str = "job-state-reasons";
    pub const JOB_URI: &'static str = "job-uri";
    pub const LAST_DOCUMENT: &'static str = "last-document";
    pub const MEDIA_COL: &'static str = "media-col";
    pub const MEDIA_COL_DATABASE: &'static str = "media-col-database";
    pub const MEDIA_COL_DEFAULT: &'static str = "media-col-default";
    pub const MEDIA_COL_READY: &'static str = "media-col-ready";
    pub const MEDIA_COL_SUPPORTED: &'static str = "media-col-supported";
    pub const MEDIA_DEFAULT: &'static str = "media-default";
    pub const MEDIA_READY: &'static str = "media-ready";
    pub const MEDIA_SOURCE_SUPPORTED: &'static str = "media-source-supported";
    pub const MEDIA_SUPPORTED: &'static str = "media-supported";
    pub const MEDIA_TYPE_SUPPORTED: &'static str = "media-type-supported";
    pub const MOPRIA_CERTIFIED: &'static str = "mopria-certified";
    pub const MULTIPLE_DOCUMENT_HANDLING: &'static str = "multiple-document-handling";
    pub const MULTIPLE_DOCUMENT_HANDLING_DEFAULT: &'static str = "multiple-document-handling-default";
    pub const MULTIPLE_DOCUMENT_HANDLING_SUPPORTED: &'static str = "multiple-document-handling-supported";
    pub const NATURAL_LANGUAGE_CONFIGURED: &'static str = "natural-language-configured";
    pub const OPERATIONS_SUPPORTED: &'static str = "operations-supported";
    pub const ORIENTATION_REQUESTED: &'static str = "orientation-requested";
    pub const ORIENTATION_REQUESTED_DEFAULT: &'static str = "orientation-requested-default";
    pub const ORIENTATION_REQUESTED_SUPPORTED: &'static str = "orientation-requested-supported";
    pub const OUTPUT_BIN: &'static str = "output-bin";
    pub const OUTPUT_BIN_DEFAULT: &'static str = "output-bin-default";
    pub const OUTPUT_BIN_SUPPORTED: &'static str = "output-bin-supported";
    pub const OUTPUT_MODE_SUPPORTED: &'static str = "output-mode-supported";
    pub const PAGES_PER_MINUTE: &'static str = "pages-per-minute";
    pub const PDL_OVERRIDE_SUPPORTED: &'static str = "pdl-override-supported";
    pub const PRINTER_DEVICE_ID: &'static str = "printer-device-id";
    pub const PRINTER_FIRMWARE_NAME: &'static str = "printer-firmware-name";
    pub const PRINTER_FIRMWARE_STRING_VERSION: &'static str = "printer-firmware-string-version";
    pub const PRINTER_INFO: &'static str = "printer-info";
    pub const PRINTER_IS_ACCEPTING_JOBS: &'static str = "printer-is-accepting-jobs";
    pub const PRINTER_LOCATION: &'static str = "printer-location";
    pub const PRINTER_MAKE_AND_MODEL: &'static str = "printer-make-and-model";
    pub const PRINTER_MORE_INFO: &'static str = "printer-more-info";
    pub const PRINTER_NAME: &'static str = "printer-name";
    pub const PRINTER_RESOLUTION: &'static str = "printer-resolution";
    pub const PRINTER_RESOLUTION_DEFAULT: &'static str = "printer-resolution-default";
    pub const PRINTER_RESOLUTION_SUPPORTED: &'static str = "printer-resolution-supported";
    pub const PRINTER_STATE: &'static str = "printer-state";
    pub const PRINTER_STATE_MESSAGE: &'static str = "printer-state-message";
    pub const PRINTER_STATE_REASONS: &'static str = "printer-state-reasons";
    pub const PRINTER_UP_TIME: &'static str = "printer-up-time";
    pub const PRINTER_URI: &'static str = "printer-uri";
    pub const PRINTER_URI_SUPPORTED: &'static str = "printer-uri-supported";
    pub const PRINTER_UUID: &'static str = "printer-uuid";
    pub const PRINT_COLOR_MODE: &'static str = "print-color-mode";
    pub const PRINT_COLOR_MODE_DEFAULT: &'static str = "print-color-mode-default";
    pub const PRINT_COLOR_MODE_SUPPORTED: &'static str = "print-color-mode-supported";
    pub const PRINT_QUALITY: &'static str = "print-quality";
    pub const PRINT_QUALITY_DEFAULT: &'static str = "print-quality-default";
    pub const PRINT_QUALITY_SUPPORTED: &'static str = "print-quality-supported";
    pub const QUEUED_JOB_COUNT: &'static str = "queued-job-count";
    pub const REQUESTED_ATTRIBUTES: &'static str = "requested-attributes";
    pub const REQUESTING_USER_NAME: &'static str = "requesting-user-name";
    pub const SIDES: &'static str = "sides";
    pub const SIDES_DEFAULT: &'static str = "sides-default";
    pub const SIDES_SUPPORTED: &'static str = "sides-supported";
    pub const STATUS_MESSAGE: &'static str = "status-message";
    pub const URI_AUTHENTICATION_SUPPORTED: &'static str = "uri-authentication-supported";
    pub const URI_SECURITY_SUPPORTED: &'static str = "uri-security-supported";

    // Per section 4.1.4. Character Set and Natural Language Operation Attributes
    // The "attributes-charset" and "attributes-natural-language" attributes MUST be the first two attributes
    // in every IPP request and response, as part of the initial Operation Attributes group of the IPP message
    // Per section 4.1.5 Operation targets
    // o  In the case where there is only one operation target attribute
    //    (i.e., either only the "printer-uri" attribute or only the
    //    "job-uri" attribute), that attribute MUST be the third attribute
    //    in the Operation Attributes group.
    // o  In the case where Job operations use two operation target
    //    attributes (i.e., the "printer-uri" and "job-id" attributes), the
    //    "printer-uri" attribute MUST be the third attribute and the
    //    "job-id" attribute MUST be the fourth attribute.
    const HEADER_ATTRS: [&'static str; 3] = [
        IppAttribute::ATTRIBUTES_CHARSET,
        IppAttribute::ATTRIBUTES_NATURAL_LANGUAGE,
        IppAttribute::PRINTER_URI,
    ];

    /// Create new instance of the attribute
    ///
    /// * `name` - Attribute name<br/>
    /// * `value` - Attribute value<br/>
    pub fn new<S>(name: S, value: IppValue) -> IppAttribute
    where
        S: AsRef<str>,
    {
        IppAttribute {
            name: name.as_ref().to_owned(),
            value,
        }
    }

    /// Return attribute name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return attribute value
    pub fn value(&self) -> &IppValue {
        &self.value
    }

    /// Consume this attribute and return the value
    pub fn into_value(self) -> IppValue {
        self.value
    }

    /// Write attribute to byte array
    pub fn to_bytes(&self) -> Result<Bytes, IppParseError> {
        let mut buffer = BytesMut::new();

        buffer.put_u8(self.value.to_tag());
        buffer.put_u16(self.name.len() as u16);
        buffer.put_slice(self.name.as_bytes());
        buffer.put(self.value.to_bytes()?);
        Ok(buffer.freeze())
    }
}

/// Attribute group
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct IppAttributeGroup {
    tag: DelimiterTag,
    attributes: HashMap<String, IppAttribute>,
}

impl IppAttributeGroup {
    /// Create new attribute group of a given type
    pub fn new(tag: DelimiterTag) -> IppAttributeGroup {
        IppAttributeGroup {
            tag,
            attributes: HashMap::new(),
        }
    }

    /// Return group type tag
    pub fn tag(&self) -> DelimiterTag {
        self.tag
    }

    /// Return read-only attributes
    pub fn attributes(&self) -> &HashMap<String, IppAttribute> {
        &self.attributes
    }

    /// Return mutable attributes
    pub fn attributes_mut(&mut self) -> &mut HashMap<String, IppAttribute> {
        &mut self.attributes
    }

    /// Consume this group and return mutable attributes
    pub fn into_attributes(self) -> HashMap<String, IppAttribute> {
        self.attributes
    }
}

/// Attribute list
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, Default)]
pub struct IppAttributes {
    groups: Vec<IppAttributeGroup>,
}

impl IppAttributes {
    /// Create attribute list
    pub fn new() -> IppAttributes {
        IppAttributes { ..Default::default() }
    }

    /// Get all groups
    pub fn groups(&self) -> &[IppAttributeGroup] {
        &self.groups
    }

    /// Get all mutable groups
    pub fn groups_mut(&mut self) -> &mut Vec<IppAttributeGroup> {
        &mut self.groups
    }

    /// Consume this attribute list and return all attribute groups
    pub fn into_groups(self) -> Vec<IppAttributeGroup> {
        self.groups
    }

    /// Get a list of attribute groups matching a given delimiter tag
    pub fn groups_of(&self, tag: DelimiterTag) -> impl Iterator<Item = &IppAttributeGroup> {
        self.groups.iter().filter(move |g| g.tag == tag)
    }

    /// Add attribute to a given group
    pub fn add(&mut self, tag: DelimiterTag, attribute: IppAttribute) {
        let group = self.groups_mut().iter_mut().find(|g| g.tag() == tag);
        if let Some(group) = group {
            group.attributes_mut().insert(attribute.name().to_owned(), attribute);
        } else {
            let mut new_group = IppAttributeGroup::new(tag);
            new_group
                .attributes_mut()
                .insert(attribute.name().to_owned(), attribute);
            self.groups_mut().push(new_group);
        }
    }

    /// Write attribute list to byte array
    pub fn to_bytes(&self) -> Result<Bytes, IppParseError> {
        let mut buffer = BytesMut::new();

        // put the required attributes first as described in section 4.1.4 of RFC8011
        buffer.put_u8(DelimiterTag::OperationAttributes as u8);

        if let Some(group) = self.groups_of(DelimiterTag::OperationAttributes).next() {
            for hdr in &IppAttribute::HEADER_ATTRS {
                if let Some(attr) = group.attributes().get(*hdr) {
                    buffer.put(attr.to_bytes()?);
                }
            }

            // now the other operation attributes
            for attr in group.attributes().values() {
                if !is_header_attr(attr.name()) {
                    buffer.put(attr.to_bytes()?);
                }
            }
        }

        // now the rest
        for group in self
            .groups()
            .iter()
            .filter(|group| group.tag() != DelimiterTag::OperationAttributes)
        {
            buffer.put_u8(group.tag() as u8);

            for attr in group.attributes().values() {
                buffer.put(attr.to_bytes()?);
            }
        }
        buffer.put_u8(DelimiterTag::EndOfAttributes as u8);

        Ok(buffer.freeze())
    }
}
