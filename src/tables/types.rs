
/// A specific item in one of the rows of the US Navy air no-decompression table
#[derive(Serialize, Deserialize)]
pub struct Group {
  pub group_letter: String,
  pub min_time: u16,
  pub max_time: u16,
}

/// row in the air No Decompression table
#[derive(Serialize, Deserialize)]
pub struct RowNdl {
  pub min_fsw: u16,
  pub max_fsw: u16,
  pub unlimited: bool,
  pub no_stop_limit: u16,
  pub values: Vec::<Group>,
  // pub notes: String,
}

/// This is the main type for the entire US NAVY air No decompression table 
#[derive(Serialize, Deserialize)]
pub struct TableNdl {
  pub table_code: String,
  pub table_name: String,
  pub table_data: Vec::<RowNdl>,
}

#[derive(Serialize, Deserialize)]
/// a row of the table for surface interval time and repetitive letter
pub struct RowRgl {
  pub group_letter: String,
  pub min_time: u16,
  pub max_time: u16,
  pub repet_letter: String,
}

/// the table for surface interval time and repetitive letter
#[derive(Serialize, Deserialize)]
pub struct TableRgl {
  pub table_code: String,
  pub table_name: String,
  pub table_data: Vec::<RowRgl>,
}

#[derive(Serialize, Deserialize)]
/// item in the row of the table for residual nitrogen time
pub struct Rnt {
  pub min_depth: u16,
  pub max_depth: u16,
  pub rnt: u16,
}

#[derive(Serialize, Deserialize)]
/// a row in the table for residual nitrogen time
pub struct RowRnt {
  pub repet_letter: String,
  pub rnt: Vec::<Rnt>,
}

#[derive(Serialize, Deserialize)]
/// the table for residual nitrogen time
pub struct TableRnt {
  pub table_code: String,
  pub table_name: String,
  pub table_note9981: String,
  pub table_data: Vec::<RowRnt>,
}

#[derive(Serialize, Deserialize)]
/// an item from a row of the table for air decompression
pub struct DecoStops {
  pub depth: u16,
  pub time: u16,
}

#[derive(Serialize, Deserialize)]
/// a row in a depth of the air decompression
pub struct RowDeco {
  pub min_time: u16,
  pub max_time: u16,
  pub air_tat: String,
  pub o2_tat: String,
  pub ttfs: String,
  pub o2cp: f32,
  pub repetgroup_letter: String,
  pub surdo2_recommended: bool,
  pub exceptional_exposure: bool,
  pub surdo2_required: bool,
  pub strict_surdo2: bool,
  pub air_deco_stops: Vec::<DecoStops>,
  pub o2_deco_stops: Vec::<DecoStops>,
}

#[derive(Serialize, Deserialize)]
/// a depth in the air decompression table
pub struct DecoDepth {
  pub min_fsw: u16,
  pub max_fsw: u16,
  pub rows: Vec::<RowDeco>,
}

/// the air decompression table
#[derive(Serialize, Deserialize)]
pub struct TableAirDeco {
  pub table_code: String,
  pub table_name: String,
  pub table_data: Vec::<DecoDepth>,
}

/// single dive plan object
#[derive(Serialize, Deserialize)]
pub struct DivePlan {
  pub depth: u16,
  pub bottom_time: u16,
  pub sit: u16,
  pub next_depth: u16,
}

/// single dive object
#[derive(Serialize, Deserialize)]
pub struct Dive {
  pub depth: u16,
  pub bottom_time: u16,
}