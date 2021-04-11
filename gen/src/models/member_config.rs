/*
 * ZeroTier Central API
 *
 * ZeroTier Central Network Management Portal API.<p>All API requests must have an API token header specified in the <code>Authorization: Bearer xxxxx</code> format.  You can generate your API key by logging into <a href=\"https://my.zerotier.com\">ZeroTier Central</a> and creating a token on the Account page.</p><p>eg. <code>curl -X GET -H \"Authorization: bearer xxxxx\" https://my.zerotier.com/api/network</code></p>
 *
 * The version of the OpenAPI document: v1
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MemberConfig {
    /// Allow the member to be a bridge on the network
    #[serde(rename = "activeBridge", skip_serializing_if = "Option::is_none")]
    pub active_bridge: Option<bool>,
    /// Is the member authorized on the network
    #[serde(rename = "authorized", skip_serializing_if = "Option::is_none")]
    pub authorized: Option<bool>,
    #[serde(rename = "capabilities", skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<Vec<i64>>,
    /// Time the member was created or first tried to join the network
    #[serde(rename = "creationTime", skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<i64>,
    /// ID of the member node.  This is the 10 digit identifier that identifies a ZeroTier node.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Public Key of the member's Identity
    #[serde(rename = "identity", skip_serializing_if = "Option::is_none")]
    pub identity: Option<String>,
    /// List of assigned IP addresses
    #[serde(rename = "ipAssignments", skip_serializing_if = "Option::is_none")]
    pub ip_assignments: Option<Vec<String>>,
    /// Time the member was authorized on the network
    #[serde(rename = "lastAuthorizedTime", skip_serializing_if = "Option::is_none")]
    pub last_authorized_time: Option<i64>,
    /// Time the member was deauthorized on the network
    #[serde(
        rename = "lastDeauthorizedTime",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_deauthorized_time: Option<i64>,
    /// Exempt this member from the IP auto assignment pool on a Network
    #[serde(rename = "noAutoAssignIps", skip_serializing_if = "Option::is_none")]
    pub no_auto_assign_ips: Option<bool>,
    /// Member record revision count
    #[serde(rename = "revision", skip_serializing_if = "Option::is_none")]
    pub revision: Option<i64>,
    /// Array of 2 member tuples of tag [ID, tag value]
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Vec<i64>>>,
    /// Major version of the client
    #[serde(rename = "vMajor", skip_serializing_if = "Option::is_none")]
    pub v_major: Option<i64>,
    /// Minor version of the client
    #[serde(rename = "vMinor", skip_serializing_if = "Option::is_none")]
    pub v_minor: Option<i64>,
    /// Revision number of the client
    #[serde(rename = "vRev", skip_serializing_if = "Option::is_none")]
    pub v_rev: Option<i64>,
    /// Protocol version of the client
    #[serde(rename = "vProto", skip_serializing_if = "Option::is_none")]
    pub v_proto: Option<i64>,
}

impl MemberConfig {
    pub fn new() -> MemberConfig {
        MemberConfig {
            active_bridge: None,
            authorized: None,
            capabilities: None,
            creation_time: None,
            id: None,
            identity: None,
            ip_assignments: None,
            last_authorized_time: None,
            last_deauthorized_time: None,
            no_auto_assign_ips: None,
            revision: None,
            tags: None,
            v_major: None,
            v_minor: None,
            v_rev: None,
            v_proto: None,
        }
    }
}
