// $$'""""'$$$\                  $$\\                       $$$  $$\  $$\\\
// $' .$$$. `$\                  $$\\                        $$\ $$\  $$\\\
// $  $$$$$$$$\.$$$$$$. $$$$$$$. $$$$$$$. .$$$$$$. $$$$$$$.  $$\ $$$$$$$\\\
// $  $$$$$$$$\$$'  `$$ $$'\\`$$ $$'  `$$ $$'  `$$ $$'  `$$  $$\\ \\\\$$\\\
// $. `$$$' .$\$$.  .$$ $$\      $$.  .$$ $$.  .$$ $$\   $$  $$\\     $$\\\
// $$.     .$$\`$$$$$$$ $$\      $$$$$$$'\`$$$$$$' $$\   $$ $$$$\     $$\\\
// $$$$$$$$$$$\ \\\\\\\ \\\\     \\\\\\\\\ \\\\\\\ \\\\  \\ \\\\\     \\\\\
// \\\\\\\\\\\\\ \\\\\\\ \\\\     \\\\\\\\\ \\\\\\\ \\\\  \\ \\\\\     \\\\
// https://en.wikipedia.org/wiki/Radiocarbon_dating

use diff::Diff;
use iocore::Path;
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


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Diff)]
pub struct HochTable {
    #[serde(skip_serializing_if = "Option::is_none")]
    filename: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<String>,
    sha1: Option<String>,
    sha512: Option<String>,
    sha224: Option<String>,
    sha384: Option<String>,
    sha256: Option<String>,
    sha512_224: Option<String>,
    sha512_256: Option<String>,
    keccak256_full: Option<String>,
    keccak256: Option<String>,
    keccak224: Option<String>,
    sha3_224: Option<String>,
    sha3_256: Option<String>,
    sha3_384: Option<String>,
    sha3_512: Option<String>,
    adler32: Option<String>,
    crc3_gsm: Option<String>,
    crc4_g704: Option<String>,
    crc5_g704: Option<String>,
    crc6_gsm: Option<String>,
    crc6_cdma_2000a: Option<String>,
    crc6_cdma_2000b: Option<String>,
    // crc5_g_704: Option<String>,
    // crc5_usb: Option<String>,
    crc8_lte: Option<String>,
    // Crc8_WCDMA: Option<String>,
    // Crc8_SMBUS: Option<String>,
    // Crc8_OPENSAFETY: Option<String>,
    // Crc8_HITAG: Option<String>,
    // Crc1_0_ATM: Option<String>,
    // Crc1_0_CDMA2000: Option<String>,
    // Crc1_0_GSM: Option<String>,
    crc11_flexray: Option<String>,
    // Crc1_1_UMTS: Option<String>,
    // Crc1_6_GSM: Option<String>,
    // Crc1_6_ISO_IEC_14443_3_A: Option<String>,
    // Crc1_6_MAXIM_DOW: Option<String>,
    // Crc1_6_MCRF4XX: Option<String>,
    // Crc1_6_MODBUS: Option<String>,
    // Crc1_6_NRSC_5: Option<String>,
    crc16_opensafety_a: Option<String>,
    crc16_opensafety_b: Option<String>,
    crc16_profibus: Option<String>,
    crc16_teledisk: Option<String>,
    crc16_usb: Option<String>,
    crc16_xmodem: Option<String>,
    crc24_ble: Option<String>,
    crc24_openpgp: Option<String>,
    // Crc3_2_AIXM: Option<String>,
    // Crc3_2_AUTOSAR: Option<String>,
    // Crc3_2_BASE91_D: Option<String>,
    crc32_bzip2: Option<String>,
    crc32_iscsi: Option<String>,
    crc32_isohdlc: Option<String>,
    crc32_jamcrc: Option<String>,
    crc32_mpeg2: Option<String>,
    crc40_gsm: Option<String>,
    crc64_ecma182: Option<String>,
    crc64_goiso: Option<String>,
    crc64_ms: Option<String>,
    crc64_redis: Option<String>,
    crc64_we: Option<String>,
    crc64_xz: Option<String>,
    crc82_darc: Option<String>,
    md5: Option<String>,
}

impl HochTable {
    pub fn new(meta: Option<String>) -> HochTable {
        let filename = meta.clone().filter(|s| Path::raw(s).is_file()).map(|f| Path::raw(f).relative_to_cwd().to_string());
        let data = meta.xor(filename.clone());
        return HochTable {
            filename: filename,
            data: data,
            md5: None,
            sha1: None,
            adler32: None,
            crc3_gsm: None,
            crc4_g704: None,
            crc5_g704: None,
            crc6_gsm: None,
            crc6_cdma_2000a: None,
            crc6_cdma_2000b: None,
            crc8_lte: None,
            crc11_flexray: None,
            crc16_teledisk: None,
            crc16_opensafety_a: None,
            crc16_opensafety_b: None,
            crc16_profibus: None,
            crc16_usb: None,
            crc16_xmodem: None,
            crc24_ble: None,
            crc24_openpgp: None,
            crc32_bzip2: None,
            crc32_iscsi: None,
            crc32_jamcrc: None,
            crc32_isohdlc: None,
            crc32_mpeg2: None,
            crc40_gsm: None,
            crc64_xz: None,
            crc64_redis: None,
            crc64_goiso: None,
            crc64_ms: None,
            crc64_we: None,
            crc64_ecma182: None,
            crc82_darc: None,
            sha512_224: None,
            sha512_256: None,
            sha224: None,
            sha256: None,
            sha384: None,
            sha512: None,
            sha3_224: None,
            sha3_256: None,
            sha3_384: None,
            sha3_512: None,
            keccak224: None,
            keccak256: None,
            keccak256_full: None,
        }
    }
    pub fn cs(&mut self, data: Vec<u8>) -> HochTable {
        let data = data.to_vec();
        let mut sha3 = Sha3_224::new();
        sha3.update(data.clone().as_slice());
        let sha3_224 = hex::encode(sha3.finalize());

        let mut sha3_256 = Sha3_256::new();
        sha3_256.update(data.clone().as_slice());
        let sha3_256 = hex::encode(sha3_256.finalize());

        let mut sha3_384 = Sha3_384::new();
        sha3_384.update(data.clone().as_slice());
        let sha3_384 = hex::encode(sha3_384.finalize());

        let mut sha3_512 = Sha3_512::new();
        sha3_512.update(data.clone().as_slice());
        let sha3_512 = hex::encode(sha3_512.finalize());

        let mut keccak224 = Keccak224::new();
        keccak224.update(data.clone().as_slice());
        let keccak224 = hex::encode(keccak224.finalize());

        let mut keccak256 = Keccak256::new();
        keccak256.update(data.clone().as_slice());
        let keccak256 = hex::encode(keccak256.finalize());

        let mut keccak256_full = Keccak256Full::new();
        keccak256_full.update(data.clone().as_slice());
        let keccak256_full = hex::encode(keccak256_full.finalize());

        let mut sha2 = Sha512_224::new();
        sha2.update(data.clone().as_slice());
        let sha512_224 = hex::encode(sha2.finalize());

        let mut sha2 = Sha512_256::new();
        sha2.update(data.clone().as_slice());
        let sha512_256 = hex::encode(sha2.finalize());

        let mut sha2 = Sha224::new();
        sha2.update(data.clone().as_slice());
        let sha224 = hex::encode(sha2.finalize());

        let mut sha2 = Sha256::new();
        sha2.update(data.clone().as_slice());
        let sha256 = hex::encode(sha2.finalize());

        let mut sha2 = Sha384::new();
        sha2.update(data.clone().as_slice());
        let sha384 = hex::encode(sha2.finalize());

        let mut sha2 = Sha512::new();
        sha2.update(data.clone().as_slice());
        let sha512 = hex::encode(sha2.finalize());

        let md5 = format!("{:064x}", md5_compute(data.clone().as_slice()));

        let adler32 = adler32(data.clone().as_slice()).map(|n| format!("{:08x}", n)).unwrap_or_else(|e| format!("adler32 error: {}", e));

        // CRC
        let crc = Crc::<u32>::new(&CRC_32_BZIP2);
        let crc32_bzip2 = format!("{:08x}", crc.checksum(data.clone().as_slice()));

        let crc = Crc::<u32>::new(&CRC_32_JAMCRC);
        let crc32_jamcrc = format!("{:08x}", crc.checksum(data.clone().as_slice()));

        let crc = Crc::<u32>::new(&CRC_32_ISCSI);
        let crc32_iscsi = format!("{:08x}", crc.checksum(data.clone().as_slice()));

        let crc = Crc::<u32>::new(&CRC_32_MPEG_2);
        let crc32_mpeg2 = format!("{:08x}", crc.checksum(data.clone().as_slice()));

        let crc = Crc::<u16>::new(&CRC_11_FLEXRAY);
        let crc11_flexray = format!("{:04x}", crc.checksum(data.clone().as_slice()));

        let crc = Crc::<u8>::new(&CRC_3_GSM);
        let crc3_gsm = format!("{:02x}", crc.checksum(data.clone().as_slice()));

        let crc = Crc::<u8>::new(&CRC_4_G_704);
        let crc4_g704 = format!("{:02x}", crc.checksum(data.clone().as_slice()));

        let crc = Crc::<u8>::new(&CRC_6_GSM);
        let crc6_gsm = format!("{:02x}", crc.checksum(data.clone().as_slice()));

        let crc = Crc::<u8>::new(&CRC_6_CDMA2000_A);
        let crc6_cdma_2000a = format!("{:02x}", crc.checksum(data.clone().as_slice()));

        let crc = Crc::<u8>::new(&CRC_6_CDMA2000_B);
        let crc6_cdma_2000b = format!("{:02x}", crc.checksum(data.clone().as_slice()));

        let crc = Crc::<u8>::new(&CRC_8_LTE);
        let crc8_lte = format!("{:02x}", crc.checksum(data.clone().as_slice()));

        let crc = Crc::<u8>::new(&CRC_5_G_704);
        let crc5_g704 = format!("{:02x}", crc.checksum(data.clone().as_slice()));

        let crc = Crc::<u16>::new(&CRC_16_TELEDISK);
        let crc16_teledisk = format!("{:04x}", crc.checksum(data.clone().as_slice()));

        let crc = Crc::<u16>::new(&CRC_16_OPENSAFETY_A);
        let crc16_opensafety_a = format!("{:04x}", crc.checksum(data.clone().as_slice()));

        let crc = Crc::<u16>::new(&CRC_16_OPENSAFETY_B);
        let crc16_opensafety_b = format!("{:04x}", crc.checksum(data.clone().as_slice()));

        let crc = Crc::<u16>::new(&CRC_16_PROFIBUS);
        let crc16_profibus = format!("{:04x}", crc.checksum(data.clone().as_slice()));

        let crc = Crc::<u16>::new(&CRC_16_USB);
        let crc16_usb = format!("{:04x}", crc.checksum(data.clone().as_slice()));

        let crc = Crc::<u32>::new(&CRC_32_ISO_HDLC);
        let crc32_isohdlc = format!("{:08x}", crc.checksum(data.clone().as_slice()));

        let crc = Crc::<u64>::new(&CRC_40_GSM);
        let crc40_gsm = format!("{:016x}", crc.checksum(data.clone().as_slice()));

        let crc = Crc::<u64>::new(&CRC_64_MS);
        let crc64_ms = format!("{:08x}", crc.checksum(data.clone().as_slice()));

        let crc = Crc::<u64>::new(&CRC_64_WE);
        let crc64_we = format!("{:08x}", crc.checksum(data.clone().as_slice()));

        let crc = Crc::<u16>::new(&CRC_16_XMODEM);
        let crc16_xmodem = format!("{:08x}", crc.checksum(data.clone().as_slice()));

        let crc = Crc::<u32>::new(&CRC_24_BLE);
        let crc24_ble = format!("{:08x}", crc.checksum(data.clone().as_slice()));

        let crc = Crc::<u32>::new(&CRC_24_OPENPGP);
        let crc24_openpgp = format!("{:08x}", crc.checksum(data.clone().as_slice()));

        let crc = Crc::<u64>::new(&CRC_64_REDIS);
        let crc64_redis = format!("{:016x}", crc.checksum(data.clone().as_slice()));

        let crc = Crc::<u64>::new(&CRC_64_XZ);
        let crc64_xz = format!("{:016x}", crc.checksum(data.clone().as_slice()));

        let crc = Crc::<u64>::new(&CRC_64_GO_ISO);
        let crc64_goiso = format!("{:016x}", crc.checksum(data.clone().as_slice()));

        let crc = Crc::<u64>::new(&CRC_64_ECMA_182);
        let crc64_ecma182 = format!("{:016x}", crc.checksum(data.clone().as_slice()));

        let crc = Crc::<u128>::new(&CRC_82_DARC);
        let crc82_darc = format!("{:032x}", crc.checksum(data.clone().as_slice()));

        let sha1 = Sha1::default().digest(data.clone().as_slice()).to_hex();

        self.md5 = Some(md5);
        self.sha1 = Some(sha1);
        self.adler32 = Some(adler32);
        self.crc3_gsm = Some(crc3_gsm);
        self.crc4_g704 = Some(crc4_g704);
        self.crc5_g704 = Some(crc5_g704);
        self.crc6_gsm = Some(crc6_gsm);
        self.crc6_cdma_2000a = Some(crc6_cdma_2000a);
        self.crc6_cdma_2000b = Some(crc6_cdma_2000b);
        self.crc8_lte = Some(crc8_lte);
        self.crc11_flexray = Some(crc11_flexray);
        self.crc16_teledisk = Some(crc16_teledisk);
        self.crc16_opensafety_a = Some(crc16_opensafety_a);
        self.crc16_opensafety_b = Some(crc16_opensafety_b);
        self.crc16_profibus = Some(crc16_profibus);
        self.crc16_usb = Some(crc16_usb);
        self.crc16_xmodem = Some(crc16_xmodem);
        self.crc24_ble = Some(crc24_ble);
        self.crc24_openpgp = Some(crc24_openpgp);
        self.crc32_bzip2 = Some(crc32_bzip2);
        self.crc32_iscsi = Some(crc32_iscsi);
        self.crc32_jamcrc = Some(crc32_jamcrc);
        self.crc32_isohdlc = Some(crc32_isohdlc);
        self.crc32_mpeg2 = Some(crc32_mpeg2);
        self.crc40_gsm = Some(crc40_gsm);
        self.crc64_xz = Some(crc64_xz);
        self.crc64_redis = Some(crc64_redis);
        self.crc64_goiso = Some(crc64_goiso);
        self.crc64_ms = Some(crc64_ms);
        self.crc64_we = Some(crc64_we);
        self.crc64_ecma182 = Some(crc64_ecma182);
        self.crc82_darc = Some(crc82_darc);
        self.sha512_224 = Some(sha512_224);
        self.sha512_256 = Some(sha512_256);
        self.sha224 = Some(sha224);
        self.sha256 = Some(sha256);
        self.sha384 = Some(sha384);
        self.sha512 = Some(sha512);
        self.sha3_224 = Some(sha3_224);
        self.sha3_256 = Some(sha3_256);
        self.sha3_384 = Some(sha3_384);
        self.sha3_512 = Some(sha3_512);
        self.keccak224 = Some(keccak224);
        self.keccak256 = Some(keccak256);
        self.keccak256_full = Some(keccak256_full);
        return self.clone()
    }
}
