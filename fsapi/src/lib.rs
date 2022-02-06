use quick_xml::events::Event;
use std::fmt::Display;

pub use error::Error;
use error::InternalError;

mod error;
mod macros;

#[derive(Debug)]
pub struct FsApi();

const FSAPI_PATH: &str = "fsapi";
const GET_PATH: &str = "GET";
const SET_PATH: &str = "SET";
const LIST_GET_NEXT_PATH: &str = "LIST_GET_NEXT";
const CREATE_SESSION_PATH: &str = "CREATE_SESSION";
const DELETE_SESSION_PATH: &str = "DELETE_SESSION";
const GET_NOTIFIES_PATH: &str = "GET_NOTIFIES";

/// Response of the fsapi server
#[derive(Debug, PartialEq)]
struct Response {
    /// Returned status from the fsapi server
    pub status: ResponseStatus,

    /// Data return from the fsapi is there is any
    pub data: Option<Data>,
}

const STATUS_FS_OK: &str = "FS_OK";
const STATUS_FS_FAIL: &str = "FS_FAIL";
const STATUS_FS_PACKET_BAD: &str = "FS_PACKET_BAD";
const STATUS_FS_NODE_BLOCKED: &str = "FS_NODE_BLOCKED";
const STATUS_FS_NODE_DOES_NOT_EXIST: &str = "FS_NODE_DOES_NOT_EXIST";
const STATUS_FS_TIMEOUT: &str = "FS_TIMEOUT";
const STATUS_FS_LIST_END: &str = "FS_LIST_END";

/// Resposn status of the fsapi server
#[derive(Debug, PartialEq)]
enum ResponseStatus {
    /// Everything went well: The command has been executed.
    Ok,

    /// The command hasn't been executed, because your value does not match the validation rules.
    Fail,

    /// You tried to set the value of an read only node.
    PacketBad,

    /// You tried to SET a node of an operation Mode which is not active.
    NodeBlocked,

    /// You tried to access an not existing node.
    NodeDoesNotExist,

    /// Your Request took to long.
    Timeout,

    /// There is no list-entry left.
    ListEnd,
}

#[derive(Debug, PartialEq)]
enum Data {
    /// Result from `fsapi/GET`
    Value(Value),

    /// Result from `fsapi/LIST_GET_NEXT`
    Items(Vec<Item>),

    /// Session id response on `fsapi/CREATE_SESSION`
    SessionID(SessionID),

    /// resonse on `/fsapi/NOTIFY`
    Notify(Vec<Notification>),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SessionID(u32);

#[derive(Debug, PartialEq)]
pub struct Notification {
    pub node: Node,
    pub value: Value,
}

#[derive(Debug, PartialEq)]
pub struct Item {
    pub key: u32,
    pub fields: Vec<Field>,
}

#[derive(Debug, PartialEq)]
pub struct Field {
    pub name: String,
    pub value: Value,
}

#[derive(Debug, PartialEq)]
pub enum Value {
    Text(String),
    U8(u8),
    S16(i16),
    U32(u32),
    Array(String),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Text(v) => write!(f, "{v}"),
            Value::U8(v) => write!(f, "{v}"),
            Value::S16(v) => write!(f, "{v}"),
            Value::U32(v) => write!(f, "{v}"),
            Value::Array(v) => write!(f, "{v}"),
        }
    }
}

// nav
const NODE_NAV_LIST: &str = "netremote.nav.list";
const NODE_NAV_NUMITEMS: &str = "netremote.nav.numItems";
const NODE_NAV_PRESETS: &str = "netremote.nav.presets";
const NODE_NAV_SEARCHTERM: &str = "netremote.nav.searchTerm";
const NODE_NAV_STATE: &str = "netremote.nav.state";
const NODE_NAV_STATUS: &str = "netremote.nav.status";
const NODE_NAV_DEPTH: &str = "netremote.nav.depth";

// nav.action
const NODE_NAV_ACTION_DABSCAN: &str = "netremote.nav.action.dabScan";
const NODE_NAV_ACTION_NAVIGATE: &str = "netremote.nav.action.navigate";
const NODE_NAV_ACTION_SELECTITEM: &str = "netremote.nav.action.selectItem";
const NODE_NAV_ACTION_SELECTPRESET: &str = "netremote.nav.action.selectPreset";

// play
const NODE_PLAY_ADDPRESET: &str = "netremote.play.addpreset";
const NODE_PLAY_CAPS: &str = "netremote.play.caps";
const NODE_PLAY_CONTROL: &str = "netremote.play.control";
const NODE_PLAY_ERRORSTR: &str = "netremote.play.errorstr";
const NODE_PLAY_FREQUENCY: &str = "netremote.play.frequency";
const NODE_PLAY_POSITION: &str = "netremote.play.position";
const NODE_PLAY_RATE: &str = "netremote.play.rate";
const NODE_PLAY_REPEAT: &str = "netremote.play.repeat";
const NODE_PLAY_SCROBBLE: &str = "netremote.play.scrobble";
const NODE_PLAY_SHUFFLE: &str = "netremote.play.shuffle";
const NODE_PLAY_SHUFFLESTATUS: &str = "netremote.play.shufflestatus";
const NODE_PLAY_SIGNALSTRENGTH: &str = "netremote.play.signalstrength";
const NODE_PLAY_STATUS: &str = "netremote.play.status";

// play.info
const NODE_PLAY_INFO_ALBUM: &str = "netremote.play.info.album";
const NODE_PLAY_INFO_ARTIST: &str = "netremote.play.info.artist";
const NODE_PLAY_INFO_DURATION: &str = "netremote.play.info.duration";
const NODE_PLAY_INFO_GRAPHICURI: &str = "netremote.play.info.graphicuri";
const NODE_PLAY_INFO_NAME: &str = "netremote.play.info.name";
const NODE_PLAY_INFO_TEXT: &str = "netremote.play.info.text";

// play.serviceIds
const NODE_PLAY_SERVICEIDS_DABENSEMBLEID: &str = "netremote.play.serviceids.dabinsembleid";
const NODE_PLAY_SERVICEIDS_DABSCIDS: &str = "netremote.play.serviceids.dabscids";
const NODE_PLAY_SERVICEIDS_DABSERVICEID: &str = "netremote.play.serviceids.dabserviceid";
const NODE_PLAY_SERVICEIDS_ECC: &str = "netremote.play.serviceids.ecc";
const NODE_PLAY_SERVICEIDS_FMRDSPI: &str = "netremote.play.serviceids.fmrdspi";

// sys
const NODE_SYS_LANG: &str = "netremote.sys.lang";
const NODE_SYS_MODE: &str = "netremote.sys.mode";
const NODE_SYS_POWER: &str = "netremote.sys.power";
const NODE_SYS_SLEEP: &str = "netremote.sys.sleep";
const NODE_SYS_STATE: &str = "netremote.sys.state";

// sys.audio
const NODE_SYS_AUDIO_EQCUSTOM_PARAM0: &str = "netremote.sys.audio.eqcustom.param0";
const NODE_SYS_AUDIO_EQCUSTOM_PARAM1: &str = "netremote.sys.audio.eqcustom.param1";

const NODE_SYS_AUDIO_EQLOUDNESS: &str = "netremote.sys.audio.eqloudness";
const NODE_SYS_AUDIO_EQPRESET: &str = "netremote.sys.audio.eqpreset";
const NODE_SYS_AUDIO_VOLUME: &str = "netremote.sys.audio.volume";
const NODE_SYS_AUDIO_MUTE: &str = "netremote.sys.audio.mute";

// sys.caps
const NODE_SYS_CAPS_CLOCKSOURCELIST: &str = "netremote.sys.caps.clocksourcelist";
const NODE_SYS_CAPS_DABFREQLIST: &str = "netremote.sys.caps.dabfreqlist";
const NODE_SYS_CAPS_EQBANDS: &str = "netremote.sys.caps.eqbands";
const NODE_SYS_CAPS_EQPRESETS: &str = "netremote.sys.caps.eqpresets";
const NODE_SYS_CAPS_VALIDMODES: &str = "netremote.sys.caps.validmodes";

const NODE_SYS_CAPS_FMFREQRANGE_LOWER: &str = "netremote.sys.caps.fmfreqrange.lower";
const NODE_SYS_CAPS_FMFREQRANGE_STEPSIZE: &str = "netremote.sys.caps.fmfreqrange.stepsize";
const NODE_SYS_CAPS_FMFREQRANGE_UPPER: &str = "netremote.sys.caps.fmfreqrange.upper";

const NODE_SYS_CAPS_VOLUMESTEPS: &str = "netremote.sys.caps.volumesteps";

// sys.clock
const NODE_SYS_CLOCK_DST: &str = "netremote.sys.clock.dst";
const NODE_SYS_CLOCK_LOCALDATE: &str = "netremote.sys.clock.localdate";
const NODE_SYS_CLOCK_LOCALTIME: &str = "netremote.sys.clock.localtime";
const NODE_SYS_CLOCK_MODE: &str = "netremote.sys.clock.mode";
const NODE_SYS_CLOCK_SOURCE: &str = "netremote.sys.clock.source";
const NODE_SYS_CLOCK_UTCOFFSET: &str = "netremote.sys.clock.utcoffset";

// sys.cfg
const NODE_SYS_CFG_IRAUTOPLAYFLAG: &str = "netremote.sys.clock.irautoplayflag";

// sys.info
const NODE_INFO_FRIENDLYNAME: &str = "netremote.sys.info.friendlyname";
const NODE_INFO_RADIOID: &str = "netremote.sys.info.radioid";
const NODE_INFO_RADIOPIN: &str = "netremote.sys.info.radiopin";
const NODE_INFO_VERSION: &str = "netremote.sys.info.version";
const NODE_INFO_CONTROLLERNAME: &str = "netremote.sys.info.controllername";

// sys.isu
const NODE_ISU_CONTROL: &str = "netremote.sys.isu.control";
const NODE_ISU_STATE: &str = "netremote.sys.isu.state";

// sys.net
const NODE_NET_IPCONFIG_ADDRESS: &str = "netremote.net.ipconfig.address";
const NODE_NET_IPCONFIG_DHCP: &str = "netremote.net.ipconfig.dhcp";
const NODE_NET_IPCONFIG_DNSPRIMARY: &str = "netremote.net.ipconfig.dnsprimary";
const NODE_NET_IPCONFIG_DNSECUNDARY: &str = "netremote.net.ipconfig.dnssecundary";
const NODE_NET_IPCONFIG_GATEWAY: &str = "netremote.net.ipconfig.gateway";
const NODE_NET_IPCONFIG_SUBNETMASK: &str = "netremote.net.ipconfig.subnetmask";
const NODE_NET_IPCONFIG_KEEPCONNECTED: &str = "netremote.net.ipconfig.keepconnected";

const NODE_NET_WIRED_INTERFACEENABLE: &str = "netremote.net.wired.interfaceenable";
const NODE_NET_WIRED_MACADDRESS: &str = "netremote.net.wired.macaddress";

const NODE_NET_WLAN_CONNECTEDSSID: &str = "netremote.net.wlan.connectedssid";
const NODE_NET_WLAN_INTERFACEENABLE: &str = "netremote.net.wlan.interfaceenable";
const NODE_NET_WLAN_MACADDRESS: &str = "netremote.net.wlan.macaddress";
const NODE_NET_WLAN_RSSI: &str = "netremote.net.wlan.rsii";
const NODE_NET_WLAN_SETAUTHTYPE: &str = "netremote.net.wlan.setauthtype";
const NODE_NET_WLAN_SETENCTYPE: &str = "netremote.net.wlan.setenctype";

// sys.rsa
const NODE_SYS_RSA_PUBLICKEY: &str = "netremote.sys.rsa.publickey";
const NODE_SYS_RSA_STATUS: &str = "netremote.sys.rsa.status";

// Airplay

#[derive(Debug, PartialEq)]
pub enum Node {
    // nav
    /// Get the menu for the current mode
    ///
    /// To prevent overhead you could get the number of items by NavNumItems
    ///
    /// The -1 Parameter is the Start-Value, you could provide 4 to get all items with an index greater 4.
    ///
    /// The following field types are known:
    ///
    ///  1. folder
    ///  2. mp3
    ///  3. mp4
    ///
    /// Method: LIST_GET_NEXT
    /// Returns: `Vec<Item(_)>`
    /// PATH: "netRemote.nav.list"
    NavList,

    /// Get the amount of entries for the current navigation-set.
    ///
    /// Method: GET
    /// Returns: `Value::S32(_)`
    /// PATH: "netRemote.nav.numItems"
    NavNumItems,

    /// Lists all favorite Radio Stations for the current mode
    ///
    /// Method: LIST_GET_NEXT
    /// Returns: `Vec<Item<_>>`
    /// PATH: "netRemote.nav.presets"
    NavPresets,

    /// Search in the current navigation (see NavList)
    ///
    /// Method: LIST_GET_NEXT
    /// Returns: `Vec<Item<_>>`
    /// PATH: "netRemote.nav.searchTerm"
    NavSearchTerm,

    /// Enables the navigation in the menu (see NavList)
    ///
    /// Every change of the system mode, will disable the nav state to reset the current menu-position.
    ///
    /// Method: GET
    /// Returns: `Value::U8(_)`
    /// PATH: "netRemote.nav.state"
    NavState,

    /// While the device prepares the menu this node is set to 0, if the menu is ready it is set to 1.
    ///
    /// Busy menus are caused by mode-change or NavActionNavigate
    ///
    /// Method: GET
    /// Returns: `Value::U8(_)`
    /// PATH: "netRemote.nav.status"
    NavStatus,

    /// Returns the current navigation menu depth
    ///
    /// Method: GET
    /// Returns: `Value::U8(_)`
    /// PATH: "netRemote.nav.depth"
    NavDepth,

    // nav.action
    /// Starts Scan for DAB Channels
    ///
    /// Method: GET
    /// Returns: `Value::S32(_)`
    /// PATH: "netRemote.nav.action.dabScan"
    NavActionDabScan,

    /// Selects the current menu entry (see NavList)
    ///
    /// For returning to the upper menu level just set it to -1.
    ///
    /// This function works only with folders (type=0), see NavList.
    ///
    /// Method: GET?, SET
    /// Returns: ?
    /// PATH: "netRemote.nav.action.navigate"
    NavActionNavigate,

    /// Selects an Menu Item (see netRemove.nav.list)
    ///
    /// This function works only on files (type > 0), see NavList.
    ///
    /// Method: GET?, SET
    /// Returns: ?
    /// PATH: "netRemote.nav.action.selectItem"
    NavActionSelectItem,

    /// Selects a  favorite Radio Stations (see NavPresets)
    ///
    /// Method: GET?, SET
    /// Returns: ?
    /// PATH: "netRemote.nav.action.selectPreset"
    NavActionSelectPreset,

    // play
    /// Add the current radio stations to the favorites menu
    ///
    /// Method: GET?, SET
    /// Returns: ?
    /// PATH: "netRemote.play.addPreset"
    PlayAddPreset,

    /// Actions the player can do
    /// 0: PlayControl: Play?
    /// 1: PlayControl: Pause or Play?
    /// 2: PlayControl: Next or Previous
    /// 3: PlayControl: Next or Previous
    /// 4:
    /// 5:
    /// 6:
    /// 7:
    /// 8:
    /// 9:
    /// 10:
    /// 11: PlayAddPreset
    ///
    /// Method: GET
    /// Returns: Value::U32(_),
    /// PATH: "netRemote.play.caps"
    PlayCaps,

    /// Sets / Return the current play-controll mode
    ///
    /// 0: play/pause toggle
    /// 1: ?
    /// 2: ?
    /// 3: next (song/station)
    /// 4: previous (song/station)
    /// ...: ?
    ///
    /// Wrong: Scope: Mediaplayer / Radio
    /// Wrong: 1=Play; 2=Pause; 3=Next (song/station); 4=Previous (song/station)
    ///
    /// Method: GET/SET
    /// Returns: Value::U8(_),
    /// PATH: "netRemote.play.control"
    PlayControl,

    /// Method: GET
    /// Returns: Value::Text(_),
    /// PATH: "netRemote.play.errorStr"
    PlayErrorStr,

    /// Sets / Returns the current frequency for fm (in herz)
    ///
    /// Method: GET/SET
    /// Returns: Value::U32(_),
    /// PATH: "netRemote.play.frequency"
    PlayFrequency,

    /// Sets / Returns the current position in the track in milliseconds
    ///
    /// Keep in mind to get the max position by PlayDuration
    ///
    /// Method: GET/SET
    /// Returns: Value::U32(_),
    /// PATH: "netRemote.play.position"
    PlayPosition,

    /// Sets / Returns the current play-rate multiplier
    ///
    /// The value determines the speed of the playback.
    /// It can be in the range of -127 to 127 where the negative part plays the current track backwards.
    ///
    /// Method: GET/SET
    /// Returns: Value::S8(_),
    /// PATH: "netRemote.play.rate"
    PlayRate,

    /// Sets / Returns the current play-rate multiplier
    ///
    /// Method: GET/SET
    /// Returns: Value::U8([0,1]),
    /// PATH: "netRemote.play.repeat"
    PlayRepeat,

    /// Returns  whether or not scrobble is enabled or not
    ///
    /// Method: GET/SET
    /// Returns: Value::U8([0,1]),
    /// PATH: "netRemote.play.scrobble"
    PlayScrobble,

    /// Sets / Returns whether or not shuffle is enabled or not (1/0)
    ///
    /// Method: GET/SET
    /// Returns: Value::U8([0,1]),
    /// PATH: "netRemote.play.shuffle"
    PlayShuffle,

    /// Sets / Returns whether or not shuffle is enabled or not (1/0)
    ///
    /// Is part of the new api version
    ///
    /// Method: GET/SET
    /// Returns: Value::U8([0,1]),
    /// PATH: "netRemote.play.shuffleStatus"
    PlayShuffleStatus,

    /// Returns the signal strenght of the current medium
    ///
    /// Method: GET
    /// Returns: Value::U8(_),
    /// PATH: "netRemote.play.signalStrength"
    PlaySignalStrength,

    /// Returns status of the player
    ///
    /// 1=buffering/loading, 2=playing, 3=paused
    ///
    /// Method: GET
    /// Returns: Value::U8([0,3]),
    /// PATH: "netRemote.play.status"
    PlayStatus,

    // play.info
    /// Returns  the name of the artist of the current song
    ///
    /// Method: GET
    /// Returns: Value::Text(_),
    /// PATH: "netRemote.play.info.album"
    PlayInfoAlbum,

    /// Returns the name of the album of the current song
    ///
    /// Method: GET
    /// Returns: Value::Text(_),
    /// PATH: "netRemote.play.info.artist"
    PlayInfoArtist,

    /// Returns the duration for the track in milliseconds
    ///
    /// Method: GET
    /// Returns: Value::U32(_),
    /// PATH: "netRemote.play.info.duration"
    PlayInfoDuration,

    /// Returns the uri of an image representing the current song / station
    ///
    /// Method: GET
    /// Returns: Value::Text(_),
    /// PATH: "netRemote.play.info.graphicUri"
    PlayInfoGraphicUri,

    /// Returns the first line of the display
    ///
    /// Method: GET
    /// Returns: Value::Text(_),
    /// PATH: "netRemote.play.info.name"
    PlayInfoName,

    /// Returns  the second line of the display
    ///
    /// Method: GET
    /// Returns: Value::Text(_),
    /// PATH: "netRemote.play.info.text"
    PlayInfoText,

    // play.serviceIds,
    /// Returns DAB Ensemble Identifier (decimal notation)
    /// Note: commonly used in Hex notation.
    ///
    /// Method: GET
    /// Returns: Value::U16(_),
    /// PATH: "netRemote.play.serviceIds.dabEnsembleId"
    PlayServiceIdsDabEnsambleId,

    /// Returns the DAB Service Component Identifier (decimal notation)
    /// Note: Nearly always 0 for audio services - Secondary Component services will
    /// have a different value
    ///
    /// Method: GET
    /// Returns: Value::U8(_),
    /// PATH: "netRemote.play.serviceIds.dabScids"
    PlayServiceIdsDabScids,

    /// Returns DAB Service Identifier (decimal notation)
    /// Note: commonly used in Hex notation.
    ///
    /// Method: GET
    /// Returns: Value::U32(_),
    /// PATH: "netRemote.play.serviceIds.dabServiceId"
    PlayServiceIdsDabServiceId,

    /// Returns Extended Country Code (decimal notation) as defined in ETSI TS 101 756
    /// Note: commonly used in Hex notation.
    ///
    /// Method: GET
    /// Returns: Value::U8(_),
    /// PATH: "netRemote.play.serviceIds.ecc"
    PlayServiceIdsEcc,

    /// Returns RDS Programme Identification code
    ///
    /// Method: GET
    /// Returns: Value::U16(_),
    /// PATH: "netRemote.play.serviceIds.fmRdsPi"
    PlayServiceIdsfmRdsPi,

    // sys
    /// Retrieve/set the radio user interface language
    ///
    /// Method: GET, SET
    /// Returns: `Value::U32(_)`
    /// PATH: "netRemote.sys.lang"
    SysLang,

    /// Sets / Returns the current operation mode
    ///
    /// see SysCapsValidModes for valid operation modes
    ///
    /// Method: GET, SET
    /// Returns: `Value::U32(_)`
    /// PATH: "netRemote.sys.mode"
    SysMode,

    /// Sets / Returns the current power state
    ///
    /// If device returns from standby it will only auto-continue to play in radio-modes.
    ///
    /// Method: GET, SET
    /// Returns: `Value::U32([0,1])``
    /// PATH: "netRemote.sys.power"
    SysPower,

    /// Sets / Returns the Time till Sleep in seconds (0 = No Sleep) [works with Firmware V2.9.10 but not with V2.6.17]
    ///
    /// Method: GET, SET
    /// Returns: `Value::U32(_)``
    /// PATH: "netRemote.sys.sleep"
    SysSleep,

    /// ???
    ///
    /// Method: GET, SET
    /// Returns: `Value::U8([0,1])``
    /// PATH: "netRemote.sys.state"
    SysState,

    // sys.audio
    /// Sets / Returns the first value for costum eq-settings (Bass)
    ///
    /// Method: GET, SET
    /// Returns: `Value::S16([-7, 7])`
    /// PATH: "netRemote.sys.audio.eqCustom.param0"
    SysAudioEqCustomParam0,

    /// Sets/Returns the first value for costum eq-settings (Treble)
    ///
    /// Method: GET, SET
    /// Returns: `Value::S16([-7, 7])`
    /// PATH: "netRemote.sys.audio.eqCustom.param1"
    SysAudioEqCustomParam1,

    /// Sets / Returns whether or not loudness is activated
    ///
    /// This function is only available if costum eq is active
    ///
    /// Method: GET, SET
    /// Returns: `Value::U8([0, 1])`
    /// PATH: netRemote.sys.audio.eqLoudness
    SysAudioEqLoudness,

    /// Sets / Returns the number of the selected eq-presets
    ///
    /// see: SysCapsEqPresets for valid presets.
    ///
    /// Method: GET, SET
    /// Returns: `Value::U8(_)`
    /// PATH: netRemote.sys.audio.eqPreset
    SysAudioEqPreset,

    /// Sets / Returns whether or not device is muted
    ///
    /// Method: GET, SET
    /// Returns: `Value::U8([0, 1])`
    /// PATH: netRemote.sys.audio.mute
    SysAudioMute,

    /// Sets / Returns the volume of the device
    ///
    /// Method: GET, SET
    /// Returns: `Value::U8([0, 20])`
    /// PATH: netRemote.sys.audio.volume
    SysAudioVolume,

    // sys.caps
    /// Fetch the list of available time sources
    ///
    /// Method: LIST_GET_NEXT
    /// Returns: `Items(_)`
    /// PATH: netRemote.sys.caps.clockSourList?maxItems=4
    SysCapsClockSourceList,

    /// Lists available dab-frequencies
    ///
    /// Method: LIST_GET_NEXT
    /// Returns: `Items(_)`
    /// PATH: netRemote.sys.caps.dabFreqList
    SysCapsDabFreqList,

    /// Lists setted modes for the eq
    ///
    /// Method: LIST_GET_NEXT
    /// Returns: `Items(_)`
    /// PATH: netRemote.sys.caps.eqBands
    SysCapsEqBands,

    /// Lists available eq-presets
    ///
    /// Method: LIST_GET_NEXT
    /// Returns: `Items(_)`
    /// PATH: netRemote.sys.caps.eqPresets
    SysCapsEqPresets,

    /// Returns the lowest available fm-frequency
    ///
    /// Method: GET
    /// Returns: `Value::U32(_)`
    /// PATH: netRemote.sys.caps.fmFreqRange.lower
    SysCapsFmFreqRangeLower,

    /// Returns the size of the steps for increasing / decreasing the frequency
    ///
    /// Method: GET
    /// Returns: `Value::U32(_)`
    /// PATH: netRemote.sys.caps.fmFreqRange.stepSize
    SysCapsFmFreqRangeStepSize,

    /// Returns the highest available fm-frequency
    ///
    /// Method: GET
    /// Returns: `Value::U32(_)`
    /// PATH: netRemote.sys.caps.fmFreqRange.upper
    SysCapsFmFreqRangeUpper,

    /// Lists valid operations modes
    /// Method: LIST_GET_NEXT
    /// Returns: `Items(_)`
    /// PATH: netRemote.sys.caps.validModes
    SysCapsValidModes,

    /// Returns the max volume level
    ///
    /// Method: GET
    /// Returns: `Value::U8(_)`
    /// PATH: netRemote.sys.caps.volumeSteps
    SysCapsVolumeSteps,

    // sys.clock
    /// Fetch/set daylight saving setting
    ///
    /// Method: SET/GET
    /// Returns: `Value::U8([0,1])`
    /// PATH: netRemote.sys.clock.dst
    SysClockDst,

    /// Returns the local Date in XML-RPC date format ( 20150914 = 2015-09-14)
    ///
    /// Method: GET
    /// Returns: `Value::Text(String)`
    /// PATH: netRemote.sys.clock.localDate
    SysClockLocalDate,

    /// Returns the local time in XML-RPC date format ( 093327 = 09:33:27)
    ///
    /// Method: GET
    /// Returns: `Value::Text(String)`
    /// PATH: netRemote.sys.clock.localTime
    SysClockLocalTime,

    /// Set the clock to 12h or 24 hour mode
    ///
    /// Values:
    /// - 0: 12h
    /// - 1: 24h
    /// Method: SET/GET
    /// Returns: `Value::U8([0,1])`
    /// PATH: netRemote.sys.clock.mode
    SysClockMode,

    /// Fetch/set the time source.
    ///
    /// Manual: 0
    /// DAB: 1
    /// FM: 2
    /// ???: 3
    /// Internet: 4
    ///
    /// Valid values can be fetched with `SysCapsClockSourceList`.
    ///
    /// Method: SET/GET
    /// Returns: `Value::U8(_)`
    /// PATH: netRemote.sys.clock.source
    SysClockSource,

    /// Get/set the offset of the local time compared to UTC.
    ///
    /// Valid values can be fetched with `SysCapsUtcSettingsList`.
    ///
    /// Method: SET/GET
    /// Returns: `Value::U8(_)`
    /// PATH: netRemote.sys.clock.utcOffset
    SysClockUtcOffset,

    // sys.cfg
    /// TODO: description
    ///
    /// Is part of the new API Version.
    ///
    /// Method: SET/GET
    /// Returns: `Value::U8(_)`
    /// PATH: netRemote.sys.cfg.irAutoPlayFlag
    SysCfgIrAutoPlayFlag,

    // sys.info
    /// Sets/ Returns the Network-Name of the Device
    ///
    /// Method: SET/GET
    /// Returns: `Value::Text(String)`
    /// PATH: netRemote.sys.info.friendlyName
    SysInfoFriendlyName,

    /// Returns unique? ID Radio-ID
    ///
    /// Method: GET
    /// Returns: `Value::Text(String)`
    /// PATH: netRemote.sys.info.radioId
    SysInfoRadioId,

    /// Change the PIN used to access the radio API.
    ///
    /// Method: SET
    /// Value: pin
    /// PATH: netRemote.sys.info.radioPin
    SysInfoRadioPin,

    /// Returns Image-Version String
    ///
    /// Method: GET
    /// Returns: `Value::Text(String)`
    /// PATH: netRemote.sys.info.version
    SysInfoVersion,

    /// Get or Set the Name of the Device which is remote-controling the radio
    ///
    /// Method: GET/SET
    /// Returns: `Value::Text(String)`
    /// PATH: netRemote.sys.info.controllerName
    SysInfoControlName,

    // sys.isu
    /// TODO: Descscription
    ///
    /// Value:
    ///     2 = search for updates
    ///
    /// Method: GET/SET
    /// Returns: `Value::U8(_)`
    /// PATH: netRemote.sys.isu.control
    SysIsuControl,

    /// Shows the update process, default 0
    ///
    /// while searching: 1
    /// after searching: 0
    /// ???: 3
    ///
    /// Method: GET
    /// Returns: `Value::U8([0,1])`
    /// PATH: netRemote.sys.isu.state
    SysIsuState,

    // sys.net
    // sys.net.ipConfig
    /// Returns/sets? the IP address for the connected network
    ///
    /// Method: GET/SET
    /// Returns: `Value::U32(_)`
    /// PATH: netRemote.sys.net.ipConfig.address
    SysNetIpConfigAddress,

    /// Returns if DHCP is enabled for the connected network
    ///
    /// Method: GET/SET
    /// Returns: `Value::U8([0,1])`
    /// PATH: netRemote.sys.net.ipConfig.dhcp
    SysNetIpConfigDhcp,

    /// Returns the primary dns for the connected network
    ///
    /// Method: GET/SET
    /// Returns: `Value::U32(_)`
    /// PATH: netRemote.sys.net.ipConfig.dnsPrimary
    SysNetIpConfigDnsPrimary,

    /// Returns the secundary dns for the connected network
    ///
    /// Method: GET/SET
    /// Returns: `Value::U32(_)`
    /// PATH: netRemote.sys.net.ipConfig.dnsSecundary
    SysNetIpConfigDnsSecundary,

    /// Returns the default gateway for the connected network
    ///
    /// Method: GET/SET
    /// Returns: `Value::U32(_)`
    /// PATH: netRemote.sys.net.ipConfig.gateway
    SysNetIpConfigGateway,

    /// Returns the subnet masky for the connected network
    ///
    /// Method: GET/SET
    /// Returns: `Value::U32(_)`
    /// PATH: netRemote.sys.net.ipConfig.subnetMask
    SysNetIpConfigSubnetMask,

    /// If set to 1 network connection is not disconnected in standby
    ///
    /// Method: GET/SET
    /// Returns: `Value::U8([0,1])`
    /// PATH: netRemote.sys.net.ipConfig.keepConnected
    SysNetIpConfigKeepConnected,

    /// Returns the NIC Status of the Ethernet Device
    ///
    /// Method: GET/SET
    /// Returns: `Value::U8(_)`
    /// PATH: netRemote.sys.net.ipConfig.wired.interfaceEnable
    SysNetWiredInterfaceEnable,

    /// Returns the MAC Address of the Ethernet Device
    ///
    /// Method: GET
    /// Returns: `Value::Text(_)`
    /// PATH: netRemote.sys.net.ipConfig.wired.macAddress
    SysNetWiredInterfaceMacAddress,

    /// Returns the SSID of the connected WIFI network
    ///
    /// Method: GET
    ///
    /// Returns: `Value::Text(_)`
    /// PATH: netRemote.sys.net.ipConfig.wired.macAddress
    SysNetWlanConnectedSSID,

    /// Returns the NIC Status of the WIFI Device
    ///
    /// Method: GET/SET
    /// Returns: `Value::U8(_)`
    /// PATH: netRemote.sys.net.ipConfig.wlan.interfaceEnable
    SysNetWlanInterfaceEnable,

    /// Returns the MAC Address of the WIFI Device
    ///
    /// Method: GET
    /// Returns: `Value::Text(_)`
    /// PATH: netRemote.sys.net.ipConfig.wlan.macAddress
    SysNetWlanInterfaceMacAddress,

    /// Returns the Signal Strenght of the connected WIFI network
    ///
    /// Method: GET
    /// Returns: `Value::u8(_)`
    /// PATH: netRemote.sys.net.ipConfig.wlan.rssi
    SysNetWlanRssi,

    /// Returns the ??? of the connected WIFI network
    ///
    /// Method: GET
    /// Returns: `Value::u8(_)`
    /// PATH: netRemote.sys.net.ipConfig.wlan.setAuthType
    SysNetWlanSetAuthType,

    /// Returns the encryption type of the connected WIFI network
    ///
    /// Method: GET
    /// Returns: `Value::u8(_)`
    /// PATH: netRemote.sys.net.ipConfig.wlan.setEncType
    SysNetWlanSetEncType,

    /// Fetch the public RSA key that is used to encrypt the Wifi password before
    /// sending it to the radio.
    ///
    /// Method: GET
    /// Returns: `Value::u32(_)`
    /// PATH: netRemote.sys.rsa.publicKey
    SysRsaPublicKey,

    /// Check the status of the RSA key generation.
    /// - 0: Generating
    /// - 1: Ready
    ///
    /// Method: GET
    /// Returns: `Value::u32(_)`
    /// PATH: netRemote.sys.rsa.status
    SysRsaStatus,
}

impl std::fmt::Display for SessionID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Node::*;
        let node = match self {
            // nav
            NavList => NODE_NAV_LIST,
            NavNumItems => NODE_NAV_NUMITEMS,
            NavPresets => NODE_NAV_PRESETS,
            NavSearchTerm => NODE_NAV_SEARCHTERM,
            NavState => NODE_NAV_STATE,
            NavStatus => NODE_NAV_STATUS,
            NavDepth => NODE_NAV_DEPTH,

            // nav.action
            NavActionDabScan => NODE_NAV_ACTION_DABSCAN,
            NavActionNavigate => NODE_NAV_ACTION_NAVIGATE,
            NavActionSelectItem => NODE_NAV_ACTION_SELECTITEM,
            NavActionSelectPreset => NODE_NAV_ACTION_SELECTPRESET,

            // play
            PlayAddPreset => NODE_PLAY_ADDPRESET,
            PlayCaps => NODE_PLAY_CAPS,
            PlayControl => NODE_PLAY_CONTROL,
            PlayErrorStr => NODE_PLAY_ERRORSTR,
            PlayFrequency => NODE_PLAY_FREQUENCY,
            PlayPosition => NODE_PLAY_POSITION,
            PlayRate => NODE_PLAY_RATE,
            PlayRepeat => NODE_PLAY_REPEAT,
            PlayScrobble => NODE_PLAY_SCROBBLE,
            PlayShuffle => NODE_PLAY_SHUFFLE,
            PlayShuffleStatus => NODE_PLAY_SHUFFLESTATUS,
            PlaySignalStrength => NODE_PLAY_SIGNALSTRENGTH,
            PlayStatus => NODE_PLAY_STATUS,

            // play.info
            PlayInfoAlbum => NODE_PLAY_INFO_ALBUM,
            PlayInfoArtist => NODE_PLAY_INFO_ARTIST,
            PlayInfoDuration => NODE_PLAY_INFO_DURATION,
            PlayInfoGraphicUri => NODE_PLAY_INFO_GRAPHICURI,
            PlayInfoName => NODE_PLAY_INFO_NAME,
            PlayInfoText => NODE_PLAY_INFO_TEXT,

            // play.serviceIds => NODE_PLAY_
            PlayServiceIdsDabEnsambleId => NODE_PLAY_SERVICEIDS_DABENSEMBLEID,
            PlayServiceIdsDabScids => NODE_PLAY_SERVICEIDS_DABSCIDS,
            PlayServiceIdsDabServiceId => NODE_PLAY_SERVICEIDS_DABSERVICEID,
            PlayServiceIdsEcc => NODE_PLAY_SERVICEIDS_ECC,
            PlayServiceIdsfmRdsPi => NODE_PLAY_SERVICEIDS_FMRDSPI,

            // sys
            SysLang => NODE_SYS_LANG,
            SysMode => NODE_SYS_MODE,
            SysPower => NODE_SYS_POWER,
            SysSleep => NODE_SYS_SLEEP,
            SysState => NODE_SYS_STATE,

            // sys.audio
            SysAudioEqCustomParam0 => NODE_SYS_AUDIO_EQCUSTOM_PARAM0,
            SysAudioEqCustomParam1 => NODE_SYS_AUDIO_EQCUSTOM_PARAM1,

            SysAudioEqLoudness => NODE_SYS_AUDIO_EQLOUDNESS,
            SysAudioEqPreset => NODE_SYS_AUDIO_EQPRESET,
            SysAudioVolume => NODE_SYS_AUDIO_VOLUME,
            SysAudioMute => NODE_SYS_AUDIO_MUTE,

            // sys.caps
            SysCapsClockSourceList => NODE_SYS_CAPS_CLOCKSOURCELIST,
            SysCapsDabFreqList => NODE_SYS_CAPS_DABFREQLIST,
            SysCapsEqBands => NODE_SYS_CAPS_EQBANDS,
            SysCapsEqPresets => NODE_SYS_CAPS_EQPRESETS,

            SysCapsFmFreqRangeLower => NODE_SYS_CAPS_FMFREQRANGE_LOWER,
            SysCapsFmFreqRangeStepSize => NODE_SYS_CAPS_FMFREQRANGE_STEPSIZE,
            SysCapsFmFreqRangeUpper => NODE_SYS_CAPS_FMFREQRANGE_UPPER,

            SysCapsValidModes => NODE_SYS_CAPS_VALIDMODES,
            SysCapsVolumeSteps => NODE_SYS_CAPS_VOLUMESTEPS,

            // sys.clock
            SysClockDst => NODE_SYS_CLOCK_DST,
            SysClockLocalDate => NODE_SYS_CLOCK_LOCALDATE,
            SysClockLocalTime => NODE_SYS_CLOCK_LOCALTIME,
            SysClockMode => NODE_SYS_CLOCK_MODE,
            SysClockSource => NODE_SYS_CLOCK_SOURCE,
            SysClockUtcOffset => NODE_SYS_CLOCK_UTCOFFSET,

            SysCfgIrAutoPlayFlag => NODE_SYS_CFG_IRAUTOPLAYFLAG,

            // sys.info
            SysInfoFriendlyName => NODE_INFO_FRIENDLYNAME,
            SysInfoRadioId => NODE_INFO_RADIOID,
            SysInfoRadioPin => NODE_INFO_RADIOPIN,
            SysInfoVersion => NODE_INFO_VERSION,
            SysInfoControlName => NODE_INFO_CONTROLLERNAME,

            // sys.isu
            SysIsuControl => NODE_ISU_CONTROL,
            SysIsuState => NODE_ISU_STATE,

            // sys.net
            // sys.net.ipConfig
            SysNetIpConfigAddress => NODE_NET_IPCONFIG_ADDRESS,
            SysNetIpConfigDhcp => NODE_NET_IPCONFIG_DHCP,
            SysNetIpConfigDnsPrimary => NODE_NET_IPCONFIG_DNSPRIMARY,
            SysNetIpConfigDnsSecundary => NODE_NET_IPCONFIG_DNSECUNDARY,
            SysNetIpConfigGateway => NODE_NET_IPCONFIG_GATEWAY,
            SysNetIpConfigSubnetMask => NODE_NET_IPCONFIG_SUBNETMASK,
            SysNetIpConfigKeepConnected => NODE_NET_IPCONFIG_KEEPCONNECTED,
            SysNetWiredInterfaceEnable => NODE_NET_WIRED_INTERFACEENABLE,
            SysNetWiredInterfaceMacAddress => NODE_NET_WIRED_MACADDRESS,
            SysNetWlanConnectedSSID => NODE_NET_WLAN_CONNECTEDSSID,
            SysNetWlanInterfaceEnable => NODE_NET_WLAN_INTERFACEENABLE,
            SysNetWlanInterfaceMacAddress => NODE_NET_WLAN_MACADDRESS,
            SysNetWlanRssi => NODE_NET_WLAN_RSSI,
            SysNetWlanSetAuthType => NODE_NET_WLAN_SETAUTHTYPE,
            SysNetWlanSetEncType => NODE_NET_WLAN_SETENCTYPE,

            // sys.rsa
            SysRsaPublicKey => NODE_SYS_RSA_PUBLICKEY,
            SysRsaStatus => NODE_SYS_RSA_STATUS,
        };
        write!(f, "{node}")
    }
}

impl TryFrom<String> for Node {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        use Node::*;
        let node = match value.as_str() {
            // nav
            NODE_NAV_LIST => NavList,
            NODE_NAV_NUMITEMS => NavNumItems,
            NODE_NAV_PRESETS => NavPresets,
            NODE_NAV_SEARCHTERM => NavSearchTerm,
            NODE_NAV_STATE => NavState,
            NODE_NAV_STATUS => NavStatus,
            NODE_NAV_DEPTH => NavDepth,

            // nav.action
            NODE_NAV_ACTION_DABSCAN => NavActionDabScan,
            NODE_NAV_ACTION_NAVIGATE => NavActionNavigate,
            NODE_NAV_ACTION_SELECTITEM => NavActionSelectItem,
            NODE_NAV_ACTION_SELECTPRESET => NavActionSelectPreset,

            // play
            NODE_PLAY_ADDPRESET => PlayAddPreset,
            NODE_PLAY_CAPS => PlayCaps,
            NODE_PLAY_CONTROL => PlayControl,
            NODE_PLAY_ERRORSTR => PlayErrorStr,
            NODE_PLAY_FREQUENCY => PlayFrequency,
            NODE_PLAY_POSITION => PlayPosition,
            NODE_PLAY_RATE => PlayRate,
            NODE_PLAY_REPEAT => PlayRepeat,
            NODE_PLAY_SCROBBLE => PlayScrobble,
            NODE_PLAY_SHUFFLE => PlayShuffle,
            NODE_PLAY_SHUFFLESTATUS => PlayShuffleStatus,
            NODE_PLAY_SIGNALSTRENGTH => PlaySignalStrength,
            NODE_PLAY_STATUS => PlayStatus,

            // play.info
            NODE_PLAY_INFO_ALBUM => PlayInfoAlbum,
            NODE_PLAY_INFO_ARTIST => PlayInfoArtist,
            NODE_PLAY_INFO_DURATION => PlayInfoDuration,
            NODE_PLAY_INFO_GRAPHICURI => PlayInfoGraphicUri,
            NODE_PLAY_INFO_NAME => PlayInfoName,
            NODE_PLAY_INFO_TEXT => PlayInfoText,

            // play.serviceIds => NODE_PLAY_
            NODE_PLAY_SERVICEIDS_DABENSEMBLEID => PlayServiceIdsDabEnsambleId,
            NODE_PLAY_SERVICEIDS_DABSCIDS => PlayServiceIdsDabScids,
            NODE_PLAY_SERVICEIDS_DABSERVICEID => PlayServiceIdsDabServiceId,
            NODE_PLAY_SERVICEIDS_ECC => PlayServiceIdsEcc,
            NODE_PLAY_SERVICEIDS_FMRDSPI => PlayServiceIdsfmRdsPi,

            // sys
            NODE_SYS_LANG => SysLang,
            NODE_SYS_MODE => SysMode,
            NODE_SYS_POWER => SysPower,
            NODE_SYS_SLEEP => SysSleep,
            NODE_SYS_STATE => SysState,

            // sys.audio
            NODE_SYS_AUDIO_EQCUSTOM_PARAM0 => SysAudioEqCustomParam0,
            NODE_SYS_AUDIO_EQCUSTOM_PARAM1 => SysAudioEqCustomParam1,

            NODE_SYS_AUDIO_EQLOUDNESS => SysAudioEqLoudness,
            NODE_SYS_AUDIO_EQPRESET => SysAudioEqPreset,
            NODE_SYS_AUDIO_VOLUME => SysAudioVolume,
            NODE_SYS_AUDIO_MUTE => SysAudioMute,

            // sys.caps
            NODE_SYS_CAPS_CLOCKSOURCELIST => SysCapsClockSourceList,
            NODE_SYS_CAPS_DABFREQLIST => SysCapsDabFreqList,
            NODE_SYS_CAPS_EQBANDS => SysCapsEqBands,
            NODE_SYS_CAPS_EQPRESETS => SysCapsEqPresets,

            NODE_SYS_CAPS_FMFREQRANGE_LOWER => SysCapsFmFreqRangeLower,
            NODE_SYS_CAPS_FMFREQRANGE_STEPSIZE => SysCapsFmFreqRangeStepSize,
            NODE_SYS_CAPS_FMFREQRANGE_UPPER => SysCapsFmFreqRangeUpper,

            NODE_SYS_CAPS_VALIDMODES => SysCapsValidModes,
            NODE_SYS_CAPS_VOLUMESTEPS => SysCapsVolumeSteps,

            // sys.clock
            NODE_SYS_CLOCK_DST => SysClockDst,
            NODE_SYS_CLOCK_LOCALDATE => SysClockLocalDate,
            NODE_SYS_CLOCK_LOCALTIME => SysClockLocalTime,
            NODE_SYS_CLOCK_MODE => SysClockMode,
            NODE_SYS_CLOCK_SOURCE => SysClockSource,
            NODE_SYS_CLOCK_UTCOFFSET => SysClockUtcOffset,

            // sys.cfg
            NODE_SYS_CFG_IRAUTOPLAYFLAG => SysCfgIrAutoPlayFlag,

            // sys.info
            NODE_INFO_FRIENDLYNAME => SysInfoFriendlyName,
            NODE_INFO_RADIOID => SysInfoRadioId,
            NODE_INFO_RADIOPIN => SysInfoRadioPin,
            NODE_INFO_VERSION => SysInfoVersion,
            NODE_INFO_CONTROLLERNAME => SysInfoControlName,

            // sys.isu
            NODE_ISU_CONTROL => SysIsuControl,
            NODE_ISU_STATE => SysIsuState,

            // sys.net
            // sys.net.ipConfig
            NODE_NET_IPCONFIG_ADDRESS => SysNetIpConfigAddress,
            NODE_NET_IPCONFIG_DHCP => SysNetIpConfigDhcp,
            NODE_NET_IPCONFIG_DNSPRIMARY => SysNetIpConfigDnsPrimary,
            NODE_NET_IPCONFIG_DNSECUNDARY => SysNetIpConfigDnsSecundary,
            NODE_NET_IPCONFIG_GATEWAY => SysNetIpConfigGateway,
            NODE_NET_IPCONFIG_SUBNETMASK => SysNetIpConfigSubnetMask,
            NODE_NET_IPCONFIG_KEEPCONNECTED => SysNetIpConfigKeepConnected,
            NODE_NET_WIRED_INTERFACEENABLE => SysNetWiredInterfaceEnable,
            NODE_NET_WIRED_MACADDRESS => SysNetWiredInterfaceMacAddress,
            NODE_NET_WLAN_CONNECTEDSSID => SysNetWlanConnectedSSID,
            NODE_NET_WLAN_INTERFACEENABLE => SysNetWlanInterfaceEnable,
            NODE_NET_WLAN_MACADDRESS => SysNetWlanInterfaceMacAddress,
            NODE_NET_WLAN_RSSI => SysNetWlanRssi,
            NODE_NET_WLAN_SETAUTHTYPE => SysNetWlanSetAuthType,
            NODE_NET_WLAN_SETENCTYPE => SysNetWlanSetEncType,

            // sys.rsa
            NODE_SYS_RSA_PUBLICKEY => SysRsaPublicKey,
            NODE_SYS_RSA_STATUS => SysRsaStatus,

            node => dbg!(Err(InternalError::new(format!("Unknown node: {node}"))))?,
        };

        Ok(node)
    }
}

impl FsApi {
    /// Gets the vlaue of an node
    pub async fn get<D: Display>(node: Node, host: D, pin: D) -> Result<Value, Error> {
        let url = format!("http://{host}/{FSAPI_PATH}/{GET_PATH}/{node}?pin={pin}");

        let reponse = Response::from_str(&reqwest::get(url).await?.text().await?)?;

        if ResponseStatus::Ok != reponse.status {
            return Err(Error::InvalidStatus);
        }

        match reponse.data {
            Some(Data::Value(value)) => Ok(value),
            _ => Err(Error::InvalidData),
        }
    }

    /// Sets the vlaue of an node
    pub async fn set<D: Display, V: Display>(
        node: Node,
        param: V,
        host: D,
        pin: D,
    ) -> Result<(), Error> {
        let url = format!("http://{host}/{FSAPI_PATH}/{SET_PATH}/{node}?pin={pin}&value={param}");

        let response = Response::from_str(&reqwest::get(url).await?.text().await?)?;

        if ResponseStatus::Ok != response.status {
            return Err(Error::InvalidStatus);
        }

        match response.data {
            None => Ok(()),
            _ => Err(Error::InvalidData),
        }
    }

    // Get the next "page" of a list stored in the node
    //
    // NOTE: currently only 1 page supported, this page has a max of 999 items
    // should be enough
    pub async fn get_item_list<D: Display>(
        node: Node,
        session_id: Option<SessionID>,
        host: D,
        pin: D,
    ) -> Result<Vec<Item>, Error> {
        let max_items = 65536;
        let url = match session_id {
            None => format!("http://{host}/{FSAPI_PATH}/{LIST_GET_NEXT_PATH}/{node}/-1?pin={pin}&maxItems={max_items}"),
            Some(sid) => format!(
                "http://{host}/{FSAPI_PATH}/{LIST_GET_NEXT_PATH}/{node}/-1?pin={pin}&SID={sid}&maxItems={max_items}"
            ),
        };

        let response = Response::from_str(&reqwest::get(url).await?.text().await?)?;

        if ResponseStatus::Ok != response.status {
            return Err(Error::InvalidStatus);
        }
        match response.status {
            ResponseStatus::Ok => (),
            ResponseStatus::Fail => return Err(Error::Fail),
            _ => return Err(Error::InvalidStatus),
        }

        match response.data {
            Some(Data::Items(items)) => Ok(items),
            _ => Err(Error::InvalidData),
        }
    }

    /// This is a special Command: The device does not close the HTTP-Request.
    ///
    /// With this command you can use the connection like a normal socket,
    /// where the device sends you updates about changed nodes.
    ///
    /// This Command is only available if a Session-ID is used to authenticate.
    ///
    /// It gives you a FS_TIMEOUT error if nothing has changed.
    pub async fn get_notifications<D: Display>(
        session_id: SessionID,
        host: D,
        pin: D,
    ) -> Result<Option<Vec<Notification>>, Error> {
        let url =
            format!("http://{host}/{FSAPI_PATH}/{GET_NOTIFIES_PATH}?pin={pin}&sid={session_id}");

        let response = Response::from_str(&reqwest::get(url).await?.text().await?)?;

        match response.status {
            ResponseStatus::Ok => (),
            ResponseStatus::Timeout => return Ok(None),
            _ => return Err(Error::InvalidStatus),
        }

        match response.data {
            Some(Data::Notify(notifications)) => Ok(Some(notifications)),
            _ => Err(Error::InvalidData),
        }
    }

    /// Login and create a new session
    ///
    /// There can only be 1 session at a time.
    /// If a new is created while another existed the old one will be purged
    pub async fn create_session<D: Display>(host: D, pin: D) -> Result<SessionID, Error> {
        let url = format!("http://{host}/{FSAPI_PATH}/{CREATE_SESSION_PATH}?pin={pin}");

        let response = Response::from_str(&reqwest::get(url).await?.text().await?)?;

        if ResponseStatus::Ok != response.status {
            return Err(Error::InvalidStatus);
        }

        match response.data {
            Some(Data::SessionID(session_id)) => Ok(session_id),
            _ => Err(Error::InvalidData),
        }
    }

    /// Logout and destroy the session
    pub async fn delete_session<D: Display>(
        session_id: SessionID,
        host: D,
        pin: D,
    ) -> Result<(), Error> {
        let url =
            format!("http://{host}/{FSAPI_PATH}/{DELETE_SESSION_PATH}?pin={pin}&sid={session_id}");

        let response = Response::from_str(&reqwest::get(url).await?.text().await?)?;

        if ResponseStatus::Ok != response.status {
            return Err(Error::InvalidStatus);
        }

        match response.data {
            None => Ok(()),
            _ => Err(Error::InvalidData),
        }
    }
}

impl Value {
    fn from_reader(
        reader: &mut quick_xml::Reader<&[u8]>,
        buf: &mut Vec<u8>,
    ) -> Result<Value, InternalError> {
        if let Event::Start(ref e) = reader.read_event(buf)? {
            match e.name() {
                b"c8_array" => Ok(Value::Text(reader.read_text(e.name(), &mut Vec::new())?)),
                b"u8" => Ok(Value::U8(
                    reader.read_text(e.name(), &mut Vec::new())?.parse()?,
                )),
                b"s16" => Ok(Value::S16(
                    reader.read_text(e.name(), &mut Vec::new())?.parse()?,
                )),
                b"u32" => Ok(Value::U32(
                    reader.read_text(e.name(), &mut Vec::new())?.parse()?,
                )),
                b"array" => Ok(Value::Array(reader.read_text(e.name(), &mut Vec::new())?)),
                _ => Err(InternalError::Value(String::from("Unknown option"))),
            }
        } else {
            Err(InternalError::Value(String::from("Incorrect value format")))
        }
    }
}

impl SessionID {
    fn from_reader(
        reader: &mut quick_xml::Reader<&[u8]>,
        buf: &mut Vec<u8>,
    ) -> Result<Self, InternalError> {
        if let Event::Text(ref e) = reader.read_event(buf)? {
            Ok(Self(String::from_utf8(e.escaped().to_vec())?.parse()?))
        } else {
            Err(InternalError::SessionId(String::from(
                "Incorrect value format",
            )))
        }
    }
}

impl Item {
    fn items_from_reader(
        first_key: u32,
        reader: &mut quick_xml::Reader<&[u8]>,
    ) -> Result<Vec<Self>, InternalError> {
        let mut items = Vec::new();
        let mut key = first_key;

        'item: loop {
            let mut fields: Vec<Field> = Vec::new();

            'field: loop {
                // Read new `Field`
                match reader.read_event(&mut Vec::new())? {
                    Event::Start(ref e) => match e.name() {
                        b"field" => {
                            let name: String = match e.attributes().next() {
                                Some(attribute) => {
                                    String::from_utf8(attribute?.value.to_mut().to_vec())?
                                }
                                None => {
                                    return Err(InternalError::Item(String::from(
                                        "Invalid field name format",
                                    )))
                                }
                            };

                            let value = Value::from_reader(reader, &mut Vec::new())?;

                            // Throw away </field>
                            reader.read_event(&mut Vec::new())?;

                            fields.push(Field { name, value });
                        }
                        _ => return Err(InternalError::Field(String::from("Unexpected start"))),
                    },
                    Event::End(ref e) => match e.name() {
                        b"item" => break 'field,
                        _ => return Err(InternalError::Field(String::from("Unexpected end"))),
                    },
                    _ => return Err(InternalError::Field(String::from("Unexpected event"))),
                }
            }

            items.push(Item { key, fields });

            // Check if there are more items, if not break 'item
            // the key for the next item is lost if not collected here
            match reader.read_event(&mut Vec::new())? {
                Event::Start(ref e) => {
                    key = match e.attributes().next() {
                        Some(attribute) => {
                            String::from_utf8(attribute?.value.to_mut().to_vec())?.parse()?
                        }
                        None => {
                            return Err(InternalError::Item(String::from("Invalid item format")))
                        }
                    };
                }
                Event::Empty(ref e) => match e.name() {
                    b"listend" => break 'item, // </listend>
                    _ => return Err(InternalError::Item(String::from("Unexpected end"))),
                },
                _ => return Err(InternalError::Item(String::from("Unexpected event"))),
            };
        }
        Ok(items)
    }
}

impl Notification {
    fn notifications_from_reader(
        first_node: Node,
        reader: &mut quick_xml::Reader<&[u8]>,
    ) -> Result<Vec<Self>, InternalError> {
        let mut notifications = Vec::new();

        let mut node = first_node;

        loop {
            // Throw away <value>
            reader.read_event(&mut Vec::new())?;
            let value = Value::from_reader(reader, &mut Vec::new())?;

            // Throw away </value>
            reader.read_event(&mut Vec::new())?;

            // Throw away </notify>
            reader.read_event(&mut Vec::new())?;

            notifications.push(Notification { node, value });

            // Check if there are more notifications, if not break 'notifu
            // the node for the next item is lost if not collected here
            node = match reader.read_event(&mut Vec::new())? {
                Event::Start(ref e) => {
                    // <notify node="...">
                    match e.attributes().next() {
                        Some(attribute) => {
                            Node::try_from(String::from_utf8(attribute?.value.to_mut().to_vec())?)?
                        }
                        None => {
                            return Err(InternalError::Notify(String::from(
                                "Invalid notify format",
                            )))
                        }
                    }
                }
                Event::End(ref e) => match e.name() {
                    b"fsapiResponse" => break, // </fsapiResponse>
                    _ => return Err(InternalError::Notify(String::from("Unexpected end"))),
                },
                _ => return Err(InternalError::Notify(String::from("Unexpected event"))),
            };
        }

        Ok(notifications)
    }
}

impl Data {
    fn from_reader(
        reader: &mut quick_xml::Reader<&[u8]>,
        buf: &mut Vec<u8>,
    ) -> Result<Option<Self>, InternalError> {
        match reader.read_event(buf)? {
            Event::Start(ref e) => match e.name() {
                b"value" => Ok(Some(Self::Value(Value::from_reader(reader, buf)?))),
                b"sessionId" => Ok(Some(Self::SessionID(SessionID::from_reader(reader, buf)?))),
                b"item" => {
                    let key: u32 = match e.attributes().next() {
                        Some(attribute) => {
                            String::from_utf8(attribute?.value.to_mut().to_vec())?.parse()?
                        }
                        None => {
                            return Err(InternalError::Item(String::from("Invalid item format")))
                        }
                    };

                    Ok(Some(Data::Items(Item::items_from_reader(key, reader)?)))
                }
                b"notify" => {
                    let node: Node = match e.attributes().next() {
                        Some(attribute) => {
                            Node::try_from(String::from_utf8(attribute?.value.to_mut().to_vec())?)?
                        }
                        None => {
                            return Err(InternalError::Notify(String::from(
                                "Invalid notify format",
                            )))
                        }
                    };

                    Ok(Some(Data::Notify(Notification::notifications_from_reader(
                        node, reader,
                    )?)))
                }
                _ => panic!("Unknown data type"),
            },
            Event::End(_) => Ok(None), // </fsapiResponse>
            _ => Err(InternalError::Response(String::from(
                "Incorrect data format",
            ))),
        }
    }
}

impl Response {
    pub fn from_str(s: &str) -> Result<Self, InternalError> {
        let mut reader = quick_xml::Reader::from_str(s);
        reader.trim_text(true);

        let mut buf = Vec::new();

        // Throw away <fsapiResponse>
        reader.read_event(&mut buf)?;

        let status = if let Event::Start(ref e) = reader.read_event(&mut buf)? {
            use ResponseStatus::*;
            match reader.read_text(e.name(), &mut Vec::new())?.as_str() {
                STATUS_FS_OK => Ok,
                STATUS_FS_FAIL => Fail,
                STATUS_FS_PACKET_BAD => PacketBad,
                STATUS_FS_NODE_BLOCKED => NodeBlocked,
                STATUS_FS_NODE_DOES_NOT_EXIST => NodeDoesNotExist,
                STATUS_FS_TIMEOUT => Timeout,
                STATUS_FS_LIST_END => ListEnd,
                status => panic!("Unknow status: {status}"), // Err(InternalError::Generic(format!("Unknow status: {status}")))?,
            }
        } else {
            return Err(InternalError::Response(String::from(
                "Incorrect response format",
            )));
        };

        let data: Option<Data> = if status == ResponseStatus::Ok {
            Data::from_reader(&mut reader, &mut buf)?
        } else {
            None
        };

        Ok(Response { status, data })
    }
}
