use std::collections::HashMap;

pub struct PivotTable<T>
where
    T: SourceData,
{
    rows: Vec<PivotGroup>,
    columns: Vec<PivotGroup>,
    criteria: HashMap<i64, PivotFilterCriteria>,
    filter_specs: Vec<PivotFilterSpecs>,
    values: Vec<PivotValue>,
    value_layout: PivotValueLayout,
    data_execution_status: DataExecutionStatus,
    // union field source_data
    source_data: T,
}
