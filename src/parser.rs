use binrw::{io::Cursor, until_eof, BinRead, BinReaderExt, PosValue};

#[derive(Debug, BinRead, PartialEq)]
pub enum DatabaseVersion {
    #[br(magic = 0u8)]
    V3,
    #[br(magic = 1u8)]
    V4,
    #[br(magic = 2u8)]
    V5,
    #[br(magic = 3u8)]
    Access2010,
    #[br(magic = 4u8)]
    Access2013,
    #[br(magic = 5u8)]
    Access2016,
    #[br(magic = 6u8)]
    Access2019,
}

#[derive(Debug, BinRead)]
pub struct DatabaseDefinition {
    #[br(magic = 0u8)]
    page_type: u8,
    #[br(pad_before = 0x12)]
    pub version: DatabaseVersion,
    #[br(count = 128)]
    pub rc4_key: Vec<u8>,
    #[br(pad_before = 0x29)]
    pub key: u32,
}

#[derive(Debug, BinRead, PartialEq)]
pub struct Data {
    #[br(magic = 1u8)]
    page_type: u8,
    #[br(pad_before = 1)]
    free_space: u16,
    #[br(pad_after = 4)]
    table_def_page: u32,
    num_rows: u16,
}

#[derive(Debug, BinRead, PartialEq)]
pub struct TableDefinition {
    #[br(magic = 2u8)]
    page_type: u8,
    // Header
    #[br(pad_before = 1)]
    table_def_id: u16,
    next_page: u32,
    // Jet4 Block
    length: u32,
    #[br(pad_before = 4)]
    num_rows: u32,
    auto_number: u32,
    auto_number_flag: u8,
    #[br(pad_before = 3)]
    complex_auto_number: u32,
    #[br(pad_before = 8, try)]
    table_type: Option<TableType>,
    max_columns: u16,
    number_variable_columns: u16,
    num_columns: u16,
    num_idx: u32,
    num_real_idx: u32,
    used_pages: u32,
    free_pages: u32,
    // #[br(count = num_real_idx)]
    // real_idxs: Vec<RealIndexDef>,
    // #[br(count = num_columns)]
    // column_definitions: Vec<ColumnDefinition>,
    // #[br(count = num_columns)]
    // column_names: Vec<ColumnNames>,
}

#[derive(Debug, BinRead, PartialEq)]
#[br(repr = u8)]
pub enum TableType {
    User = 0x4e,
    System = 0x53,
}

#[derive(Debug, BinRead, PartialEq)]
pub struct RealIndexDef {
    #[br(pad_before = 4, pad_after = 4)]
    num_idx_rows: u32,
}

#[derive(Debug, BinRead, PartialEq)]
pub struct ColumnDefinition {
    column_type: u8,
    unknown: u32,
    col_number_inc_deleted: u16,
    offset_v: u16,
    col_number: u16,
    misc: u16,
    misc_ext: u16,
    bitmask: u8,
    misc_flags: u8,
    #[br(pad_before = 4)]
    offset_f: u16,
    col_len: u16,
}

#[derive(Debug, BinRead, PartialEq)]
pub struct ColumnNames {
    name_len: u16,
    #[br(count = name_len / 2)]
    name: Vec<u16>,
}

#[derive(Debug, BinRead, PartialEq)]
pub struct IntermediateIndex {
    #[br(magic = 3u8)]
    page_type: u8,
}

#[derive(Debug, BinRead, PartialEq)]
pub struct LeafIndex {
    #[br(magic = 4u8)]
    page_type: u8,
}

#[derive(Debug, BinRead, PartialEq)]
pub struct PageUseBitMaps {
    #[br(magic = 5u8)]
    page_type: u8,
}

#[derive(Debug, BinRead, PartialEq)]
pub struct Unknown {
    #[br(magic = 9u8)]
    page_type: u8,
}

#[derive(Debug, BinRead)]
#[br(import(page_size: u32))]
pub enum Page {
    DatabaseDefinition(#[br(pad_size_to = page_size)] DatabaseDefinition),
    Data(#[br(pad_size_to = page_size)] Data),
    TableDefinition(#[br(pad_size_to = page_size)] TableDefinition),
    IntermediateIndex(#[br(pad_size_to = page_size)] IntermediateIndex),
    LeafIndex(#[br(pad_size_to = page_size)] LeafIndex),
    PageUseBitMaps(#[br(pad_size_to = page_size)] PageUseBitMaps),
    Unknown(#[br(pad_size_to = page_size)] Unknown),
}

#[derive(Debug, BinRead)]
#[br(import(page_size: u32))]
pub struct Database {
    #[br(parse_with = until_eof, args(page_size))]
    pub pages: Vec<PosValue<Page>>,
}

pub fn parse_access_file(input: &[u8]) -> Database {
    let mut reader = Cursor::new(input);
    reader.read_le_args((4096,)).unwrap()
}
