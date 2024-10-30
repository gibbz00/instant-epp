//! Types for EPP host delete request

use instant_xml::ToXml;

use super::XMLNS;
use crate::common::{NoExtension, EPP_XMLNS};
use crate::request::{Command, Transaction};

impl Transaction<NoExtension> for HostDelete<'_> {}

impl Command for HostDelete<'_> {
    type Response = ();
    const COMMAND: &'static str = "delete";
}

impl<'a> HostDelete<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            host: HostDeleteRequest { name },
        }
    }
}

/// Type for data under the host `<delete>` tag
#[derive(Debug, ToXml)]
#[xml(rename = "delete", ns(XMLNS))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct HostDeleteRequest<'a> {
    /// The host to be deleted
    name: &'a str,
}

/// Type for EPP XML `<delete>` command for hosts
#[derive(Debug, ToXml)]
#[xml(rename = "delete", ns(EPP_XMLNS))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct HostDelete<'a> {
    /// The instance holding the data for the host to be deleted
    #[cfg_attr(feature = "serde", serde(borrow, flatten))]
    host: HostDeleteRequest<'a>,
}

#[cfg(test)]
mod tests {
    use super::HostDelete;
    use crate::response::ResultCode;
    use crate::tests::{assert_serialized, response_from_file, CLTRID, SUCCESS_MSG, SVTRID};

    #[test]
    fn command() {
        let object = HostDelete::new("ns1.eppdev-1.com");
        assert_serialized("request/host/delete.xml", &object);
    }

    #[test]
    fn response() {
        let object = response_from_file::<HostDelete>("response/host/delete.xml");
        assert_eq!(object.result.code, ResultCode::CommandCompletedSuccessfully);
        assert_eq!(object.result.message, SUCCESS_MSG);
        assert_eq!(object.tr_ids.client_tr_id.unwrap(), CLTRID);
        assert_eq!(object.tr_ids.server_tr_id, SVTRID);
    }
}
