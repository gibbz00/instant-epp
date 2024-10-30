//! Types for EPP contact delete request

use std::borrow::Cow;

use instant_xml::ToXml;

use super::XMLNS;
use crate::common::{NoExtension, EPP_XMLNS};
use crate::request::{Command, Transaction};

impl Transaction<NoExtension> for ContactDelete<'_> {}

impl Command for ContactDelete<'_> {
    type Response = ();
    const COMMAND: &'static str = "delete";
}

/// Type containing the data for the `<delete>` tag for contacts
#[derive(Debug, ToXml)]
#[xml(rename = "delete", ns(XMLNS))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct ContactDeleteRequest<'a> {
    /// The id of the contact to be deleted
    id: Cow<'a, str>,
}

/// The `<delete>` type for the contact delete EPP command
#[derive(Debug, ToXml)]
#[xml(rename = "delete", ns(EPP_XMLNS))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct ContactDelete<'a> {
    /// The data for the `<delete>` tag for a contact delete command
    #[cfg_attr(feature = "serde", serde(flatten))]
    contact: ContactDeleteRequest<'a>,
}

impl<'a> ContactDelete<'a> {
    pub fn new(id: &'a str) -> Self {
        Self {
            contact: ContactDeleteRequest { id: id.into() },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ContactDelete;
    use crate::response::ResultCode;
    use crate::tests::{assert_serialized, response_from_file, CLTRID, SUCCESS_MSG, SVTRID};

    #[test]
    fn command() {
        let object = ContactDelete::new("eppdev-contact-3");
        assert_serialized("request/contact/delete.xml", &object);
    }

    #[test]
    fn response() {
        let object = response_from_file::<ContactDelete>("response/contact/delete.xml");
        assert_eq!(object.result.code, ResultCode::CommandCompletedSuccessfully);
        assert_eq!(object.result.message, SUCCESS_MSG);
        assert_eq!(object.tr_ids.client_tr_id.unwrap(), CLTRID);
        assert_eq!(object.tr_ids.server_tr_id, SVTRID);
    }
}
