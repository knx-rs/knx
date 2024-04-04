use std::collections::HashMap;

use crate::xml::DataPointType;

pub struct Master {
    data_point_types: Vec<DataPointType>,
    manufacturers: HashMap<String, String>,
}
