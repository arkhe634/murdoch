//! # Murdoch-Charsets
//! `murdoch-charsets` provides API for name of character sets defined by Internet Assigned Numbers Authority (IANA).
//!
//! In all settings, `murdoch-charset` provides `CharacterSet` traits and `InvalidCharacterSetNameError`.
//!
//! ## Default Setting
//!
//! By default, `murdoch-charsets` provides types corresponding character sets defined by [IANA],
//! and `CharacterSetEnum` to hold any of them.
//!
//! [IANA]: http://www.iana.org/assignments/character-sets/character-sets.xhtml
//!
//! ## Custom Character-Sets
//!
//! You can customize the character sets by using `no_charset` feature
//!
//! ```cargo
//! [dependencies]
//! murdoch-charsets = { features = [ "no_charset" ] }
//! ```
//!
//! then `murdoch-charsets` exports `character_sets!` macro.
//!
//! ```fragment
//! character_sets!(
//!     UnicodeCharacterSet,
//!     UTF_8,                           // character-set type/variant name
//!     None,                            // preferred_mime_name
//!     "UTF-8",                         // name
//!     ["UTF8", "UTF_8",],              // aliases
//!     ["UTF-8", "UTF8", "UTF_8",],     // uppercase set of names (for FromStr)
//!     UTF_16,
//!     None,
//!     "UTF-16",
//!     ["UTF16", "UTF_16",],
//!     ["UTF-16", "UTF16", "UTF_16",],
//!     UTF_32,
//!     None,
//!     "UTF-32",
//!     ["UTF32", "UTF_32",],
//!     ["UTF-32", "UTF32", "UTF_32",],
//! );
//! ```
//!
//! By calling the macro as above, the following types and implementations of traits (`Debug`,
//! `Display`, `Copy`, `Clone`, `PartialEq`, `Eq`, `FromStr`, `AsRef<str>`, `AsRef<[u8]>`) are generated.
//!
//! ```fragment
//! pub struct UTF_8;
//! pub struct UTF_16;
//! pub struct UTF_32;
//! pub enum UnicodeCharacterSet
//! {
//!     UTF_8,
//!     UTF_16,
//!     UTF_32,
//! }
//! ```
extern crate self as murdoch_charsets;

#[cfg(feature = "no_charset")]
#[macro_export]
macro_rules! character_sets {
	($character_set_enum:ident, $($ident:ident, $preferred_mime_name:expr, $name:expr, [$($aliases:expr,)*], [$($upper:expr,)*],)*) => {
		$(
			#[allow(non_camel_case_types)]
			#[derive(Debug, Copy, Clone, Eq)]
			pub struct $ident;

			impl murdoch_charsets::CharacterSet for $ident
			{
				fn preferred_mime_name(&self) -> std::option::Option<&'static str>
				{
					$preferred_mime_name
				}

				fn name(&self) -> &'static str
				{
					$name
				}

				fn aliases(&self) -> &'static[&'static str]
				{
					&[$($aliases),*]
				}
			}

			impl std::fmt::Display for $ident
			{
				fn fmt(&self, f:&mut std::fmt::Formatter) -> std::fmt::Result
				{
					write!(f, "{}", murdoch_charsets::CharacterSet::name(self))
				}
			}

			impl<T:murdoch_charsets::CharacterSet> std::cmp::PartialEq<T> for $ident
			{
				fn eq(&self, other:&T) -> bool
				{
					murdoch_charsets::CharacterSet::name(self) == murdoch_charsets::CharacterSet::name(other)
				}
			}

			impl std::str::FromStr for $ident
			{
				type Err = murdoch_charsets::InvalidCharacterSetNameError;

				fn from_str(s: &str) -> std::result::Result<Self, Self::Err>
				{
					match s.to_ascii_uppercase().as_ref()
					{
						$($upper => std::result::Result::Ok(Self),)*
						_ => std::result::Result::Err(murdoch_charsets::InvalidCharacterSetNameError::new(&[$($upper,)*], s))
					}
				}
			}

			impl std::convert::AsRef<str> for $ident
			{
				fn as_ref(&self) -> &str
				{
					murdoch_charsets::CharacterSet::name(self)
				}
			}

			impl std::convert::AsRef<[u8]> for $ident
			{
				fn as_ref(&self) -> &[u8]
				{
					std::convert::AsRef::<[u8]>::as_ref(murdoch_charsets::CharacterSet::name(self))
				}
			}
		)*

		#[derive(Debug, Copy, Clone, Eq)]
		pub enum $character_set_enum
		{
			$(
				#[allow(non_camel_case_types)]
				$ident,
			)*
		}

		impl murdoch_charsets::CharacterSet for $character_set_enum
		{
			fn preferred_mime_name(&self) -> std::option::Option<&'static str>
			{
				match self
				{
					$($character_set_enum::$ident => murdoch_charsets::CharacterSet::preferred_mime_name(&$ident),)*
				}
			}

			fn name(&self) -> &'static str
			{
				match self
				{
					$($character_set_enum::$ident => murdoch_charsets::CharacterSet::name(&$ident),)*
				}
			}

			fn aliases(&self) -> &'static[&'static str]
			{
				match self
				{
					$($character_set_enum::$ident => murdoch_charsets::CharacterSet::aliases(&$ident),)*
				}
			}
		}

		impl std::fmt::Display for $character_set_enum
		{
			fn fmt(&self, f:&mut std::fmt::Formatter) -> std::fmt::Result
			{
				match self
				{
					$($character_set_enum::$ident => std::fmt::Display::fmt(&$ident, f),)*
				}
			}
		}

		impl<T:murdoch_charsets::CharacterSet> std::cmp::PartialEq<T> for $character_set_enum
		{
			fn eq(&self, other:&T) -> bool
			{
				murdoch_charsets::CharacterSet::name(self) == murdoch_charsets::CharacterSet::name(other)
			}
		}

		impl std::str::FromStr for $character_set_enum
		{
			type Err = murdoch_charsets::InvalidCharacterSetNameError;

			fn from_str(s: &str) -> std::result::Result<Self, Self::Err>
			{
				match s.to_ascii_uppercase().as_ref()
				{
					$($($upper => std::result::Result::Ok($character_set_enum::$ident),)*)*
					_ => std::result::Result::Err(murdoch_charsets::InvalidCharacterSetNameError::new(&[$($($upper,)*)*], s))
				}
			}
		}

		impl std::convert::AsRef<str> for $character_set_enum
		{
			fn as_ref(&self) -> &str
			{
				murdoch_charsets::CharacterSet::name(self)
			}
		}

		impl std::convert::AsRef<[u8]> for $character_set_enum
		{
			fn as_ref(&self) -> &[u8]
			{
				std::convert::AsRef::<[u8]>::as_ref(murdoch_charsets::CharacterSet::name(self))
			}
		}
	};
}

#[cfg(not(feature = "no_charset"))]
macro_rules! character_sets {
	($character_set_enum:ident, $($ident:ident, $preferred_mime_name:expr, $name:expr, [$($aliases:expr,)*], [$($upper:expr,)*],)*) => {
		$(
			#[allow(non_camel_case_types)]
			#[derive(Debug, Copy, Clone, Eq)]
			pub struct $ident;

			impl murdoch_charsets::CharacterSet for $ident
			{
				fn preferred_mime_name(&self) -> std::option::Option<&'static str>
				{
					$preferred_mime_name
				}

				fn name(&self) -> &'static str
				{
					$name
				}

				fn aliases(&self) -> &'static[&'static str]
				{
					&[$($aliases),*]
				}
			}

			impl std::fmt::Display for $ident
			{
				fn fmt(&self, f:&mut std::fmt::Formatter) -> std::fmt::Result
				{
					write!(f, "{}", murdoch_charsets::CharacterSet::name(self))
				}
			}

			impl<T:murdoch_charsets::CharacterSet> std::cmp::PartialEq<T> for $ident
			{
				fn eq(&self, other:&T) -> bool
				{
					murdoch_charsets::CharacterSet::name(self) == murdoch_charsets::CharacterSet::name(other)
				}
			}

			impl std::str::FromStr for $ident
			{
				type Err = murdoch_charsets::InvalidCharacterSetNameError;

				fn from_str(s: &str) -> std::result::Result<Self, Self::Err>
				{
					match s.to_ascii_uppercase().as_ref()
					{
						$($upper => std::result::Result::Ok(Self),)*
						_ => std::result::Result::Err(murdoch_charsets::InvalidCharacterSetNameError::new(&[$($upper,)*], s))
					}
				}
			}

			impl std::convert::AsRef<str> for $ident
			{
				fn as_ref(&self) -> &str
				{
					murdoch_charsets::CharacterSet::name(self)
				}
			}

			impl std::convert::AsRef<[u8]> for $ident
			{
				fn as_ref(&self) -> &[u8]
				{
					std::convert::AsRef::<[u8]>::as_ref(murdoch_charsets::CharacterSet::name(self))
				}
			}
		)*

		#[derive(Debug, Copy, Clone, Eq)]
		pub enum $character_set_enum
		{
			$(
				#[allow(non_camel_case_types)]
				$ident,
			)*
		}

		impl murdoch_charsets::CharacterSet for $character_set_enum
		{
			fn preferred_mime_name(&self) -> std::option::Option<&'static str>
			{
				match self
				{
					$($character_set_enum::$ident => murdoch_charsets::CharacterSet::preferred_mime_name(&$ident),)*
				}
			}

			fn name(&self) -> &'static str
			{
				match self
				{
					$($character_set_enum::$ident => murdoch_charsets::CharacterSet::name(&$ident),)*
				}
			}

			fn aliases(&self) -> &'static[&'static str]
			{
				match self
				{
					$($character_set_enum::$ident => murdoch_charsets::CharacterSet::aliases(&$ident),)*
				}
			}
		}

		impl std::fmt::Display for $character_set_enum
		{
			fn fmt(&self, f:&mut std::fmt::Formatter) -> std::fmt::Result
			{
				match self
				{
					$($character_set_enum::$ident => std::fmt::Display::fmt(&$ident, f),)*
				}
			}
		}

		impl<T:murdoch_charsets::CharacterSet> std::cmp::PartialEq<T> for $character_set_enum
		{
			fn eq(&self, other:&T) -> bool
			{
				murdoch_charsets::CharacterSet::name(self) == murdoch_charsets::CharacterSet::name(other)
			}
		}

		impl std::str::FromStr for $character_set_enum
		{
			type Err = murdoch_charsets::InvalidCharacterSetNameError;

			fn from_str(s: &str) -> std::result::Result<Self, Self::Err>
			{
				match s.to_ascii_uppercase().as_ref()
				{
					$($($upper => std::result::Result::Ok($character_set_enum::$ident),)*)*
					_ => std::result::Result::Err(murdoch_charsets::InvalidCharacterSetNameError::new(&[$($($upper,)*)*], s))
				}
			}
		}

		impl std::convert::AsRef<str> for $character_set_enum
		{
			fn as_ref(&self) -> &str
			{
				murdoch_charsets::CharacterSet::name(self)
			}
		}

		impl std::convert::AsRef<[u8]> for $character_set_enum
		{
			fn as_ref(&self) -> &[u8]
			{
				std::convert::AsRef::<[u8]>::as_ref(murdoch_charsets::CharacterSet::name(self))
			}
		}
	};
}

pub struct InvalidCharacterSetNameError
{
	required: &'static [&'static str],
	found: std::string::String,
}

impl InvalidCharacterSetNameError
{
	pub fn new(required: &'static [&'static str], found: &str) -> Self
	{
		Self {
			required,
			found: found.to_owned(),
		}
	}
}

impl std::fmt::Display for InvalidCharacterSetNameError
{
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
	{
		write!(
			f,
			"\"required {:?} but {} found.",
			self.required, self.found
		)
	}
}

pub trait CharacterSet
{
	fn preferred_mime_name(&self) -> Option<&'static str>;

	fn name(&self) -> &'static str;

	fn aliases(&self) -> &'static [&'static str];
}

#[cfg(not(feature = "no_charset"))]
character_sets!(
	CharacterSetEnum,
	US_ASCII,
	std::option::Option::Some("US-ASCII"),
	"US-ASCII",
	[
		"iso-ir-6",
		"ANSI_X3.4-1968",
		"ANSI_X3.4-1986",
		"ISO_646.irv:1991",
		"ISO646-US",
		"US-ASCII",
		"us",
		"IBM367",
		"cp367",
		"csASCII",
	],
	[
		"ANSI_X3.4-1986",
		"ISO_646.IRV:1991",
		"US-ASCII",
		"IBM367",
		"CSASCII",
		"ISO-IR-6",
		"US",
		"ISO646-US",
		"ANSI_X3.4-1968",
		"CP367",
	],
	ISO_8859_1_1987,
	std::option::Option::Some("ISO-8859-1"),
	"ISO_8859-1:1987",
	[
		"iso-ir-100",
		"ISO_8859-1",
		"ISO-8859-1",
		"latin1",
		"l1",
		"IBM819",
		"CP819",
		"csISOLatin1",
	],
	[
		"L1",
		"ISO_8859-1:1987",
		"ISO-IR-100",
		"ISO-8859-1",
		"ISO_8859-1",
		"LATIN1",
		"CP819",
		"IBM819",
		"CSISOLATIN1",
	],
	ISO_8859_2_1987,
	std::option::Option::Some("ISO-8859-2"),
	"ISO_8859-2:1987",
	[
		"iso-ir-101",
		"ISO_8859-2",
		"ISO-8859-2",
		"latin2",
		"l2",
		"csISOLatin2",
	],
	[
		"ISO-8859-2",
		"ISO_8859-2:1987",
		"ISO-IR-101",
		"CSISOLATIN2",
		"LATIN2",
		"L2",
		"ISO_8859-2",
	],
	ISO_8859_3_1988,
	std::option::Option::Some("ISO-8859-3"),
	"ISO_8859-3:1988",
	[
		"iso-ir-109",
		"ISO_8859-3",
		"ISO-8859-3",
		"latin3",
		"l3",
		"csISOLatin3",
	],
	[
		"ISO_8859-3:1988",
		"ISO-IR-109",
		"LATIN3",
		"ISO_8859-3",
		"ISO-8859-3",
		"CSISOLATIN3",
		"L3",
	],
	ISO_8859_4_1988,
	std::option::Option::Some("ISO-8859-4"),
	"ISO_8859-4:1988",
	[
		"iso-ir-110",
		"ISO_8859-4",
		"ISO-8859-4",
		"latin4",
		"l4",
		"csISOLatin4",
	],
	[
		"ISO-8859-4",
		"ISO_8859-4:1988",
		"ISO_8859-4",
		"LATIN4",
		"ISO-IR-110",
		"L4",
		"CSISOLATIN4",
	],
	ISO_8859_5_1988,
	std::option::Option::Some("ISO-8859-5"),
	"ISO_8859-5:1988",
	[
		"iso-ir-144",
		"ISO_8859-5",
		"ISO-8859-5",
		"cyrillic",
		"csISOLatinCyrillic",
	],
	[
		"CYRILLIC",
		"CSISOLATINCYRILLIC",
		"ISO_8859-5",
		"ISO-IR-144",
		"ISO-8859-5",
		"ISO_8859-5:1988",
	],
	ISO_8859_6_1987,
	std::option::Option::Some("ISO-8859-6"),
	"ISO_8859-6:1987",
	[
		"iso-ir-127",
		"ISO_8859-6",
		"ISO-8859-6",
		"ECMA-114",
		"ASMO-708",
		"arabic",
		"csISOLatinArabic",
	],
	[
		"CSISOLATINARABIC",
		"ISO_8859-6",
		"ECMA-114",
		"ARABIC",
		"ISO-8859-6",
		"ASMO-708",
		"ISO_8859-6:1987",
		"ISO-IR-127",
	],
	ISO_8859_7_1987,
	std::option::Option::Some("ISO-8859-7"),
	"ISO_8859-7:1987",
	[
		"iso-ir-126",
		"ISO_8859-7",
		"ISO-8859-7",
		"ELOT_928",
		"ECMA-118",
		"greek",
		"greek8",
		"csISOLatinGreek",
	],
	[
		"ISO_8859-7",
		"ISO_8859-7:1987",
		"ISO-IR-126",
		"GREEK",
		"ISO-8859-7",
		"ELOT_928",
		"GREEK8",
		"CSISOLATINGREEK",
		"ECMA-118",
	],
	ISO_8859_8_1988,
	std::option::Option::Some("ISO-8859-8"),
	"ISO_8859-8:1988",
	[
		"iso-ir-138",
		"ISO_8859-8",
		"ISO-8859-8",
		"hebrew",
		"csISOLatinHebrew",
	],
	[
		"ISO-8859-8",
		"ISO_8859-8:1988",
		"CSISOLATINHEBREW",
		"HEBREW",
		"ISO-IR-138",
		"ISO_8859-8",
	],
	ISO_8859_9_1989,
	std::option::Option::Some("ISO-8859-9"),
	"ISO_8859-9:1989",
	[
		"iso-ir-148",
		"ISO_8859-9",
		"ISO-8859-9",
		"latin5",
		"l5",
		"csISOLatin5",
	],
	[
		"ISO-8859-9",
		"ISO-IR-148",
		"ISO_8859-9:1989",
		"L5",
		"ISO_8859-9",
		"LATIN5",
		"CSISOLATIN5",
	],
	ISO_8859_10,
	std::option::Option::Some("ISO-8859-10"),
	"ISO-8859-10",
	[
		"iso-ir-157",
		"l6",
		"ISO_8859-10:1992",
		"csISOLatin6",
		"latin6",
	],
	[
		"ISO-8859-10",
		"L6",
		"ISO-IR-157",
		"LATIN6",
		"CSISOLATIN6",
		"ISO_8859-10:1992",
	],
	ISO_6937_2_ADD,
	std::option::Option::None,
	"ISO_6937-2-add",
	["iso-ir-142", "csISOTextComm",],
	["ISO-IR-142", "CSISOTEXTCOMM", "ISO_6937-2-ADD",],
	JIS_X0201,
	std::option::Option::None,
	"JIS_X0201",
	["X0201", "csHalfWidthKatakana",],
	["JIS_X0201", "CSHALFWIDTHKATAKANA", "X0201",],
	JIS_ENCODING,
	std::option::Option::None,
	"JIS_Encoding",
	["csJISEncoding",],
	["CSJISENCODING", "JIS_ENCODING",],
	SHIFT_JIS,
	std::option::Option::Some("Shift_JIS"),
	"Shift_JIS",
	["MS_Kanji", "csShiftJIS",],
	["CSSHIFTJIS", "MS_KANJI", "SHIFT_JIS",],
	EXTENDED_UNIX_CODE_PACKED_FORMAT_FOR_JAPANESE,
	std::option::Option::Some("EUC-JP"),
	"Extended_UNIX_Code_Packed_Format_for_Japanese",
	["csEUCPkdFmtJapanese", "EUC-JP",],
	[
		"EXTENDED_UNIX_CODE_PACKED_FORMAT_FOR_JAPANESE",
		"CSEUCPKDFMTJAPANESE",
		"EUC-JP",
	],
	EXTENDED_UNIX_CODE_FIXED_WIDTH_FOR_JAPANESE,
	std::option::Option::None,
	"Extended_UNIX_Code_Fixed_Width_for_Japanese",
	["csEUCFixWidJapanese",],
	[
		"EXTENDED_UNIX_CODE_FIXED_WIDTH_FOR_JAPANESE",
		"CSEUCFIXWIDJAPANESE",
	],
	BS_4730,
	std::option::Option::None,
	"BS_4730",
	["iso-ir-4", "ISO646-GB", "gb", "uk", "csISO4UnitedKingdom",],
	[
		"UK",
		"BS_4730",
		"ISO-IR-4",
		"ISO646-GB",
		"GB",
		"CSISO4UNITEDKINGDOM",
	],
	SEN_850200_C,
	std::option::Option::None,
	"SEN_850200_C",
	["iso-ir-11", "ISO646-SE2", "se2", "csISO11SwedishForNames",],
	[
		"CSISO11SWEDISHFORNAMES",
		"ISO-IR-11",
		"SEN_850200_C",
		"SE2",
		"ISO646-SE2",
	],
	IT,
	std::option::Option::None,
	"IT",
	["iso-ir-15", "ISO646-IT", "csISO15Italian",],
	["CSISO15ITALIAN", "ISO-IR-15", "ISO646-IT", "IT",],
	ES,
	std::option::Option::None,
	"ES",
	["iso-ir-17", "ISO646-ES", "csISO17Spanish",],
	["ISO-IR-17", "CSISO17SPANISH", "ES", "ISO646-ES",],
	DIN_66003,
	std::option::Option::None,
	"DIN_66003",
	["iso-ir-21", "de", "ISO646-DE", "csISO21German",],
	["ISO646-DE", "CSISO21GERMAN", "DIN_66003", "DE", "ISO-IR-21",],
	NS_4551_1,
	std::option::Option::None,
	"NS_4551-1",
	[
		"iso-ir-60",
		"ISO646-NO",
		"no",
		"csISO60DanishNorwegian",
		"csISO60Norwegian1",
	],
	[
		"ISO-IR-60",
		"CSISO60DANISHNORWEGIAN",
		"CSISO60NORWEGIAN1",
		"NS_4551-1",
		"ISO646-NO",
		"NO",
	],
	NF_Z_62_010,
	std::option::Option::None,
	"NF_Z_62-010",
	["iso-ir-69", "ISO646-FR", "fr", "csISO69French",],
	[
		"ISO646-FR",
		"CSISO69FRENCH",
		"ISO-IR-69",
		"FR",
		"NF_Z_62-010",
	],
	ISO_10646_UTF_1,
	std::option::Option::None,
	"ISO-10646-UTF-1",
	["csISO10646UTF1",],
	["ISO-10646-UTF-1", "CSISO10646UTF1",],
	ISO_646_BASIC_1983,
	std::option::Option::None,
	"ISO_646.basic:1983",
	["ref", "csISO646basic1983",],
	["ISO_646.BASIC:1983", "CSISO646BASIC1983", "REF",],
	INVARIANT,
	std::option::Option::None,
	"INVARIANT",
	["csINVARIANT",],
	["CSINVARIANT", "INVARIANT",],
	ISO_646_IRV_1983,
	std::option::Option::None,
	"ISO_646.irv:1983",
	["iso-ir-2", "irv", "csISO2IntlRefVersion",],
	[
		"ISO_646.IRV:1983",
		"CSISO2INTLREFVERSION",
		"ISO-IR-2",
		"IRV",
	],
	NATS_SEFI,
	std::option::Option::None,
	"NATS-SEFI",
	["iso-ir-8-1", "csNATSSEFI",],
	["NATS-SEFI", "CSNATSSEFI", "ISO-IR-8-1",],
	NATS_SEFI_ADD,
	std::option::Option::None,
	"NATS-SEFI-ADD",
	["iso-ir-8-2", "csNATSSEFIADD",],
	["ISO-IR-8-2", "NATS-SEFI-ADD", "CSNATSSEFIADD",],
	NATS_DANO,
	std::option::Option::None,
	"NATS-DANO",
	["iso-ir-9-1", "csNATSDANO",],
	["ISO-IR-9-1", "NATS-DANO", "CSNATSDANO",],
	NATS_DANO_ADD,
	std::option::Option::None,
	"NATS-DANO-ADD",
	["iso-ir-9-2", "csNATSDANOADD",],
	["NATS-DANO-ADD", "CSNATSDANOADD", "ISO-IR-9-2",],
	SEN_850200_B,
	std::option::Option::None,
	"SEN_850200_B",
	[
		"iso-ir-10",
		"FI",
		"ISO646-FI",
		"ISO646-SE",
		"se",
		"csISO10Swedish",
	],
	[
		"CSISO10SWEDISH",
		"ISO646-SE",
		"ISO646-FI",
		"SE",
		"FI",
		"ISO-IR-10",
		"SEN_850200_B",
	],
	KS_C_5601_1987,
	std::option::Option::None,
	"KS_C_5601-1987",
	[
		"iso-ir-149",
		"KS_C_5601-1989",
		"KSC_5601",
		"korean",
		"csKSC56011987",
	],
	[
		"KS_C_5601-1989",
		"KSC_5601",
		"ISO-IR-149",
		"KOREAN",
		"KS_C_5601-1987",
		"CSKSC56011987",
	],
	ISO_2022_KR,
	std::option::Option::Some("ISO-2022-KR"),
	"ISO-2022-KR",
	["csISO2022KR",],
	["ISO-2022-KR", "CSISO2022KR",],
	EUC_KR,
	std::option::Option::Some("EUC-KR"),
	"EUC-KR",
	["csEUCKR",],
	["CSEUCKR", "EUC-KR",],
	ISO_2022_JP,
	std::option::Option::Some("ISO-2022-JP"),
	"ISO-2022-JP",
	["csISO2022JP",],
	["CSISO2022JP", "ISO-2022-JP",],
	ISO_2022_JP_2,
	std::option::Option::Some("ISO-2022-JP-2"),
	"ISO-2022-JP-2",
	["csISO2022JP2",],
	["CSISO2022JP2", "ISO-2022-JP-2",],
	JIS_C6220_1969_JP,
	std::option::Option::None,
	"JIS_C6220-1969-jp",
	[
		"JIS_C6220-1969",
		"iso-ir-13",
		"katakana",
		"x0201-7",
		"csISO13JISC6220jp",
	],
	[
		"KATAKANA",
		"CSISO13JISC6220JP",
		"X0201-7",
		"JIS_C6220-1969",
		"JIS_C6220-1969-JP",
		"ISO-IR-13",
	],
	JIS_C6220_1969_RO,
	std::option::Option::None,
	"JIS_C6220-1969-ro",
	["iso-ir-14", "jp", "ISO646-JP", "csISO14JISC6220ro",],
	[
		"JP",
		"JIS_C6220-1969-RO",
		"ISO-IR-14",
		"ISO646-JP",
		"CSISO14JISC6220RO",
	],
	PT,
	std::option::Option::None,
	"PT",
	["iso-ir-16", "ISO646-PT", "csISO16Portuguese",],
	["ISO-IR-16", "PT", "CSISO16PORTUGUESE", "ISO646-PT",],
	GREEK7_OLD,
	std::option::Option::None,
	"greek7-old",
	["iso-ir-18", "csISO18Greek7Old",],
	["CSISO18GREEK7OLD", "GREEK7-OLD", "ISO-IR-18",],
	LATIN_GREEK,
	std::option::Option::None,
	"latin-greek",
	["iso-ir-19", "csISO19LatinGreek",],
	["CSISO19LATINGREEK", "ISO-IR-19", "LATIN-GREEK",],
	NF_Z_62_010_1973,
	std::option::Option::None,
	"NF_Z_62-010_(1973)",
	["iso-ir-25", "ISO646-FR1", "csISO25French",],
	[
		"NF_Z_62-010_(1973)",
		"ISO646-FR1",
		"ISO-IR-25",
		"CSISO25FRENCH",
	],
	LATIN_GREEK_1,
	std::option::Option::None,
	"Latin-greek-1",
	["iso-ir-27", "csISO27LatinGreek1",],
	["ISO-IR-27", "CSISO27LATINGREEK1", "LATIN-GREEK-1",],
	ISO_5427,
	std::option::Option::None,
	"ISO_5427",
	["iso-ir-37", "csISO5427Cyrillic",],
	["ISO_5427", "CSISO5427CYRILLIC", "ISO-IR-37",],
	JIS_C6226_1978,
	std::option::Option::None,
	"JIS_C6226-1978",
	["iso-ir-42", "csISO42JISC62261978",],
	["JIS_C6226-1978", "CSISO42JISC62261978", "ISO-IR-42",],
	BS_VIEWDATA,
	std::option::Option::None,
	"BS_viewdata",
	["iso-ir-47", "csISO47BSViewdata",],
	["BS_VIEWDATA", "ISO-IR-47", "CSISO47BSVIEWDATA",],
	INIS,
	std::option::Option::None,
	"INIS",
	["iso-ir-49", "csISO49INIS",],
	["CSISO49INIS", "INIS", "ISO-IR-49",],
	INIS_8,
	std::option::Option::None,
	"INIS-8",
	["iso-ir-50", "csISO50INIS8",],
	["CSISO50INIS8", "INIS-8", "ISO-IR-50",],
	INIS_CYRILLIC,
	std::option::Option::None,
	"INIS-cyrillic",
	["iso-ir-51", "csISO51INISCyrillic",],
	["ISO-IR-51", "INIS-CYRILLIC", "CSISO51INISCYRILLIC",],
	ISO_5427_1981,
	std::option::Option::None,
	"ISO_5427:1981",
	["iso-ir-54", "ISO5427Cyrillic1981", "csISO54271981",],
	[
		"ISO5427CYRILLIC1981",
		"ISO_5427:1981",
		"ISO-IR-54",
		"CSISO54271981",
	],
	ISO_5428_1980,
	std::option::Option::None,
	"ISO_5428:1980",
	["iso-ir-55", "csISO5428Greek",],
	["ISO-IR-55", "ISO_5428:1980", "CSISO5428GREEK",],
	GB_1988_80,
	std::option::Option::None,
	"GB_1988-80",
	["iso-ir-57", "cn", "ISO646-CN", "csISO57GB1988",],
	[
		"ISO-IR-57",
		"CSISO57GB1988",
		"CN",
		"GB_1988-80",
		"ISO646-CN",
	],
	GB_2312_80,
	std::option::Option::None,
	"GB_2312-80",
	["iso-ir-58", "chinese", "csISO58GB231280",],
	["CSISO58GB231280", "CHINESE", "ISO-IR-58", "GB_2312-80",],
	NS_4551_2,
	std::option::Option::None,
	"NS_4551-2",
	["ISO646-NO2", "iso-ir-61", "no2", "csISO61Norwegian2",],
	[
		"NO2",
		"CSISO61NORWEGIAN2",
		"NS_4551-2",
		"ISO-IR-61",
		"ISO646-NO2",
	],
	VIDEOTEX_SUPPL,
	std::option::Option::None,
	"videotex-suppl",
	["iso-ir-70", "csISO70VideotexSupp1",],
	["VIDEOTEX-SUPPL", "ISO-IR-70", "CSISO70VIDEOTEXSUPP1",],
	PT2,
	std::option::Option::None,
	"PT2",
	["iso-ir-84", "ISO646-PT2", "csISO84Portuguese2",],
	["ISO646-PT2", "CSISO84PORTUGUESE2", "ISO-IR-84", "PT2",],
	ES2,
	std::option::Option::None,
	"ES2",
	["iso-ir-85", "ISO646-ES2", "csISO85Spanish2",],
	["CSISO85SPANISH2", "ISO-IR-85", "ISO646-ES2", "ES2",],
	MSZ_7795_3,
	std::option::Option::None,
	"MSZ_7795.3",
	["iso-ir-86", "ISO646-HU", "hu", "csISO86Hungarian",],
	[
		"MSZ_7795.3",
		"CSISO86HUNGARIAN",
		"ISO-IR-86",
		"ISO646-HU",
		"HU",
	],
	JIS_C6226_1983,
	std::option::Option::None,
	"JIS_C6226-1983",
	["iso-ir-87", "x0208", "JIS_X0208-1983", "csISO87JISX0208",],
	[
		"CSISO87JISX0208",
		"JIS_X0208-1983",
		"X0208",
		"JIS_C6226-1983",
		"ISO-IR-87",
	],
	GREEK7,
	std::option::Option::None,
	"greek7",
	["iso-ir-88", "csISO88Greek7",],
	["GREEK7", "ISO-IR-88", "CSISO88GREEK7",],
	ASMO_449,
	std::option::Option::None,
	"ASMO_449",
	["ISO_9036", "arabic7", "iso-ir-89", "csISO89ASMO449",],
	[
		"ISO-IR-89",
		"ISO_9036",
		"ASMO_449",
		"ARABIC7",
		"CSISO89ASMO449",
	],
	ISO_IR_90,
	std::option::Option::None,
	"iso-ir-90",
	["csISO90",],
	["CSISO90", "ISO-IR-90",],
	JIS_C6229_1984_A,
	std::option::Option::None,
	"JIS_C6229-1984-a",
	["iso-ir-91", "jp-ocr-a", "csISO91JISC62291984a",],
	[
		"CSISO91JISC62291984A",
		"JIS_C6229-1984-A",
		"JP-OCR-A",
		"ISO-IR-91",
	],
	JIS_C6229_1984_B,
	std::option::Option::None,
	"JIS_C6229-1984-b",
	[
		"iso-ir-92",
		"ISO646-JP-OCR-B",
		"jp-ocr-b",
		"csISO92JISC62991984b",
	],
	[
		"ISO-IR-92",
		"CSISO92JISC62991984B",
		"JP-OCR-B",
		"JIS_C6229-1984-B",
		"ISO646-JP-OCR-B",
	],
	JIS_C6229_1984_B_ADD,
	std::option::Option::None,
	"JIS_C6229-1984-b-add",
	["iso-ir-93", "jp-ocr-b-add", "csISO93JIS62291984badd",],
	[
		"ISO-IR-93",
		"CSISO93JIS62291984BADD",
		"JIS_C6229-1984-B-ADD",
		"JP-OCR-B-ADD",
	],
	JIS_C6229_1984_HAND,
	std::option::Option::None,
	"JIS_C6229-1984-hand",
	["iso-ir-94", "jp-ocr-hand", "csISO94JIS62291984hand",],
	[
		"ISO-IR-94",
		"JP-OCR-HAND",
		"CSISO94JIS62291984HAND",
		"JIS_C6229-1984-HAND",
	],
	JIS_C6229_1984_HAND_ADD,
	std::option::Option::None,
	"JIS_C6229-1984-hand-add",
	["iso-ir-95", "jp-ocr-hand-add", "csISO95JIS62291984handadd",],
	[
		"JIS_C6229-1984-HAND-ADD",
		"CSISO95JIS62291984HANDADD",
		"JP-OCR-HAND-ADD",
		"ISO-IR-95",
	],
	JIS_C6229_1984_KANA,
	std::option::Option::None,
	"JIS_C6229-1984-kana",
	["iso-ir-96", "csISO96JISC62291984kana",],
	[
		"JIS_C6229-1984-KANA",
		"CSISO96JISC62291984KANA",
		"ISO-IR-96",
	],
	ISO_2033_1983,
	std::option::Option::None,
	"ISO_2033-1983",
	["iso-ir-98", "e13b", "csISO2033",],
	["E13B", "ISO-IR-98", "ISO_2033-1983", "CSISO2033",],
	ANSI_X3_110_1983,
	std::option::Option::None,
	"ANSI_X3.110-1983",
	["iso-ir-99", "CSA_T500-1983", "NAPLPS", "csISO99NAPLPS",],
	[
		"ANSI_X3.110-1983",
		"CSA_T500-1983",
		"CSISO99NAPLPS",
		"ISO-IR-99",
		"NAPLPS",
	],
	T_61_7BIT,
	std::option::Option::None,
	"T.61-7bit",
	["iso-ir-102", "csISO102T617bit",],
	["T.61-7BIT", "ISO-IR-102", "CSISO102T617BIT",],
	T_61_8BIT,
	std::option::Option::None,
	"T.61-8bit",
	["T.61", "iso-ir-103", "csISO103T618bit",],
	["T.61-8BIT", "T.61", "ISO-IR-103", "CSISO103T618BIT",],
	ECMA_CYRILLIC,
	std::option::Option::None,
	"ECMA-cyrillic",
	["iso-ir-111", "KOI8-E", "csISO111ECMACyrillic",],
	[
		"ISO-IR-111",
		"CSISO111ECMACYRILLIC",
		"KOI8-E",
		"ECMA-CYRILLIC",
	],
	CSA_Z243_4_1985_1,
	std::option::Option::None,
	"CSA_Z243.4-1985-1",
	[
		"iso-ir-121",
		"ISO646-CA",
		"csa7-1",
		"csa71",
		"ca",
		"csISO121Canadian1",
	],
	[
		"CSISO121CANADIAN1",
		"CSA_Z243.4-1985-1",
		"CSA7-1",
		"ISO-IR-121",
		"CA",
		"CSA71",
		"ISO646-CA",
	],
	CSA_Z243_4_1985_2,
	std::option::Option::None,
	"CSA_Z243.4-1985-2",
	[
		"iso-ir-122",
		"ISO646-CA2",
		"csa7-2",
		"csa72",
		"csISO122Canadian2",
	],
	[
		"CSISO122CANADIAN2",
		"ISO-IR-122",
		"ISO646-CA2",
		"CSA72",
		"CSA7-2",
		"CSA_Z243.4-1985-2",
	],
	CSA_Z243_4_1985_GR,
	std::option::Option::None,
	"CSA_Z243.4-1985-gr",
	["iso-ir-123", "csISO123CSAZ24341985gr",],
	["CSA_Z243.4-1985-GR", "CSISO123CSAZ24341985GR", "ISO-IR-123",],
	ISO_8859_6_E,
	std::option::Option::Some("ISO-8859-6-E"),
	"ISO_8859-6-E",
	["csISO88596E", "ISO-8859-6-E",],
	["ISO-8859-6-E", "CSISO88596E", "ISO_8859-6-E",],
	ISO_8859_6_I,
	std::option::Option::Some("ISO-8859-6-I"),
	"ISO_8859-6-I",
	["csISO88596I", "ISO-8859-6-I",],
	["CSISO88596I", "ISO_8859-6-I", "ISO-8859-6-I",],
	T_101_G2,
	std::option::Option::None,
	"T.101-G2",
	["iso-ir-128", "csISO128T101G2",],
	["CSISO128T101G2", "ISO-IR-128", "T.101-G2",],
	ISO_8859_8_E,
	std::option::Option::Some("ISO-8859-8-E"),
	"ISO_8859-8-E",
	["csISO88598E", "ISO-8859-8-E",],
	["CSISO88598E", "ISO_8859-8-E", "ISO-8859-8-E",],
	ISO_8859_8_I,
	std::option::Option::Some("ISO-8859-8-I"),
	"ISO_8859-8-I",
	["csISO88598I", "ISO-8859-8-I",],
	["ISO_8859-8-I", "ISO-8859-8-I", "CSISO88598I",],
	CSN_369103,
	std::option::Option::None,
	"CSN_369103",
	["iso-ir-139", "csISO139CSN369103",],
	["CSISO139CSN369103", "CSN_369103", "ISO-IR-139",],
	JUS_I_B1_002,
	std::option::Option::None,
	"JUS_I.B1.002",
	["iso-ir-141", "ISO646-YU", "js", "yu", "csISO141JUSIB1002",],
	[
		"ISO646-YU",
		"ISO-IR-141",
		"CSISO141JUSIB1002",
		"JS",
		"JUS_I.B1.002",
		"YU",
	],
	IEC_P27_1,
	std::option::Option::None,
	"IEC_P27-1",
	["iso-ir-143", "csISO143IECP271",],
	["CSISO143IECP271", "ISO-IR-143", "IEC_P27-1",],
	JUS_I_B1_003_SERB,
	std::option::Option::None,
	"JUS_I.B1.003-serb",
	["iso-ir-146", "serbian", "csISO146Serbian",],
	[
		"CSISO146SERBIAN",
		"ISO-IR-146",
		"SERBIAN",
		"JUS_I.B1.003-SERB",
	],
	JUS_I_B1_003_MAC,
	std::option::Option::None,
	"JUS_I.B1.003-mac",
	["macedonian", "iso-ir-147", "csISO147Macedonian",],
	[
		"CSISO147MACEDONIAN",
		"JUS_I.B1.003-MAC",
		"ISO-IR-147",
		"MACEDONIAN",
	],
	GREEK_CCITT,
	std::option::Option::None,
	"greek-ccitt",
	["iso-ir-150", "csISO150", "csISO150GreekCCITT",],
	[
		"ISO-IR-150",
		"GREEK-CCITT",
		"CSISO150GREEKCCITT",
		"CSISO150",
	],
	NC_NC00_10_81,
	std::option::Option::None,
	"NC_NC00-10:81",
	["cuba", "iso-ir-151", "ISO646-CU", "csISO151Cuba",],
	[
		"ISO-IR-151",
		"NC_NC00-10:81",
		"CSISO151CUBA",
		"CUBA",
		"ISO646-CU",
	],
	ISO_6937_2_25,
	std::option::Option::None,
	"ISO_6937-2-25",
	["iso-ir-152", "csISO6937Add",],
	["ISO-IR-152", "CSISO6937ADD", "ISO_6937-2-25",],
	GOST_19768_74,
	std::option::Option::None,
	"GOST_19768-74",
	["ST_SEV_358-88", "iso-ir-153", "csISO153GOST1976874",],
	[
		"ST_SEV_358-88",
		"ISO-IR-153",
		"GOST_19768-74",
		"CSISO153GOST1976874",
	],
	ISO_8859_SUPP,
	std::option::Option::None,
	"ISO_8859-supp",
	["iso-ir-154", "latin1-2-5", "csISO8859Supp",],
	["CSISO8859SUPP", "ISO-IR-154", "ISO_8859-SUPP", "LATIN1-2-5",],
	ISO_10367_BOX,
	std::option::Option::None,
	"ISO_10367-box",
	["iso-ir-155", "csISO10367Box",],
	["ISO-IR-155", "CSISO10367BOX", "ISO_10367-BOX",],
	LATIN_LAP,
	std::option::Option::None,
	"latin-lap",
	["lap", "iso-ir-158", "csISO158Lap",],
	["CSISO158LAP", "LATIN-LAP", "ISO-IR-158", "LAP",],
	JIS_X0212_1990,
	std::option::Option::None,
	"JIS_X0212-1990",
	["x0212", "iso-ir-159", "csISO159JISX02121990",],
	[
		"ISO-IR-159",
		"JIS_X0212-1990",
		"X0212",
		"CSISO159JISX02121990",
	],
	DS_2089,
	std::option::Option::None,
	"DS_2089",
	["DS2089", "ISO646-DK", "dk", "csISO646Danish",],
	["ISO646-DK", "DK", "DS_2089", "DS2089", "CSISO646DANISH",],
	US_DK,
	std::option::Option::None,
	"us-dk",
	["csUSDK",],
	["US-DK", "CSUSDK",],
	DK_US,
	std::option::Option::None,
	"dk-us",
	["csDKUS",],
	["DK-US", "CSDKUS",],
	KSC5636,
	std::option::Option::None,
	"KSC5636",
	["ISO646-KR", "csKSC5636",],
	["CSKSC5636", "ISO646-KR", "KSC5636",],
	UNICODE_1_1_UTF_7,
	std::option::Option::None,
	"UNICODE-1-1-UTF-7",
	["csUnicode11UTF7",],
	["CSUNICODE11UTF7", "UNICODE-1-1-UTF-7",],
	ISO_2022_CN,
	std::option::Option::None,
	"ISO-2022-CN",
	["csISO2022CN",],
	["CSISO2022CN", "ISO-2022-CN",],
	ISO_2022_CN_EXT,
	std::option::Option::None,
	"ISO-2022-CN-EXT",
	["csISO2022CNEXT",],
	["CSISO2022CNEXT", "ISO-2022-CN-EXT",],
	UTF_8,
	std::option::Option::None,
	"UTF-8",
	["csUTF8",],
	["UTF-8", "CSUTF8",],
	ISO_8859_13,
	std::option::Option::None,
	"ISO-8859-13",
	["csISO885913",],
	["ISO-8859-13", "CSISO885913",],
	ISO_8859_14,
	std::option::Option::None,
	"ISO-8859-14",
	[
		"iso-ir-199",
		"ISO_8859-14:1998",
		"ISO_8859-14",
		"latin8",
		"iso-celtic",
		"l8",
		"csISO885914",
	],
	[
		"ISO-IR-199",
		"ISO_8859-14:1998",
		"ISO-8859-14",
		"LATIN8",
		"ISO-CELTIC",
		"ISO_8859-14",
		"CSISO885914",
		"L8",
	],
	ISO_8859_15,
	std::option::Option::None,
	"ISO-8859-15",
	["ISO_8859-15", "Latin-9", "csISO885915",],
	["ISO-8859-15", "CSISO885915", "LATIN-9", "ISO_8859-15",],
	ISO_8859_16,
	std::option::Option::None,
	"ISO-8859-16",
	[
		"iso-ir-226",
		"ISO_8859-16:2001",
		"ISO_8859-16",
		"latin10",
		"l10",
		"csISO885916",
	],
	[
		"ISO-8859-16",
		"ISO_8859-16",
		"ISO_8859-16:2001",
		"ISO-IR-226",
		"L10",
		"CSISO885916",
		"LATIN10",
	],
	GBK,
	std::option::Option::None,
	"GBK",
	["CP936", "MS936", "windows-936", "csGBK",],
	["CSGBK", "MS936", "WINDOWS-936", "CP936", "GBK",],
	GB18030,
	std::option::Option::None,
	"GB18030",
	["csGB18030",],
	["GB18030", "CSGB18030",],
	OSD_EBCDIC_DF04_15,
	std::option::Option::None,
	"OSD_EBCDIC_DF04_15",
	["csOSDEBCDICDF0415",],
	["CSOSDEBCDICDF0415", "OSD_EBCDIC_DF04_15",],
	OSD_EBCDIC_DF03_IRV,
	std::option::Option::None,
	"OSD_EBCDIC_DF03_IRV",
	["csOSDEBCDICDF03IRV",],
	["CSOSDEBCDICDF03IRV", "OSD_EBCDIC_DF03_IRV",],
	OSD_EBCDIC_DF04_1,
	std::option::Option::None,
	"OSD_EBCDIC_DF04_1",
	["csOSDEBCDICDF041",],
	["CSOSDEBCDICDF041", "OSD_EBCDIC_DF04_1",],
	ISO_11548_1,
	std::option::Option::None,
	"ISO-11548-1",
	["ISO_11548-1", "ISO_TR_11548-1", "csISO115481",],
	[
		"CSISO115481",
		"ISO_TR_11548-1",
		"ISO-11548-1",
		"ISO_11548-1",
	],
	KZ_1048,
	std::option::Option::None,
	"KZ-1048",
	["STRK1048-2002", "RK1048", "csKZ1048",],
	["KZ-1048", "CSKZ1048", "RK1048", "STRK1048-2002",],
	ISO_10646_UCS_2,
	std::option::Option::None,
	"ISO-10646-UCS-2",
	["csUnicode",],
	["ISO-10646-UCS-2", "CSUNICODE",],
	ISO_10646_UCS_4,
	std::option::Option::None,
	"ISO-10646-UCS-4",
	["csUCS4",],
	["CSUCS4", "ISO-10646-UCS-4",],
	ISO_10646_UCS_BASIC,
	std::option::Option::None,
	"ISO-10646-UCS-Basic",
	["csUnicodeASCII",],
	["ISO-10646-UCS-BASIC", "CSUNICODEASCII",],
	ISO_10646_UNICODE_LATIN1,
	std::option::Option::None,
	"ISO-10646-Unicode-Latin1",
	["csUnicodeLatin1", "ISO-10646",],
	["CSUNICODELATIN1", "ISO-10646-UNICODE-LATIN1", "ISO-10646",],
	ISO_10646_J_1,
	std::option::Option::None,
	"ISO-10646-J-1",
	["csUnicodeJapanese",],
	["ISO-10646-J-1", "CSUNICODEJAPANESE",],
	ISO_UNICODE_IBM_1261,
	std::option::Option::None,
	"ISO-Unicode-IBM-1261",
	["csUnicodeIBM1261",],
	["ISO-UNICODE-IBM-1261", "CSUNICODEIBM1261",],
	ISO_UNICODE_IBM_1268,
	std::option::Option::None,
	"ISO-Unicode-IBM-1268",
	["csUnicodeIBM1268",],
	["CSUNICODEIBM1268", "ISO-UNICODE-IBM-1268",],
	ISO_UNICODE_IBM_1276,
	std::option::Option::None,
	"ISO-Unicode-IBM-1276",
	["csUnicodeIBM1276",],
	["ISO-UNICODE-IBM-1276", "CSUNICODEIBM1276",],
	ISO_UNICODE_IBM_1264,
	std::option::Option::None,
	"ISO-Unicode-IBM-1264",
	["csUnicodeIBM1264",],
	["CSUNICODEIBM1264", "ISO-UNICODE-IBM-1264",],
	ISO_UNICODE_IBM_1265,
	std::option::Option::None,
	"ISO-Unicode-IBM-1265",
	["csUnicodeIBM1265",],
	["CSUNICODEIBM1265", "ISO-UNICODE-IBM-1265",],
	UNICODE_1_1,
	std::option::Option::None,
	"UNICODE-1-1",
	["csUnicode11",],
	["CSUNICODE11", "UNICODE-1-1",],
	SCSU,
	std::option::Option::None,
	"SCSU",
	["csSCSU",],
	["CSSCSU", "SCSU",],
	UTF_7,
	std::option::Option::None,
	"UTF-7",
	["csUTF7",],
	["CSUTF7", "UTF-7",],
	UTF_16BE,
	std::option::Option::None,
	"UTF-16BE",
	["csUTF16BE",],
	["CSUTF16BE", "UTF-16BE",],
	UTF_16LE,
	std::option::Option::None,
	"UTF-16LE",
	["csUTF16LE",],
	["UTF-16LE", "CSUTF16LE",],
	UTF_16,
	std::option::Option::None,
	"UTF-16",
	["csUTF16",],
	["UTF-16", "CSUTF16",],
	CESU_8,
	std::option::Option::None,
	"CESU-8",
	["csCESU8", "csCESU-8",],
	["CESU-8", "CSCESU-8", "CSCESU8",],
	UTF_32,
	std::option::Option::None,
	"UTF-32",
	["csUTF32",],
	["UTF-32", "CSUTF32",],
	UTF_32BE,
	std::option::Option::None,
	"UTF-32BE",
	["csUTF32BE",],
	["UTF-32BE", "CSUTF32BE",],
	UTF_32LE,
	std::option::Option::None,
	"UTF-32LE",
	["csUTF32LE",],
	["CSUTF32LE", "UTF-32LE",],
	BOCU_1,
	std::option::Option::None,
	"BOCU-1",
	["csBOCU1", "csBOCU-1",],
	["BOCU-1", "CSBOCU-1", "CSBOCU1",],
	ISO_8859_1_WINDOWS_3_0_LATIN_1,
	std::option::Option::None,
	"ISO-8859-1-Windows-3.0-Latin-1",
	["csWindows30Latin1",],
	["ISO-8859-1-WINDOWS-3.0-LATIN-1", "CSWINDOWS30LATIN1",],
	ISO_8859_1_WINDOWS_3_1_LATIN_1,
	std::option::Option::None,
	"ISO-8859-1-Windows-3.1-Latin-1",
	["csWindows31Latin1",],
	["CSWINDOWS31LATIN1", "ISO-8859-1-WINDOWS-3.1-LATIN-1",],
	ISO_8859_2_WINDOWS_LATIN_2,
	std::option::Option::None,
	"ISO-8859-2-Windows-Latin-2",
	["csWindows31Latin2",],
	["ISO-8859-2-WINDOWS-LATIN-2", "CSWINDOWS31LATIN2",],
	ISO_8859_9_WINDOWS_LATIN_5,
	std::option::Option::None,
	"ISO-8859-9-Windows-Latin-5",
	["csWindows31Latin5",],
	["CSWINDOWS31LATIN5", "ISO-8859-9-WINDOWS-LATIN-5",],
	HP_ROMAN8,
	std::option::Option::None,
	"hp-roman8",
	["roman8", "r8", "csHPRoman8",],
	["R8", "CSHPROMAN8", "ROMAN8", "HP-ROMAN8",],
	ADOBE_STANDARD_ENCODING,
	std::option::Option::None,
	"Adobe-Standard-Encoding",
	["csAdobeStandardEncoding",],
	["CSADOBESTANDARDENCODING", "ADOBE-STANDARD-ENCODING",],
	VENTURA_US,
	std::option::Option::None,
	"Ventura-US",
	["csVenturaUS",],
	["VENTURA-US", "CSVENTURAUS",],
	VENTURA_INTERNATIONAL,
	std::option::Option::None,
	"Ventura-International",
	["csVenturaInternational",],
	["CSVENTURAINTERNATIONAL", "VENTURA-INTERNATIONAL",],
	DEC_MCS,
	std::option::Option::None,
	"DEC-MCS",
	["dec", "csDECMCS",],
	["DEC-MCS", "CSDECMCS", "DEC",],
	IBM850,
	std::option::Option::None,
	"IBM850",
	["cp850", "850", "csPC850Multilingual",],
	["850", "CSPC850MULTILINGUAL", "IBM850", "CP850",],
	PC8_DANISH_NORWEGIAN,
	std::option::Option::None,
	"PC8-Danish-Norwegian",
	["csPC8DanishNorwegian",],
	["CSPC8DANISHNORWEGIAN", "PC8-DANISH-NORWEGIAN",],
	IBM862,
	std::option::Option::None,
	"IBM862",
	["cp862", "862", "csPC862LatinHebrew",],
	["CSPC862LATINHEBREW", "IBM862", "862", "CP862",],
	PC8_TURKISH,
	std::option::Option::None,
	"PC8-Turkish",
	["csPC8Turkish",],
	["CSPC8TURKISH", "PC8-TURKISH",],
	IBM_SYMBOLS,
	std::option::Option::None,
	"IBM-Symbols",
	["csIBMSymbols",],
	["CSIBMSYMBOLS", "IBM-SYMBOLS",],
	IBM_THAI,
	std::option::Option::None,
	"IBM-Thai",
	["csIBMThai",],
	["IBM-THAI", "CSIBMTHAI",],
	HP_LEGAL,
	std::option::Option::None,
	"HP-Legal",
	["csHPLegal",],
	["HP-LEGAL", "CSHPLEGAL",],
	HP_PI_FONT,
	std::option::Option::None,
	"HP-Pi-font",
	["csHPPiFont",],
	["HP-PI-FONT", "CSHPPIFONT",],
	HP_MATH8,
	std::option::Option::None,
	"HP-Math8",
	["csHPMath8",],
	["HP-MATH8", "CSHPMATH8",],
	ADOBE_SYMBOL_ENCODING,
	std::option::Option::None,
	"Adobe-Symbol-Encoding",
	["csHPPSMath",],
	["ADOBE-SYMBOL-ENCODING", "CSHPPSMATH",],
	HP_DESKTOP,
	std::option::Option::None,
	"HP-DeskTop",
	["csHPDesktop",],
	["HP-DESKTOP", "CSHPDESKTOP",],
	VENTURA_MATH,
	std::option::Option::None,
	"Ventura-Math",
	["csVenturaMath",],
	["VENTURA-MATH", "CSVENTURAMATH",],
	MICROSOFT_PUBLISHING,
	std::option::Option::None,
	"Microsoft-Publishing",
	["csMicrosoftPublishing",],
	["CSMICROSOFTPUBLISHING", "MICROSOFT-PUBLISHING",],
	WINDOWS_31J,
	std::option::Option::None,
	"Windows-31J",
	["csWindows31J",],
	["WINDOWS-31J", "CSWINDOWS31J",],
	GB2312,
	std::option::Option::Some("GB2312"),
	"GB2312",
	["csGB2312",],
	["GB2312", "CSGB2312",],
	BIG5,
	std::option::Option::Some("Big5"),
	"Big5",
	["csBig5",],
	["BIG5", "CSBIG5",],
	MACINTOSH,
	std::option::Option::None,
	"macintosh",
	["mac", "csMacintosh",],
	["MAC", "MACINTOSH", "CSMACINTOSH",],
	IBM037,
	std::option::Option::None,
	"IBM037",
	[
		"cp037",
		"ebcdic-cp-us",
		"ebcdic-cp-ca",
		"ebcdic-cp-wt",
		"ebcdic-cp-nl",
		"csIBM037",
	],
	[
		"EBCDIC-CP-WT",
		"EBCDIC-CP-US",
		"EBCDIC-CP-CA",
		"EBCDIC-CP-NL",
		"CSIBM037",
		"IBM037",
		"CP037",
	],
	IBM038,
	std::option::Option::None,
	"IBM038",
	["EBCDIC-INT", "cp038", "csIBM038",],
	["CSIBM038", "EBCDIC-INT", "CP038", "IBM038",],
	IBM273,
	std::option::Option::None,
	"IBM273",
	["CP273", "csIBM273",],
	["IBM273", "CSIBM273", "CP273",],
	IBM274,
	std::option::Option::None,
	"IBM274",
	["EBCDIC-BE", "CP274", "csIBM274",],
	["IBM274", "CP274", "CSIBM274", "EBCDIC-BE",],
	IBM275,
	std::option::Option::None,
	"IBM275",
	["EBCDIC-BR", "cp275", "csIBM275",],
	["CSIBM275", "EBCDIC-BR", "IBM275", "CP275",],
	IBM277,
	std::option::Option::None,
	"IBM277",
	["EBCDIC-CP-DK", "EBCDIC-CP-NO", "csIBM277",],
	["EBCDIC-CP-NO", "IBM277", "CSIBM277", "EBCDIC-CP-DK",],
	IBM278,
	std::option::Option::None,
	"IBM278",
	["CP278", "ebcdic-cp-fi", "ebcdic-cp-se", "csIBM278",],
	[
		"IBM278",
		"CSIBM278",
		"EBCDIC-CP-FI",
		"EBCDIC-CP-SE",
		"CP278",
	],
	IBM280,
	std::option::Option::None,
	"IBM280",
	["CP280", "ebcdic-cp-it", "csIBM280",],
	["CP280", "EBCDIC-CP-IT", "CSIBM280", "IBM280",],
	IBM281,
	std::option::Option::None,
	"IBM281",
	["EBCDIC-JP-E", "cp281", "csIBM281",],
	["EBCDIC-JP-E", "IBM281", "CP281", "CSIBM281",],
	IBM284,
	std::option::Option::None,
	"IBM284",
	["CP284", "ebcdic-cp-es", "csIBM284",],
	["CSIBM284", "IBM284", "EBCDIC-CP-ES", "CP284",],
	IBM285,
	std::option::Option::None,
	"IBM285",
	["CP285", "ebcdic-cp-gb", "csIBM285",],
	["CP285", "EBCDIC-CP-GB", "CSIBM285", "IBM285",],
	IBM290,
	std::option::Option::None,
	"IBM290",
	["cp290", "EBCDIC-JP-kana", "csIBM290",],
	["EBCDIC-JP-KANA", "IBM290", "CP290", "CSIBM290",],
	IBM297,
	std::option::Option::None,
	"IBM297",
	["cp297", "ebcdic-cp-fr", "csIBM297",],
	["CSIBM297", "EBCDIC-CP-FR", "CP297", "IBM297",],
	IBM420,
	std::option::Option::None,
	"IBM420",
	["cp420", "ebcdic-cp-ar1", "csIBM420",],
	["IBM420", "CSIBM420", "CP420", "EBCDIC-CP-AR1",],
	IBM423,
	std::option::Option::None,
	"IBM423",
	["cp423", "ebcdic-cp-gr", "csIBM423",],
	["IBM423", "CP423", "EBCDIC-CP-GR", "CSIBM423",],
	IBM424,
	std::option::Option::None,
	"IBM424",
	["cp424", "ebcdic-cp-he", "csIBM424",],
	["IBM424", "CP424", "CSIBM424", "EBCDIC-CP-HE",],
	IBM437,
	std::option::Option::None,
	"IBM437",
	["cp437", "437", "csPC8CodePage437",],
	["CSPC8CODEPAGE437", "437", "CP437", "IBM437",],
	IBM500,
	std::option::Option::None,
	"IBM500",
	["CP500", "ebcdic-cp-be", "ebcdic-cp-ch", "csIBM500",],
	[
		"IBM500",
		"CP500",
		"EBCDIC-CP-BE",
		"EBCDIC-CP-CH",
		"CSIBM500",
	],
	IBM851,
	std::option::Option::None,
	"IBM851",
	["cp851", "851", "csIBM851",],
	["CP851", "IBM851", "851", "CSIBM851",],
	IBM852,
	std::option::Option::None,
	"IBM852",
	["cp852", "852", "csPCp852",],
	["852", "IBM852", "CP852", "CSPCP852",],
	IBM855,
	std::option::Option::None,
	"IBM855",
	["cp855", "855", "csIBM855",],
	["CP855", "IBM855", "CSIBM855", "855",],
	IBM857,
	std::option::Option::None,
	"IBM857",
	["cp857", "857", "csIBM857",],
	["CP857", "IBM857", "857", "CSIBM857",],
	IBM860,
	std::option::Option::None,
	"IBM860",
	["cp860", "860", "csIBM860",],
	["CSIBM860", "860", "CP860", "IBM860",],
	IBM861,
	std::option::Option::None,
	"IBM861",
	["cp861", "861", "cp-is", "csIBM861",],
	["CP861", "CP-IS", "CSIBM861", "861", "IBM861",],
	IBM863,
	std::option::Option::None,
	"IBM863",
	["cp863", "863", "csIBM863",],
	["CSIBM863", "IBM863", "863", "CP863",],
	IBM864,
	std::option::Option::None,
	"IBM864",
	["cp864", "csIBM864",],
	["CSIBM864", "IBM864", "CP864",],
	IBM865,
	std::option::Option::None,
	"IBM865",
	["cp865", "865", "csIBM865",],
	["IBM865", "865", "CP865", "CSIBM865",],
	IBM868,
	std::option::Option::None,
	"IBM868",
	["CP868", "cp-ar", "csIBM868",],
	["CSIBM868", "CP868", "IBM868", "CP-AR",],
	IBM869,
	std::option::Option::None,
	"IBM869",
	["cp869", "869", "cp-gr", "csIBM869",],
	["IBM869", "869", "CP869", "CSIBM869", "CP-GR",],
	IBM870,
	std::option::Option::None,
	"IBM870",
	["CP870", "ebcdic-cp-roece", "ebcdic-cp-yu", "csIBM870",],
	[
		"EBCDIC-CP-ROECE",
		"CSIBM870",
		"IBM870",
		"CP870",
		"EBCDIC-CP-YU",
	],
	IBM871,
	std::option::Option::None,
	"IBM871",
	["CP871", "ebcdic-cp-is", "csIBM871",],
	["CP871", "CSIBM871", "IBM871", "EBCDIC-CP-IS",],
	IBM880,
	std::option::Option::None,
	"IBM880",
	["cp880", "EBCDIC-Cyrillic", "csIBM880",],
	["IBM880", "CP880", "CSIBM880", "EBCDIC-CYRILLIC",],
	IBM891,
	std::option::Option::None,
	"IBM891",
	["cp891", "csIBM891",],
	["CP891", "CSIBM891", "IBM891",],
	IBM903,
	std::option::Option::None,
	"IBM903",
	["cp903", "csIBM903",],
	["CP903", "IBM903", "CSIBM903",],
	IBM904,
	std::option::Option::None,
	"IBM904",
	["cp904", "904", "csIBBM904",],
	["IBM904", "CP904", "CSIBBM904", "904",],
	IBM905,
	std::option::Option::None,
	"IBM905",
	["CP905", "ebcdic-cp-tr", "csIBM905",],
	["CP905", "IBM905", "EBCDIC-CP-TR", "CSIBM905",],
	IBM918,
	std::option::Option::None,
	"IBM918",
	["CP918", "ebcdic-cp-ar2", "csIBM918",],
	["CP918", "EBCDIC-CP-AR2", "IBM918", "CSIBM918",],
	IBM1026,
	std::option::Option::None,
	"IBM1026",
	["CP1026", "csIBM1026",],
	["IBM1026", "CSIBM1026", "CP1026",],
	EBCDIC_AT_DE,
	std::option::Option::None,
	"EBCDIC-AT-DE",
	["csIBMEBCDICATDE",],
	["CSIBMEBCDICATDE", "EBCDIC-AT-DE",],
	EBCDIC_AT_DE_A,
	std::option::Option::None,
	"EBCDIC-AT-DE-A",
	["csEBCDICATDEA",],
	["EBCDIC-AT-DE-A", "CSEBCDICATDEA",],
	EBCDIC_CA_FR,
	std::option::Option::None,
	"EBCDIC-CA-FR",
	["csEBCDICCAFR",],
	["EBCDIC-CA-FR", "CSEBCDICCAFR",],
	EBCDIC_DK_NO,
	std::option::Option::None,
	"EBCDIC-DK-NO",
	["csEBCDICDKNO",],
	["EBCDIC-DK-NO", "CSEBCDICDKNO",],
	EBCDIC_DK_NO_A,
	std::option::Option::None,
	"EBCDIC-DK-NO-A",
	["csEBCDICDKNOA",],
	["EBCDIC-DK-NO-A", "CSEBCDICDKNOA",],
	EBCDIC_FI_SE,
	std::option::Option::None,
	"EBCDIC-FI-SE",
	["csEBCDICFISE",],
	["CSEBCDICFISE", "EBCDIC-FI-SE",],
	EBCDIC_FI_SE_A,
	std::option::Option::None,
	"EBCDIC-FI-SE-A",
	["csEBCDICFISEA",],
	["EBCDIC-FI-SE-A", "CSEBCDICFISEA",],
	EBCDIC_FR,
	std::option::Option::None,
	"EBCDIC-FR",
	["csEBCDICFR",],
	["CSEBCDICFR", "EBCDIC-FR",],
	EBCDIC_IT,
	std::option::Option::None,
	"EBCDIC-IT",
	["csEBCDICIT",],
	["EBCDIC-IT", "CSEBCDICIT",],
	EBCDIC_PT,
	std::option::Option::None,
	"EBCDIC-PT",
	["csEBCDICPT",],
	["EBCDIC-PT", "CSEBCDICPT",],
	EBCDIC_ES,
	std::option::Option::None,
	"EBCDIC-ES",
	["csEBCDICES",],
	["CSEBCDICES", "EBCDIC-ES",],
	EBCDIC_ES_A,
	std::option::Option::None,
	"EBCDIC-ES-A",
	["csEBCDICESA",],
	["CSEBCDICESA", "EBCDIC-ES-A",],
	EBCDIC_ES_S,
	std::option::Option::None,
	"EBCDIC-ES-S",
	["csEBCDICESS",],
	["EBCDIC-ES-S", "CSEBCDICESS",],
	EBCDIC_UK,
	std::option::Option::None,
	"EBCDIC-UK",
	["csEBCDICUK",],
	["EBCDIC-UK", "CSEBCDICUK",],
	EBCDIC_US,
	std::option::Option::None,
	"EBCDIC-US",
	["csEBCDICUS",],
	["EBCDIC-US", "CSEBCDICUS",],
	UNKNOWN_8BIT,
	std::option::Option::None,
	"UNKNOWN-8BIT",
	["csUnknown8BiT",],
	["UNKNOWN-8BIT", "CSUNKNOWN8BIT",],
	MNEMONIC,
	std::option::Option::None,
	"MNEMONIC",
	["csMnemonic",],
	["MNEMONIC", "CSMNEMONIC",],
	MNEM,
	std::option::Option::None,
	"MNEM",
	["csMnem",],
	["CSMNEM", "MNEM",],
	VISCII,
	std::option::Option::None,
	"VISCII",
	["csVISCII",],
	["CSVISCII", "VISCII",],
	VIQR,
	std::option::Option::None,
	"VIQR",
	["csVIQR",],
	["CSVIQR", "VIQR",],
	KOI8_R,
	std::option::Option::Some("KOI8-R"),
	"KOI8-R",
	["csKOI8R",],
	["CSKOI8R", "KOI8-R",],
	HZ_GB_2312,
	std::option::Option::None,
	"HZ-GB-2312",
	[],
	["HZ-GB-2312",],
	IBM866,
	std::option::Option::None,
	"IBM866",
	["cp866", "866", "csIBM866",],
	["CP866", "IBM866", "CSIBM866", "866",],
	IBM775,
	std::option::Option::None,
	"IBM775",
	["cp775", "csPC775Baltic",],
	["CSPC775BALTIC", "IBM775", "CP775",],
	KOI8_U,
	std::option::Option::None,
	"KOI8-U",
	["csKOI8U",],
	["KOI8-U", "CSKOI8U",],
	IBM00858,
	std::option::Option::None,
	"IBM00858",
	[
		"CCSID00858",
		"CP00858",
		"PC-Multilingual-850+euro",
		"csIBM00858",
	],
	[
		"IBM00858",
		"PC-MULTILINGUAL-850+EURO",
		"CP00858",
		"CSIBM00858",
		"CCSID00858",
	],
	IBM00924,
	std::option::Option::None,
	"IBM00924",
	["CCSID00924", "CP00924", "ebcdic-Latin9--euro", "csIBM00924",],
	[
		"IBM00924",
		"CP00924",
		"EBCDIC-LATIN9--EURO",
		"CCSID00924",
		"CSIBM00924",
	],
	IBM01140,
	std::option::Option::None,
	"IBM01140",
	["CCSID01140", "CP01140", "ebcdic-us-37+euro", "csIBM01140",],
	[
		"EBCDIC-US-37+EURO",
		"CCSID01140",
		"CSIBM01140",
		"IBM01140",
		"CP01140",
	],
	IBM01141,
	std::option::Option::None,
	"IBM01141",
	["CCSID01141", "CP01141", "ebcdic-de-273+euro", "csIBM01141",],
	[
		"CCSID01141",
		"CSIBM01141",
		"CP01141",
		"IBM01141",
		"EBCDIC-DE-273+EURO",
	],
	IBM01142,
	std::option::Option::None,
	"IBM01142",
	[
		"CCSID01142",
		"CP01142",
		"ebcdic-dk-277+euro",
		"ebcdic-no-277+euro",
		"csIBM01142",
	],
	[
		"EBCDIC-NO-277+EURO",
		"EBCDIC-DK-277+EURO",
		"CP01142",
		"CCSID01142",
		"IBM01142",
		"CSIBM01142",
	],
	IBM01143,
	std::option::Option::None,
	"IBM01143",
	[
		"CCSID01143",
		"CP01143",
		"ebcdic-fi-278+euro",
		"ebcdic-se-278+euro",
		"csIBM01143",
	],
	[
		"EBCDIC-SE-278+EURO",
		"IBM01143",
		"CSIBM01143",
		"CP01143",
		"EBCDIC-FI-278+EURO",
		"CCSID01143",
	],
	IBM01144,
	std::option::Option::None,
	"IBM01144",
	["CCSID01144", "CP01144", "ebcdic-it-280+euro", "csIBM01144",],
	[
		"CP01144",
		"IBM01144",
		"CCSID01144",
		"EBCDIC-IT-280+EURO",
		"CSIBM01144",
	],
	IBM01145,
	std::option::Option::None,
	"IBM01145",
	["CCSID01145", "CP01145", "ebcdic-es-284+euro", "csIBM01145",],
	[
		"IBM01145",
		"CCSID01145",
		"CP01145",
		"EBCDIC-ES-284+EURO",
		"CSIBM01145",
	],
	IBM01146,
	std::option::Option::None,
	"IBM01146",
	["CCSID01146", "CP01146", "ebcdic-gb-285+euro", "csIBM01146",],
	[
		"CP01146",
		"CSIBM01146",
		"EBCDIC-GB-285+EURO",
		"CCSID01146",
		"IBM01146",
	],
	IBM01147,
	std::option::Option::None,
	"IBM01147",
	["CCSID01147", "CP01147", "ebcdic-fr-297+euro", "csIBM01147",],
	[
		"EBCDIC-FR-297+EURO",
		"IBM01147",
		"CCSID01147",
		"CP01147",
		"CSIBM01147",
	],
	IBM01148,
	std::option::Option::None,
	"IBM01148",
	[
		"CCSID01148",
		"CP01148",
		"ebcdic-international-500+euro",
		"csIBM01148",
	],
	[
		"IBM01148",
		"EBCDIC-INTERNATIONAL-500+EURO",
		"CSIBM01148",
		"CCSID01148",
		"CP01148",
	],
	IBM01149,
	std::option::Option::None,
	"IBM01149",
	["CCSID01149", "CP01149", "ebcdic-is-871+euro", "csIBM01149",],
	[
		"CCSID01149",
		"CSIBM01149",
		"CP01149",
		"EBCDIC-IS-871+EURO",
		"IBM01149",
	],
	BIG5_HKSCS,
	std::option::Option::None,
	"Big5-HKSCS",
	["csBig5HKSCS",],
	["BIG5-HKSCS", "CSBIG5HKSCS",],
	IBM1047,
	std::option::Option::None,
	"IBM1047",
	["IBM-1047", "csIBM1047",],
	["IBM1047", "CSIBM1047", "IBM-1047",],
	PTCP154,
	std::option::Option::None,
	"PTCP154",
	["csPTCP154", "PT154", "CP154", "Cyrillic-Asian",],
	["CSPTCP154", "CYRILLIC-ASIAN", "CP154", "PT154", "PTCP154",],
	AMIGA_1251,
	std::option::Option::None,
	"Amiga-1251",
	["Ami1251", "Amiga1251", "Ami-1251", "csAmiga1251",],
	[
		"AMIGA1251",
		"CSAMIGA1251",
		"AMIGA-1251",
		"AMI1251",
		"AMI-1251",
	],
	KOI7_SWITCHED,
	std::option::Option::None,
	"KOI7-switched",
	["csKOI7switched",],
	["KOI7-SWITCHED", "CSKOI7SWITCHED",],
	BRF,
	std::option::Option::None,
	"BRF",
	["csBRF",],
	["BRF", "CSBRF",],
	TSCII,
	std::option::Option::None,
	"TSCII",
	["csTSCII",],
	["TSCII", "CSTSCII",],
	CP51932,
	std::option::Option::None,
	"CP51932",
	["csCP51932",],
	["CP51932", "CSCP51932",],
	WINDOWS_874,
	std::option::Option::None,
	"windows-874",
	["cswindows874",],
	["WINDOWS-874", "CSWINDOWS874",],
	WINDOWS_1250,
	std::option::Option::None,
	"windows-1250",
	["cswindows1250",],
	["CSWINDOWS1250", "WINDOWS-1250",],
	WINDOWS_1251,
	std::option::Option::None,
	"windows-1251",
	["cswindows1251",],
	["WINDOWS-1251", "CSWINDOWS1251",],
	WINDOWS_1252,
	std::option::Option::None,
	"windows-1252",
	["cswindows1252",],
	["WINDOWS-1252", "CSWINDOWS1252",],
	WINDOWS_1253,
	std::option::Option::None,
	"windows-1253",
	["cswindows1253",],
	["WINDOWS-1253", "CSWINDOWS1253",],
	WINDOWS_1254,
	std::option::Option::None,
	"windows-1254",
	["cswindows1254",],
	["CSWINDOWS1254", "WINDOWS-1254",],
	WINDOWS_1255,
	std::option::Option::None,
	"windows-1255",
	["cswindows1255",],
	["CSWINDOWS1255", "WINDOWS-1255",],
	WINDOWS_1256,
	std::option::Option::None,
	"windows-1256",
	["cswindows1256",],
	["CSWINDOWS1256", "WINDOWS-1256",],
	WINDOWS_1257,
	std::option::Option::None,
	"windows-1257",
	["cswindows1257",],
	["CSWINDOWS1257", "WINDOWS-1257",],
	WINDOWS_1258,
	std::option::Option::None,
	"windows-1258",
	["cswindows1258",],
	["CSWINDOWS1258", "WINDOWS-1258",],
	TIS_620,
	std::option::Option::None,
	"TIS-620",
	["csTIS620", "ISO-8859-11",],
	["ISO-8859-11", "TIS-620", "CSTIS620",],
	CP50220,
	std::option::Option::None,
	"CP50220",
	["csCP50220",],
	["CSCP50220", "CP50220",],
);
