use crate::errors::Error;
use iocore::FSNode;
use adler32::adler32;
use crc::{
    Crc,
    CRC_3_GSM,
    CRC_4_G_704,
    CRC_5_G_704,
    CRC_6_CDMA2000_A,
    CRC_6_CDMA2000_B,
    CRC_6_GSM,
    CRC_8_LTE,
    CRC_11_FLEXRAY,
    CRC_16_TELEDISK,
    CRC_16_OPENSAFETY_A,
    CRC_16_OPENSAFETY_B,
    CRC_16_PROFIBUS,
    CRC_16_USB,
    CRC_16_XMODEM,
    CRC_24_BLE,
    CRC_24_OPENPGP,
    CRC_32_BZIP2,
    CRC_32_ISCSI,
    CRC_32_ISO_HDLC,
    CRC_32_JAMCRC,
    CRC_32_MPEG_2,
    CRC_40_GSM,
    CRC_64_GO_ISO,
    CRC_64_REDIS,
    CRC_64_ECMA_182,
    CRC_64_XZ,
    CRC_64_MS,
    CRC_64_WE,
    CRC_82_DARC
};
pub use sha::sha1::Sha1;
pub use sha::utils::Digest;
pub use sha::utils::DigestExt;
pub use sha2::{
    Sha512,
    Sha224,
    Sha384,
    Sha256,
    Sha512_224,
    Sha512_256,
};
use sha2::Digest as Sha2Digest;

pub use sha3::{
    Keccak256Full,
    Keccak256,
    Keccak224,
    Sha3_224,
    Sha3_256,
    Sha3_384,
    Sha3_512,
};
pub use md5::compute as md5_compute;
use serde::{Serialize,Deserialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HochTable {
    meta: Option<FSNode>,
    sha1: String,
    sha512: String,
    sha224: String,
    sha384: String,
    sha256: String,
    sha512_224: String,
    sha512_256: String,
    keccak256full: String,
    keccak256: String,
    keccak224: String,
    sha3_224: String,
    sha3_256: String,
    sha3_384: String,
    sha3_512: String,
    adler32: String,
    crc3gsm: String,
    crc4g704: String,
    crc5g704: String,
    crc6gsm: String,
    crc6cdma2000a: String,
    crc6cdma2000b: String,
    // crc5_g_704: String,
    // crc5usb: String,
    crc8lte: String,
    // Crc8_WCDMA: String,
    // Crc8_SMBUS: String,
    // Crc8_OPENSAFETY: String,
    // Crc8_HITAG: String,
    // Crc10_ATM: String,
    // Crc10_CDMA2000: String,
    // Crc10_GSM: String,
    crc11flexray: String,
    // Crc11_UMTS: String,
    // Crc16_GSM: String,
    // Crc16_ISO_IEC_14443_3_A: String,
    // Crc16_MAXIM_DOW: String,
    // Crc16_MCRF4XX: String,
    // Crc16_MODBUS: String,
    // Crc16_NRSC_5: String,
    crc16opensafetya: String,
    crc16opensafetyb: String,
    crc16profibus: String,
    crc16teledisk: String,
    crc16usb: String,
    crc16xmodem: String,
    crc24ble: String,
    crc24openpgp: String,
    // Crc32_AIXM: String,
    // Crc32_AUTOSAR: String,
    // Crc32_BASE91_D: String,
    crc32bzip2: String,
    crc32iscsi: String,
    crc32isohdlc: String,
    crc32jamcrc: String,
    crc32mpeg2: String,
    crc40gsm: String,
    crc64ecma182: String,
    crc64goiso: String,
    crc64ms: String,
    crc64redis: String,
    crc64we: String,
    crc64xz: String,
    crc82darc: String,
    md5: String,
}

impl HochTable {
    pub fn new(name: String, data: &[u8], hexonly: bool) -> Result<HochTable, Error> {
        let mut sha3 = Sha3_224::new();
        sha3.update(data);
        let sha3_224 = hex::encode(sha3.finalize());

        let mut sha3 = Sha3_256::new();
        sha3.update(data);
        let sha3_256 = hex::encode(sha3.finalize());

        let mut sha3 = Sha3_384::new();
        sha3.update(data);
        let sha3_384 = hex::encode(sha3.finalize());

        let mut sha3 = Sha3_512::new();
        sha3.update(data);
        let sha3_512 = hex::encode(sha3.finalize());

        let mut sha3 = Keccak224::new();
        sha3.update(data);
        let keccak224 = hex::encode(sha3.finalize());

        let mut sha3 = Keccak256::new();
        sha3.update(data);
        let keccak256 = hex::encode(sha3.finalize());

        let mut sha3 = Keccak256Full::new();
        sha3.update(data);
        let keccak256full = hex::encode(sha3.finalize());

        let mut sha2 = Sha512_224::new();
        sha2.update(data);
        let sha512_224 = hex::encode(sha2.finalize());

        let mut sha2 = Sha512_256::new();
        sha2.update(data);
        let sha512_256 = hex::encode(sha2.finalize());

        let mut sha2 = Sha224::new();
        sha2.update(data);
        let sha224 = hex::encode(sha2.finalize());

        let mut sha2 = Sha256::new();
        sha2.update(data);
        let sha256 = hex::encode(sha2.finalize());

        let mut sha2 = Sha384::new();
        sha2.update(data);
        let sha384 = hex::encode(sha2.finalize());

        let mut sha2 = Sha512::new();
        sha2.update(data);
        let sha512 = hex::encode(sha2.finalize());

        let md5 = format!("{:064x}", md5_compute(data));

        let adler32chk = format!("{:08x}", adler32(data)?);

        // CRC
        let crc = Crc::<u32>::new(&CRC_32_BZIP2);
        let crc32bzip2 = format!("{:08x}", crc.checksum(data));

        let crc = Crc::<u32>::new(&CRC_32_JAMCRC);
        let crc32jamcrc = format!("{:08x}", crc.checksum(data));

        let crc = Crc::<u32>::new(&CRC_32_ISCSI);
        let crc32iscsi = format!("{:08x}", crc.checksum(data));

        let crc = Crc::<u32>::new(&CRC_32_MPEG_2);
        let crc32mpeg2 = format!("{:08x}", crc.checksum(data));

        let crc = Crc::<u16>::new(&CRC_11_FLEXRAY);
        let crc11flexray = format!("{:04x}", crc.checksum(data));

        let crc = Crc::<u8>::new(&CRC_3_GSM);
        let crc3gsm = format!("{:02x}", crc.checksum(data));

        let crc = Crc::<u8>::new(&CRC_4_G_704);
        let crc4g704 = format!("{:02x}", crc.checksum(data));

        let crc = Crc::<u8>::new(&CRC_6_GSM);
        let crc6gsm = format!("{:02x}", crc.checksum(data));

        let crc = Crc::<u8>::new(&CRC_6_CDMA2000_A);
        let crc6cdma2000a = format!("{:02x}", crc.checksum(data));

        let crc = Crc::<u8>::new(&CRC_6_CDMA2000_B);
        let crc6cdma2000b = format!("{:02x}", crc.checksum(data));

        let crc = Crc::<u8>::new(&CRC_8_LTE);
        let crc8lte = format!("{:02x}", crc.checksum(data));

        let crc = Crc::<u8>::new(&CRC_5_G_704);
        let crc5g704 = format!("{:02x}", crc.checksum(data));

        let crc = Crc::<u16>::new(&CRC_16_TELEDISK);
        let crc16teledisk = format!("{:04x}", crc.checksum(data));

        let crc = Crc::<u16>::new(&CRC_16_OPENSAFETY_A);
        let crc16opensafetya = format!("{:04x}", crc.checksum(data));

        let crc = Crc::<u16>::new(&CRC_16_OPENSAFETY_B);
        let crc16opensafetyb = format!("{:04x}", crc.checksum(data));

        let crc = Crc::<u16>::new(&CRC_16_PROFIBUS);
        let crc16profibus = format!("{:04x}", crc.checksum(data));

        let crc = Crc::<u16>::new(&CRC_16_USB);
        let crc16usb = format!("{:04x}", crc.checksum(data));

        let crc = Crc::<u32>::new(&CRC_32_ISO_HDLC);
        let crc32isohdlc = format!("{:08x}", crc.checksum(data));

        let crc = Crc::<u64>::new(&CRC_40_GSM);
        let crc40gsm = format!("{:016x}", crc.checksum(data));

        let crc = Crc::<u64>::new(&CRC_64_MS);
        let crc64ms = format!("{:08x}", crc.checksum(data));

        let crc = Crc::<u64>::new(&CRC_64_WE);
        let crc64we = format!("{:08x}", crc.checksum(data));

        let crc = Crc::<u16>::new(&CRC_16_XMODEM);
        let crc16xmodem = format!("{:08x}", crc.checksum(data));

        let crc = Crc::<u32>::new(&CRC_24_BLE);
        let crc24ble = format!("{:08x}", crc.checksum(data));

        let crc = Crc::<u32>::new(&CRC_24_OPENPGP);
        let crc24openpgp = format!("{:08x}", crc.checksum(data));

        let crc = Crc::<u64>::new(&CRC_64_REDIS);
        let crc64redis = format!("{:016x}", crc.checksum(data));

        let crc = Crc::<u64>::new(&CRC_64_XZ);
        let crc64xz = format!("{:016x}", crc.checksum(data));

        let crc = Crc::<u64>::new(&CRC_64_GO_ISO);
        let crc64goiso = format!("{:016x}", crc.checksum(data));

        let crc = Crc::<u64>::new(&CRC_64_ECMA_182);
        let crc64ecma182 = format!("{:016x}", crc.checksum(data));

        let crc = Crc::<u128>::new(&CRC_82_DARC);
        let crc82darc = format!("{:032x}", crc.checksum(data));

        let sha1 = Sha1::default().digest(data).to_hex();
        Ok(HochTable {
            meta: if hexonly { None } else { Some(FSNode::new(name.into())) },
            md5: md5,
            sha1: sha1,
            adler32: adler32chk,
            crc3gsm: crc3gsm,
            crc4g704: crc4g704,
            crc5g704: crc5g704,
            crc6gsm: crc6gsm,
            crc6cdma2000a: crc6cdma2000a,
            crc6cdma2000b: crc6cdma2000b,
            crc8lte: crc8lte,
            crc11flexray: crc11flexray,
            crc16teledisk: crc16teledisk,
            crc16opensafetya: crc16opensafetya,
            crc16opensafetyb: crc16opensafetyb,
            crc16profibus: crc16profibus,
            crc16usb: crc16usb,
            crc16xmodem: crc16xmodem,
            crc24ble: crc24ble,
            crc24openpgp: crc24openpgp,
            crc32bzip2: crc32bzip2,
            crc32iscsi: crc32iscsi,
            crc32jamcrc: crc32jamcrc,
            crc32isohdlc: crc32isohdlc,
            crc32mpeg2: crc32mpeg2,
            crc40gsm: crc40gsm,
            crc64xz: crc64xz,
            crc64redis: crc64redis,
            crc64goiso: crc64goiso,
            crc64ms: crc64ms,
            crc64we: crc64we,
            crc64ecma182: crc64ecma182,
            crc82darc: crc82darc,
            sha512_224: sha512_224,
            sha512_256: sha512_256,
            sha224: sha224,
            sha256: sha256,
            sha384: sha384,
            sha512: sha512,
            sha3_224: sha3_224,
            sha3_256: sha3_256,
            sha3_384: sha3_384,
            sha3_512: sha3_512,
            keccak224: keccak224,
            keccak256: keccak256,
            keccak256full: keccak256full,
        })
    }
}
