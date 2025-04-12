
//! Constantes de flags usadas pelo protocolo do relay.

pub type RelayFlag = u16;

// --- Requisições do cliente ---> Servidor
pub const STORE: RelayFlag = 0x000;      // Registra (ID -> Endereço)
pub const DISCOVER: RelayFlag = 0x011;   // Descobre endereço de outro peer
pub const WAITING_PUNCH: RelayFlag = 0x010; // Informa que está aguardando o hole‑punch

// --- Respostas do servidor ---> Cliente
// STORE
pub const STORED: RelayFlag = 0x001;
pub const NOT_STORED: RelayFlag = 0x002;
// DISCOVER
pub const PRESENT: RelayFlag = 0x111;
pub const NOT_PRESENT: RelayFlag = 0x110;
// HOLE‑PUNCH
pub const PUNCH_WAITING_TIMEOUT: RelayFlag = 0x101; // Timeout para o hole-punch
pub const PUNCH: RelayFlag = 0x102;

pub const INTERNAL_ERROR: RelayFlag = 0x999; // Erro interno do servidor
pub const INVALID_REQUEST_FORMAT: RelayFlag = 0x998; // Formato de requisição inválido



// Requisitons type
// Header  -> flag
// content -> data
// | separated information
//#  example
// STORE|ID|
// DISCOVER|USERID|DiscoverUserID
// WAITING_PUNCH|USERID|PEERID
// PUNCH 
// STORED
// NOT_STORED
// PRESENT|ID|IP|PORT
