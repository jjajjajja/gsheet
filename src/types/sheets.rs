use std::any::Any;
use std::collections::hash_map::HashMap;
pub struct Sheet {
    properties: SheetProperties,
    data: GridData,
    merges: Vec<GridRange>,
    conditional_formats: Vec<ConditionalFormatRules>,
    filter_views: Vec<FilterView>,
    protected_ranges: Vec<ProtectedRange>,
    basic_filter: BasicFilter,
    charts: Vec<EmbeddedChart>,
    banded_ranges: Vec<BandedRange>,
    developer_metadata: Vec<DeveloperMetadata>,
    row_groups: Vec<DimensionGroup>,
    column_groups: Vec<DimensionGroup>,
    slicers: Vec<Slicer>,
}

pub struct SheetProperties {
    sheed_id: i64,
    title: String,
    index: i64,
    sheet_type: SheetType,
    grid_properties: GridProperties,
    hidden: bool,
    #[deprecated]
    tab_color: Color,
    tab_color_style: ColorStyle,
    right_to_left: bool,
    datasourcesheet_properties: DataSourceSheetProperties,
}

pub enum SheetType {
    SHEET_TYPE_UNSPECIFIED,
    GRID,
    OBJECT,
    DATA_SOURCE,
}

pub struct GridProperties {
    row_count: i64,
    column_count: i64,
    frozen_row_count: i64,
    frozen_column_count: i64,
    hide_grid_lines: bool,
    row_group_control_after: bool,
    column_group_control_after: bool,
}

pub struct DataSourceSheetProperties {
    datasource_id: String,
    columns: Vec<DataSourceColumn>,
    data_execution_status: DataExecutionStatus,
}

pub struct GridData {
    start_row: i64,
    start_column: i64,
    row_data: Vec<RowData>,
    row_metadata: Vec<DimensionProperties>,
    column_metadata: Vec<DimensionProperties>,
}

pub struct RowData {
    values: Vec<CellData>,
}

pub struct FilterCriteria {
    hidden_values: Vec<String>,
    condition: BooleanCondition,
    #[deprecated]
    visible_background_color: Color,
    visible_background_color_style: ColorStyle,
    #[deprecated]
    visible_foreground_color: Color,
    visible_foreground_color_style: ColorStyle,
}

pub struct DimensionProperties {
    hidden_by_filter: bool,
    hidden_by_user: bool,
    pixel_size: i64,
    devloper_metadata: DeveloperMetadata,
    data_source_column_reference: DataSourceColumnReference,
}

pub struct ConditionalFormatRules<T> {
    ranges: Vec<GridRange>,
    // Union filed rule can be onlky one of the following
    rule: T,
}

// union field rule
pub struct BooleanRule {
    condition: BooleanCondition,
    format: CellFormat,
}
// union field rule
pub struct GradientRule {
    minpoint: InterpolationPoint,
    midpoint: InterpolationPoint,
    maxpoint: InterpolationPoint,
}

pub struct InterpolationPoint {
    #[deprecated]
    color: Color,
    color_style: ColorStyle,
    interpolation_point_type: InterpolationPointType,
    value: String,
}

pub enum InterpolationPointType {
    INTERPOLATION_POINT_TYPE_UNSPECIFIED,
    MIN,
    MAX,
    NUMBER,
    PERCENT,
    PERCENTILE,
}

pub struct FilterView {
    filter_view_id: i64,
    title: String,
    range: GridRange,
    named_range_id: String,
    sort_specs: Vec<SortSpec>,
    filter_specs: Vec<FilterSpec>,
    #[deprecated]
    criteria: HashMap<i64, Any>,
}

pub struct Editors {
    users: Vec<String>,
    groups: Vec<String>,
    domain_users_can_edit: bool,
}
pub struct BasicFilter {
    range: GridRange,
    sort_specs: Vec<SortSpec>,
    filter_specs: Vec<FilterSpec>,
    #[deprecated]
    criteria: HashMap<i64, FilterCriteria>,
}

pub struct BandedRange {
    banded_range_id: i64,
    range: GridRange,
    row_properties: BandingProperties,
    column_properties: BandingProperties,
}

pub struct BandingProperties {
    #[deprecated]
    header_color: Color,
    header_color_style: ColorStyle,
    first_band_color: Color,
    first_band_color_style: ColorStyle,
    #[deprecated]
    second_band_color: Color,
    second_band_color_style: ColorStyle,
    #[deprecated]
    footer_color: Color,
    footer_color_style: ColorStyle,
}

pub struct DimensionGroup {
    rnage: DimensionRange,
    depth: i64,
    collapsed: bool,
}

pub struct Slicer {
    slicer_id: i64,
    spec: SlicerSpec,
    position: EmbeddedObjectPosition,
}

pub struct SlicerSpec {
    data_range: GridRange,
    filter_criteria: FilterCriteria,
    column_index: i64,
    apply_to_pivot_tables: bool,
    title: String,
    text_format: TextFormat,
    #[deprecated]
    background_color: Color,
    background_color_style: ColorStyle,
    horizontal_alignment: HorizontalAlign,
}
