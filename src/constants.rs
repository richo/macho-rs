pub const MH_MAGIC_64: u32 = 0xfeedfacf;
pub const MH_CIGAM_64: u32 = 0xcffaedfe;

const LC_REQ_DYLD: u32 = 0x80000000;

#[repr(u32)]
#[derive(Eq,PartialEq)]
#[allow(non_camel_case_types)]
pub enum LcType {
    /// After MacOS X 10.1 when a new load command is added that is required to be
    /// understood by the dynamic linker for the image to execute properly the
    /// LC_REQ_DYLD bit will be or'ed into the load command constant.  If the dynamic
    /// linker sees such a load command it it does not understand will issue a
    /// "unknown load command required for execution" error and refuse to use the
    /// image.  Other load commands without this bit that are not understood will
    /// simply be ignored.
    LC_REQ_DYLD = LC_REQ_DYLD,

    /// segment of this file to be mapped
    LC_SEGMENT = 0x1,

    /// link-edit stab symbol table info
    LC_SYMTAB = 0x2,

    /// link-edit gdb symbol table info (obsolete)
    LC_SYMSEG = 0x3,

    /// thread
    LC_THREAD = 0x4,

    /// unix thread (includes a stack)
    LC_UNIXTHREAD = 0x5,

    /// load a specified fixed VM shared library
    LC_LOADFVMLIB = 0x6,

    /// fixed VM shared library identification
    LC_IDFVMLIB = 0x7,

    /// object identification info (obsolete)
    LC_IDENT = 0x8,

    /// fixed VM file inclusion (internal use)
    LC_FVMFILE = 0x9,

    /// prepage command (internal use)
    LC_PREPAGE = 0xa,

    /// dynamic link-edit symbol table info
    LC_DYSYMTAB = 0xb,

    /// load a dynamically linked shared library
    LC_LOAD_DYLIB = 0xc,

    /// dynamically linked shared lib ident
    LC_ID_DYLIB = 0xd,

    /// load a dynamic linker
    LC_LOAD_DYLINKER = 0xe,

    /// dynamic linker identification
    LC_ID_DYLINKER = 0xf,

    /// modules prebound for a dynamically linked shared library
    LC_PREBOUND_DYLIB = 0x10,

    /// image routines
    LC_ROUTINES = 0x11,

    /// sub framework
    LC_SUB_FRAMEWORK = 0x12,

    /// sub umbrella
    LC_SUB_UMBRELLA = 0x13,

    /// sub client
    LC_SUB_CLIENT = 0x14,

    /// sub library
    LC_SUB_LIBRARY = 0x15,

    /// two-level namespace lookup hints
    LC_TWOLEVEL_HINTS = 0x16,

    /// prebind checksum
    LC_PREBIND_CKSUM = 0x17,

    /// load a dynamically linked shared library that is allowed to be missing
    /// (all symbols are weak imported).
    LC_LOAD_WEAK_DYLIB = (0x18 | LC_REQ_DYLD),

    /// 64-bit segment of this file to be mapped
    LC_SEGMENT_64 = 0x19,

    /// 64-bit image routines
    LC_ROUTINES_64 = 0x1a,

    /// the uuid
    LC_UUID = 0x1b,

    /// runpath additions
    LC_RPATH = (0x1c | LC_REQ_DYLD),

    /// local of code signature
    LC_CODE_SIGNATURE = 0x1d,

    /// local of info to split segments
    LC_SEGMENT_SPLIT_INFO = 0x1e,

    /// load and re-export dylib
    LC_REEXPORT_DYLIB = (0x1f | LC_REQ_DYLD),

    /// delay load of dylib until first use
    LC_LAZY_LOAD_DYLIB = 0x20,

    /// encrypted segment information
    LC_ENCRYPTION_INFO = 0x21,

    /// compressed dyld information
    LC_DYLD_INFO = 0x22,

    /// compressed dyld information only
    LC_DYLD_INFO_ONLY = (0x22|LC_REQ_DYLD),

    /// load upward dylib
    LC_LOAD_UPWARD_DYLIB = (0x23 | LC_REQ_DYLD),

    /// build for MacOSX min OS version
    LC_VERSION_MIN_MACOSX = 0x24,

    /// build for iPhoneOS min OS version
    LC_VERSION_MIN_IPHONEOS = 0x25,

    /// compressed table of function start addresses
    LC_FUNCTION_STARTS = 0x26,

    /// string for dyld to treat like environment variable
    LC_DYLD_ENVIRONMENT = 0x27,

    /// replacement for LC_UNIXTHREAD
    LC_MAIN = (0x28|LC_REQ_DYLD),

    /// table of non-instructions in __text
    LC_DATA_IN_CODE = 0x29,

    /// source version used to build binary
    LC_SOURCE_VERSION = 0x2A,

    /// Code signing DRs copied from linked dylibs
    LC_DYLIB_CODE_SIGN_DRS = 0x2B,

    /// 64-bit encrypted segment information
    LC_ENCRYPTION_INFO_64 = 0x2C,

    /// linker options in MH_OBJECT files
    LC_LINKER_OPTION = 0x2D,

    /// optimization hints in MH_OBJECT files
    LC_LINKER_OPTIMIZATION_HINT = 0x2E,

    /// build for Watch min OS version
    LC_VERSION_MIN_WATCHOS = 0x30,
}
