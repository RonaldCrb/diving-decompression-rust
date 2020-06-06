
/// A specific item in one of the rows of the US Navy air no-decompression table
#[derive(Serialize, Deserialize)]
pub struct Group {
  /// the repetitive group letter
  pub group_letter: String,
  /// the lower end of the timeframe for a specific row expressed in minutes
  pub min_time: u16,
  /// the higher end of the timeframe for a specific row expressed in minutes
  pub max_time: u16,
}

/// row in the air No Decompression table
#[derive(Serialize, Deserialize)]
pub struct RowNdl {
  /// the lower end of allowed depths for a specific
  /// row expressed in Feet of sea water
  pub min_fsw: u16,
  /// the higher end of allowed depths for a specific 
  /// row expressed in Feet of sea water
  pub max_fsw: u16,
  /// the row has unlimited no-decompression limit?
  /// (usually no deeper than 30 Feet of sea water)
  pub unlimited: bool,
  /// the no decompression limit expressed in minutes
  pub no_stop_limit: u16,
  /// a specific item in one of the rows of the US Navy air no-decompression table
  pub values: Vec::<Group>,
}

/// This is the main type for the entire US NAVY air No decompression table 
#[derive(Serialize, Deserialize)]
pub struct TableNdl {
  /// unique id for a table within the diving-decompression project.
  pub table_code: String,
  /// oficially recognized name for a table.
  pub table_name: String,
  /// table data
  pub table_data: Vec::<RowNdl>,
}

#[derive(Serialize, Deserialize)]
/// a row of the table for surface interval time and repetitive letter
pub struct RowRgl {
  /// group letter before the surface interval time
  pub group_letter: String,
  /// the lower end of allowed times for a specific row expressed in minutes
  pub min_time: u16,
  /// the higher end of allowed times for a specific row expressed in minutes  
  pub max_time: u16,
  /// group letter after the surface interval time
  pub repet_letter: String,
}

/// the table for surface interval time and repetitive letter
#[derive(Serialize, Deserialize)]
pub struct TableRgl {
  /// represents a unique id for a table within the 
  /// diving-decompression project.
  pub table_code: String,
  /// oficially recognized name for a table beyond the
  /// diving-decompression project.
  pub table_name: String,
  /// a row of the table for surface interval time and repetitive letter
  pub table_data: Vec::<RowRgl>,
}

#[derive(Serialize, Deserialize)]
/// item in the row of the table for residual nitrogen time
pub struct Rnt {
  /// the lower end of allowed depth for a specific RNT item
  /// expressed feet of sea water
  pub min_depth: u16,
  /// the higher end of allowed depth for a specific RNT item
  /// expressed feet of sea water
  pub max_depth: u16,
  /// the residual nitrogen time expressed in minutes
  pub rnt: u16,
}

#[derive(Serialize, Deserialize)]
/// a row in the table for residual nitrogen time
pub struct RowRnt {
  /// the repet group letter for the residual nitrogen time item
  pub repet_letter: String,
  /// item in the row of the table for residual nitrogen time
  pub rnt: Vec::<Rnt>,
}

#[derive(Serialize, Deserialize)]
/// the table for residual nitrogen time
pub struct TableRnt {
  /// represents a unique id for a table within the 
  /// diving-decompression project.
  pub table_code: String,
  /// oficially recognized name for a table beyond the
  /// diving-decompression project.
  pub table_name: String,
  /// a note displayed when the rnt exceeds the no 
  /// decompression limit and the profile has an unlimited
  /// no decompression limit in the no decompression table
  /// for shallower depths as per the US Navy dive manual
  pub table_note9981: String,
  /// a row in the table for residual nitrogen time
  pub table_data: Vec::<RowRnt>,
}

#[derive(Serialize, Deserialize)]
/// an item from a row of the table for air decompression
pub struct DecoStops {
  /// the nominal depth of a specific decompression stop
  /// expressed in feet of sea water
  pub depth: u16,
  /// the nominal time of a specific decompression stop
  /// expressed in minutes
  pub time: u16,
}

#[derive(Serialize, Deserialize)]
/// a row in a depth of the air decompression
pub struct RowDeco {
  /// the lower end of allowed times for a specific row expressed in minutes
  pub min_time: u16,
  /// the higher end of allowed times for a specific row expressed in minutes
  pub max_time: u16,
  /// total ascent time when using the Air based decompression protocol
  /// expressed in minutes
  pub air_tat: String,
  /// total ascent time when using in water decompression with Oxygen
  /// expressed in minutes
  pub o2_tat: String,
  /// the time from leave bottom to the first scheduled decompression stop
  pub ttfs: String,
  /// number of chamber periods when using the SurdO2 (surface decompression
  /// with oxygen)
  pub o2cp: f32,
  /// repetitive dive group letter after the decompression protocol
  pub repetgroup_letter: String,
  /// SurdO2 is recommended due to the extent of the decompression profile
  pub surdo2_recommended: bool,
  /// exceptional exposure dives are considered an anti-pattern and should
  /// only occur in extreme situations. planning a dive with exceptional
  /// exposure is an anti-pattern and a tremendous risk for divers health
  pub exceptional_exposure: bool,
  /// surdO2 is required due to the extent of the decompression profile
  pub surdo2_required: bool,
  /// the dive must use the SurdO2 protocol. planning these dives with for
  /// in water decompression is an anti-pattern and must be avoided
  pub strict_surdo2: bool,
  /// an air decompression stop
  pub air_deco_stops: Vec::<DecoStops>,
  /// an o2 decompression stop
  pub o2_deco_stops: Vec::<DecoStops>,
}

#[derive(Serialize, Deserialize)]
/// a depth in the air decompression table
pub struct DecoDepth {
  /// the lower end of allowed depth for a specific profile in the 
  /// air decompression tables expressed feet of sea water
  pub min_fsw: u16,
  /// the higher end of allowed depth for a specific profile in the 
  /// air decompression tables expressed feet of sea water
  pub max_fsw: u16,
  /// a row in a depth of the air decompression
  pub rows: Vec::<RowDeco>,
}

/// the air decompression table
#[derive(Serialize, Deserialize)]
pub struct TableAirDeco {
  /// represents a unique id for a table within the 
  /// diving-decompression project.
  pub table_code: String,
  /// oficially recognized name for a table beyond the
  /// diving-decompression project.
  pub table_name: String,
  /// table data  
  pub table_data: Vec::<DecoDepth>,
}

/// single dive plan object
#[derive(Serialize, Deserialize)]
pub struct DivePlan {
  /// the depth of the first planned dive expressed in feet of sea water
  pub depth: u16,
  /// the bottom time of the first planned dive expressed in minutes
  pub bottom_time: u16,
  /// the planned surface interval time expressed in minutes
  pub sit: u16,
  /// the depth of the next planned dive expressed in feet of sea water
  pub next_depth: u16,
}

/// single dive object
#[derive(Serialize, Deserialize)]
pub struct Dive {
  /// depth of the dive expressed in feet of sea water
  pub depth: u16, 
  /// bottom time of the dive expressed in minutes
  pub bottom_time: u16,
}