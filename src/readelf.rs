pub struct Elf<'a> {
    header: ElfHeader,
    rawdata: &'a[u8],
}

struct ElfHeader {
    e_ident:	    e_ident,
    e_type:	        &'static str,
    e_machine:	    &'static str,
    e_version:	    u32,
    e_entry:	    u64,	        // 32bitなら本来はu32だが、u64だと十分かつ64bit対応にできるのでこのまま
    e_phoff:	    u64,	        // 同上
    e_shoff:	    u64,	        // 同上
    e_flags:	    u32,
    e_ehsize:	    u16,
    e_phentsize:	u16,
    e_phnum:	    u16,
    e_shentsize:	u16,
    e_shnum:	    u16,
    e_shstrndx:	    u16
}

#[allow(non_camel_case_types)]
struct e_ident {
    class:	    &'static str,
    endian:	    &'static str,
    version:	u8,
    osabi:	    &'static str,
    abiver:	    u8,
}

const ELF_MAGIC: [u8; 4] = [0x7f, 0x45, 0x4c, 0x46];

impl Elf <'_>{
    pub fn parse_elf_header(data: &[u8]) -> Result<Elf, &'static str> {

        // elf check
        if data[0..=3] != ELF_MAGIC {
            return Err("Not ELF format file");
        }

        let class = match data[4] {
            1 => "ELF32",
            2 => "ELF64",
            _ => return Err("Invalid ELF Class"),
        };


        let endian = match data[5] {
            1 => "Little",
            2 => "Big",
            _ => return Err("Invalid Byte Order"),
        };

        let version = match data[6] {
            1 => 1,
            _ => return Err("Invalid ELF Version"),
        };

        // too many, no validation
        let osabi = match data[7] {
            0 => "UNIX System V ABI", 
            3 => "Object uses GNU ELF extensions",
            64 => "ARM EABI",
            97 => "ARM",
            255 => "Standalone (embedded) application",
            _ => "Unknwon or Invalid ABI",
        };

        let abiver = data[8];

        let e_type = match byte_to_num(endian, &data[16..=17]) {
            1 => "REL (Relocatable file)",
            2 => "EXEC (Executable file)",
            3 => "DYN (Shared object file)",
            4 => "CORE (Core file)",
            0xfe00..=0xfeff => "OS-specific file type",
            0xff00..=0xffff => "Processor-specific file type",
            _ => return Err("Invalid Object File Type"),
        };

        // too many, no validation
        let e_machine = match byte_to_num(endian, &data[18..=19]) {
            3 => "Intel 80380",
            40 => "ARM",
            62 => "AMD X86-64",
            183 => "AArch64 (ARM64)",
            243 => "RISC-V",
            _ => "Unknown or Invalid Architecture"
        };

        let e_version = match byte_to_num(endian, &data[20..=23]) {
            1 => 1u32,
            _ => return Err("Invalid ELF Version"),
        };

        let e_ident = e_ident {
            class,
            endian,
            version,
            osabi,
            abiver,
        };

        Ok(
            if class == "ELF64" {
                Elf{
                    header: ElfHeader {
                        e_ident,
                        e_type,
                        e_machine,
                        e_version,
                        e_entry: byte_to_num(endian, &data[24..=31]),
                        e_phoff: byte_to_num(endian, &data[32..=39]),
                        e_shoff: byte_to_num(endian, &data[40..=47]),
                        e_flags: byte_to_num(endian, &data[48..=51]) as u32,
                        e_ehsize: byte_to_num(endian, &data[52..=53]) as u16,
                        e_phentsize: byte_to_num(endian, &data[54..=55]) as u16,
                        e_phnum: byte_to_num(endian, &data[56..=57]) as u16,
                        e_shentsize: byte_to_num(endian, &data[58..=59]) as u16,
                        e_shnum: byte_to_num(endian, &data[60..=61]) as u16,
                        e_shstrndx: byte_to_num(endian, &data[62..=63]) as u16,
                    },
                    rawdata: data,
                }

            // return ELF32
            } else {
                Elf{
                    header: ElfHeader {
                        e_ident,
                        e_type,
                        e_machine,
                        e_version,
                        e_entry: byte_to_num(endian, &data[24..=27]),
                        e_phoff: byte_to_num(endian, &data[28..=31]),
                        e_shoff: byte_to_num(endian, &data[32..=35]),
                        e_flags: byte_to_num(endian, &data[36..=39]) as u32,
                        e_ehsize: byte_to_num(endian, &data[40..=41]) as u16,
                        e_phentsize: byte_to_num(endian, &data[42..=43]) as u16,
                        e_phnum: byte_to_num(endian, &data[44..=45]) as u16,
                        e_shentsize: byte_to_num(endian, &data[46..=47]) as u16,
                        e_shnum: byte_to_num(endian, &data[48..=49]) as u16,
                        e_shstrndx: byte_to_num(endian, &data[50..=51]) as u16,
                    },
                    rawdata: data,
                }
            }
        )
    }

    pub fn print_elf_header(&self) {
        println!("ELF Header:");
        println!("  {:<35} {}", "Class:", self.header.e_ident.class);
        println!("  {:<35} 2's complement, {} endian", "Data Encoding:", self.header.e_ident.endian);
        println!("  {:<35} {}", "ELF Version:", self.header.e_ident.version);
        println!("  {:<35} {}", "OS/ABI:", self.header.e_ident.osabi);
        println!("  {:<35} {}", "ABI Version:", self.header.e_ident.abiver);
        println!("  {:<35} {}", "Object file type:", self.header.e_type);
        println!("  {:<35} {}", "Architecture:", self.header.e_machine);
        println!("  {:<35} 0x{}", "ELF Version:", self.header.e_version);
        println!("  {:<35} {:#x}", "Entry Point Address:", self.header.e_entry);
        println!("  {:<35} {} (bytes)", "Program header table offset:", self.header.e_phoff);
        println!("  {:<35} {} (bytes)", "Section header table offset:", self.header.e_shoff);
        println!("  {:<35} {:#x}", "Flags:", self.header.e_flags);
        println!("  {:<35} {} (bytes)", "Size of this header:", self.header.e_ehsize);
        println!("  {:<35} {} (bytes)", "Size of program headers:", self.header.e_phentsize);
        println!("  {:<35} {}", "Number of program headers:", self.header.e_phnum);
        println!("  {:<35} {} (bytes)", "Size of section headers:", self.header.e_shentsize);
        println!("  {:<35} {}", "Number of section headers:", self.header.e_shnum);
        println!("  {:<35} {}", "Section header string table index:", self.header.e_shstrndx);
    }
}

fn byte_to_num(endian: &str, data: &[u8]) -> u64 {
    let mut result = 0u64;
    if endian == "Little" {
        for (i, &byte) in data.iter().enumerate() {
            result += (byte as u64) << (8*i);
        }
    } else if endian == "Big" {
        todo!();
    }
    result
}
