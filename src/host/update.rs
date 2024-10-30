//! Types for EPP host update request

use std::borrow::Cow;
use std::net::IpAddr;

use instant_xml::ToXml;

use super::{serialize_host_addrs_option, Status, XMLNS};
use crate::common::{NoExtension, EPP_XMLNS};
use crate::request::{Command, Transaction};

impl Transaction<NoExtension> for HostUpdate<'_> {}

impl Command for HostUpdate<'_> {
    type Response = ();
    const COMMAND: &'static str = "update";
}

impl<'a> HostUpdate<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            host: HostUpdateRequest {
                name: name.into(),
                add: None,
                remove: None,
                change_info: None,
            },
        }
    }

    /// Sets the data for the `<chg>` element of the host update
    pub fn info(&mut self, info: HostChangeInfo<'a>) {
        self.host.change_info = Some(info);
    }

    /// Sets the data for the `<add>` element of the host update
    pub fn add(&mut self, add: HostAdd) {
        self.host.add = Some(add);
    }

    /// Sets the data for the `<rem>` element of the host update
    pub fn remove(&mut self, remove: HostRemove) {
        self.host.remove = Some(remove);
    }
}

/// Type for data under the `<chg>` tag
#[derive(Debug, ToXml)]
#[xml(rename = "chg", ns(XMLNS))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct HostChangeInfo<'a> {
    /// The new name for the host
    pub name: &'a str,
}

/// Type for data under the `<add>` and `<rem>` tags
#[derive(Debug, ToXml)]
#[xml(rename = "add", ns(XMLNS))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct HostAdd {
    /// The IP addresses to be added to or removed from the host
    #[xml(rename = "host:addr", serialize_with = "serialize_host_addrs_option")]
    pub addresses: Option<Vec<IpAddr>>,
    /// The statuses to be added to or removed from the host
    #[xml(rename = "host:status")]
    pub statuses: Option<Vec<Status>>,
}

/// Type for data under the `<add>` and `<rem>` tags
#[derive(Debug, ToXml)]
#[xml(rename = "rem", ns(XMLNS))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct HostRemove {
    /// The IP addresses to be added to or removed from the host
    #[xml(rename = "host:addr", serialize_with = "serialize_host_addrs_option")]
    pub addresses: Option<Vec<IpAddr>>,
    /// The statuses to be added to or removed from the host
    #[xml(rename = "host:status")]
    pub statuses: Option<Vec<Status>>,
}

/// Type for data under the host `<update>` tag
#[derive(Debug, ToXml)]
#[xml(rename = "update", ns(XMLNS))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct HostUpdateRequest<'a> {
    /// The name of the host
    name: Cow<'a, str>,
    /// The IP addresses and statuses to be added to the host
    #[xml(rename = "host:add")]
    add: Option<HostAdd>,
    /// The IP addresses and statuses to be removed from the host
    #[xml(rename = "host:rem")]
    remove: Option<HostRemove>,
    /// The host details that need to be updated
    #[xml(rename = "host:chg")]
    #[cfg_attr(feature = "serde", serde(borrow))]
    change_info: Option<HostChangeInfo<'a>>,
}

/// Type for EPP XML `<update>` command for hosts
#[derive(Debug, ToXml)]
#[xml(rename = "update", ns(EPP_XMLNS))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct HostUpdate<'a> {
    /// The instance holding the data for the host to be updated
    #[cfg_attr(feature = "serde", serde(borrow, flatten))]
    host: HostUpdateRequest<'a>,
}

#[cfg(test)]
mod tests {
    use super::IpAddr;
    use super::{HostAdd, HostChangeInfo, HostRemove, HostUpdate, Status};
    use crate::response::ResultCode;
    use crate::tests::{assert_serialized, response_from_file, CLTRID, SUCCESS_MSG, SVTRID};

    #[test]
    fn command() {
        let addr = vec![IpAddr::from([
            0x2404, 0x6800, 0x4001, 0x801, 0, 0, 0, 0x200e,
        ])];

        let add = HostAdd {
            addresses: Some(addr),
            statuses: None,
        };

        let remove = HostRemove {
            addresses: None,
            statuses: Some(vec![Status::ClientDeleteProhibited]),
        };

        let mut object = HostUpdate::new("host1.eppdev-1.com");

        object.add(add);
        object.remove(remove);
        object.info(HostChangeInfo {
            name: "host2.eppdev-1.com",
        });

        assert_serialized("request/host/update.xml", &object);
    }

    #[test]
    fn response() {
        let object = response_from_file::<HostUpdate>("response/host/update.xml");

        assert_eq!(object.result.code, ResultCode::CommandCompletedSuccessfully);
        assert_eq!(object.result.message, SUCCESS_MSG);
        assert_eq!(object.tr_ids.client_tr_id.unwrap(), CLTRID);
        assert_eq!(object.tr_ids.server_tr_id, SVTRID);
    }
}
