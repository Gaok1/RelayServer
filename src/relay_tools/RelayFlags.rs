

pub type RelayFlag = u16;

// Requisitons[client]
pub const STORE : RelayFlag = 0x0; //to save peer data [id -> addr]
pub const DISCOVER : RelayFlag = 0x11; // [(peerID) --> (peerIpAddr)] to discover peers

//answers [client]
pub const WAITING_PUNCH : RelayFlag = 0x10;

//answers [server]
// STORE TYPE ANSWERS
pub const STORED : RelayFlag = 0x01; //to save peer data [id -> addr]
pub const NOT_STORED : RelayFlag = 0x02; //to save peer data [id -> addr]

// DISCOVER TYPE ANSWERS
pub const NOT_PRESENT : RelayFlag = 0x110;
pub const PRESENT : RelayFlag = 0x111;

// PUNCH TYPE ANSWERS
pub const PUNCH : RelayFlag = 0x102;



// Requisitons type
// Header  -> flag
// content -> data
// | separated information
//#  example
// STORE|ID|
// DISCOVER|ID
// WAITING_PUNCH
// PUNCH 
// STORED
// NOT_STORED
// PRESENT|ID|IP|PORT
